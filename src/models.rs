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
    pub key: String,
    pub value: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = tetra_failed_slots)]
pub struct TetraFailedSlots {
    pub time: DateTime<Utc>,
    pub station: Uuid,
    pub burst_type: i32,
    pub slot_type: i32,
    pub first_slot_logical_channel: i32,
    pub first_slot_data: Vec<u8>,
    pub first_slot_crc_ok: bool,
    pub second_slot_present: bool,
    pub second_slot_logical_channel: Option<i32>,
    pub second_slot_data: Option<Vec<u8>>,
    pub second_slot_crc_ok: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::models::{TetraFailedSlots, TetraTelegram};

    #[test]
    fn tetra_telegram_serde() {
        let tetra_telegram: TetraTelegram = serde_json::from_str(
            r#"{"key":"LogicalLinkControlPacket","protocol_version":0,"station":"00000000-0000-0000-0000-000000000000","time":"2024-07-29T14:46:43+0200","value":{"address_":{"country_code_":null,"event_label_":null,"network_code_":null,"smi_":null,"sna_":null,"ssi_":6183,"usage_marker_":null,"ussi_":null},"basic_link_information_":null,"basic_slot_granting_element_":16,"burst_type_":4,"channel_allocation_element_":null,"encrypted_":false,"encryption_mode_":null,"fragmentation_":false,"fragmentation_on_stealling_channel_":false,"immediate_napping_permission_flag_":null,"logical_channel_":0,"position_of_grant_":0,"power_control_element_":null,"random_access_flag_":1,"reservation_requirement_":null,"tl_sdu_":{"data_":[],"len_":0,"read_offset_":0},"tm_sdu_":{"data_":[true,false,false,false,false],"len_":5,"read_offset_":0},"type_":0}}"#,
        ).unwrap();

        println!("{:?}", tetra_telegram);
    }

    #[test]
    fn optional_tetra_failed_slots() {
        let tetra_failed_slots: TetraFailedSlots = serde_json::from_str(
            r#"{"burst_type":3,"slot_type":1,"first_slot_crc_ok":true,"first_slot_logical_channel":3,"first_slot_data":[0,0,1,0,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,1,1,0,1,1,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,1,0,1,0,0,0,1,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,1,0,0,0,1,0,0,1,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,1,1,1,1,1,0,1,1,0,1,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,1,1,0,0,0,1,0,0,0,1,1,1,1,1,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"second_slot_present":false,"station":"00000000-0000-0000-0000-000000001005","time":"2024-07-04T22:14:42+0200"}"#,
        ).unwrap();

        println!("{:?}", tetra_failed_slots);
    }
}
