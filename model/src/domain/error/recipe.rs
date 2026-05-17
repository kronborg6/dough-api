use crate::domain::error::visibility::VisibilityError;

#[derive(Debug)]
pub enum RecipeError {
    RecipeTittleError,
    Error,
    Visibility(VisibilityError),
}
