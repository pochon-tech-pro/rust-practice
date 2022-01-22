use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;

#[derive(Debug)]
enum MyError {
    Error1(u64),
    Error2(String),
}

fn basic_error() -> Result<(), MyError> {
    // ...この関数はMyErrorのResult型しか返せないので、他のResult型が返せない
    // let f = File::open("sample_sample_ohh_sample.txt");
    // if let Err(ref error) = f {
    //     return f;
    // }
    return Err(MyError::Error1(404));
}

// https://cha-shu00.hatenablog.com/entry/2020/12/08/060000
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::MyError::*;
        match self {
            Error1(ui) => write!(f, "Error1: {}", ui),
            Error2(str) => write!(f, "Error2: {}", str),
        }
    }
}

impl Error for MyError {}

fn multi_error() -> Result<(), Box<dyn Error>> {
    // 1つめのError
    // File::open("sample_sample_ohh_sample.txt")?;
    // 2つめのError
    Err(MyError::Error2("Error!!".to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_case1() {
        let result = basic_error();
        println!("{:?}", result);
        if let Err(ref my_error) = result {
            println!("Result型のErr: {:?}", result);
            println!("Errの中身抽出: {:?}", my_error);
        }
    }

    #[test]
    fn error_case2() {
        let result = multi_error();
        println!("{:?}", result);
    }
}
