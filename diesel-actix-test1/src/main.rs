// main.rs

extern crate diesel;
// use crate::diesel;

pub mod schema;
pub mod models;
pub mod db_conn;
// use crate::database_con::db_connection;
// use crate::db_conn::db_connection;
// use crate::db_conn::hello_world;



// use diesel::prelude::*;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
// use diesel::r2d2::{self, ConnectionManager};
use crate::schema::*;


// Request handlers
mod handlers {
    use crate::db_conn;

    use crate::models::{NewUser, User};
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

        // println!("Displaying {} users", results.len());
        // for user in results {
        //     println!("{}", user.username);
        //     println!("-----------\n");
        //     println!("{}", user.email);
        // }

      

        web::Json(results)

        // HttpResponse::Ok().json(results)
    }

    use crate::schema::users::dsl::id;
    use crate::schema::users::dsl::username;
    use crate::schema::users::dsl::email;

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
                HttpResponse::InternalServerError().json(json!({"error": "Internal Server Error"}))
            }
        }
        
    }


    // pub async fn get_user(pool: web::Data<r2d2::Pool<Connection<PgConnection>>>, user_id: web::Path<i32>) -> impl Responder {
    //     let user = web::block(move || {
    //         use crate::diesel::users::dsl::*;

    //         users.find(user_id.into_inner()).first::<User>(&pool.get().unwrap())
    //     })
    //     .await
    //     .unwrap();

    //     web::Json(user)
    // }

    // pub async fn create_user(pool: web::Data<r2d2::Pool<Connection<PgConnection>>>, new_user: web::Json<crate::diesel::NewUser>) -> impl Responder {
    //     let user = web::block(move || {
    //         use crate::diesel::users::dsl::*;

    //         diesel::insert_into(users).values(&new_user).get_result::<User>(&pool.get().unwrap())
    //     })
    //     .await
    //     .unwrap();

    //     web::Json(user)
    // }

    pub fn create_user() -> User {
        // use crate::schema::users::dsl::*;
        use crate::schema::users;

        let mut dbconn = db_conn::db_connection().expect("Failed to get connection");

        let name_ = "name1111".to_string();
        let email_ = "email1111".to_string();
    
        let new_user = NewUser {
            username: name_,
            email: email_,
        };
    
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut dbconn)
            .expect("Error saving new user")
    }

    pub async fn db_conn_test() -> impl Responder {
        // "db connection established"      
        match db_conn::db_connection() {
            Ok(db_conn) => {
                // Connection is established, perform any necessary operations
                HttpResponse::Ok().body("Database connection established")
            },
            Err(_) => HttpResponse::InternalServerError().body("Failed to establish database connection"),
        }
    }

    // pub fn establish_connection() -> PgConnection {
    //     dotenv().ok();
    
    //     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    //     PgConnection::establish(&database_url)
    //         .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    // }
}

// Routes
mod routes {
    // use crate::handlers::{_create_user, _db_connection_established, _get_user, _get_users};
    use crate::handlers::{db_conn_test, get_users, get_user_by_id};
    use actix_web::web;

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.route("/users", web::get().to(get_users));
        cfg.route("/users/{user_id}", web::get().to(get_user_by_id));
        // cfg.route("/users", web::post().to(create_user));
        cfg.route("/db_con", web::get().to(db_conn_test));
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {

//     // Start the HTTP server
//     HttpServer::new(move || {
//         let pool_clone = pool.clone();

//         App::new()
//             .data(pool_clone)
//             .configure(routes::configure)
//     })
//     .bind("127.0.0.1:4444")?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Start the HTTP server
    HttpServer::new(move || {

        App::new()
            .configure(routes::configure)
    })
    .bind("127.0.0.1:4444")?
    .run()
    .await
}

//  fn main() {
//     let new_user = handlers::create_user();
//     println!("new user: created {} {}", new_user.username, new_user.email);
//     println!("============ Fetching users=============");
//     handlers::get_users();
//     // handlers::test();
//     println!("============ END=============");
// }
