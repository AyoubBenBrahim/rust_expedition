use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::future::ok;
use serde::Deserialize;
use serde_json::json;

// use crate::email_service::send_invitation;
use crate::models::{Invitation, Pool};

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

pub async fn health() -> HttpResponse
{
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    // run diesel blocking code
    let res = web::block(move || create_invitation(invitation_data.into_inner().email, pool)).await??;

    // let res_string = serde_json::to_string(&res).unwrap();


    Ok(HttpResponse::Ok().json(json!({
        "registration_link": res
    })))
    // Ok(HttpResponse::Ok().finish())

    // let message = format!("Invitation sent to: {}", invitation_data.email);
    // Ok(HttpResponse::Ok().json(message))

}

fn create_invitation(
    eml: String,
    mut pool: web::Data<Pool>,
) -> Result<String, crate::errors::ServiceError> {
    // let message = format!("Invitation will be sent to: {}", eml);
    let invitation = dbg!(query(eml, pool)?);
    // send_invitation(&invitation)


    let message = format!(
        "http://localhost:8080/api/register/{}",invitation.id
    );

    Ok(message)
}

/// Diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, crate::errors::ServiceError> 
{
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = eml.into();
    let mut conn = pool.get()?;

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(&mut conn)?;

    println!("Invitation inserted: {:?}", inserted_invitation);
    Ok(inserted_invitation)
}
