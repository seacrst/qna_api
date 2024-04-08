use thiserror::Error;

pub mod postgres_error_codes {
  pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUuid(String),
    #[error("Database error occured")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>)
}

pub mod questions_dao {
  use async_trait::async_trait;
  use crate::dto::*;
  use sqlx::PgPool;
  use super::DbError;
  
  #[async_trait]
  pub trait QuestionsDao {
      async fn create_question(&self, question: QuestionReq) -> Result<QuestionRes, DbError>;
      async fn delete_question(&self, id: String) -> Result<(), DbError>;
      async fn get_questions(&self) -> Result<Vec<QuestionRes>, DbError>;
    }
    
    
    pub struct QuestionPool {
      db: PgPool
    }
    
    impl QuestionPool {
      pub fn new(db: PgPool) -> Self {
        Self { db }
      }
    }
    
    #[async_trait]
    impl QuestionsDao for QuestionPool {
    async fn create_question(&self, question: QuestionReq) -> Result<QuestionRes, DbError> {
      let record = sqlx::query!(
        r#"
          INSERT INTO questions (title, description)
          VALUES ($1, $2)
          RETURNING *
        "#,
        question.title, question.description
      ).fetch_one(&self.db)
      .await
      .map_err(|err| DbError::Other(Box::new(err)))?;

      // Ok(QuestionRes {
      //   title: record.title,
      //   description: record.description,
      //   created_at: record.created_at,
      //   question_uuid: record.question_uuid
      // })
      todo!()
    }
    async fn delete_question(&self, id: String) -> Result<(), DbError> {
      todo!()
    }
    async fn get_questions(&self) -> Result<Vec<QuestionRes>, DbError> {
      todo!()
    }
  }
}