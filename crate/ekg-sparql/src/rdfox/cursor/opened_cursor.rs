#![cfg(feature = "_rdfox")]

use {
    crate::rdfox::{Cursor, Transaction},
    ekg_metadata::consts::LOG_TARGET_DATABASE,
    std::{ptr, sync::Arc},
};

#[derive(Debug)]
pub struct OpenedCursor<'a> {
    pub tx:     Arc<Transaction>,
    pub cursor: &'a Cursor,
    /// the arity (i.e., the number of columns) of the answers that the
    /// cursor computes.
    pub arity:  usize,
}

impl<'a> OpenedCursor<'a> {
    /// Open the cursor, get the details like arity and argument info and
    /// return it as a tuple with all the details (except multiplicity)
    /// as an `OpenedCursor` and the multiplicity of the first row.
    pub(crate) fn new(
        cursor: &'a mut Cursor,
        tx: Arc<Transaction>,
    ) -> Result<(Self, usize), ekg_error::Error> {
        let c_cursor = cursor.inner;
        let multiplicity = Self::open(cursor.inner)?;
        let arity = Self::arity(c_cursor)?;
        let opened_cursor = OpenedCursor { tx, cursor, arity };
        Ok((opened_cursor, multiplicity))
    }

    fn open(c_cursor: *mut rdfox_sys::CCursor) -> Result<usize, ekg_error::Error> {
        let skip_to_offset = 0_usize;
        let mut multiplicity = 0_usize;
        rdfox_sys::database_call!(
            "Opening a cursor",
            rdfox_sys::CCursor_open(c_cursor, skip_to_offset, &mut multiplicity)
        )?;
        tracing::debug!(
            target: LOG_TARGET_DATABASE,
            "Opened cursor with multiplicity {multiplicity}"
        );
        Ok(multiplicity)
    }

    /// Returns the arity (i.e., the number of columns) of the answers that the
    /// cursor computes.
    fn arity(c_cursor: *mut rdfox_sys::CCursor) -> Result<usize, ekg_error::Error> {
        let mut arity = 0_usize;
        rdfox_sys::database_call!(
            "Getting the arity",
            rdfox_sys::CCursor_getArity(c_cursor, &mut arity)
        )?;
        Ok(arity)
    }

    /// TODO: Check why this panics when called after previous call returned
    /// zero
    pub fn advance(&mut self) -> Result<usize, ekg_error::Error> {
        let mut multiplicity = 0_usize;
        rdfox_sys::database_call!(
            format!("Advancing cursor {:?}", self.cursor.inner).as_str(),
            rdfox_sys::CCursor_advance(self.cursor.inner, &mut multiplicity)
        )?;
        tracing::trace!(
            target: LOG_TARGET_DATABASE,
            "Cursor {:?} advanced, multiplicity={multiplicity}",
            self.cursor.inner
        );
        Ok(multiplicity)
    }

    pub fn update_and_commit<T, U>(&mut self, f: T) -> Result<U, ekg_error::Error>
    where T: FnOnce(&mut OpenedCursor) -> Result<U, ekg_error::Error> {
        Transaction::begin_read_write(&self.cursor.connection)?.update_and_commit(|_tx| f(self))
    }

    pub fn execute_and_rollback<T, U>(&mut self, f: T) -> Result<U, ekg_error::Error>
    where T: FnOnce(&mut OpenedCursor) -> Result<U, ekg_error::Error> {
        Transaction::begin_read_only(&self.cursor.connection)?.execute_and_rollback(|_tx| f(self))
    }

    /// Get the variable name used in the executed SPARQL statement representing
    /// the given column in the output.
    pub fn get_answer_variable_name(&self, index: usize) -> Result<String, ekg_error::Error> {
        let mut c_buf: *const std::os::raw::c_char = ptr::null();
        rdfox_sys::database_call!(
            "getting a variable name",
            rdfox_sys::CCursor_getAnswerVariableName(self.cursor.inner, index, &mut c_buf)
        )?;
        let c_name = unsafe { std::ffi::CStr::from_ptr(c_buf) };
        Ok(c_name.to_str().unwrap().to_owned())
    }
}
