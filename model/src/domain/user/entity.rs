use std::ops::Deref;

use crate::domain::shared::id::UserId;
use uuid::Uuid;

pub struct User {
    id: UserId,
}

fn gg() {
    let gg = User {
        id: UserId(Uuid::new_v4()),
    };
}
