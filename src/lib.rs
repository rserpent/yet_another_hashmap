mod hash_map;
mod hash_table;
pub use hash_map::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn insert_values() {
        let hm = hash_map::HashMap::<String>::new();
        for i in 0..10 {
            hm.insert(i, format!("test value {}", i));
        }
        assert_eq!(hm.len(), 10);
    }

    #[test]
    fn get_value() {
        let hm = hash_map::HashMap::<String>::new();
        for i in -200..-100 {
            hm.insert(i, format!("test value {}", i));
        }

        let res = hm.get(-150);
        assert_eq!(res, Some("test value -150".to_string()));
    }

    #[test]
    fn delete_value() {
        let hm = hash_map::HashMap::<String>::new();
        for i in 500..1000 {
            hm.insert(i, format!("test value {}", i));
        }
        hm.delete(550);
        let res = hm.get(550);
        assert_eq!(res, None);
    }

    #[test]
    fn many_threads_insert() {
        let mut threads = Vec::new();
        let map: Arc<HashMap<f32>> = Arc::new(HashMap::<f32>::new());
        for _t in 0..10 {
            let map = map.clone();
            threads.push(thread::spawn(move || {
                for i in -200..200 {
                    map.insert(i, i as f32 / 1.45);
                }
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(map.len(), 4000);
    }
    #[test]
    fn many_threads_insert_and_get() {
        let mut threads = Vec::new();
        let map: Arc<HashMap<f32>> = Arc::new(HashMap::<f32>::new());
        for _t in 0..10 {
            let map = map.clone();
            threads.push(thread::spawn(move || {
                for i in -200..200 {
                    map.insert(i, i as f32 / 1.45);
                }
            }));
        }
        for _t in 0..10 {
            let map = map.clone();
            threads.push(thread::spawn(move || {
                for i in -200..200 {
                    let data = map.get(i);
                    if data.is_some() {
                        assert_eq!(data, Some(i as f32 / 1.45));
                    }
                }
            }));
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
