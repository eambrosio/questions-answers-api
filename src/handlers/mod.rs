use std::time::SystemTime;

use crate::models::*;
use rocket::{delete, get, post, serde::json::Json};

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(question: Json<Question>) -> Json<QuestionDetail> {
    Json(QuestionDetail {
        question_id: "question_id".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json(vec![QuestionDetail {
        question_id: "question_id".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(question_uuid: Json<QuestionId>) {
    ()
}

#[post("/answer", data = "<answer>")]
pub async fn create_answer(answer: Json<Answer>) -> Json<AnswerDetail> {
    Json(AnswerDetail {
        answer_id: "answer_id".to_owned(),
        question_id: "question_id".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    })
}

#[get("/answers", data = "<question_id>")]
pub async fn read_answers(question_id: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json(vec![AnswerDetail {
        answer_id: "answer_id".to_owned(),
        question_id: "answer_id".to_owned(),
        content: "content".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

#[delete("/answer", data = "<answer_id>")]
pub async fn delete_answer(answer_id: Json<AnswerId>) {
    ()
}
