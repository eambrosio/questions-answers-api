#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

mod cors;
mod handlers;
mod models;

use std::env;

use cors::*;
use dotenvy::dotenv;
use handlers::*;
use sqlx::postgres::PgPoolOptions;

#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();

    // TODO: Initialize dotenv
    dotenv().expect("failed to load .env");

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("invalid database URL"))
        .await
        .expect("error while creating the connection pool");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    let records = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .expect("error while fetching the records");

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    
        info!("{:?}", records);


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
