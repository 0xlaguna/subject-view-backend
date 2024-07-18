use crate::models::user::{ 
    Entity as UserEntity, 
    Model as UserModel,
    ActiveModel as UserActiveModel
};
use sea_orm::*;

use crate::{Result, Error, auth::util::hash_password};

pub struct AbstractUser;

impl AbstractUser {

    pub async fn fetch_user(db: &DbConn, id: i32) -> Result<UserModel> {
        let user = UserEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "find_one", 
                with: "sessions",
                info: e.to_string()
            })?
            .ok_or(Error::NotFound)?;

        Ok(user)
    }

    /// Create a new user
    pub async fn create_user(
        db: &DbConn,
        email: String,
        password: String,
        first_name: String,
        middle_name: String,
        last_name: String
    ) -> Result<UserModel> {

        let password = hash_password(password)?;

        let user = UserActiveModel {
            id: NotSet,
            username: NotSet,
            email: Set(email),
            password: Set(password),
            first_name: Set(first_name),
            middle_name: Set(middle_name),
            last_name: Set(last_name),
            disabled: Set(false)
        };

        let user = user
            .insert(db)
            .await
            .map_err(|e| Error::DatabaseError { 
                operation: "create_user", 
                with: "sessions",
                info: e.to_string()
            })?;

        Ok(user)
    }

}
