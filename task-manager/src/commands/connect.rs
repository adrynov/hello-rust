use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, MySql, MySqlPool, Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost:5432/task-manager")
        .await
}

pub async fn connect_mysql() -> Result<Pool<MySql>, Error> {
    MySqlPool::connect("mysql://root:root@localhost:3306/task_manager").await
}
