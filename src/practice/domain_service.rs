use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use derive_new::new;
use anyhow::Result;

// -------------------------
// Domain Model
// -------------------------
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

// -------------------------
// Repository (IF)
// -------------------------
trait IAccountsRepository {
    fn save(&self, accounts: Accounts) -> Result<()>; // Resultは処理失敗の可能性考慮
    fn of(&self, id: Id) -> Result<Option<Accounts>>; // Resultは処理失敗or存在しない可能性考慮
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