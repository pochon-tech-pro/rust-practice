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

impl Accounts {
    pub fn withdraw(&mut self, amount: u16) {
        self.balance = self.balance - amount;
    }
    pub fn deposit(&mut self, amount: u16) {
        self.balance = self.balance + amount;
    }
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
        return Ok(target);
    }
}

// -------------------------
// Domain Service
// -------------------------
#[derive(new)]
pub struct AccountsService {}

impl AccountsService {
    // ある口座が別の口座へお金を移す業務は口座Eに持たせるのは不自然なため用意
    pub fn transfer(&self, mut from: Accounts, mut to: Accounts, amount: u16) -> (Accounts, Accounts) {
        from.withdraw(amount);
        to.deposit(amount);
        return (from, to);
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

    #[test]
    fn domain_service() {
        let before_a = Accounts::new(Id::new("1".to_string()), BankName::new("A".to_string()), 1000);
        let before_b = Accounts::new(Id::new("2".to_string()), BankName::new("B".to_string()), 1000);

        let account_service = AccountsService::new();
        let (updated_a, updated_b) = account_service.transfer(before_a.clone(), before_b.clone(), 500);

        println!("before {:?}, after {:?}", &before_a, &updated_a);
        assert_ne!(before_a.balance(), updated_a.balance());
    }
}