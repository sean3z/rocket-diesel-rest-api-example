use std::ops::{Deref, DerefMut};

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;

/// An alias to the type for a pool of Diesel MySQL connections.
pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

/// Initialize the database pool from the `DATABASE_URL` environment variable.
pub fn connect() -> MysqlPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

/// Connection request guard: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = match request.guard::<&State<MysqlPool>>().await {
            Outcome::Success(pool) => pool,
            Outcome::Error(error) => return Outcome::Error(error),
            Outcome::Forward(_) => {
                return Outcome::Error((Status::InternalServerError, ()));
            }
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Error((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Connection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
