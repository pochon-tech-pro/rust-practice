use derive_new::new;

#[derive(Debug, PartialEq, Eq, Clone, new)]
struct Id(String);

#[derive(Debug, PartialEq, Eq, Clone, new)]
struct BankName(String);


#[derive(Debug, PartialEq, Eq, Clone, new)]
struct Accounts {
    id: Id,
    name: BankName,
    balance: u16,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_new() {
        let id = Id::new("X9111".to_string());
        let bank_name = BankName::new("X9111".to_string());
        println!("{:?}, {:?}", id, bank_name);
    }
}