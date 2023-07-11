use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error{
    pub code: u16,
    pub message: String,
}

impl std::error::Error for Error{}

impl Display for Error{
    fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Error: {} {}", self.code, self.message)
    }
}

impl Error{
    pub fn new(code:u16, message:String) -> Self {
        Error{code,message}
    }

    pub fn get_message(&self) -> &str{
        &self.message
    }

    pub fn get_code(&self) -> u16 {
        self.code
    }
}

impl From<serde_json::Error> for Error{
    fn from(error: serde_json::Error) ->Self{
        Self::new(400, "Bad Request, Failed to Parse".to_string())
    }
}

