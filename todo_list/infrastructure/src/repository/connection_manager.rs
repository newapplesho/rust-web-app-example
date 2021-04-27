use dotenv::dotenv;
use log::error;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[derive(Deserialize)]
pub struct DatabaseConfig {
    database_url_write: String,
    database_url_read: String,
    pool_size: Option<u32>,
}

pub type PgPoolConn = Pool<Postgres>;

#[derive(Clone)]
pub struct DBContext {
    pub pool_write: PgPoolConn,
    pub pool_read: PgPoolConn,
}

impl DBContext {
    pub async fn new() -> Self {
        dotenv().ok();

        let config = match envy::from_env::<DatabaseConfig>() {
            Ok(val) => val,
            Err(err) => {
                panic!("{}", err);
            }
        };

        let database_url_write = config.database_url_write.clone();
        let database_url_read = config.database_url_read.clone();

        DBContext {
            pool_write: init_pool(&config, &database_url_write).await,
            pool_read: init_pool(&config, &database_url_read).await,
        }
    }
}

pub async fn init_pool(config: &DatabaseConfig, database_url: &String) -> PgPoolConn {
    let pool_size = config.pool_size.unwrap_or(10);

    let pool = match PgPoolOptions::new()
        .max_connections(pool_size)
        .connect(database_url.as_ref())
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to initialize db pool: {:?}", e);
            panic!("");
        }
    };
    pool
}

#[cfg(test)]
mod tests {
    use crate::repository::connection_manager::DBContext;

    #[actix_rt::test]
    async fn test_db_connection_pool() {
        let db_context = DBContext::new().await;

        let res = sqlx::query("SELECT * FROM todo_list")
            .fetch_one(&db_context.pool_read)
            .await;

        assert!(res.is_err());

        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&db_context.pool_read)
            .await
            .unwrap();

        assert_eq!(row.0, 150);

        let row: (i64,) = sqlx::query_as("SELECT count(*) from todo_list")
            .fetch_one(&db_context.pool_read)
            .await
            .unwrap();

        assert_eq!(row.0, 0);
    }
}
