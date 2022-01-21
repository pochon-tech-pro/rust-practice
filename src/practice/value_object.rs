#[derive(Debug, Clone, PartialEq)]
pub struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    pub fn new(first_name: &str, last_name: &str) -> FullName {
        Self {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_1() {
        let expect = FullName { last_name: "Smith".to_string(), first_name: "John".to_string() };
        let actual = FullName::new("John", "Smith");
        assert_eq!(expect, actual);
    }

    #[test]
    fn new_2() {
        let expect = FullName { last_name: String::from("Smith"), first_name: String::from("John") };
        let actual = FullName::new("John", "Smith");
        assert_eq!(expect, actual);
    }

    #[test]
    fn clone_is_equal() {
        let expect = FullName { last_name: String::from("Smith"), first_name: String::from("John") };
        let actual = FullName::new("John", "Smith").clone();
        assert_eq!(expect, actual);
    }

    #[test]
    fn clone_address_is_different() {
        let expect = FullName::new("John", "Smith");
        let actual = expect.clone();
        assert!(!std::ptr::eq(&expect, &actual));
    }
}

