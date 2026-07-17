use crate::envs::Envs;
// use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_db_pool() -> DbPool {
    let db_url = Envs::database_url();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub trait DbPoolExt {
    fn to_diesel_error(e: PoolError) -> diesel::result::Error {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::Unknown,
            Box::new(e.to_string()),
        )
    }
}
