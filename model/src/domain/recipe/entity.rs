use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::domain::{
    error::recipe::RecipeError,
    recipe::recipe_summary::RecipeSummary,
    shared::{
        id::RecipeId,
        visibility::{Visibility, VisibilityOptions},
    },
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Recipe {
    id: RecipeId,
    original: Option<Rc<RecipeSummary>>,
    title: String,
    visibility: Visibility,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Recipe {
    pub fn new(
        title: String,
        visibility: Option<Visibility>,
        original: Option<Rc<RecipeSummary>>,
    ) -> Result<Self, RecipeError> {
        let now = Utc::now();

        if title.is_empty() || title.len() < 4 {
            return Err(RecipeError::RecipeTittleError);
        }

        Ok(Self {
            id: RecipeId::new(),
            original,
            title,
            visibility: visibility.unwrap_or_default(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    pub fn change_visibility(&mut self, value: VisibilityOptions) -> Result<(), RecipeError> {
        self.visibility
            .change(value)
            .map_err(RecipeError::Visibility)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn id(&self) -> &RecipeId {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn create_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    pub fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }
    pub fn original(&self) -> Option<&RecipeSummary> {
        self.original.as_deref()
    }
}
