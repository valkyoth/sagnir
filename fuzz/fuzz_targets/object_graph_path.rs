#![no_main]

use libfuzzer_sys::fuzz_target;
use sagnir_core::ID_BYTES;
use sagnir_object::{
    HashAlgorithm, ObjectGraph, ObjectGraphEntry, ObjectId, ObjectReference, ObjectType,
};

fn object_type(raw: u8) -> ObjectType {
    match raw % 6 {
        0 => ObjectType::Blob,
        1 => ObjectType::Tree,
        2 => ObjectType::StateRoot,
        3 => ObjectType::Change,
        4 => ObjectType::World,
        _ => ObjectType::Bundle,
    }
}

fn slot_type(data: &[u8], slot: usize) -> ObjectType {
    object_type(data.get(1 + slot).copied().unwrap_or(slot as u8))
}

fn slot_id(data: &[u8], slot: usize) -> Result<ObjectId, sagnir_core::SagnirError> {
    let digest = [slot as u8; ID_BYTES];
    ObjectId::from_digest_slice(HashAlgorithm::Sha256, slot_type(data, slot), &digest)
}

fuzz_target!(|data: &[u8]| {
    let mut graph = ObjectGraph::<8, 16>::new();
    let entry_count = data.first().map_or(0, |byte| usize::from(byte % 9));

    let mut slot = 0;
    while slot < entry_count {
        if let Ok(id) = slot_id(data, slot) {
            let _ = graph.insert_entry(ObjectGraphEntry::new(id));
        }
        slot += 1;
    }

    let mut cursor = 9;
    while cursor + 1 < data.len() && cursor < 41 {
        let source_slot = usize::from(data[cursor] % 8);
        let target_slot = usize::from(data[cursor + 1] % 8);
        if let (Ok(source), Ok(target)) = (slot_id(data, source_slot), slot_id(data, target_slot)) {
            let _ = ObjectReference::new(source, target, target.object_type())
                .and_then(|reference| graph.insert_reference(reference));
        }
        cursor += 2;
    }

    if data.len() >= 43
        && let (Ok(source), Ok(target)) = (
            slot_id(data, usize::from(data[41] % 8)),
            slot_id(data, usize::from(data[42] % 8)),
        )
    {
        let _ = graph.contains_path(source, target);
    }
});
