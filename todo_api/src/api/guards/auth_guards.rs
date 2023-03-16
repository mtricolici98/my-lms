use rocket::{request::{FromRequest, Outcome}, Request, http::Status, State};

use crate::db::{auth::{User, UserDB}, self};
#[derive(Debug)]
pub enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = LoginError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_db = req.rocket().state::<UserDB>().unwrap();

        let user = req.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| user_db.get_user_by_id(id));

        match user { 
            Some(user) => {
                let user_result = user.await;
                match user_result {
                    Ok(user) => Outcome::Success(user),
                    Err(err) => Outcome::Failure((Status::BadRequest, LoginError::InvalidData))
                }
            }
                ,
            None => Outcome::Failure((Status::Unauthorized, LoginError::UsernameDoesNotExist))
        }

        // match req.headers().get_one("x-api-key") {
        //     None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        //     Some(key) if is_valid(key) => Outcome::Success(),
        //     Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
        // }
    }
}
