use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(ChargingStation::Table)
                    .if_not_exists()
                    .col(pk_auto(ChargingStation::Id))
                    .col(string(ChargingStation::Addresses))
                    .col(string(ChargingStation::Area))
                    .col(string(ChargingStation::City))
                    .col(string(ChargingStation::Company))
                    .col(date_time(ChargingStation::CreatedTime))
                    .col(string(ChargingStation::Ctype))
                    .col(string(ChargingStation::Dial))
                    .col(string(ChargingStation::Features))
                    .col(string(ChargingStation::Geometry))
                    .col(string(ChargingStation::Grestr))
                    .col(string(ChargingStation::Height))
                    .col(string(ChargingStation::Heightrestricted))
                    .col(string(ChargingStation::Images))
                    .col(string(ChargingStation::Info))
                    .col(string(ChargingStation::IsDeleted))
                    .col(string(ChargingStation::Lat))
                    .col(string(ChargingStation::Lng))
                    .col(string(ChargingStation::Num))
                    .col(string(ChargingStation::Openinghours))
                    .col(string(ChargingStation::Phone))
                    .col(string(ChargingStation::Postcode))
                    .col(string(ChargingStation::Pr))
                    .col(string(ChargingStation::Priceschema))
                    .col(string(ChargingStation::Ptype))
                    .col(string(ChargingStation::Rating))
                    .col(string(ChargingStation::Street))
                    .col(string(ChargingStation::Surface))
                    .col(string(ChargingStation::Timezoneid))
                    .col(string(ChargingStation::Title))
                    .col(string(ChargingStation::Type))
                    .col(string(ChargingStation::Typeid))
                    .col(date_time(ChargingStation::UpdatedTime))
                    .col(string(ChargingStation::Url))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(ChargingStation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ChargingStation {
    Table,
    Id,
    Addresses,
    Area,
    City,
    Company,
    CreatedTime,
    Ctype,
    Dial,
    Features,
    Geometry,
    Grestr,
    Height,
    Heightrestricted,
    Images,
    Info,
    IsDeleted,
    Lat,
    Lng,
    Num,
    Openinghours,
    Phone,
    Postcode,
    Pr,
    Priceschema,
    Ptype,
    Rating,
    Street,
    Surface,
    Timezoneid,
    Title,
    Type,
    Typeid,
    UpdatedTime,
    Url
}
