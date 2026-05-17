use model::domain::{
    error::recipe::RecipeError,
    recipe::entity::Recipe,
    shared::visibility::{Visibility, VisibilityOptions},
};
use uuid::{Uuid, Variant};

#[test]
fn new_recipe() -> Result<(), RecipeError> {
    let mut recipe = Recipe::new(
        "cheesecake".to_string(),
        Some(Visibility::new(VisibilityOptions::Public)),
        None,
    )?;

    assert_ne!(recipe.id().0, Uuid::nil());
    assert_eq!(Variant::RFC4122, recipe.id().0.get_variant());
    assert_eq!(4, recipe.id().0.get_version_num());

    assert!(!recipe.title().is_empty());

    assert!(recipe.visibility().is_public());

    assert_eq!(recipe.deleted_at(), None);

    let now = chrono::Utc::now();

    assert!(recipe.create_at() < now);
    assert!(recipe.updated_at() < now);

    let past = recipe.updated_at();

    recipe.change_visibility(VisibilityOptions::Private)?;

    assert_ne!(recipe.updated_at(), past);
    assert!(recipe.visibility().is_private());

    Ok(())
}

#[test]
fn recipe_title_to_small() -> Result<(), ()> {
    let recipe = Recipe::new(
        "he".to_string(),
        Some(Visibility::new(VisibilityOptions::Private)),
        None,
    );

    assert!(recipe.is_err());
    assert!(matches!(recipe, Err(RecipeError::RecipeTittleError)));

    Ok(())
}
