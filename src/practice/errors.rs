use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use anyhow::{Context, Result};
use thiserror::Error;

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


#[derive(Debug)]
enum MyErrorAnyHow {
    Error3(u64),
    Error4(String),
}

impl Display for MyErrorAnyHow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use self::MyErrorAnyHow::*;
        match self {
            Error3(ui) => write!(f, "Error13: {}", ui),
            Error4(str) => write!(f, "Error4: {}", str),
        }
    }
}

impl Error for MyErrorAnyHow {}

// ここの戻り値型は、anyhow::Result<()>に変更。この型は、std::result::Result<(), anyhow::Error>のエイリアス
fn any_how_error() -> Result<()> {
    let filename = "sample_sample_ohh_sample.txt";
    let f = File::open(filename).context(format!("failed open {}", { filename }))?;
    Err(MyErrorAnyHow::Error4("Error!!".to_string()))?;
    Ok(())
}

// MyErrorAnyHowAndThisErrorに対して、deriveマクロのthiserror::Errorを使う
#[derive(Debug, Error)]
enum MyErrorAnyHowAndThisError {
    // Displayの実装をしてくれる
    #[error("Error5: {0}")]
    Error5(u64),
    #[error("Error6: {0}")]
    Error6(String),
    // エラー値は#[error(transparent)]と#[from]を用いることで他のエラー型にDisplay相当の機能を委譲可能
    #[error(transparent)]
    Other(#[from] anyhow::Error), // エラーのバリアントを追加
}

fn any_how_and_this_error() -> Result<()> {
    let filename = "sample_sample_ohh_sample.txt";
    let f = File::open(filename).context(format!("failed open {}", { filename }))?;
    Err(MyErrorAnyHowAndThisError::Error6("Error!!".to_string()))?;
    Err(MyErrorAnyHowAndThisError::Other(anyhow::anyhow!("Other Error")))?;
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

    #[test]
    fn error_case3() {
        let result = any_how_error();
        println!("{:?}", result);
    }

    #[test]
    fn error_case4() {
        let result = any_how_and_this_error();
        println!("{:?}", result);
    }
}
