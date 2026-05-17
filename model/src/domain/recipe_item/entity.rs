use std::result;

use crate::domain::{
    error::recipe_item::RecipeItemError, shared::id::RecipeItemId, user::user_summary::UserSummary,
};

pub struct RecipeItem {
    id: RecipeItemId,
    name: String,
    user: Option<UserSummary>,
}

impl RecipeItem {
    pub fn new(name: String, user: Option<UserSummary>) -> Result<Self, RecipeItemError> {
        if name.is_empty() || name.len() < 5 {
            return Err(RecipeItemError::Createion);
        }

        Ok(Self {
            id: RecipeItemId::new(),
            name,
            user,
        })
    }
}
