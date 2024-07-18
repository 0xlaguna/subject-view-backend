use rocket::serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    EnumIter, 
    DeriveActiveEnum,
    Deserialize,
    Serialize
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "sex_type")]
pub enum Sex {
    #[sea_orm(string_value = "Male")]
    Male,
    #[sea_orm(string_value = "Female")]
    Female,
}

impl Sex {
    pub fn from_str_value(input: &str) -> Self {
        match input {
            "Male" => Self::Male,
            "Female" => Self::Female,
            _ => Self::Male,
        }
    }
}

#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    EnumIter, 
    DeriveActiveEnum,
    Deserialize,
    Serialize
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status_type")]
pub enum Status {
    #[sea_orm(string_value = "InScreening")]
    InScreening,
    #[sea_orm(string_value = "Enrolled")]
    Enrolled,
    #[sea_orm(string_value = "Failed")]
    Failed,
}

#[derive(
    Clone,
    Debug, 
    PartialEq, 
    Eq, 
    DeriveEntityModel, 
    Deserialize, 
    Serialize
)]
#[serde(crate = "rocket::serde")]
#[sea_orm(table_name = "subject")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    /// Name
    pub name: String,

    /// Sex
    pub sex: Sex,

    /// Diagnosis Date
    pub diagnosis_date: DateTimeWithTimeZone,

    /// Status
    pub status: Status,

    /// Created At
    pub created_at: DateTimeWithTimeZone,

    /// Updated At
    pub updated_at: Option<DateTimeWithTimeZone>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
