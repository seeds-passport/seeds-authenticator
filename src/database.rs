use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}