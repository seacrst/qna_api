use std::{net::SocketAddr};

use axum::{
    routing::{get, post, delete},
    Router
};

use crate::handlers::*;

mod handlers;
mod dto;

const PORT:u16 = 7878;

#[tokio::main]
async fn main() {
    let question = Question::config();
    let answer = Answer::config();

    let routes = Router::new()
        .route(question.questions_path, get(Question::read))
        .route(question.question_path, post(Question::create))
        .route(question.question_path, post(Question::delete))
        .route(answer.answers_path, post(Answer::read))
        .route(answer.answer_path, post(Answer::create))
        .route(answer.answer_path, post(Answer::delete));


    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .unwrap();

    axum::serve(listener, routes).await.unwrap();
}
