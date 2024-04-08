use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct QuestionId(pub String);

#[derive(Serialize, Deserialize)]
pub struct QuestionReq {
  pub title: String,
  pub description: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionRes {
  pub title: String,
  pub description: String,
  pub question_uuid: String,
  pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId(pub String);

#[derive(Serialize, Deserialize)]
pub struct AnswerReq {
  pub content: String,
  pub question_uuid: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AnswerRes {
  pub question_uuid: String,
  pub answer_uuid: String,
  pub content: String,
  pub created_at: String
}