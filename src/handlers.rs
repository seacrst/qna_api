use axum::{response::IntoResponse, Json};

use crate::dto::*;


pub trait Config {
    fn config() -> Self;
}

pub struct Question {
  pub question_path: &'static str,
  pub questions_path: &'static str
}

impl Question {
  pub async fn create(Json(question): Json<QuestionRes>) -> impl IntoResponse {
    todo!()
  }

  pub async fn read() -> impl IntoResponse {
    todo!()
  }

  pub async fn delete(Json(id): Json<QuestionId>) -> impl IntoResponse {
    todo!()
  }
}

pub struct Answer {
  pub answer_path: &'static str,
  pub answers_path: &'static str
}

impl Answer {
  pub async fn create(Json(answer): Json<AnswerRes>) -> impl IntoResponse {
    todo!()
  }

  pub async fn read() -> impl IntoResponse {
    todo!()
  }

  pub async fn delete(Json(id): Json<AnswerId>) -> impl IntoResponse {
    todo!()
  }
}


impl Config for Question {
  fn config() -> Self {
    Question {
      question_path: "/question",
      questions_path: "/questions"
    }
  }
}

impl Config for Answer {
  fn config() -> Self {
    Answer {
      answer_path: "/answer",
      answers_path: "/answers",
    }
  }
}