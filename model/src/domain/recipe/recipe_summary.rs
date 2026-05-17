use chrono::{DateTime, Local};

use crate::domain::shared::{id::RecipeId, visibility::Visibility};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecipeSummary {
    id: RecipeId,
    title: String,
    pub visibility: Visibility,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl RecipeSummary {
    pub fn new(title: String, visibility: Option<Visibility>) -> Self {
        let now = Local::now();
        Self {
            id: RecipeId::new(),
            title,
            visibility: visibility.unwrap_or_default(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}
