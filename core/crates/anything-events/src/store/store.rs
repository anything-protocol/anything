#![allow(unused)]
use std::fmt::Debug;
use std::fmt::Formatter;

use anything_core::error::AnythingResult;
use anything_core::AnythingConfig;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

use crate::errors::EventsResult;
use crate::{
    config::{AnythingEventsConfig, DatabaseConfig},
    models::Event,
};

use super::sqlite::SqliteStoreAdapter;

#[derive(Debug, Clone)]
pub enum StoreAdapterType {
    SQLITE,
}

#[async_trait::async_trait]
pub trait StoreAdapter {
    async fn init(&self, database_config: &AnythingEventsConfig) -> EventsResult<bool>;
    async fn save(&self, event: Event) -> EventsResult<bool>;
    async fn all(&self) -> EventsResult<Vec<Event>>;
    async fn get(&self, id: i64) -> EventsResult<Event>;

    fn pool(&self) -> &Pool<Sqlite>;
}

pub struct Store {
    // Get the ANY out of here
    pub store: Box<dyn StoreAdapter + Sync + Send>,
    config: AnythingEventsConfig,
}

impl Store {
    pub async fn from_config(config: &AnythingEventsConfig) -> EventsResult<Self> {
        let store = match determine_store_backend(config.database.uri.clone()) {
            StoreAdapterType::SQLITE => instantiate_sqlite_store(config),
        }
        .await?;

        Ok(Self {
            store,
            config: config.clone(),
        })
    }

    // TODO: Make this database independent
    pub fn pool(&self) -> &Pool<Sqlite> {
        self.store.pool()
    }

    pub async fn init(&self) -> EventsResult<()> {
        self.store.init(&self.config).await.unwrap();
        Ok(())
    }

    pub async fn default() -> Self {
        let pool = SqliteStoreAdapter::default().await;
        Self {
            store: Box::new(pool),
            config: AnythingEventsConfig::default(),
        }
    }
}

impl Debug for Store {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("config", &self.config)
            .finish()
    }
}

fn determine_store_backend(database_uri: String) -> StoreAdapterType {
    match database_uri {
        database_uri if database_uri.starts_with("sqlite") => StoreAdapterType::SQLITE,
        _ => panic!("Unsupported database type"),
    }
}

async fn instantiate_sqlite_store(
    config: &AnythingEventsConfig,
) -> EventsResult<Box<dyn StoreAdapter + Sync + Send>> {
    // For sqlite databases, we use the root directory
    let root_dir = config.root_dir.clone();
    let root_dir = config.root_dir.clone();
    let db_dir = root_dir.join("database");

    let database_file = db_dir.join("eventurous.db");
    let database_uri = format!("sqlite://{}", database_file.to_str().unwrap());

    let options = SqliteConnectOptions::new()
        .filename(database_file)
        .create_if_missing(true);

    let mut pool = SqlitePoolOptions::new();
    if let Some(max_connections) = config.database.max_connections {
        pool = pool.max_connections(max_connections as u32);
    }

    let pool = pool.connect_with(options).await?;
    let store = SqliteStoreAdapter::new(pool);
    store.init(config).await?;
    Ok(Box::new(store))
}