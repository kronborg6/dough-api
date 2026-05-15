// use uuid::Uuid;

use crate::impl_uuid_id;

// pub struct UserId(pub Uuid);
// pub struct RecipeId(pub Uuid);
// pub struct RecipeItemId(pub Uuid);

impl_uuid_id!(UserId);
impl_uuid_id!(RecipeId);
impl_uuid_id!(RecipeItemId);
