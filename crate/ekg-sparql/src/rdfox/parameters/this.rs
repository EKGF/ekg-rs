use {
    crate::{
        persistence_mode::PersistenceMode,
        rdfox::{
            self,
            parameters::{
                builder::ParametersBuilder,
                handle::CParametersHandle,
                SENSITIVE_PARAMETERS,
            },
            DataStoreType,
        },
        FactDomain,
    },
    std::{
        ffi::{CStr, CString},
        fmt::{Display, Formatter},
        os::raw::c_char,
        path::Path,
        sync::{Arc, Mutex},
    },
};

#[derive(Debug, Clone)]
pub struct Parameters {
    pub(crate) inner: Arc<Mutex<CParametersHandle>>,
}

unsafe impl Sync for Parameters {}

unsafe impl Send for Parameters {}

impl Eq for Parameters {}

impl PartialEq for Parameters {
    fn eq(&self, _other: &Self) -> bool { Arc::ptr_eq(&self.inner, &_other.inner) }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parameters[]") // TODO: show keys and values (currently not
        // possible)
    }
}

impl Parameters {
    pub fn builder() -> ParametersBuilder { ParametersBuilder::default_builder() }

    pub fn empty() -> Result<Self, ekg_error::Error> {
        Ok(Parameters {
            inner: Arc::new(Mutex::new(CParametersHandle::new()?)),
        })
    }

    pub fn set_string(&mut self, key: &str, value: &str) -> Result<(), ekg_error::Error> {
        let c_key = CString::new(key).unwrap();
        let c_value = CString::new(value).unwrap();
        let msg = if SENSITIVE_PARAMETERS.contains(&c_key.to_str().unwrap()) {
            format!(
                "Setting parameter {}=[***]",
                c_key.to_str().unwrap(),
            )
        } else {
            format!(
                "Setting parameter {}=[{}]",
                c_key.to_str().unwrap(),
                c_value.to_str().unwrap()
            )
        };
        let mut c_ptr = self.inner.lock().unwrap();
        Ok(rdfox_sys::database_call!(
            msg.as_str(),
            rdfox_sys::CParameters_setString(c_ptr.cast_mut(), c_key.as_ptr(), c_value.as_ptr())
        )?)
    }

    pub fn get_string(&self, key: &str, default: &str) -> Result<String, ekg_error::Error> {
        let c_key = CString::new(key).unwrap();
        let c_default = CString::new(default).unwrap();
        let mut c_value: *const c_char = std::ptr::null();
        let msg = format!(
            "Getting parameter {} with default value {}",
            c_key.to_str().unwrap(),
            c_default.to_str().unwrap()
        );
        let c_params = self.inner.lock().unwrap(); // TODO: handle exception
        rdfox_sys::database_call!(
            msg.as_str(),
            rdfox_sys::CParameters_getString(
                c_params.cast_const(),
                c_key.as_ptr(),
                c_default.as_ptr(),
                &mut c_value as *mut *const c_char
            )
        )?;
        let c_version = unsafe { CStr::from_ptr(c_value) };
        Ok(c_version.to_str().unwrap().to_owned())
    }

    pub fn fact_domain(&mut self, fact_domain: &FactDomain) -> Result<&mut Self, ekg_error::Error> {
        match fact_domain {
            FactDomain::ASSERTED => self.set_string("fact-domain", "explicit")?,
            FactDomain::INFERRED => self.set_string("fact-domain", "derived")?,
            FactDomain::ALL => self.set_string("fact-domain", "all")?,
        };
        Ok(self)
    }

    pub fn switch_off_file_access_sandboxing(&mut self) -> Result<&mut Self, ekg_error::Error> {
        self.set_string("sandbox-directory", "")?;
        Ok(self)
    }

    pub fn persist_datastore(
        &mut self,
        mode: &PersistenceMode,
    ) -> Result<&mut Self, ekg_error::Error> {
        #[cfg(feature = "rdfox-7-0a")]
        match mode {
            PersistenceMode::File => self.set_string("persistence", "file")?,
            PersistenceMode::FileSequence => self.set_string("persistence", "file-sequence")?,
            PersistenceMode::Off => self.set_string("persistence", "off")?,
        };
        #[cfg(not(feature = "rdfox-7-0a"))]
        match mode {
            PersistenceMode::File => self.set_string("persist-ds", "file")?,
            PersistenceMode::FileSequence => self.set_string("persist-ds", "file-sequence")?,
            PersistenceMode::Off => self.set_string("persist-ds", "off")?,
        };
        Ok(self)
    }

    #[cfg(not(feature = "rdfox-7-0a"))]
    pub fn persist_roles(self, mode: PersistenceMode) -> Result<Self, ekg_error::Error> {
        self.set_string("persist-roles", &mode.to_string())?;
        Ok(self)
    }

    pub fn server_directory(&mut self, dir: &Path) -> Result<&mut Self, ekg_error::Error> {
        if dir.is_dir() {
            self.set_string("server-directory", dir.to_str().unwrap())?;
            Ok(self)
        } else {
            panic!("{dir:?} is not a directory")
        }
    }

    pub fn license_file(&mut self, file: &Path) -> Result<&mut Self, ekg_error::Error> {
        if file.is_file() {
            self.set_string("license-file", file.to_str().unwrap())?;
            Ok(self)
        } else {
            panic!("{file:?} does not exist")
        }
    }

    pub fn license_content(&mut self, content: &str) -> Result<&mut Self, ekg_error::Error> {
        // Content that comes in via an environment variable can have literal `\\n`
        // strings in them that should be replaced by actual line-feeds
        let content = content.replace("\r\n", "\n").replace("\\n", "\n");
        // Add a line feed at the end of the content just to make sure, if it's
        // missing the last field in the license key will not be recognized
        self.set_string("license-content", format!("{content}\n").as_str())?;
        Ok(self)
    }

    pub fn set_license(
        &mut self,
        database_dir: Option<&Path>,
    ) -> Result<&mut Self, ekg_error::Error> {
        match rdfox::find_license(database_dir)? {
            (Some(license_file_name), None) => {
                return self.license_file(license_file_name.as_path());
            },
            (None, Some(license_content)) => return self.license_content(license_content.as_str()),
            _ => {},
        };
        Ok(self)
    }

    pub fn import_rename_user_blank_nodes(
        &mut self,
        setting: bool,
    ) -> Result<&mut Self, ekg_error::Error> {
        self.set_string(
            "import.rename-user-blank-nodes",
            format!("{setting:?}").as_str(),
        )?;
        Ok(self)
    }

    /// If true, all API calls are recorded in a script that
    /// the shell can replay later. later.
    /// The default value is false.
    pub fn api_log(&mut self, on: bool) -> Result<&mut Self, ekg_error::Error> {
        if on {
            self.set_string("api-log", "on")?;
        } else {
            self.set_string("api-log", "off")?;
        }
        Ok(self)
    }

    /// Specifies the directory into which API logs will be written.
    /// Default is directory api-log within the configured server directory.
    pub fn api_log_directory(&mut self, dir: &Path) -> Result<&mut Self, ekg_error::Error> {
        if dir.exists() {
            let x = self.api_log(true)?;
            x.set_string("api-log.directory", dir.to_str().unwrap())?;
            Ok(x)
        } else {
            tracing::error!(
                "Could not enable logging since directory does not exist: {}",
                dir.to_str().unwrap()
            );
            Ok(self)
        }
    }

    pub fn data_store_type(
        &mut self,
        data_store_type: DataStoreType,
    ) -> Result<&mut Self, ekg_error::Error> {
        match data_store_type {
            DataStoreType::ParallelNN => self.set_string("type", "parallel-nn")?,
            DataStoreType::ParallelNW => self.set_string("type", "parallel-nw")?,
            DataStoreType::ParallelWW => self.set_string("type", "parallel-ww")?,
        }
        Ok(self)
    }
}
