use crate::envs::Envs;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_db_pool() -> DbPool {
    let db_url = Envs::database_url();
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
