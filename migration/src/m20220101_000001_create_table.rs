use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    Username,
    CreateAt,
}
#[derive(DeriveIden)]
enum UserInfo {
    Table,
    Id,
    UserId,
    FirstName,
    LastName,
}
#[derive(DeriveIden)]
enum IngredientCategory {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum RecipeItem {
    Table,
    Id,
    Name,
    IngredientCategoryId,
    UserId,
    Visibility,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
#[derive(DeriveIden)]
enum RecipeItemVolume {
    Table,
    Id,
    RecipeItemId,
    Amunt,
    Unit,
}
#[derive(DeriveIden)]
enum Recipe {
    Table,
    Id,
    UserId,
    Visibility,
    Original,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
#[derive(DeriveIden)]
enum MerticType {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum RecipeMetric {
    Table,
    Id,
    MetricTypeId,
    Value,
    CreateAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string_uniq(User::Email).not_null())
                    .col(string_uniq(User::Username).not_null())
                    .col(date_time(User::CreateAt))
                    .to_owned(),
            )
            .await
            .expect("failed to create user table");

        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(pk_uuid(UserInfo::Id))
                    .col(uuid_uniq(UserInfo::UserId).not_null())
                    .col(string(UserInfo::FirstName).not_null())
                    .col(string(UserInfo::LastName).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserInfo::Table, UserInfo::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
            .expect("failed to create UserInfo table");

        manager
            .create_table(
                Table::create()
                    .table(IngredientCategory::Table)
                    .if_not_exists()
                    .col(pk_uuid(IngredientCategory::Id))
                    .col(string_uniq(IngredientCategory::Name).not_null())
                    .to_owned(),
            )
            .await
            .expect("failed to create IngredientCategory table");

        manager
            .create_table(
                Table::create()
                    .table(RecipeItem::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecipeItem::Id))
                    .col(string(RecipeItem::Name).not_null())
                    .col(uuid(RecipeItem::UserId).null())
                    .col(
                        ColumnDef::new(RecipeItem::Visibility)
                            .small_integer()
                            .not_null(),
                    )
                    .col(uuid(RecipeItem::IngredientCategoryId).null())
                    .col(date_time(RecipeItem::CreatedAt).not_null())
                    .col(date_time(RecipeItem::UpdatedAt).not_null())
                    .col(date_time_null(RecipeItem::DeletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeItem::Table, RecipeItem::IngredientCategoryId)
                            .to(IngredientCategory::Table, IngredientCategory::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeItem::Table, RecipeItem::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .check(Expr::col(RecipeItem::Visibility).between(0, 255))
                    .to_owned(),
            )
            .await
            .expect("failed to create RecipeItem table");

        manager
            .create_index(
                Index::create()
                    .name("idx-recipe-items-user-id")
                    .table(RecipeItem::Table)
                    .col(RecipeItem::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-recipe-items-ingredient-category-id")
                    .table(RecipeItem::Table)
                    .col(RecipeItem::IngredientCategoryId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-recipe-items-name")
                    .table(RecipeItem::Table)
                    .col(RecipeItem::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RecipeItemVolume::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecipeItemVolume::Id))
                    .col(uuid(RecipeItemVolume::RecipeItemId).not_null())
                    .col(float(RecipeItemVolume::Amunt).not_null())
                    .col(string(RecipeItemVolume::Unit).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeItemVolume::Table, RecipeItemVolume::RecipeItemId)
                            .to(RecipeItem::Table, RecipeItem::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
            .expect("failed to create RecipeItemVolume table");
        manager
            .create_index(
                Index::create()
                    .name("idx-recipe-item-volumes-recipe-item-id")
                    .table(RecipeItemVolume::Table)
                    .col(RecipeItemVolume::RecipeItemId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(pk_uuid(Recipe::Id))
                    .col(uuid(Recipe::Original).null())
                    .col(uuid(Recipe::Original).null())
                    .col(date_time(Recipe::CreatedAt).not_null())
                    .col(date_time(Recipe::UpdatedAt).not_null())
                    .col(date_time_null(Recipe::DeletedAt).null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table("post").to_owned())
            .await
    }
}
