
use std::collections::{HashMap, BTreeMap};

pub struct Version {
    pub value: String,
    pub dependencies: Vec<String>,
}

pub struct Item {
    versions: BTreeMap<i64, Version>,
    current: i64
}

impl Item {
    pub fn new() -> Item {
        Item{ versions: BTreeMap::new(), current: 0 }
    }

    pub fn get(&self) -> Option<&Version> {
        // returns the latest committed version
        if self.current > 0 {
            let version = self.versions.get(&self.current).unwrap();
            return Some(version);
        }
        None
    }

    pub fn insert(&mut self, value: String,
                  dependencies: Vec<String>,
                  timestamp: i64) {
        let v = Version{value: value, dependencies: dependencies};
        self.versions.insert(timestamp, v);
    }

    pub fn commit(&mut self, timestamp: i64) {
        if timestamp > self.current {
            self.current = timestamp;
        }
    }

    pub fn get_version(&self, timestamp: i64) -> Option<&Version> {
        if let Some(version) = self.versions.get(&self.current) {
            return Some(version);
        }
        None
    }
}

#[test]
fn test_get_item_version() {
    let mut i = Item::new();
    i.insert("tiny baby".to_string(), deps(), 2);
    assert_eq!(0, i.current);
    i.commit(2);
    assert_eq!(2, i.current);
    i.insert("tiny baby foot".to_string(), deps(), 1);
    i.commit(1);
    assert_eq!(2, i.current);

    //

}

pub struct Database {
    items: HashMap<String, Item>,
    open_transactions: HashMap<i64, Transaction>,
}
struct Transaction {
    keys: Vec<String>
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction{ keys: Vec::new() }
    }
}

impl Database {
    // not thread safe.  you need to wrap the db with a channel
    // last write wins databse
    pub fn new() -> Database {
        let x = HashMap::new();
        let t = HashMap::new();
        Database{items:x, open_transactions: t}
    }

    pub fn prepare(&mut self, key: String, value: String,
                   dependencies: Vec<String>, timestamp: i64) {

        // set up a Transaction instance if it doesn't exist
        if !self.open_transactions.contains_key(&timestamp) {
            let t = Transaction::new();
            self.open_transactions.insert(timestamp, t);
        }

        // this will always be Some(x)
        if let Some(x) = self.open_transactions.get_mut(&timestamp) {
            x.keys.push(key.clone());
        }
        // does the item exist in items?
        if !self.items.contains_key(&key) {
            self.items.insert(key.clone(), Item::new());
        }

        let item = self.items.get_mut(&key).unwrap();
        item.insert(value, dependencies, timestamp);
    }

    pub fn exists(&self, key: String ) -> bool {
        self.items.contains_key(&key)
    }

    pub fn open_transaction_count(&self) -> i64 {
        self.open_transactions.len() as i64
    }

    pub fn commit(&mut self, timestamp: i64) {
        // commits all items in the transaction
        if let Some(x) = self.open_transactions.get(&timestamp) {
            // X is our Item
            // get and commit each of the transactions
            for key in x.keys.iter() {
                let item = self.items.get_mut(&key.clone()).unwrap();
                item.current = timestamp;
            }
        }
        self.open_transactions.remove(&timestamp);
    }

    pub fn versions(&self, key: String) {

    }

    pub fn get(&self, key: String) -> Option<&Version> {
        // returns latest committed Version
        // returns None if value has just been prepared
        if let Some(x) = self.items.get(&key) {
            if x.current > 0 {
                return x.versions.get(&x.current);
            }
            return None;
        }
        None
    }
    pub fn get_version(&self, key: String, timestamp: i64) -> Option<&Version> {
        if let Some(item) = self.items.get(&key) {
            return item.get_version(timestamp);
        }
        None
    }
    pub fn get_item(&self, key: String) -> Option<&Item> {
        self.items.get(&key)
    }
}


fn deps() -> Vec<String> {
    vec!("a".to_string(), "b".to_string())
}

#[test]
fn test_commit_flow() {
    let mut db = Database::new();
    assert_eq!(false, db.exists("test".to_string()));
    db.prepare("test".to_string(), "value".to_string(), deps(), 1);
    // we should have a new open transaction
    assert_eq!(1, db.open_transaction_count());
    assert_eq!(true, db.exists("test".to_string()));

    match db.get("test".to_string()) {
        None => {},
        Some(version) => { panic!("should not have a version yet ")}
    }

    {
        let item = db.get_item("test".to_string()).unwrap();
        assert_eq!(item.current, 0);
    }
    // is there a new item?
    db.commit(1);
    {
        let item = db.get_item("test".to_string()).unwrap();
        assert_eq!(item.current, 1);
    }
    match db.get("test".to_string()) {
        None => panic!("failed to find the value"),
        Some(version) => { }
    }

    // i should now be able to get the value out of the DB

    // once the transaction is committed we should no longer have it in the open list
    assert_eq!(0, db.open_transaction_count());
}
