use uuid::Uuid;

use crate::domain::{shared::id::UserId, user::username::Username};

pub struct User {
    id: UserId,
    username: Username,
}

fn gg() {
    let name = Username::new("Kronborg").expect("hardcoded vaule username should be valid");
    let user = User {
        id: UserId(Uuid::new_v4()),
        username: name,
    };

    if user.username.contains("r") {
        println!("{:?}", user.username.to_string());
    }
}
