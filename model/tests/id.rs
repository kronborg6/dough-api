use model::impl_uuid_id;
use uuid::{Uuid, Variant};

#[test]
fn ids_are_unique() {
    impl_uuid_id!(TestId);

    let id1 = TestId::default();

    assert_eq!(Variant::RFC4122, id1.get_variant());
    assert_ne!(id1.0, Uuid::nil());

    let id2 = TestId::new();

    assert_ne!(id1, id2);
}

#[test]
fn create_v4_uuid() {
    impl_uuid_id!(TestId);

    let id = TestId::new();

    assert_eq!(Variant::RFC4122, id.get_variant());
    assert_eq!(4, id.get_version_num());
}
