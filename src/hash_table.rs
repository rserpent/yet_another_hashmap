use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash(capacity: usize, key: i32) -> i32 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let res = (hasher.finish() as u32) % capacity as u32;
    res as i32
}

// Simple inner hash function for double hashing
fn inner_hash(capacity: usize, key: i32) -> i32 {
    key % (capacity as i32 - 1) + 1
}

#[derive(Debug)]
pub struct HashTable<T>
where
    T: Clone,
{
    pub(crate) data: Vec<Entry<T>>,
}

impl<T> HashTable<T>
where
    T: Clone,
{
    pub fn with_capacity(capacity: usize) -> HashTable<T> {
        let data = vec![Entry::Vacant; capacity];
        HashTable {
            data,
        }
    }

    pub fn get(&self, key: i32) -> Option<T> {
        let index = hash(self.data.len(), key);
        match lookup(&LookupAction::Get(key), &self.data, index) {
            Some(_index) => match &self.data[_index as usize] {
                Entry::Occupied(pair) => Some(pair.value()),
                Entry::Vacant | Entry::Deleted => None,
            },
            None => None,
        }
    }

    pub fn insert(&mut self, key: i32, value: T) {
        let index = hash(self.data.len(), key);
        let _index =
            lookup(&LookupAction::Insert(key), &self.data, index).expect("Unable to insert value");
        self.data[_index as usize] = Entry::Occupied(OccupiedEntry::new(key, value));
    }

    pub fn delete(&mut self, key: i32) {
        let index = hash(self.data.len(), key);
        match lookup(&LookupAction::Delete(key), &self.data, index) {
            Some(_index) => self.data[index as usize] = Entry::Deleted,
            None => (),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Entry<T>
where
    T: Clone,
{
    Vacant,
    Occupied(OccupiedEntry<T>),
    Deleted,
}

#[derive(Debug, Clone)]
pub struct OccupiedEntry<T>
where
    T: Clone,
{
    key: i32,
    value: T,
}

impl<T> OccupiedEntry<T>
where
    T: Clone,
{
    pub(crate) fn new(key: i32, value: T) -> OccupiedEntry<T> {
        OccupiedEntry { key, value }
    }

    pub(crate) fn key(&self) -> i32 {
        self.key
    }

    pub(crate) fn value(&self) -> T {
        self.value.clone()
    }
}

enum LookupAction {
    Get(i32),
    Insert(i32),
    Delete(i32),
}

fn lookup<T>(action: &LookupAction, data: &Vec<Entry<T>>, start: i32) -> Option<i32>
where
    T: Clone,
{
    let mut index = start;
    let resolve_collision = |i: i32| -> i32 { inner_hash(data.len(), i) };

    loop {
        let entry = &data[index as usize];
        match action {
            // Returns index of occupied entry(Some(i32))
            // if searching key, that equals key in entry
            // Returns None if entry by key not found
            LookupAction::Get(key) => match entry {
                Entry::Occupied(pair) => {
                    if &pair.key() == key {
                        return Some(index);
                    } else {
                        index = resolve_collision(index);
                    }
                }
                Entry::Deleted => {
                    index = resolve_collision(index);
                }
                Entry::Vacant => return None,
            },
            // Returns index of first visited vacant entry
            // Returns index of occupied entry if it have the same key
            LookupAction::Insert(key) => match entry {
                Entry::Vacant => {
                    return Some(index);
                }
                Entry::Occupied(pair) => {
                    if &pair.key() == key {
                        return Some(index);
                    } else {
                        index = resolve_collision(index);
                    }
                }
                Entry::Deleted => index = resolve_collision(index),
            },
            // Returns index of occupied entry if it have the same key
            // Returns None if entry by key not found
            LookupAction::Delete(key) => match entry {
                Entry::Occupied(pair) => {
                    if &pair.key() == key {
                        return Some(index);
                    } else {
                        index = resolve_collision(index);
                    }
                }
                Entry::Deleted => {
                    index = resolve_collision(index);
                }
                Entry::Vacant => return None,
            },
        }
    }
}
