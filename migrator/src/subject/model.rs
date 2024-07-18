use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Subject {
    Table,
    Id,
    Name,
    Sex,
    DiagnosisDate,
    Status,
    CreatedAt,
    UpdatedAt
}

#[derive(DeriveIden)]
pub enum Sex {
    #[sea_orm(iden = "sex_type")]
    Enum,
    #[sea_orm(iden = "Male")]
    Male,
    #[sea_orm(iden = "Female")]
    Female,
}

pub struct SexType;

impl Iden for SexType {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "sex_type").unwrap();
    }
}

#[derive(DeriveIden)]
pub enum Status {
    #[sea_orm(iden = "status_type")]
    Enum,
    #[sea_orm(iden = "InScreening")]
    InScreening,
    #[sea_orm(iden = "Enrolled")]
    Enrolled,
    #[sea_orm(iden = "Failed")]
    Failed,
}

pub struct StatusType;

impl Iden for StatusType {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(s, "status_type").unwrap();
    }
}
