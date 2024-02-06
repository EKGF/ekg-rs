#![cfg(feature = "_rdfox")]

use {
    crate::rdfox::{DataStore, DataStoreConnection, RoleCreds, Server},
    ekg_namespace::consts::LOG_TARGET_DATABASE,
    std::{
        ffi::{CStr, CString},
        ptr,
        sync::Arc,
    },
};

/// A connection to a given [`Server`].
#[derive(Debug)]
pub struct ServerConnection {
    #[allow(dead_code)]
    role_creds: RoleCreds,
    server:     Arc<Server>,
    inner:      *mut rdfox_sys::CServerConnection,
}

unsafe impl Sync for ServerConnection {}

unsafe impl Send for ServerConnection {}

impl Drop for ServerConnection {
    fn drop(&mut self) {
        assert!(
            !self.inner.is_null(),
            "Could not drop ServerConnection"
        );
        unsafe {
            rdfox_sys::CServerConnection_destroy(self.inner);
        }
        self.inner = ptr::null_mut();
        tracing::debug!(target: LOG_TARGET_DATABASE, "Dropped {self:}");
    }
}

impl std::fmt::Display for ServerConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "connection to {:}", self.server)
    }
}

impl ServerConnection {
    pub(crate) fn new(
        role_creds: RoleCreds,
        server: Arc<Server>,
        server_connection_ptr: *mut rdfox_sys::CServerConnection,
    ) -> Self {
        assert!(!server_connection_ptr.is_null());
        assert!(
            server.is_running(),
            "cannot connect to an RDFox server that is not running"
        );
        let connection = Self { role_creds, server, inner: server_connection_ptr };
        tracing::debug!(
            target: LOG_TARGET_DATABASE,
            "Established {connection:}"
        );
        connection
    }

    /// Return the version number of the underlying database engine
    ///
    /// CRDFOX const CException*
    /// CServerConnection_getVersion(
    ///     CServerConnection* serverConnection,
    ///     const char** version
    /// );
    pub fn get_version(&self) -> Result<String, ekg_error::Error> {
        let mut c_buf: *const std::os::raw::c_char = ptr::null();
        rdfox_sys::database_call!(
            "Getting the version",
            rdfox_sys::CServerConnection_getVersion(self.inner, &mut c_buf)
        )?;
        let c_version = unsafe { CStr::from_ptr(c_buf) };
        Ok(c_version.to_str().unwrap().to_owned())
    }

    pub fn get_number_of_threads(&self) -> Result<u32, ekg_error::Error> {
        let mut number_of_threads = 0_usize;
        rdfox_sys::database_call!(
            format!(
                "Getting the number of server-threads via {}",
                self
            )
            .as_str(),
            rdfox_sys::CServerConnection_getNumberOfThreads(self.inner, &mut number_of_threads)
        )?;
        tracing::trace!(
            target: LOG_TARGET_DATABASE,
            "Got number of threads: {}",
            number_of_threads
        );
        Ok(number_of_threads as u32)
    }

    pub fn set_number_of_threads(&self, number_of_threads: usize) -> Result<(), ekg_error::Error> {
        assert!(!self.inner.is_null());
        let msg = format!(
            "Setting the number of threads to {}",
            number_of_threads
        );
        Ok(rdfox_sys::database_call!(
            msg.as_str(),
            rdfox_sys::CServerConnection_setNumberOfThreads(self.inner, number_of_threads)
        )?)
    }

    pub fn get_memory_use(&self) -> Result<(usize, usize), ekg_error::Error> {
        let mut max_used_bytes = 0_usize;
        let mut available_bytes = 0_usize;
        rdfox_sys::database_call!(rdfox_sys::CServerConnection_getMemoryUse(
            self.inner,
            &mut max_used_bytes,
            &mut available_bytes
        ))?;
        Ok((max_used_bytes, available_bytes))
    }

    pub fn delete_data_store(&self, data_store: &DataStore) -> Result<(), ekg_error::Error> {
        assert!(!self.inner.is_null());
        let msg = format!("Deleting {data_store}");
        let c_name = CString::new(data_store.name.as_str()).unwrap();
        Ok(rdfox_sys::database_call!(
            msg.as_str(),
            rdfox_sys::CServerConnection_deleteDataStore(self.inner, c_name.as_ptr())
        )?)
    }

    pub fn create_data_store(&self, data_store: &DataStore) -> Result<(), ekg_error::Error> {
        tracing::trace!(
            target: LOG_TARGET_DATABASE,
            "Creating {data_store:}"
        );
        assert!(!self.inner.is_null());
        let c_name = CString::new(data_store.name.as_str()).unwrap();
        let c_params = data_store.parameters.inner.lock().unwrap(); // TODO: handle exception
        rdfox_sys::database_call!(
            "creating a datastore",
            rdfox_sys::CServerConnection_createDataStore(
                self.inner,
                c_name.as_ptr(),
                c_params.cast_const(),
            )
        )?;
        tracing::debug!(
            target: LOG_TARGET_DATABASE,
            "Created {data_store:}"
        );
        Ok(())
    }

    pub fn connect_to_data_store(
        self: &Arc<Self>,
        data_store: &Arc<DataStore>,
    ) -> Result<Arc<DataStoreConnection>, ekg_error::Error> {
        tracing::debug!(
            target: LOG_TARGET_DATABASE,
            "Connecting to {}",
            data_store
        );
        assert!(!self.inner.is_null());
        let mut ds_connection = DataStoreConnection::new(self, data_store, ptr::null_mut());
        let c_name = CString::new(data_store.name.as_str()).unwrap();
        tracing::debug!(
            target: LOG_TARGET_DATABASE,
            conn = ds_connection.number,
            "Creating datastore"
        );
        rdfox_sys::database_call!(
            "Creating datastore connection",
            rdfox_sys::CServerConnection_newDataStoreConnection(
                self.inner,
                c_name.as_ptr(),
                &mut ds_connection.inner,
            )
        )?;
        tracing::info!(
            target: LOG_TARGET_DATABASE,
            conn = ds_connection.number,
            "Connection to {}",
            data_store,
        );
        Ok(Arc::new(ds_connection))
    }
}
