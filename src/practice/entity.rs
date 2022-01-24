use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntityError {
    #[error("TypeError: {0}")]
    TypeError(String),
    #[error("InternalServerError: {0}")]
    InternalServerError(String),
}

impl EntityError {
    pub fn type_error(s: &str) -> EntityError {
        EntityError::TypeError(s.to_string())
    }

    pub fn internal_server_error(s: &str) -> EntityError {
        EntityError::InternalServerError(s.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Result<Self, EntityError> {
        if name.chars().count() < 3 {
            return Err(EntityError::type_error("usernameは3文字以上"));
        }
        return Ok(Self { name: name.to_string() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_1() {
        let user = User::new("B");
        match user {
            // Refをつけることで所有権Moveを回避
            Ok(ref user) => println!("{:?}", user),
            Err(ref e) => println!("{:?}", e)
        }
        // unwrap_orでError時にPanicを起こさずにDefaultの値を返すようにできる
        println!("{:?}", user.unwrap_or(User { name: "".to_string() }));
    }
}