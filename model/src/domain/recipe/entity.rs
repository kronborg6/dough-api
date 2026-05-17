use std::rc::Rc;

use crate::domain::shared::{
    id::{RecipeId, RecipeItemId},
    visibility::Visibility,
};
use chrono::{DateTime, Local};

pub struct Recipe {
    pub id: RecipeId,
    pub original: Option<Rc<Recipe>>,
    pub title: String,
    pub visibility: Visibility,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl Recipe {
    pub fn new(
        title: String,
        visibility: Option<Visibility>,
        original: Option<Rc<Recipe>>,
    ) -> Self {
        let now = Local::now();
        Self {
            id: RecipeId::new(),
            original,
            title,
            visibility: visibility.unwrap_or_default(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}
