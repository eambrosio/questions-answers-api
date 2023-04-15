#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

mod cors;
mod handlers;
mod models;
mod persistance;

use std::env;

use cors::*;
use dotenvy::dotenv;
use handlers::*;
use sqlx::postgres::PgPoolOptions;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().expect("failed to load .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("invalid database URL"))
        .await
        .expect("error while creating the connection pool");

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
