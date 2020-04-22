#![crate_name = "r2d2_arangors"]

pub mod pool;
use pool::ArangoDBConnectionManager;

#[cfg(test)]
mod tests {
    use crate::pool::ArangoDBConnectionManager;
    use arangors::Connection;
    use std::time::Duration;
    #[test]
    #[should_panic]
    fn connect() {
        let m = ArangoDBConnectionManager::new("http://server:8529/", "root", "password", true);
        let pool = r2d2::Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::new(10, 0))
            .build(m)
            .expect("Connection failed");

        let c = pool.get();

        match c {
            Ok(_) => println!("Valid"),
            Err(e) => println!("{}", e),
        }
    }
}
