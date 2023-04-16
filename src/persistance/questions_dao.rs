use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::{DBError, Question, QuestionDetail};

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"
                INSERT INTO questions ( title, description )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            question.title,
            question.description
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        sqlx::query!(
            r#"
                DELETE FROM questions WHERE question_uuid = $1
            "#,
            uuid
        )
        .execute(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query!(
            r#"
                SELECT * FROM questions
            "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| DBError::Other(Box::new(e)))?;

        let questions = records
            .iter()
            .map(|record| QuestionDetail {
                question_uuid: record.question_uuid.to_string(),
                title: record.title.to_owned(),
                description: record.description.to_owned(),
                created_at: record.created_at.to_string(),
            })
            .collect();

        Ok(questions)
    }
}
