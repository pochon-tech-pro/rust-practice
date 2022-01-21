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

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boy {
    name: String,
    age: Option<u16>,
}

impl Boy {
    pub fn new(name: &str, age: Option<u16>) -> Boy {
        Self {
            name: name.to_string(),
            age,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn age(&self) -> Option<u16> {
        self.age
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

    #[test]
    fn getter() {
        let expect = "John".to_string();

        let full_name = FullName::new("John", "Smith");
        let actual = full_name.first_name();
        assert_eq!(expect, actual, "Unit Test Failed . : actual = {:?}", actual);
    }

    #[test]
    fn option_type() {
        let boy1 = Boy::new("John", Some(10));
        assert_eq!(Some(10), boy1.age());
        let boy2 = Boy::new("John", None);
        assert_eq!(None, boy2.age());
        match boy1.age() {
            Some(age) => println!("Option型の中身を使う: {}", age * age),
            None => println!("None")
        }
        let age = boy1.age().unwrap();
        println!("unwrapでmatchを省略できる (Noneの時にPanicになるので注意): {}", age);
        // 引数の型のみ使い、型名を返す関数
        fn get_typename<T>(_: T) -> String {
            std::any::type_name::<T>().to_string()
        }
        println!("unwrapで取得した値の型名: {}", get_typename(age));
    }
}

