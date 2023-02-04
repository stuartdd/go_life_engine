pub mod lists {

    use std::clone::Clone;
    use std::fmt;

    #[derive(Clone)]
    pub struct Node {
        x: i32,
        y: i32,
        index: i64,
        next: Option<Box<Node>>,
    }
    const INDEX_MULT: i64 = 100_000_000;
    pub const LIST_COUNT: usize = 3;

    pub struct LinkedLists {
        lists: [Option<Box<Node>>; LIST_COUNT],
        active: usize,
    }

    pub fn calc_node_index(x: i32, y: i32) -> i64 {
        return i64::from(x) * INDEX_MULT + i64::from(y);
    }

    impl<'n> LinkedLists {
        #![allow(unused)]
        pub fn new() -> Self {
            LinkedLists {
                lists: [None, None, None],
                active: 0,
            }
        }

        pub fn count(&self) -> i32 {
            self.count_at(self.active)
        }

        pub fn count_at(&self, at: usize) -> i32 {
            if at >= self.lists.len() || !self.has_root_at(at) {
                return 0;
            }
            let mut c: i32 = 0;
            let mut current = self.get_root_at(at);
            loop {
                c = c + 1;
                if !current.has_next() {
                    return c;
                }
                current = current.get_next();
            }
            return 0;
        }

        pub fn get_node(&self, x: i32, y: i32) -> Option<Box<Node>> {
            self.get_node_at(x, y, self.active)
        }

        pub fn get_node_at(&self, x: i32, y: i32, at: usize) -> Option<Box<Node>> {
            if at >= self.lists.len() || !self.has_root_at(at) {
                return None;
            }
            let mut current = self.get_root_at(at);
            loop {
                if current.x == x && current.y == y {
                    return Some(Box::new(current.clone()));
                }
                if !current.has_next() {
                    return None;
                }
                current = current.get_next();
            }
        }

        pub fn add_node(&mut self, x: i32, y: i32) {
            self.add_node_at(x, y, self.active)
        }

        pub fn add_node_at(&mut self, x: i32, y: i32, at: usize) {
            if at >= self.lists.len() {
                panic!(
                    "Index out of bounds for LinkedLists:lists[{}]:add_node_at({})",
                    self.lists.len(),
                    at,
                );
            }
            if self.lists[at].is_none() {
                //
                // Root is None so make root!
                //
                self.lists[at] = Some(Box::new(Node::new(x, y)));
            } else {
                //
                // derive the insertion point
                //
                let ni = calc_node_index(x, y);
                //
                // Start at the root
                //
                let mut current = self.get_root_mut_at(at);
                //
                // Test insert at root (current)
                //
                if current.get_index() > ni {
                    let mut root = self.get_root_at(at).clone();
                    let mut n3 = Node::new(x, y);
                    n3.set_next(root);
                    self.lists[at] = Some(Box::new(n3));
                    return;
                }

                loop {
                    //
                    // Test for equality
                    //
                    if current.get_index() == ni {
                        return;
                    }
                    //
                    // If at the end of the list just append
                    //
                    if !current.has_next() {
                        current.set_next(Node::new(x, y));
                        return;
                    }
                    //
                    // Test for insert position
                    //
                    if current.get_next().get_index() > ni {
                        let n2 = current.get_next().clone();
                        let mut n3 = Node::new(x, y);
                        n3.set_next(n2);
                        current.set_next(n3);
                        return;
                    }
                    //
                    // Next in the list
                    //
                    current = current.get_next_mut();
                }
            }
        }

        pub fn swap(&mut self) {
            if self.active == 0 {
                self.active = 1;
                self.lists[0] = None;
            } else {
                self.active = 0;
                self.lists[1] = None;
            }
        }

        pub fn get_active_(self) -> usize {
            return self.active;
        }

        pub fn has_root(&self) -> bool {
            self.has_root_at(self.active)
        }

        pub fn has_root_at(&self, at: usize) -> bool {
            if at >= self.lists.len() {
                return false;
            }
            return self.lists[at].is_some();
        }

        pub fn get_root(&self) -> &Node {
            self.get_root_at(self.active)
        }

        pub fn get_root_at(&self, at: usize) -> &Node {
            if at >= self.lists.len() {
                panic!(
                    "Index out of bounds for LinkedLists:lists[{}]:get_root_at({})",
                    self.lists.len(),
                    at,
                );
            }
            match self.lists[at].as_deref() {
                Some(v) => return v,
                None => {
                    panic!("Life::get_root: Root for lists {} is 'None'", at);
                }
            }
        }

        pub fn get_root_mut(&mut self) -> &mut Node {
            self.get_root_mut_at(self.active)
        }

        pub fn get_root_mut_at(&mut self, at: usize) -> &mut Node {
            if at >= self.lists.len() {
                panic!(
                    "Index out of bounds for LinkedLists:lists[{}]:get_root_mut_at({})",
                    self.lists.len(),
                    at,
                );
            }
            match self.lists[at].as_mut() {
                Some(v) => return v,
                None => {
                    panic!("Life::get_root_mut: Root for lists {} is 'None'", at);
                }
            }
        }
    }

    impl<'n> Node {
        #![allow(unused)]
        pub fn new(x: i32, y: i32) -> Self {
            Node {
                x: x,
                y: y,
                next: None,
                index: calc_node_index(x, y),
            }
        }

        pub fn has_next(&self) -> bool {
            return self.next.is_some();
        }

        pub fn get_index(&self) -> i64 {
            return self.index;
        }

        pub fn set_next(&mut self, n: Node) {
            self.next = Some(Box::new(n))
        }

        pub fn get_next(&self) -> &Node {
            return self.next.as_deref().unwrap();
        }

        pub fn get_next_mut(&mut self) -> &mut Node {
            return self.next.as_mut().unwrap();
        }
    }
    //
    // Implement the Display trait for Node. Similar to String!
    //
    impl fmt::Display for Node {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.next.is_some() {
                write!(f, "(Node x:{}, y:{} id:{} +)", self.x, self.y, self.index)
            } else {
                write!(f, "(Node x:{}, y:{} id:{})", self.x, self.y, self.index)
            }
        }
    }

    impl fmt::Display for LinkedLists {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut s = format!("lists={}", self.lists.len());
            for i in 0..self.lists.len() {
                let c = self.count_at(i);
                if self.active == i {
                    s = format!("{} *LEN[{}]={}", s, i, c);
                } else {
                    s = format!("{} len[{}]={}", s, i, c);
                }
            }
            return write!(f, "{}", s);
        }
    }
}
