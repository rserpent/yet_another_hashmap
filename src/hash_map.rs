use crate::hash_table::{Entry, HashTable};
use std::sync::Mutex;
use std::sync::RwLock;

#[derive(Debug)]
pub struct HashMap<T>
where
    T: Clone,
{
    data: RwLock<HashTable<T>>,
    count: Mutex<usize>,
    capacity: Mutex<usize>,
}

impl<T> HashMap<T>
where
    T: Clone,
{
    // Creates new HashMap with default(32) capacity
    pub fn new() -> HashMap<T> {
        let data = RwLock::new(HashTable::with_capacity(32));
        HashMap {
            data,
            count: Mutex::new(0),
            capacity: Mutex::new(32),
        }
    }

    // Insert value in table
    //
    // If item with given key already exists in table, then
    // value will be rewritten
    pub fn insert(&self, key: i32, value: T) {
        let mut capacity = self.capacity.lock().unwrap();
        let mut count = self.count.lock().unwrap();
        let _capacity = *capacity;
        let g = _capacity as f32 * 0.8;
        if g <= (*count as f32) {
            *capacity = self.resize();
        }
        let mut data = self.data.write().unwrap();
        data.insert(key, value);
        *count += 1;
    }

    // Searches for a key and returns value from table
    pub fn get(&self, key: i32) -> Option<T> {
        let data = self.data.read().unwrap();
        data.get(key)
    }

    // Delete item by key if it exists
    pub fn delete(&self, key: i32) {
        let mut data = self.data.write().unwrap();
        let mut count = self.count.lock().unwrap();
        data.delete(key);
        *count -= 1;
    }

    // Resizes HashMap x2
    // Returns length of resized HashMap
    pub fn resize(&self) -> usize {
        let mut data = self.data.write().unwrap();
        let ref array = *data.data;

        let new_capacity = array.len() * 2;
        let mut resized_ht = HashTable::<T>::with_capacity(new_capacity);

        // Moving and rehashing entries from old HashMap to resized HashMap
        for i in array {
            match i {
                Entry::Occupied(pair) => {
                    resized_ht.insert(pair.key(), pair.value());
                }
                Entry::Vacant | Entry::Deleted => (),
            }
        }
        *data = resized_ht;
        new_capacity
    }

    pub fn len(&self) -> usize {
        let count = self.count.lock().unwrap();
        *count
    }
}
