use std::str::FromStr;
use regex::Regex;
use rust_decimal::Decimal;

/**
 * 値オブジェクトで検証する
 * エラーハンドリングは色々検討が必要そう
 **/

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = &'static str;
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"^[a-zA-Z]+$"#).unwrap();
        if re.is_match(name) {
            Ok(Name(name.to_string()))
        } else {
            Err("許可されていません")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    pub fn new(first_name: Name, last_name: Name) -> FullName {
        FullName { first_name, last_name }
    }
}

pub struct Money {
    amount: Decimal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        // Pattern MatchでResultをCheckする
        let valid = "a1".parse::<Name>();
        match valid {
            Ok(name) => println!("{:?}", name),
            Err(err) => println!("{}", err),
        }

        // Pattern Matchを使わずに片方だけ使いたい場合
        let valid2 = Name::from_str("a");
        if let Err(e) = valid2 {
            println!("エラーで早期リターンしたい時とかに使える: {:?}", e);
        }
        if let Ok(name) = valid2 {
            println!("成功時のValue値(今回、Resultを参照にしていないので所有権がMoveしてます。): {:?}", name);
        }
    }

    #[test]
    fn full_name() {
        let first_name = "John".parse().unwrap();
        let last_name = "Smith".parse().unwrap();

        // 未許可の文字はunwrap時にエラーが発生してるので保証されている。
        let full_name = FullName::new(first_name, last_name);
        println!("{:?}", full_name);
    }

    #[test]
    fn decimal() {
        let d_1 = Decimal::from_str("1.1").unwrap();
        let d_2 = Decimal::from_str("2.2").unwrap();
        println!("Decimal同士計算: {:?}", d_1 + d_2);
        let f_1 = 1.1;
        let f_2 = 2.2;
        println!("f64同士の計算のずれ: {:?}", f_1 + f_2);
    }
}