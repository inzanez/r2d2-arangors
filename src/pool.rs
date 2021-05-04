use arangors::error::ClientError;
use r2d2;
use std::result::Result;

pub use uclient::ClientExt;

#[derive(Clone, Debug)]
pub struct ArangoDBConnectionManager {
    url: String,
    username: String,
    password: String,
    use_jwt: bool,
}

impl ArangoDBConnectionManager {
    pub fn new(
        url: &str,
        username: &str,
        password: &str,
        use_jwt: bool,
    ) -> ArangoDBConnectionManager {
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
    type Error = ClientError;

    fn connect(&self) -> Result<arangors::Connection, ClientError> {
        if self.use_jwt == true {
            arangors::Connection::establish_jwt(&self.url, &self.username, &self.password)
        } else {
            arangors::Connection::establish_basic_auth(&self.url, &self.username, &self.password)
        }
    }

    fn is_valid(&self, conn: &mut arangors::Connection) -> Result<(), ClientError> {
        arangors::connection::GenericConnection::<uclient::reqwest::ReqwestClient>::validate_server(&conn.url().to_string())
    }

    fn has_broken(&self, conn: &mut arangors::Connection) -> bool {
        arangors::connection::GenericConnection::<uclient::reqwest::ReqwestClient>::validate_server(&conn.url().to_string()).is_err()
    }
}
