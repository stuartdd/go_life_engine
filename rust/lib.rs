use std::clone::Clone;
use std::fmt;
#[path = "lib_tests.rs"]
mod lib_tests;

#[derive(Clone)]
struct Node {
    x: i32,
    y: i32,
    index: i64,
    next: Option<Box<Node>>,
}
const INDEX_MULT: i64 = 100_000_000;

struct Life {
    phase: [Option<Box<Node>>; 2],
    active: usize,
}

pub fn calc_node_index(x: i32, y: i32) -> i64 {
    return i64::from(x) * INDEX_MULT + i64::from(y);
}

impl<'n> Life {
    #![allow(unused)]
    pub fn new() -> Self {
        Life {
            phase: [None, None],
            active: 0,
        }
    }

    pub fn get_node(&self, x: i32, y: i32) -> Option<Box<Node>> {
        self.get_node_at(x, y, self.active)
    }

    pub fn get_node_at(&self, x: i32, y: i32, at: usize) -> Option<Box<Node>> {
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
        if self.phase[at].is_none() {
            //
            // Root is None so make root!
            //
            self.phase[at] = Some(Box::new(Node::new(x, y)));
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
                self.phase[at] = Some(Box::new(n3));
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
            self.phase[0] = None;
        } else {
            self.active = 0;
            self.phase[1] = None;
        }
    }

    pub fn has_root(&self) -> bool {
        self.has_root_at(self.active)
    }

    pub fn has_root_at(&self, at: usize) -> bool {
        return self.phase[at].is_some();
    }

    pub fn get_root(&self) -> &Node {
        self.get_root_at(self.active)
    }

    pub fn get_root_at(&self, at: usize) -> &Node {
        match self.phase[at].as_deref() {
            Some(v) => return v,
            None => {
                panic!("Life::get_root: Root for phase {} is 'None'", at);
            }
        }
    }

    pub fn get_root_mut(&mut self) -> &mut Node {
        self.get_root_mut_at(self.active)
    }

    pub fn get_root_mut_at(&mut self, at: usize) -> &mut Node {
        match self.phase[at].as_mut() {
            Some(v) => return v,
            None => {
                panic!("Life::get_root_mut: Root for phase {} is 'None'", at);
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

impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.has_root() {
            let r = self.get_root();
            write!(
                f,
                "[{}] (Root x:{}, y:{} id:{})",
                self.active, r.x, r.y, r.index
            )
        } else {
            write!(f, "[{}] None", self.active)
        }
    }
}
