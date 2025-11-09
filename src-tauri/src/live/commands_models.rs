/*
f64 is used in the models even when it doesn't make sense due to limitations with serde serializing u128 as a JSON number instead of a string
*/

#[derive(specta::Type, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeaderInfo {
    pub total_dmg: f64,
    pub elapsed_ms: f64,
    pub time_last_combat_packet_ms: f64,
}
