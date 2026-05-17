use crate::domain::{shared::id::UserId, user::username::Username};

#[derive(Debug, PartialEq, Eq)]
pub struct UserSummary {
    id: UserId,
    username: Username,
}
