// https://leetcode-cn.com/problems/lru-cache/
use std::collections::HashMap;

#[derive(Debug)]
pub struct LRUCache {
    capacity: usize,
    size: usize,
    head: Option<i32>,
    tail: Option<i32>,
    caches: HashMap<i32, Cache>,
}

#[derive(Clone, Debug)]
struct Cache {
    value: i32,
    next: Option<i32>,
    prev: Option<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            size: 0,
            head: None,
            tail: None,
            caches: HashMap::with_capacity(capacity as usize),
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        self.detach(key);
        self.attach(key);
        self.caches.get(&key).map(|c| c.value).unwrap_or(-1)
    }
    pub fn put(&mut self, key: i32, value: i32) {
        use std::collections::hash_map::Entry;
        let is_new = {
            let entry = self.caches.entry(key);
            match entry {
                Entry::Occupied(mut val) => {
                    let mut cache = val.get_mut();
                    cache.value = value;
                    false
                }
                Entry::Vacant(val) => {
                    let _ = val.insert(Cache {
                        value,
                        prev: None,
                        next: None,
                    });
                    true
                }
            }
        };
        if is_new {
            if self.size == self.capacity {
                if let Some(tail) = self.tail {
                    self.remove(tail);
                }
            }
            self.size += 1;
        } else {
            self.detach(key);
        }
        self.attach(key);
    }
    fn remove(&mut self, key: i32) -> Option<i32> {
        self.detach(key);
        if let Some(c) = self.caches.remove(&key) {
            self.size -= 1;
            return Some(c.value);
        }
        None
    }
    fn detach(&mut self, key: i32) {
        // node.prev.next = node.next
        // node.next.prev = node.prev
        if let Some((prev, next)) = self.caches.get(&key).map(|cache| (cache.prev, cache.next)) {
            if let Some(pr) = prev {
                let _ = self.caches.get_mut(&pr).map(|pr| {
                    pr.next = next;
                });
            }
            if let Some(ne) = next {
                let _ = self.caches.get_mut(&ne).map(|ne| {
                    ne.prev = prev;
                });
            }
            if self.head == Some(key) {
                self.head = prev;
            }
            if self.tail == Some(key) {
                self.tail = next;
            }
        }
    }
    fn attach(&mut self, key: i32) {
        // node.next = head.next
        // node.prev = head
        // head.next = node
        // head = node
        let h = self.head.and_then(|head| {
            if let Some(c) = self.caches.get(&head) {
                return Some((head, c.prev, c.next));
            }
            None
        });
        if let Some((head, _, head_next)) = h {
            let _ = self.caches.get_mut(&key).map(|c| {
                c.next = head_next;
                c.prev = Some(head);
            });
            let _ = self.caches.get_mut(&head).map(|c| {
                c.next = Some(key);
            });
        }
        self.head = Some(key);
        if self.tail.is_none() {
            self.tail = Some(key);
        }
    }
}

#[test]
fn test_lru_cache() {
    let capacity = 2;
    let mut cache = LRUCache::new(capacity);
    cache.put(1, 1);
    println!("A - {:?}", cache);
    cache.put(2, 2);
    println!("B - {:?}", cache);
    assert_eq!(cache.get(1), 1);
    println!("C - {:?}", cache);
    cache.put(3, 3);
    println!("D - {:?}", cache);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}

// macro_rules! vec2 {
//     ($($x:expr,)*) => {
//         let mut vec = vec![];

//     }
// }

#[test]
fn test_lru_cache2() {
    let actions = [
        "LRUCache", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get",
        "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "get",
        "get", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "put", "put",
        "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put",
        "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put",
        "get", "put", "get", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get",
        "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "get", "put", "put",
        "put", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get",
        "put", "put", "put", "put", "put", "put", "put",
    ];
    let params = vec![
        vec![10],
        vec![10, 13],
        vec![3, 17],
        vec![6, 11],
        vec![10, 5],
        vec![9, 10],
        vec![13],
        vec![2, 19],
        vec![2],
        vec![3],
        vec![5, 25],
        vec![8],
        vec![9, 22],
        vec![5, 5],
        vec![1, 30],
        vec![11],
        vec![9, 12],
        vec![7],
        vec![5],
        vec![8],
        vec![9],
        vec![4, 30],
        vec![9, 3],
        vec![9],
        vec![10],
        vec![10],
        vec![6, 14],
        vec![3, 1],
        vec![3],
        vec![10, 11],
        vec![8],
        vec![2, 14],
        vec![1],
        vec![5],
        vec![4],
        vec![11, 4],
        vec![12, 24],
        vec![5, 18],
        vec![13],
        vec![7, 23],
        vec![8],
        vec![12],
        vec![3, 27],
        vec![2, 12],
        vec![5],
        vec![2, 9],
        vec![13, 4],
        vec![8, 18],
        vec![1, 7],
        vec![6],
        vec![9, 29],
        vec![8, 21],
        vec![5],
        vec![6, 30],
        vec![1, 12],
        vec![10],
        vec![4, 15],
        vec![7, 22],
        vec![11, 26],
        vec![8, 17],
        vec![9, 29],
        vec![5],
        vec![3, 4],
        vec![11, 30],
        vec![12],
        vec![4, 29],
        vec![3],
        vec![9],
        vec![6],
        vec![3, 4],
        vec![1],
        vec![10],
        vec![3, 29],
        vec![10, 28],
        vec![1, 20],
        vec![11, 13],
        vec![3],
        vec![3, 12],
        vec![3, 8],
        vec![10, 9],
        vec![3, 26],
        vec![8],
        vec![7],
        vec![5],
        vec![13, 17],
        vec![2, 27],
        vec![11, 15],
        vec![12],
        vec![9, 19],
        vec![2, 15],
        vec![3, 16],
        vec![1],
        vec![12, 17],
        vec![9, 1],
        vec![6, 19],
        vec![4],
        vec![5],
        vec![5],
        vec![8, 1],
        vec![11, 7],
        vec![5, 2],
        vec![9, 28],
        vec![1],
        vec![2, 2],
        vec![7, 4],
        vec![4, 22],
        vec![7, 24],
        vec![9, 26],
        vec![13, 28],
        vec![11, 26],
    ];
    let expects = [
        -2, -2, -2, -2, -2, -2, -1, -2, 19, 17, -2, -1, -2, -2, -2, -1, -2, -1, 5, -1, 12, -2, -2,
        3, 5, 5, -2, -2, 1, -2, -1, -2, 30, 5, 30, -2, -2, -2, -1, -2, -1, 24, -2, -2, 18, -2, -2,
        -2, -2, -1, -2, -2, 18, -2, -2, -1, -2, -2, -2, -2, -2, 18, -2, -2, -1, -2, 4, 29, 30, -2,
        12, -1, -2, -2, -2, -2, 29, -2, -2, -2, -2, 17, 22, 18, -2, -2, -2, -1, -2, -2, -2, 20, -2,
        -2, -2, -1, 18, 18, -2, -2, -2, -2, 20, -2, -2, -2, -2, -2, -2, -2,
    ];
    let mut cache = LRUCache::new(10);
    for (i, action) in actions.into_iter().enumerate() {
        match *action {
            "put" => {
                cache.put(params[i][0], params[i][1]);
            }
            "get" => {
                let value = cache.get(params[i][0]);
                assert_eq!(value, expects[i]);
            }
            _ => {}
        }
    }
}
