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
enum RecipeItemType {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum RecipeItem {
    Table,
    Id,
    Name,
    RecipeItemTypeId,
    UserId,
    Visibility,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}
#[derive(DeriveIden)]
enum RecipeItemAmount {
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
enum RecipeStep {
    Table,
    Id,
    RecipeId,
    Order,
    RecipeItemId,
    Tittle,
    Desc,
    Duration,
    Temperature,
    Power,
}
#[derive(DeriveIden)]
enum MetricType {
    Table,
    Id,
    Name,
}
#[derive(DeriveIden)]
enum RecipeMetric {
    Table,
    Id,
    RecipeId,
    MetricTypeId,
    Value,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .table(RecipeItemType::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecipeItemType::Id))
                    .col(string_uniq(RecipeItemType::Name).not_null())
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
                    .col(uuid(RecipeItem::RecipeItemTypeId).null())
                    .col(date_time(RecipeItem::CreatedAt).not_null())
                    .col(date_time(RecipeItem::UpdatedAt).not_null())
                    .col(date_time_null(RecipeItem::DeletedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeItem::Table, RecipeItem::RecipeItemTypeId)
                            .to(RecipeItemType::Table, RecipeItemType::Id)
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
                    .col(RecipeItem::RecipeItemTypeId)
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
                    .table(RecipeItemAmount::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecipeItemAmount::Id))
                    .col(uuid(RecipeItemAmount::RecipeItemId).not_null())
                    .col(float(RecipeItemAmount::Amunt).not_null())
                    .col(string(RecipeItemAmount::Unit).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeItemAmount::Table, RecipeItemAmount::RecipeItemId)
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
                    .table(RecipeItemAmount::Table)
                    .col(RecipeItemAmount::RecipeItemId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(pk_uuid(Recipe::Id))
                    .col(uuid(Recipe::UserId).null())
                    .col(uuid(Recipe::Original).null())
                    .col(
                        ColumnDef::new(Recipe::Visibility)
                            .small_integer()
                            .not_null(),
                    )
                    .col(date_time(Recipe::CreatedAt).not_null())
                    .col(date_time(Recipe::UpdatedAt).not_null())
                    .col(date_time_null(Recipe::DeletedAt).null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recipe::Table, Recipe::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recipe::Table, Recipe::Original)
                            .to(Recipe::Table, Recipe::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .check(Expr::col(Recipe::Visibility).between(0, 255))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RecipeStep::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecipeStep::Id))
                    .col(ColumnDef::new(RecipeStep::Order).small_integer().not_null())
                    .col(uuid(RecipeStep::RecipeId).not_null())
                    .col(uuid(RecipeStep::RecipeItemId).not_null())
                    .col(string(RecipeStep::Tittle).not_null())
                    .col(string(RecipeStep::Desc).null())
                    .col(string(RecipeStep::Temperature).null())
                    .col(string(RecipeStep::Power).null())
                    .col(string(RecipeStep::Duration).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MetricType::Table)
                    .if_not_exists()
                    .col(pk_uuid(MetricType::Id))
                    .col(string(MetricType::Name).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(MetricType::Table)
                    .name("idx-mertic-type-name")
                    .col(MetricType::Name)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RecipeMetric::Table)
                    .if_not_exists()
                    .col(pk_uuid(RecipeMetric::Id))
                    .col(uuid(RecipeMetric::MetricTypeId).not_null())
                    .col(uuid(RecipeMetric::RecipeId).not_null())
                    .col(string(RecipeMetric::Value).not_null())
                    .col(date_time(RecipeMetric::CreatedAt).not_null())
                    .col(date_time(RecipeMetric::UpdatedAt).not_null())
                    .col(date_time_null(RecipeMetric::DeletedAt).null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeMetric::Table, RecipeMetric::RecipeId)
                            .to(Recipe::Table, Recipe::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeMetric::Table, RecipeMetric::MetricTypeId)
                            .to(MetricType::Table, MetricType::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(RecipeMetric::Table)
                    .name("idx-recipe-metric-mertic-type-id")
                    .col(RecipeMetric::MetricTypeId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(RecipeMetric::Table)
                    .name("idx-recipe-mertic-recipe-id")
                    .col(RecipeMetric::RecipeId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!()

        // manager
        //     .drop_table(Table::drop().table("post").to_owned())
        //     .await
    }
}
