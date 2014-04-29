/*
** Copyright (c) 2011, Brian Smith <brian@linuxfood.net>
** All rights reserved.
**
** Redistribution and use in source and binary forms, with or without
** modification, are permitted provided that the following conditions are met:
**
**   * Redistributions of source code must retain the above copyright notice,
**     this list of conditions and the following disclaimer.
**
**   * Redistributions in binary form must reproduce the above copyright notice,
**     this list of conditions and the following disclaimer in the documentation
**     and/or other materials provided with the distribution.
**
**   * Neither the name of Brian Smith nor the names of its contributors
**     may be used to endorse or promote products derived from this software
**     without specific prior written permission.
**
** THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
** AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
** IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
** ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
** LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
** CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
** SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
** INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
** CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
** ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
** POSSIBILITY OF SUCH DAMAGE.
*/

use types::SQLITE_OK;

use types::{dbh,
            SqliteError,
            SqliteMaybe,
            SqliteResult};

use statement::Statement;
use ffi;

use libc::c_int;
use std::ptr;
use std::str;

/// The database connection.
pub struct Database {
    dbh: *dbh,
}

pub fn database_with_handle(dbh: *dbh) -> Database {
    Database { dbh: dbh }
}

impl Drop for Database {
    /// Closes the database connection.
    /// See http://www.sqlite.org/c3ref/close.html
    fn drop(&mut self) {
        debug!("`Database.drop()`: dbh={:?}", self.dbh);
        unsafe {
            ffi::sqlite3_close(self.dbh);
        }
    }
}

impl Database {
    /// Returns the error message of the the most recent call.
    /// See http://www.sqlite.org/c3ref/errcode.html
    pub fn get_errmsg(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(ffi::sqlite3_errmsg(self.dbh)) }
    }

    /// Prepares/compiles an SQL statement.
    /// See http://www.sqlite.org/c3ref/prepare.html
    pub fn prepare<'db>(&'db self, sql: &str, _tail: &Option<&str>) -> SqliteResult<Statement<'db>> {
        let new_stmt = ptr::null();
        let r = sql.with_c_str( |_sql| {
            unsafe {
                ffi::sqlite3_prepare_v2(self.dbh, _sql, sql.len() as c_int, &new_stmt, ptr::null())
            }
        });
        if r == SQLITE_OK {
            debug!("`Database.prepare()`: stmt={:?}", new_stmt);
            Ok(Statement::new(self, new_stmt))
        } else {
            Err(SqliteError::from_code_and_db(r, self))
        }
    }

    /// Executes an SQL statement.
    /// See http://www.sqlite.org/c3ref/exec.html
    pub fn exec(&self, sql: &str) -> SqliteResult<bool> {
        let code = sql.with_c_str( |_sql| {
            unsafe {
                ffi::sqlite3_exec(self.dbh, _sql, ptr::null(), ptr::null(), ptr::null())
            }
        });

        match code {
          SQLITE_OK => Ok(true),
          _ => Err(SqliteError::from_code_and_db(code, self))
        }
    }

    /// Returns the number of modified/inserted/deleted rows by the most recent
    /// call.
    /// See http://www.sqlite.org/c3ref/changes.html
    pub fn get_changes(&self) -> int {
        unsafe {
            ffi::sqlite3_changes(self.dbh) as int
        }
    }

    /// Returns the ID of the last inserted row.
    /// See http://www.sqlite.org/c3ref/last_insert_rowid.html
    pub fn get_last_insert_rowid(&self) -> i64 {
        unsafe {
            ffi::sqlite3_last_insert_rowid(self.dbh)
        }
    }

    /// Sets a busy timeout.
    /// See http://www.sqlite.org/c3ref/busy_timeout.html
    pub fn set_busy_timeout(&self, ms: int) -> SqliteMaybe {
        let code = unsafe { ffi::sqlite3_busy_timeout(self.dbh, ms as c_int) };

        match code {
          SQLITE_OK => None,
          _ => Some(SqliteError::from_code_and_db(code, self))
        }
    }
}
