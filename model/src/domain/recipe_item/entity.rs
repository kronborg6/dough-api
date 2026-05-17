use crate::domain::{shared::id::RecipeItemId, user::entity::User};

pub struct RecipeItem {
    id: RecipeItemId,
    name: String,
    User: Option<User>,
}
