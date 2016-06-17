use parser::Token;

#[derive(Debug)]
pub enum JsonError {
    UnexpectedToken(String),
    UnexpectedCharacter(char),
    UnexpectedEndOfJson,
    CantCastCodepointToCharacter(u32),
    ArrayIndexOutOfBounds,
    WrongType(String),
    UndefinedField(String),
}

impl JsonError {
    pub fn unexpected_token(token: Token) -> Self {
        JsonError::UnexpectedToken(format!("{:?}", token))
    }

    pub fn wrong_type(expected: &str) -> Self {
        JsonError::WrongType(expected.into())
    }

    pub fn undefined(field: &str) -> Self {
        JsonError::UndefinedField(field.into())
    }
}
