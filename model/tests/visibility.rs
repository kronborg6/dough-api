use std::{any::Any, ptr::eq};

use model::domain::shared::visibility::{Visibility, VisibilityOptions};

#[test]
fn new_visibility() {
    let visibility = Visibility::new(VisibilityOptions::Public);

    assert!(visibility.is_public())
}

#[test]
fn from() {
    let vis = Visibility::try_from(2).unwrap();

    assert!(vis.is_private());
    assert_eq!(vis, Visibility::new(VisibilityOptions::Private));
}

#[test]
fn as_u8() {
    let vis = Visibility::new(VisibilityOptions::Hidden);

    assert!(vis.as_u8() > 0);
    assert_eq!(vis.as_u8(), 4u8)
}
