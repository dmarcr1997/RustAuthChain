use std::collection::HashMap;

struct Transaction {
    id: String,
    category: String,
    data: u64
}

pub struct Mempool {
    transactions: HashMap<String, Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool { 
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.insert(transaction.id.clone(), transaction);
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.values().cloned().collect()
    }

    pub fn remove_transaction(&mut self, transaction_id: &str) {
        self.transactions.remove(transaction_id);
    }

    pub fn clear(&mut self) {
        self.transactions.clear();
    }
}