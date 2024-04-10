// pub mod database_con {
    // use crate::errors::api::ApiError;
    use diesel::prelude::*;
    use diesel::r2d2::{ self, ConnectionManager };
    use diesel_migrations::{ embed_migrations, EmbeddedMigrations, MigrationHarness };
    use lazy_static::lazy_static;
    use log::info;
    use std::env;

    type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
    pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    fn run_migration(conn: &mut PgConnection) {
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    }

    lazy_static! {
        static ref POOL: Pool = {
            dotenv::dotenv().ok();
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let manager = ConnectionManager::<PgConnection>::new(database_url);
            Pool::new(manager).expect("Failed to create pool.")
        };
    }

    pub fn init() {
        info!("Initializing database");
        lazy_static::initialize(&POOL);
        // let mut conn = db_connection().expect("Failed to get connection");
        if let Ok(mut conn) = POOL.get() {
            run_migration(&mut conn);
        } else {
            panic!("Failed to get connection");
        } 

        // run_migration(&mut conn);
    }

    // pub fn db_connection() -> DbConnection {
    //     POOL.get().expect("Failed to get connection");
    // }
    pub fn db_connection() -> Result<DbConnection, Box<dyn std::error::Error>> {
        POOL.get().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
// }

