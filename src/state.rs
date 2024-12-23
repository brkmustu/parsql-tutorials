use std::sync::Arc;

use tokio_postgres::Client;

use crate::db::init_db;

pub struct AppState {
    pub db: Arc<Client>,
}

impl AppState {
    pub async fn new() -> AppState {
        let db = init_db().await.expect("Veritabanı bağlantısı başarısız oldu");
        AppState { db: Arc::new(db) }
    }
}