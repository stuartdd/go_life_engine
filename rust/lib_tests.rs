#[cfg(test)]
mod lib_tests {
    #![allow(unused)]
    use super::super::*;

    #[test]
    fn test_life_get_call() {
        let mut life = Life::new();
        assert_eq!(format!("{life}"), "[0] None");
        life.add_node(10, 10);
        let c = life.get_node(10, 10).unwrap();
        assert_eq!(format!("{c}"), "(Node x:10, y:10 id:1000000010)");
        life.add_node(30, 30);
        life.add_node(20, 20);
        life.add_node(40, 40);
        life.add_node(35, 35);
        life.add_node(35, 35);
        life.add_node(36, 36);
        let arr: [i64; 6] = [
            1000000010, 2000000020, 3000000030, 3500000035, 3600000036, 4000000040,
        ];
        looper(life.get_root(), &arr);

        //
        // Test get first cell
        //
        let c = life.get_node(10, 10).unwrap();
        assert_eq!(format!("{c}"), "(Node x:10, y:10 id:1000000010 +)");
        //
        // Test get not found cell
        //
        let c = life.get_node(5, 5);
        assert_eq!(c.is_none(), true, "");
        //
        // Test get last cell
        //
        let c = life.get_node(40, 40).unwrap();
        assert_eq!(format!("{c}"), "(Node x:40, y:40 id:4000000040)");
        //
        // Test get mid cell
        //
        let c = life.get_node(35, 35).unwrap();
        assert_eq!(format!("{c}"), "(Node x:35, y:35 id:3500000035 +)");
        let c = life.get_node(20, 20).unwrap();
        assert_eq!(format!("{c}"), "(Node x:20, y:20 id:2000000020 +)");

        //
        // Test get not found cell y and not found x
        //
        let c = life.get_node(35, 5);
        assert_eq!(c.is_none(), true, "");
        let c = life.get_node(5, 35);
        assert_eq!(c.is_none(), true, "");
    }

    #[test]
    fn test_life_many_nodes_ordered() {
        let mut life = Life::new();
        assert_eq!(format!("{life}"), "[0] None");
        life.add_node(10, 10);
        assert_eq!(format!("{life}"), "[0] (Root x:10, y:10 id:1000000010)");
        life.add_node(30, 30);
        let arr: [i64; 2] = [1000000010, 3000000030];
        looper(life.get_root(), &arr);
        life.add_node(20, 20);
        let arr: [i64; 3] = [1000000010, 2000000020, 3000000030];
        looper(life.get_root(), &arr);
        life.add_node(40, 40);
        let arr: [i64; 4] = [1000000010, 2000000020, 3000000030, 4000000040];
        looper(life.get_root(), &arr);
        life.add_node(35, 35);
        let arr: [i64; 5] = [1000000010, 2000000020, 3000000030, 3500000035, 4000000040];
        looper(life.get_root(), &arr);
        life.add_node(35, 35);
        let arr: [i64; 5] = [1000000010, 2000000020, 3000000030, 3500000035, 4000000040];
        looper(life.get_root(), &arr);
        life.add_node(36, 36);
        let arr: [i64; 6] = [
            1000000010, 2000000020, 3000000030, 3500000035, 3600000036, 4000000040,
        ];
        looper(life.get_root(), &arr);
        //
        // Insert at the start
        //
        life.add_node(1, 1);
        let arr: [i64; 7] = [
            100000001, 1000000010, 2000000020, 3000000030, 3500000035, 3600000036, 4000000040,
        ];
        looper(life.get_root(), &arr);

        life.swap();
        assert_eq!(format!("{life}"), "[1] None");
    }

    #[test]
    fn test_life_many_nodes() {
        let mut life = Life::new();
        assert_eq!(format!("{life}"), "[0] None");
        life.add_node(10, 10);
        assert_eq!(format!("{life}"), "[0] (Root x:10, y:10 id:1000000010)");
        life.add_node(20, 20);
        let arr: [i64; 2] = [1000000010, 2000000020];
        looper(life.get_root(), &arr);
        life.add_node(30, 30);
        let arr: [i64; 3] = [1000000010, 2000000020, 3000000030];
        looper(life.get_root(), &arr);
        life.swap();
        assert_eq!(format!("{life}"), "[1] None");
    }

    #[test]
    fn test_life_single_node() {
        let mut life = Life::new();
        assert_eq!(format!("{life}"), "[0] None");
        life.add_node(10, 10);
        assert_eq!(format!("{life}"), "[0] (Root x:10, y:10 id:1000000010)");
        life.swap();
        assert_eq!(format!("{life}"), "[1] None");
        life.add_node(20, 20);
        assert_eq!(format!("{life}"), "[1] (Root x:20, y:20 id:2000000020)");
        life.swap();
        assert_eq!(format!("{life}"), "[0] None");
        life.swap();
        assert_eq!(format!("{life}"), "[1] None");
    }

    #[test]
    fn test_life_has_root() {
        let mut life = Life::new();
        assert!(!life.has_root(), "Does not return false if root is None");
    }

    #[test]
    fn test_node_clone() {
        let mut root = Node::new(1, 2);
        let next = Node::new(4, 5);
        root.set_next(next);
        assert_eq!(format!("{root}"), "(Node x:1, y:2 id:100000002 +)");
        let clo = root.clone();
        assert_eq!(format!("{clo}"), "(Node x:1, y:2 id:100000002 +)");
        let clo_next = clo.get_next();
        assert_eq!(format!("{clo_next}"), "(Node x:4, y:5 id:400000005)");
    }

    #[test]
    #[should_panic]
    fn test_life_single_none() {
        let mut life = Life::new();
        let root = life.get_root();
    }

    #[test]
    #[should_panic]
    fn test_life_single_none_mut() {
        let mut life = Life::new();
        let root = life.get_root_mut();
    }

    #[test]
    fn test_nodes() {
        let mut root = Node::new(1, 2);
        assert_eq!(format!("{root}"), "(Node x:1, y:2 id:100000002)");

        let next = Node::new(4, 5);
        assert_eq!(format!("{next}"), "(Node x:4, y:5 id:400000005)");

        root.next = Some(Box::new(next));
        assert_eq!(format!("{root}"), "(Node x:1, y:2 id:100000002 +)");
        if root.has_next() {
            let m = root.get_next_mut();
            assert_eq!(format!("{m}"), "(Node x:4, y:5 id:400000005)");
            m.set_next(Node::new(99, 98));
            assert_eq!(format!("{m}"), "(Node x:4, y:5 id:400000005 +)");
        }

        if root.next.is_some() {
            let n = root.get_next();
            assert_eq!(format!("{n}"), "(Node x:4, y:5 id:400000005 +)");
            if n.has_next() {
                let l = n.get_next();
                assert_eq!(format!("{l}"), "(Node x:99, y:98 id:9900000098)");
            }
        }
        let arr: [i64; 3] = [100000002, 400000005, 9900000098];
        looper(&root, &arr);
    }

    fn looper(mut root: &Node, v: &[i64]) {
        let mut i = 0;
        loop {
            assert_eq!(
                root.get_index(),
                v[i],
                "index at {} '{}' does not equal '{}'",
                i,
                root.get_index(),
                v[i]
            );
            if !root.has_next() {
                if (i + 1) != v.len() {
                    panic!("Node count [{}]< array len [{}]", i + 1, v.len())
                }
                return;
            }
            root = root.get_next();
            i = i + 1
        }
    }
}
