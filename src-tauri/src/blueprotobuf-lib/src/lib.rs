pub mod blueprotobuf {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/blueprotobuf_package.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/blueprotobuf_package.serde.rs"));
}

use crate::blueprotobuf::EEntityType;

impl From<i64> for EEntityType {
    fn from(entity_type: i64) -> Self {
        match entity_type & 0xffff {
            64 => EEntityType::EntMonster,
            640 => EEntityType::EntChar,
            _ => EEntityType::EntErrType,
        }
    }
}
