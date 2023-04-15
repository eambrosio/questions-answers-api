use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct QuestionDetail {
    pub question_id: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_id: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_id: String,
    pub question_id: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_id: String,
}
