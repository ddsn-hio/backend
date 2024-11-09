use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(Events::Table)
                    .col(pk_auto(Events::Id))
                    .col(timestamp_with_time_zone(Events::Start))
                    .col(timestamp_with_time_zone(Events::End))
                    .col(decimal(Events::Lat))
                    .col(decimal(Events::Long))
                    .col(string(Events::Title))
                    .col(text(Events::Contents))
                    .col(integer(Events::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-events-users")
                            .from(Events::Table, Events::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Start,
    End,
    Lat,
    Long,
    Title,
    Contents,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
