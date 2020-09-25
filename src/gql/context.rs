use crate::database::connection::MysqlPooledConnectionManager;
use crate::config::Config;

impl juniper::Context for Context {}

pub struct Context {
    pub database_pool: MysqlPooledConnectionManager,
    pub config: Config,
}

impl Context {
    pub fn new(database_pool: MysqlPooledConnectionManager, config: Config) -> Self {
        Context {
            config,
            database_pool,
        }
    }
}
