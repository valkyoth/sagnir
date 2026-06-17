use super::*;
use sagnir_core::{FORMAT_VERSION, StateId};
extern crate std;
use std::{format, string::String, vec::Vec};

#[test]
fn object_id_keeps_type_separate_from_digest() {
    let id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [1; ID_BYTES]);
    assert_eq!(id.algorithm(), HashAlgorithm::Sha256);
    assert_eq!(id.object_type(), ObjectType::Tree);
    assert_eq!(id.digest(), [1; ID_BYTES]);
}

#[test]
fn object_id_has_constant_time_equality_api() {
    let left = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [1; ID_BYTES]);
    let right = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [1; ID_BYTES]);
    let different_type = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Blob, [1; ID_BYTES]);
    let different_digest = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [2; ID_BYTES]);

    assert!(left.ct_eq(&right));
    assert!(!left.ct_eq(&different_type));
    assert!(!left.ct_eq(&different_digest));
}

#[test]
fn domain_tags_are_type_separated() {
    assert_ne!(domain_tag(ObjectType::Blob), domain_tag(ObjectType::Tree));
}

#[test]
fn all_domain_tags_are_unique() {
    let tags: [&[u8]; 9] = [
        domain_tag(ObjectType::Blob),
        domain_tag(ObjectType::Tree),
        domain_tag(ObjectType::StateRoot),
        domain_tag(ObjectType::Change),
        domain_tag(ObjectType::ChangeRevision),
        domain_tag(ObjectType::World),
        domain_tag(ObjectType::Fact),
        domain_tag(ObjectType::Operation),
        domain_tag(ObjectType::Bundle),
    ];

    let mut left = 0;
    while left < tags.len() {
        let mut right = left + 1;
        while right < tags.len() {
            assert_ne!(tags[left], tags[right]);
            right += 1;
        }
        left += 1;
    }
}

#[test]
fn state_root_records_format_version() {
    let state_id = StateId::new([2; ID_BYTES]);
    let object_id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Tree, [3; ID_BYTES]);
    let root = StateRootRef::new(state_id, object_id, FORMAT_VERSION);
    assert_eq!(root.format_version(), FORMAT_VERSION);
    assert_eq!(root.format_version().get(), 1);
}

#[test]
fn unknown_hash_algorithm_fails_closed() {
    assert_eq!(parse_hash_algorithm(999), Err(SagnirError::InvalidValue));
    assert_eq!(
        parse_hash_algorithm_name("sha-256"),
        Err(SagnirError::InvalidValue)
    );
}

#[test]
fn object_type_parser_fails_closed() {
    assert_eq!(parse_object_type(1), Ok(ObjectType::Blob));
    assert_eq!(parse_object_type(9), Ok(ObjectType::Bundle));
    assert_eq!(parse_object_type(0), Err(SagnirError::InvalidValue));
    assert_eq!(parse_object_type(10), Err(SagnirError::InvalidValue));
    assert_eq!(parse_object_type_name("blob"), Ok(ObjectType::Blob));
    assert_eq!(
        parse_object_type_name("state_root"),
        Err(SagnirError::InvalidValue)
    );
}

#[test]
fn digest_slice_admission_checks_algorithm_length() {
    assert!(
        ObjectId::from_digest_slice(HashAlgorithm::Sha256, ObjectType::Blob, &[7; ID_BYTES])
            .is_ok()
    );
    assert_eq!(
        ObjectId::from_digest_slice(HashAlgorithm::Sha256, ObjectType::Blob, &[7; ID_BYTES - 1]),
        Err(SagnirError::InvalidValue)
    );
}

#[test]
fn object_ids_display_and_parse() {
    let id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Blob, [0xabu8; ID_BYTES]);
    let text = format!("{id}");
    assert_eq!(
        text,
        String::from(
            "sagnir-object-v1:blob:sha256:abababababababababababababababababababababababababababababababab"
        )
    );
    assert_eq!(text.parse::<ObjectId>(), Ok(id));
    assert_eq!(parse_object_id(&text), Ok(id));
}

#[test]
fn object_id_parse_fails_closed() {
    assert_eq!(parse_object_id(""), Err(SagnirError::InvalidValue));
    assert_eq!(
        parse_object_id(
            "sagnir-object-v2:blob:sha256:abababababababababababababababababababababababababababababababab"
        ),
        Err(SagnirError::InvalidValue)
    );
    assert_eq!(
        parse_object_id(
            "sagnir-object-v1:blob:sha512:abababababababababababababababababababababababababababababababab"
        ),
        Err(SagnirError::InvalidValue)
    );
    assert_eq!(
        parse_object_id(
            "sagnir-object-v1:tree:sha256:abababababababababababababababababababababababababababababababa"
        ),
        Err(SagnirError::InvalidValue)
    );
    assert_eq!(
        parse_object_id(
            "sagnir-object-v1:tree:sha256:Abababababababababababababababababababababababababababababababab"
        ),
        Err(SagnirError::InvalidValue)
    );
    assert_eq!(
        parse_object_id(
            "sagnir-object-v1:tree:sha256:ggababababababababababababababababababababababababababababababab"
        ),
        Err(SagnirError::InvalidValue)
    );
}

#[test]
fn same_digest_in_different_object_kinds_is_not_equal() {
    let digest = [9_u8; ID_BYTES];
    let ids: Vec<ObjectId> = [
        ObjectType::Blob,
        ObjectType::Tree,
        ObjectType::StateRoot,
        ObjectType::Change,
        ObjectType::ChangeRevision,
        ObjectType::World,
        ObjectType::Fact,
        ObjectType::Operation,
        ObjectType::Bundle,
    ]
    .into_iter()
    .map(|object_type| ObjectId::new(HashAlgorithm::Sha256, object_type, digest))
    .collect();

    let mut left = 0;
    while left < ids.len() {
        let mut right = left + 1;
        while right < ids.len() {
            assert_ne!(ids[left], ids[right]);
            assert!(!ids[left].ct_eq(&ids[right]));
            right += 1;
        }
        left += 1;
    }
}

#[test]
fn object_id_debug_redacts_digest() {
    let id = ObjectId::new(HashAlgorithm::Sha256, ObjectType::Blob, [4; ID_BYTES]);
    assert_eq!(
        format!("{id:?}"),
        String::from(
            "ObjectId { algorithm: Sha256, object_type: Blob, digest: [32 bytes redacted] }"
        )
    );
}
