use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TcUser::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TcUser::Id).text().not_null().primary_key())
                    .col(
                        ColumnDef::new(TcUser::Username)
                            .string_len(100)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(TcUser::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(TcUser::Password).string().not_null())
                    .col(ColumnDef::new(TcUser::Role).string_len(50).default("user"))
                    .col(ColumnDef::new(TcUser::State).string_len(50).default("active"))
                    .col(
                        ColumnDef::new(TcUser::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TcUser::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TcUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TcUser {
    Table,
    Id,
    Username,
    Email,
    Password,
    Role,
    State,
    #[sea_orm(iden = "createdAt")]
    CreatedAt,
    #[sea_orm(iden = "updatedAt")]
    UpdatedAt,
}
