use std::str::FromStr;
use regex::Regex;

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
}