use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use derive_new::new;
use derive_getters::Getters;
use anyhow::Result;

// -------------------------
// Domain Model
// -------------------------
// HashMapのキーにするために、Hashトレイトを継承させる
#[derive(Debug, PartialEq, Eq, Clone, new, Hash, Default)]
pub struct Id(String);

#[derive(Debug, PartialEq, Eq, Clone, new, Default)]
pub struct BankName(String);


#[derive(Debug, PartialEq, Eq, Clone, new, Getters, Default)]
pub struct Accounts {
    id: Id,
    name: BankName,
    balance: u16,
}

// -------------------------
// Repository (IF)
// -------------------------
trait IAccountsRepository {
    fn save(&self, accounts: Accounts) -> Result<()>;
    // Resultは処理失敗の可能性考慮
    fn of(&self, id: Id) -> Result<Option<Accounts>>; // Resultは処理失敗or存在しない可能性考慮
}

// -------------------------
// Repository (Impl)
// -------------------------
#[derive(Clone, new)]
pub struct InMemoryAccountsRepository {
    // インメモリ上で参照を共有したいのでArcMutexで包む
    store: Arc<Mutex<HashMap<Id, Accounts>>>,
}

impl IAccountsRepository for InMemoryAccountsRepository {
    fn save(&self, accounts: Accounts) -> Result<()> {
        let store = self.store.clone(); // consider using a `let` binding to create a longer lived value
        let mut store = store.lock().unwrap();
        store.insert(accounts.id.clone(), accounts);
        return Ok(());
    }

    fn of(&self, id: Id) -> Result<Option<Accounts>> {
        let store = self.store.clone();
        let store = store.lock().unwrap();
        let target = store.get(&id).cloned(); // cloned: Optionの中のAccountsがCloneされる
        return Ok(target)
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
        let repo = InMemoryAccountsRepository::new(Arc::new(Mutex::new(HashMap::new())));
        let id = Id::new("X9111".to_string());
        let bank_name = BankName::new("X9111".to_string());
        let accounts = Accounts::new(id.clone(), bank_name.clone(), 1000);
        repo.save(accounts.clone());
        let target = repo.of(id).unwrap().unwrap();
        println!("before {:?}, after {:?}", &accounts, &target);
        assert_eq!(accounts, target);
    }
}