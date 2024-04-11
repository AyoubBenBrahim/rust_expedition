
extern crate diesel;
// use crate::diesel;

pub mod schema;
pub mod models;
pub mod db_conn;

use actix_web::{App, HttpServer};
use crate::schema::*;


// Request handlers
mod handlers {
    use crate::db_conn;

    use crate::models::User;
    use actix_web::{web, Responder, HttpResponse};
    use serde_json::json;

    use diesel::prelude::*;
    // use crate::schema::*;
    use crate::users::dsl::users;
    

    pub async fn get_users() -> impl Responder
    {
        let mut dbconn = db_conn::db_connection().expect("Failed to get connection");

        let results = users
            .load::<User>(&mut dbconn)
            .expect("Error loading users");

        web::Json(results)

        // HttpResponse::Ok().json(results)
    }

    pub async fn get_user_by_id(user_id: web::Path<i32>) -> HttpResponse {
        let user_id = user_id.into_inner();
        let mut dbconn = db_conn::db_connection().expect("Failed to get connection");

        let user_result = users
            .find(user_id)
            .first::<User>(&mut dbconn);
            // .expect("Error loading user");

        match user_result {
            Ok(user) => {
                if user.id == user_id {
                    println!("User found: {}", user.username);
                    HttpResponse::Ok().json(user)
                } else {
                    println!("User not found");
                    HttpResponse::NotFound().json(json!({"error": "User not found"}))
                }
            }
            Err(_) => {
                println!("Error loading user");
                HttpResponse::NotFound().json(json!({"error": "UserId not found"}))
            }
        }
        
    }


    //=========
    pub async fn create_user(user_via_api: web::Json<User>) -> HttpResponse {
        

        // Extract the user from the request
        let userr = user_via_api.into_inner();

        println!("new user: extracted {} {}", userr.username, userr.email);
        // Example: Save the user to a database
        let saved_user = save_user_to_database(userr).await;
    
        // Return an appropriate response, 201 Created, with the created user.
        // HttpResponse::Created().json(saved_user)
        match saved_user {
            Ok(user) => {
                HttpResponse::Created().json(user)
            },
            Err(_) => {
                HttpResponse::InternalServerError().json(json!({"error": "Failed to save user, unique constraint violated"}))
            }
        }
    }
    
    pub async fn save_user_to_database(user: User) -> Result<User, diesel::result::Error>
    {
        use crate::schema::users;

        // extract last saved id from db
        let mut dbconn = db_conn::db_connection().expect("Failed to get connection");
        let last_id;
        // last_id =  users
        //     .select(id)
        //     .order(id.desc())
        //     .first::<i32>(&mut dbconn);
            // .unwrap_or(0);

            match users::table
            .select(users::id)
            .order(users::id.desc())
            .first::<i32>(&mut dbconn) {
                Ok(id_) => {
                    last_id = id_ + 1;
                },
                Err(_) => {
                    last_id = 1;
                }
            }

        let saved_user = User {
            id: last_id,
            username: user.username.clone(),
            email: user.email.clone(),
            // Assign other user fields as needed
        };

        println!("Prepared user to be inserted: {:?}", saved_user);
        
        match diesel::insert_into(users::table)
            .values(&saved_user)
            .execute(&mut dbconn) {
            Ok(_) => {
                println!("User saved successfully: {:?}", saved_user);
            },
            Err(_) => {
                // println!("Error saving user");
                return Err(diesel::result::Error::NotFound);
            }
            
        }
    
        Ok(saved_user)
    }
    //====

    pub async fn del_user_by_id(user_id: web::Path<i32>) -> HttpResponse {
        let user_id = user_id.into_inner();
        let mut dbconn = db_conn::db_connection().expect("Failed to get connection");
    
        let result = diesel::delete(users.find(user_id)).execute(&mut dbconn);
    
        match result {
            Ok(rows_affected) if rows_affected > 0 => {
                println!("User {} deleted successfully", user_id);
                HttpResponse::Ok().json(json!({"message": "User deleted successfully"}))
            },
            Ok(_) => {
                println!("User {} does not exist", user_id);
                HttpResponse::NotFound().json(json!({"error": "User does not exist"}))
            },
            Err(_) => {
                println!("Error deleting user");
                HttpResponse::InternalServerError().json(json!({"error": "Internal server error"}))
            }
        }
    }

    pub async fn hello_test() -> impl Responder
    {
        HttpResponse::Ok().body(
            format!("Hello, the GET works")
        )
    }

    pub async fn not_found() -> HttpResponse {
        HttpResponse::NotFound().json(json!({
            "error": "Not Found",
            "message": "The requested route was not found"
        }))
    }


}

// Routes
mod routes {
    use crate::handlers::{create_user, hello_test, get_user_by_id, get_users, del_user_by_id};

    use actix_web::web;

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.route("/hello", web::get().to(hello_test));
        cfg.route("/users", web::get().to(get_users));
        cfg.route("/users/{user_id}", web::get().to(get_user_by_id));
        cfg.route("/users", web::post().to(create_user));
        cfg.route("/users/{user_id}", web::delete().to(del_user_by_id));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::web;

    // Start the HTTP server
    HttpServer::new(move || {

        App::new()
            .configure(routes::configure)
            .default_service(web::route().to(handlers::not_found))
    })
    .bind("127.0.0.1:4444")?
    .run()
    .await
}
