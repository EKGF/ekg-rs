#![cfg(feature = "_rdfox")]

use {
    super::{CursorRow, OpenedCursor},
    crate::{
        rdfox::{DataStoreConnection, Transaction},
        statement::Statement,
        Parameters,
    },
    ekg_util::log::LOG_TARGET_DATABASE,
    std::{ffi::CString, fmt::Debug, ptr, sync::Arc},
};

/// A Cursor handles a query result.
///
/// [RDFox documentation](https://docs.oxfordsemantic.tech/apis.html#cursors)
#[derive(Debug)]
pub struct Cursor {
    pub inner:             *mut rdfox_sys::CCursor,
    pub(crate) connection: Arc<DataStoreConnection>,
    statement:             Statement,
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe {
            if !self.inner.is_null() {
                // let msg = format!("{:?}", self.inner);
                rdfox_sys::CCursor_destroy(self.inner);
                self.inner = ptr::null_mut();
                // tracing::debug!(target: LOG_TARGET_DATABASE, "Dropped cursor
                // {msg}");
            }
        }
    }
}

impl Cursor {
    // noinspection DuplicatedCode
    pub fn create(
        connection: &Arc<DataStoreConnection>,
        parameters: Parameters,
        statement: &Statement,
    ) -> Result<Self, ekg_error::Error> {
        assert!(!connection.inner.is_null());
        let mut c_cursor: *mut rdfox_sys::CCursor = ptr::null_mut();
        let c_query = CString::new(statement.text.as_str()).unwrap();
        let c_query_len = c_query.as_bytes().len();
        tracing::trace!(
            target: LOG_TARGET_DATABASE,
            sparql = ?c_query,
            "Starting a cursor"
        );
        let fixed_params = statement.complete_parameters(parameters)?;
        let parameters = fixed_params.inner.lock().unwrap();
        rdfox_sys::database_call!(
            "Starting a cursor",
            rdfox_sys::CDataStoreConnection_createCursor(
                connection.inner,
                c_query.as_ptr(),
                c_query_len,
                parameters.cast_const(),
                &mut c_cursor,
            )
        )?;
        let cursor = Cursor {
            inner:      c_cursor,
            connection: connection.clone(),
            statement:  statement.clone(),
        };
        tracing::debug!(
            target: LOG_TARGET_DATABASE,
            "Created cursor for SPARQL statement:\n{}",
            &cursor.statement
        );
        Ok(cursor)
    }

    pub fn sparql_string(&self) -> &str { self.statement.text.as_str() }

    pub fn count(&mut self, tx: &Arc<Transaction>) -> Result<usize, ekg_error::Error> {
        self.consume(tx, 1000000000, |_row| Ok(()))
    }

    #[tracing::instrument(
    target = "database",
    skip_all,
    fields(
    max.row = max_row,
    conn = % self.connection.number,
    )
    )]
    pub fn consume<T, E>(
        &mut self,
        tx: &Arc<Transaction>,
        max_row: usize,
        mut f: T,
    ) -> Result<usize, E>
    where
        T: FnMut(&CursorRow) -> Result<(), E>,
        E: From<ekg_error::Error> + Debug,
    {
        let sparql_str = self.statement.text.clone();
        let (mut opened_cursor, mut multiplicity) = OpenedCursor::new(self, tx.clone())?;
        let mut rowid = 0_usize;
        let mut count = 0_usize;
        while multiplicity > 0_usize {
            if multiplicity >= max_row {
                return Err(
                    ekg_error::Error::MultiplicityExceededMaximumNumberOfRows {
                        maxrow: max_row,
                        multiplicity,
                        query: sparql_str,
                    }
                    .into(),
                );
            }
            rowid += 1;
            if rowid >= max_row {
                return Err(ekg_error::Error::ExceededMaximumNumberOfRows {
                    maxrow: max_row,
                    query:  sparql_str,
                }
                .into());
            }
            count += multiplicity;
            let row = CursorRow {
                opened:       &opened_cursor,
                multiplicity: &multiplicity,
                count:        &count,
                rowid:        &rowid,
            };
            if let Err(err) = f(&row) {
                tracing::error!(target: LOG_TARGET_DATABASE, "Error while consuming row: {:?}", err);
                Err(err)?;
            }
            multiplicity = opened_cursor.advance()?;
        }
        Ok(count)
    }

    pub fn update_and_commit<T, U>(
        &mut self,
        maxrow: usize,
        f: T,
    ) -> Result<usize, ekg_error::Error>
    where
        T: FnMut(&CursorRow) -> Result<(), ekg_error::Error>,
    {
        let tx = Transaction::begin_read_write(&self.connection)?;
        self.update_and_commit_in_transaction(tx, maxrow, f)
    }

    pub fn execute_and_rollback<T>(
        &mut self,
        maxrow: usize,
        f: T,
    ) -> Result<usize, ekg_error::Error>
    where
        T: FnMut(&CursorRow) -> Result<(), ekg_error::Error>,
    {
        let tx = Transaction::begin_read_only(&self.connection)?;
        self.execute_and_rollback_in_transaction(&tx, maxrow, f)
    }

    pub fn execute_and_rollback_in_transaction<T>(
        &mut self,
        tx: &Arc<Transaction>,
        maxrow: usize,
        f: T,
    ) -> Result<usize, ekg_error::Error>
    where
        T: FnMut(&CursorRow) -> Result<(), ekg_error::Error>,
    {
        tx.execute_and_rollback(|ref tx| self.consume(tx, maxrow, f))
    }

    pub fn update_and_commit_in_transaction<T>(
        &mut self,
        tx: Arc<Transaction>,
        maxrow: usize,
        f: T,
    ) -> Result<usize, ekg_error::Error>
    where
        T: FnMut(&CursorRow) -> Result<(), ekg_error::Error>,
    {
        tx.update_and_commit(|ref tx| self.consume(tx, maxrow, f))
    }
}
