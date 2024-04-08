use axum::{
    routing::{get, post, delete},
    Router
};
use dotenv;
use sqlx::postgres::PgPoolOptions;

use crate::handlers::*;

mod handlers;
mod persistance;
mod dto;

const PORT:u16 = 7878;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await
        .expect("Failed to create postgres connection pool");
    
    let question = Question::config();
    let answer = Answer::config();

    let routes = Router::new()
        .route(question.questions_path, get(Question::read))
        .route(question.question_path, post(Question::create))
        .route(question.question_path, delete(Question::delete))
        .route(answer.answers_path, get(Answer::read))
        .route(answer.answer_path, post(Answer::create))
        .route(answer.answer_path, delete(Answer::delete));


    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .unwrap();

    axum::serve(listener, routes).await.unwrap();
}
