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
}
