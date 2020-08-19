use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    Internal,
    NotFound,
    UniqueViolation,
}

#[derive(Debug)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, err_type: ErrorType) -> AppError {
        AppError {
            message: message.to_string(),
            err_type,
        }
    }

    pub fn from_diesel_err(
        err: diesel::result::Error,
        context: &str,
    ) -> AppError {
        AppError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(db_err, _) => match db_err
                {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        ErrorType::UniqueViolation
                    }
                    _ => ErrorType::Internal,
                },
                diesel::result::Error::NotFound => ErrorType::NotFound,
                // If needed we can handle other cases
                _ => ErrorType::Internal,
            },
        )
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
