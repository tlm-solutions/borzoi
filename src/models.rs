use chrono::{DateTime, Utc};
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::schema::tetra_data;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = tetra_data)]
pub struct TetraTelegram {
    pub time: DateTime<Utc>,
    pub station: Uuid,
    pub source_ssi: i32,
    pub destination_ssi: i32,
    pub protocol_identifier: i32,
    pub telegram_type: String,
    pub data: Vec<u8>,
    pub arbitrary: Option<serde_json::Value>,
}
