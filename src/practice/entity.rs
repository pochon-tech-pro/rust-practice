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

/**
 * Primitive版
 **/
#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Result<Self, EntityError> {
        let mut user = Self {
            name: Default::default()
        };
        user.change_name(name)?;
        return Ok(user);
    }

    pub fn change_name(&mut self, name: &str) -> Result<(), EntityError> {
        if name.chars().count() < 3 {
            return Err(EntityError::type_error("usernameは3文字以上"));
        }
        self.name = name.to_string();
        Ok(())
    }
}


/**
 * VO型活用版
 **/
#[derive(Clone, Debug)]
pub struct UserName(String);

// UserName型のVOを定義
impl UserName {
    pub fn new(str: &str) -> Result<Self, EntityError> {
        if str.chars().count() < 3 {
            return Err(EntityError::type_error("usernameは3文字以上"));
        }
        return Ok(UserName(str.to_string()));
    }
}

#[derive(Clone, Debug)]
pub struct UserOther {
    name: UserName,
}

impl UserOther {
    pub fn new(name: UserName) -> Self {
        Self { name }
    }

    // 検証ロジックはVOに委譲
    pub fn change_name(&mut self, name: UserName) {
        self.name = name;
    }

    pub fn name(&self) -> UserName {
        return self.name.clone();
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
            Ok(ref user) => println!("Suceessです。 {:?}", user),
            Err(ref e) => println!("Errorです。 {:?}", e)
        }
        // unwrap_orでError時にPanicを起こさずにDefaultの値を返すようにできる
        println!("{:?}", user.unwrap_or(User { name: "".to_string() }));
    }

    #[test]
    fn new_2() {
        let username = UserName::new("U");
        if let Err(ref e) = username {
            println!("Error Response : {:?}", e);
            return
        }
        let user = UserOther::new(username.unwrap());
        println!("{:?}", user);
    }
}