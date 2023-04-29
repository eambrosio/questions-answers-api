use std::f32::consts::E;

use rocket::{serde::json::Json, State};

use crate::{
    models::*,
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

use self::handlers_inner::HandlerError;

mod handlers_inner;

#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => APIError::BadRequest(s),
            HandlerError::InternalError(s) => APIError::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    // Example of how to add state to a route
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<QuestionDetail>, APIError> {
    let result = handlers_inner::create_question(question.0, questions_dao).await;

    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
) -> Result<Json<Vec<QuestionDetail>>, APIError> {
    let result = handlers_inner::read_questions(questions_dao).await;

    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Send + Sync>>, // add the appropriate type annotation
) -> Result<(), APIError> {
    let result = handlers_inner::delete_question(question_uuid.0, questions_dao).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(APIError::from(e)),
    }
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
) -> Result<Json<AnswerDetail>, APIError> {
    let result = handlers_inner::create_answer(answer.0, answers_dao).await;

    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>, // add the appropriate type annotation
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    let result = handlers_inner::read_answers(question_uuid.0, answers_dao).await;

    match result {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(APIError::from(e)),
    }
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Sync + Send>>, // add the appropriate type annotation
) -> Result<(), APIError> {
    let result = handlers_inner::delete_answer(answer_uuid.0, answers_dao).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(APIError::from(e)),
    }
}
