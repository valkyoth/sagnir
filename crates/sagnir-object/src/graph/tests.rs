use super::*;
use crate::{HashAlgorithm, ObjectId};
use sagnir_core::ID_BYTES;

fn id(object_type: ObjectType, byte: u8) -> ObjectId {
    ObjectId::new(HashAlgorithm::Sha256, object_type, [byte; ID_BYTES])
}

#[test]
fn graph_verifies_complete_small_graph() {
    let tree = id(ObjectType::Tree, 1);
    let blob = id(ObjectType::Blob, 2);
    let mut graph = ObjectGraph::<4, 4>::new();

    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(tree)), Ok(()));
    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(blob)), Ok(()));
    assert_eq!(
        ObjectReference::new(tree, blob, ObjectType::Blob)
            .and_then(|reference| graph.insert_reference(reference)),
        Ok(())
    );

    assert_eq!(graph.verify(), ObjectGraphReport::Complete);
    assert_eq!(graph.entry_len(), 2);
    assert_eq!(graph.reference_len(), 1);
}

#[test]
fn graph_reports_exact_missing_reference() {
    let tree = id(ObjectType::Tree, 1);
    let blob = id(ObjectType::Blob, 2);
    let mut graph = ObjectGraph::<4, 4>::new();
    let reference = ObjectReference::new(tree, blob, ObjectType::Blob);

    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(tree)), Ok(()));
    assert_eq!(
        reference.and_then(|reference| graph.insert_reference(reference)),
        Ok(())
    );

    assert_eq!(
        graph.verify(),
        ObjectGraphReport::MissingReference(ObjectReference {
            source: tree,
            target: blob,
            target_type: ObjectType::Blob,
        })
    );
}

#[test]
fn graph_rejects_reference_type_mismatch() {
    let tree = id(ObjectType::Tree, 1);
    let blob = id(ObjectType::Blob, 2);

    assert_eq!(
        ObjectReference::new(tree, blob, ObjectType::Tree),
        Err(SagnirError::InvalidValue)
    );
}

#[test]
fn graph_rejects_blob_as_reference_source() {
    let blob = id(ObjectType::Blob, 1);
    let tree = id(ObjectType::Tree, 2);
    let mut graph = ObjectGraph::<4, 4>::new();

    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(blob)), Ok(()));
    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(tree)), Ok(()));
    assert_eq!(
        ObjectReference::new(blob, tree, ObjectType::Tree)
            .and_then(|reference| graph.insert_reference(reference)),
        Err(SagnirError::InvalidValue)
    );
}

#[test]
fn graph_reports_cycle() {
    let left = id(ObjectType::Tree, 1);
    let right = id(ObjectType::Tree, 2);
    let mut graph = ObjectGraph::<4, 4>::new();

    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(left)), Ok(()));
    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(right)), Ok(()));
    assert_eq!(
        ObjectReference::new(left, right, ObjectType::Tree)
            .and_then(|reference| graph.insert_reference(reference)),
        Ok(())
    );
    assert_eq!(
        ObjectReference::new(right, left, ObjectType::Tree)
            .and_then(|reference| graph.insert_reference(reference)),
        Ok(())
    );

    assert_eq!(graph.verify(), ObjectGraphReport::Cycle(left));
}

#[test]
fn graph_traversal_follows_references() {
    let world = id(ObjectType::World, 1);
    let tree = id(ObjectType::Tree, 2);
    let blob = id(ObjectType::Blob, 3);
    let mut graph = ObjectGraph::<4, 4>::new();

    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(world)), Ok(()));
    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(tree)), Ok(()));
    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(blob)), Ok(()));
    assert_eq!(
        ObjectReference::new(world, tree, ObjectType::Tree)
            .and_then(|reference| graph.insert_reference(reference)),
        Ok(())
    );
    assert_eq!(
        ObjectReference::new(tree, blob, ObjectType::Blob)
            .and_then(|reference| graph.insert_reference(reference)),
        Ok(())
    );

    assert_eq!(graph.contains_path(world, blob), Ok(true));
    assert_eq!(graph.contains_path(blob, world), Ok(false));
}

#[test]
fn graph_rejects_capacity_overflow_and_duplicate_entries() {
    let one = id(ObjectType::Tree, 1);
    let two = id(ObjectType::Tree, 2);
    let mut graph = ObjectGraph::<1, 1>::new();

    assert_eq!(graph.insert_entry(ObjectGraphEntry::new(one)), Ok(()));
    assert_eq!(
        graph.insert_entry(ObjectGraphEntry::new(one)),
        Err(SagnirError::InvalidValue)
    );
    assert_eq!(
        graph.insert_entry(ObjectGraphEntry::new(two)),
        Err(SagnirError::InvalidValue)
    );
}
