use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use derive_new::new;
use anyhow::Result;

// -------------------------
// Domain Model
// -------------------------
// HashMapのキーにするために、Hashトレイトを継承させる
#[derive(Debug, PartialEq, Eq, Clone, new, Hash)]
pub struct Id(String);

#[derive(Debug, PartialEq, Eq, Clone, new)]
pub struct BankName(String);


#[derive(Debug, PartialEq, Eq, Clone, new)]
pub struct Accounts {
    id: Id,
    name: BankName,
    balance: u16,
}

// -------------------------
// Repository (IF)
// -------------------------
trait IAccountsRepository{
    fn save(&self, accounts: Accounts) -> Result<()>;
    // Resultは処理失敗の可能性考慮
    fn of(&self, id: Id) -> Result<Option<Accounts>>; // Resultは処理失敗or存在しない可能性考慮
}

// -------------------------
// Repository (Impl)
// -------------------------
#[derive(Clone, new)]
pub struct InMemoryAccountsRepository {
    store: HashMap<Id, Accounts>,
}

impl IAccountsRepository for InMemoryAccountsRepository {
    fn save(&self, accounts: Accounts) -> Result<()> {
        let mut store = self.store.clone();
        store.insert(accounts.id.clone(), accounts);
        return Ok(());
    }

    fn of(&self, id: Id) -> Result<Option<Accounts>> {
        todo!()
    }
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

    #[test]
    fn repository() {
        let repo = InMemoryAccountsRepository::new(HashMap::new());
        let id = Id::new("X9111".to_string());
        let bank_name = BankName::new("X9111".to_string());
        let accounts = Accounts::new(id, bank_name, 1000);
    }
}