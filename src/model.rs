use serde_json::{Value, json};
use serde::Serialize;
use serde::Deserialize;

use crate::error;

//entity
#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub course_id: String,
    pub course_name: String,
    pub course_category: String,
}

//ResponseWrapper - custom response object
pub struct ResponseWrapper{
    pub status_code:u16,
    pub body:Value,
}

impl ResponseWrapper{
    pub fn new(status_code:u16, body:Value) -> Self {
        Self{
            status_code,
            body
        }
    }
}

impl From<error::Error> for ResponseWrapper{
    fn from(error: error::Error) ->Self{
        Self::new(error.code, json!({
            "message": error.message
        }))
    }
}

pub struct Query{
    pub query_param: String,
}

//enum
pub enum PathParam {
    Course(Option<String>),
    NotSupported
}

pub enum QueryParam{
    Query(Option<String>),
    NotSupported
}