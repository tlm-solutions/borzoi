use std::hash::Hash;

use serde_json;
use chrono::{DateTime, Utc};
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use std::hash::Hasher;
use uuid::Uuid;

use crate::schema::tetra_data;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = tetra_data)]
pub struct TetraTelegram {
    pub id: i64,
    pub time: DateTime<Utc>,
    pub station: Uuid,
    pub source_ssi: i32,
    pub destination_ssi: i32,
    pub protocol_identifier: i32,
    pub telegram_type: String,
    pub data: Vec<u8>,
    pub arbitrary: Option<serde_json::Value>
}

impl Hash for TetraTelegram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.source_ssi.hash(state);
        self.destination_ssi.hash(state);
        self.protocol_identifier.hash(state);
        self.telegram_type.hash(state);
        self.data.hash(state);
    }
}
