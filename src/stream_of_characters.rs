use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug)]
struct StreamChecker {
    tree: Node,
    sb: String,
}

#[derive(Debug)]
struct Node {
    child: HashMap<char, Node>,
    is_end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            child: HashMap::new(),
            is_end: false,
        }
    }
    fn insert_recursive<I: Iterator<Item = char>>(&mut self, mut letters: I) {
        if let Some(val) = letters.next() {
            let node = self.query(val);
            node.insert_recursive(letters);
        } else {
            self.is_end = true;
        }
    }
    fn match_recursive<I: Iterator<Item = char>>(&mut self, mut letters: I) -> bool {
        if self.is_end {
            return true;
        }
        letters
            .next()
            .and_then(|val| self.child.get_mut(&val))
            .map(|node| node.match_recursive(letters))
            .unwrap_or(false)
    }
    fn query(&mut self, c: char) -> &mut Node {
        match self.child.entry(c) {
            Entry::Occupied(val) => val.into_mut(),
            Entry::Vacant(val) => val.insert(Node::new()),
        }
    }
    fn len(&self) -> usize {
        self.child.len()
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {

    fn new(words: Vec<String>) -> Self {
        let mut root = Node::new();
        words.into_iter().for_each(|word| {
            root.insert_recursive(word.chars().rev());
        });
        StreamChecker {
            tree: root,
            sb: String::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.sb.push(letter);
        self.tree.match_recursive(self.sb.chars().rev())
    }
}


#[test]
fn test_aaa() {
    let mut obj = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert_eq!(obj.query('a'), false);
    assert_eq!(obj.query('b'), false);
    assert_eq!(obj.query('c'), false);
    assert_eq!(obj.query('d'), true);
    assert_eq!(obj.query('e'), false);
    assert_eq!(obj.query('f'), true);
    assert_eq!(obj.query('g'), false);
    assert_eq!(obj.query('h'), false);
    assert_eq!(obj.query('i'), false);
    assert_eq!(obj.query('j'), false);
    assert_eq!(obj.query('k'), false);
    assert_eq!(obj.query('l'), true);
    assert_eq!(obj.query('m'), false);
}

#[test]
fn test_aaa2() {
    let sts = ["ab", "ba", "aaab", "abab", "baa"];
    let list = [
        ["a"],
        ["a"],
        ["a"],
        ["a"],
        ["a"],
        ["b"],
        ["a"],
        ["b"],
        ["a"],
        ["b"],
        ["b"],
        ["b"],
        ["a"],
        ["b"],
        ["a"],
        ["b"],
        ["b"],
        ["b"],
        ["b"],
        ["a"],
        ["b"],
        ["a"],
        ["b"],
        ["a"],
        ["a"],
        ["a"],
        ["b"],
        ["a"],
        ["a"],
        ["a"],
    ];
    let result = [
        false, false, false, false, false, true, true, true, true, true, false, false, true, true,
        true, true, false, false, false, true, true, true, true, true, true, false, true, true,
        true, false,
    ];
    let mut obj = StreamChecker::new(sts.into_iter().map(|s| s.to_string()).collect());
    let now = std::time::Instant::now();
    for (i, r) in list.into_iter().enumerate() {
        let letter = r[0].chars().next().unwrap();
        let flag = obj.query(letter);
        println!("test {} {:?} -> {}", i, letter, flag);
        if flag != result[i] {
            println!("asset error {:>?}", obj);
            break;
        }
    }
    println!("test end used {:?}", now.elapsed());
}

// pass