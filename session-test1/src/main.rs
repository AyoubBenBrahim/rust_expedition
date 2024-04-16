// use actix_session::{SessionMiddleware, storage::RedisSessionStore};


use actix_session::{SessionMiddleware, storage::CookieSessionStore};

use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer, HttpResponse, Error};
use rand::Rng;
use actix_session::Session;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     // Configure Redis connection details

//     let redis_connection_string = "redis://127.0.0.1:6379";
//     let store = RedisSessionStore::new(redis_connection_string).await.unwrap();

//     let mut rng = rand::thread_rng();
//     let secret_key =  Key::derive_from(&mut rng.gen::<[u8; 32]>());

//     HttpServer::new(move ||
//         App::new()
//         .wrap(SessionMiddleware::new(
//             store.clone(),
//             secret_key.clone()
//         ))
//         .default_service(web::to(|| HttpResponse::Ok())))
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// fn index(session: Session) -> Result<&'static str, Error> {
//     // access the session state
//     if let Some(count) = session.get::<i32>("counter")? {
//         println!("SESSION value: {}", count);
//         // modify the session state
//         session.insert("counter", count + 1)?;
//     } else {
//         session.insert("counter", 1)?;
//     }

//     Ok("Welcome!")
// }

async fn index(session: Session) -> Result<HttpResponse, Error> {
    // access session data
    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok(HttpResponse::Ok().body(format!(
        "Count is {:?}!",
        session.get::<i32>("counter")?.unwrap()
    )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .service(web::resource("/").to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
