use postgres::{Client, Error, NoTls};
// generate a new client
pub fn new(host: &str, user: &str, pass: &str, db: &str) -> Result<Client, Error> {
    match Client::connect(
        format!(
            "host={} user={} password={} dbname={}",
            host, user, pass, db
        )
        .as_str(),
        NoTls,
    ) {
        Ok(value) => Ok(value),
        Err(error) => Err(error),
    }
}
