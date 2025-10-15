use std::result::Result;
use failure::Error;
use std::fmt;
use r2d2;

#[derive(Debug)]
pub struct ArangoError(Error);

impl fmt::Display for ArangoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ArangoError {

}

#[derive(Clone, Debug)]
pub struct ArangoDBConnectionManager {
    url: String,
    username: String,
    password: String,
    use_jwt: bool,
}

impl ArangoDBConnectionManager {
    pub fn new(url: &str, username: &str, password: &str, use_jwt: bool) -> ArangoDBConnectionManager {
        ArangoDBConnectionManager {
            url: url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            use_jwt,
        }
    }
}

impl r2d2::ManageConnection for ArangoDBConnectionManager {
    type Connection = arangors::Connection;
    type Error = ArangoError;

    fn connect(&self) -> Result<arangors::Connection, ArangoError> {
        if self.use_jwt == true {
            arangors::Connection::establish_jwt(
                &self.url,
                &self.username,
                &self.password
            ).map_err(|e| ArangoError(e.into()))
        } else {
            arangors::Connection::establish_basic_auth(
                &self.url,
                &self.username,
                &self.password
            ).map_err(|e| ArangoError(e.into()))
        }
    }

    fn is_valid(&self, conn: &mut arangors::Connection) -> Result<(), ArangoError> {
        Ok(())
    }

    fn has_broken(&self, conn: &mut arangors::Connection) -> bool {
        false
    }
}