use chrono::{DateTime, Utc};
use diesel::Insertable;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use crate::schema::{tetra_data, tetra_failed_slots};

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

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = tetra_failed_slots)]
pub struct TetraFailedSlots {
    pub time: DateTime<Utc>,
    pub station: Uuid,
    pub burst_type: i32,
    pub first_slot_logical_channel: i32,
    pub first_slot_data: Vec<u8>,
    pub first_slot_crc_ok: bool,
    pub second_slot_present: bool,
    pub second_slot_logical_channel: Option<i32>,
    pub second_slot_data: Option<Vec<u8>>,
    pub second_slot_crc_ok: Option<bool>,
}
