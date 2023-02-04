pub mod life {
    #![allow(unused)]
    use crate::lists::lists::*;
    use std::fmt;
    pub struct Life {
        generations: LinkedLists,
    }

    impl<'n> Life {
        pub fn new() -> Self {
            Life {
                generations: LinkedLists::new(),
            }
        }
    }

    impl fmt::Display for Life {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(Life x:{} +)", self.generations)
        }
    }
}
