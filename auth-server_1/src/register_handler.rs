use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool, SlimUser, User};
use crate::utils::hash_password;
// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {

    let user = web::block(move || query(invitation_id.into_inner(), user_data.into_inner(), pool))
        .await??;

    Ok(HttpResponse::Ok().json(&user))

    // let user = user_data.into_inner();

    // Ok(HttpResponse::Ok().json(json!({
    //     "id" : invitation_id.into_inner(),
    //     "email": user.email,
    //     "password": user.password
    // })))
}

fn query(
    invitation_id: String,
    user_data: UserData,
     pool: web::Data<Pool>,
) -> Result<SlimUser, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::{email, id, invitations};
    use crate::schema::users::dsl::users;
    let invitation_id = uuid::Uuid::parse_str(&invitation_id)?;

    let mut conn = pool.get()?;
    invitations
        .filter(id.eq(invitation_id))
        .filter(email.eq(&user_data.email))
        .load::<Invitation>(&mut conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|mut result| {
            if let Some(invitation) = result.pop() {
                // if invitation is not expired
                if invitation.expires_at > chrono::Local::now().naive_local() {
                    // try hashing the password, else return the error that will be converted to ServiceError
                    let password: String = hash_password(&user_data.password)?;
                    let user = User::from_details(invitation.email, password);
                    let inserted_user: User =
                        diesel::insert_into(users).values(&user).get_result(&mut conn)?;
                    dbg!(&inserted_user);
                    return Ok(inserted_user.into());
                }
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
