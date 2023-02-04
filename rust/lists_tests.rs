#[cfg(test)]
mod tests {
    use super::super::*;
    #[test]
    fn test_list_get_call() {
        let mut llist = LinkedLists::new();
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=0 len[1]=0 len[2]=0");

        let c = llist.get_node(35, 5);
        assert_eq!(c.is_none(), true, "Enpty list should return None");
        assert_eq!(llist.count(), 0, "Enpty list should return 0");

        llist.add_node(10, 10);
        let c = llist.get_node(10, 10).unwrap();
        assert_eq!(format!("{c}"), "(Node x:10, y:10 id:1000000010)");
        assert_eq!(llist.count(), 1, "List should return 1");

        llist.add_node(30, 30);
        assert_eq!(llist.count(), 2, "List should return 2");
        llist.add_node(20, 20);
        llist.add_node(40, 40);
        llist.add_node(35, 35);
        llist.add_node(35, 35);
        llist.add_node(36, 36);
        assert_eq!(llist.count(), 6, "List should return 6");

        let arr: [i64; 6] = [
            1000000010, 2000000020, 3000000030, 3500000035, 3600000036, 4000000040,
        ];
        looper(llist.get_root(), &arr);

        //
        // Test get first cell
        //
        let c = llist.get_node(10, 10).unwrap();
        assert_eq!(format!("{c}"), "(Node x:10, y:10 id:1000000010 +)");
        //
        // Test get not found cell
        //
        let c = llist.get_node(5, 5);
        assert_eq!(c.is_none(), true, "");
        //
        // Test get last cell
        //
        let c = llist.get_node(40, 40).unwrap();
        assert_eq!(format!("{c}"), "(Node x:40, y:40 id:4000000040)");
        //
        // Test get mid cell
        //
        let c = llist.get_node(35, 35).unwrap();
        assert_eq!(format!("{c}"), "(Node x:35, y:35 id:3500000035 +)");
        let c = llist.get_node(20, 20).unwrap();
        assert_eq!(format!("{c}"), "(Node x:20, y:20 id:2000000020 +)");

        //
        // Test get not found cell y and not found x
        //
        let c = llist.get_node(35, 5);
        assert_eq!(c.is_none(), true, "");
        let c = llist.get_node(5, 35);
        assert_eq!(c.is_none(), true, "");

        assert_eq!(llist.count(), 6, "List should return 6");
        assert_eq!(llist.count_at(0), 6, "List[0] should return 6");
        assert_eq!(llist.count_at(1), 0, "List[1] should return 0");
        assert_eq!(llist.count_at(2), 0, "List[2] should return 0");
        assert_eq!(llist.count_at(3), 0, "List[3] should return 0");
        assert_eq!(llist.count_at(3), 0, "List[3] should return 0");

        llist.swap();
        assert_eq!(llist.count_at(0), 0, "List[0] should return 0");
        assert_eq!(llist.count_at(1), 0, "List[0] should return 0");
    }

    #[test]
    #[should_panic]
    fn test_list_bounds_panic() {
        let mut llist = LinkedLists::new();
        llist.add_node_at(0, 1, LIST_COUNT)
    }

    #[test]
    fn test_list_bounds_ok() {
        let mut llist = LinkedLists::new();
        llist.add_node_at(0, 1, LIST_COUNT - 1);
        let c = llist.get_node_at(0, 1, LIST_COUNT - 1).unwrap();
        assert_eq!(format!("{c}"), "(Node x:0, y:1 id:1)");
        llist.add_node_at(1, 0, LIST_COUNT - 1);
        let c = llist.get_node_at(1, 0, LIST_COUNT - 1).unwrap();
        assert_eq!(format!("{c}"), "(Node x:1, y:0 id:100000000)");
    }

    #[test]
    fn test_list_many_nodes_ordered() {
        let mut llist = LinkedLists::new();
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=0 len[1]=0 len[2]=0");
        llist.add_node(10, 10);
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=1 len[1]=0 len[2]=0");
        llist.add_node(30, 30);
        let arr: [i64; 2] = [1000000010, 3000000030];
        looper(llist.get_root(), &arr);
        llist.add_node(20, 20);
        let arr: [i64; 3] = [1000000010, 2000000020, 3000000030];
        looper(llist.get_root(), &arr);
        llist.add_node(40, 40);
        let arr: [i64; 4] = [1000000010, 2000000020, 3000000030, 4000000040];
        looper(llist.get_root(), &arr);
        llist.add_node(35, 35);
        let arr: [i64; 5] = [1000000010, 2000000020, 3000000030, 3500000035, 4000000040];
        looper(llist.get_root(), &arr);
        llist.add_node(35, 35);
        let arr: [i64; 5] = [1000000010, 2000000020, 3000000030, 3500000035, 4000000040];
        looper(llist.get_root(), &arr);
        llist.add_node(36, 36);
        let arr: [i64; 6] = [
            1000000010, 2000000020, 3000000030, 3500000035, 3600000036, 4000000040,
        ];
        looper(llist.get_root(), &arr);
        //
        // Insert at the start
        //
        llist.add_node(1, 1);
        let arr: [i64; 7] = [
            100000001, 1000000010, 2000000020, 3000000030, 3500000035, 3600000036, 4000000040,
        ];
        looper(llist.get_root(), &arr);

        llist.swap();
        assert_eq!(format!("{llist}"), "lists=3 len[0]=0 *LEN[1]=0 len[2]=0");
    }

    #[test]
    fn test_list_many_nodes() {
        let mut llist = LinkedLists::new();
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=0 len[1]=0 len[2]=0");
        llist.add_node(10, 10);
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=1 len[1]=0 len[2]=0");
        llist.add_node(20, 20);
        let arr: [i64; 2] = [1000000010, 2000000020];
        looper(llist.get_root(), &arr);
        llist.add_node(30, 30);
        let arr: [i64; 3] = [1000000010, 2000000020, 3000000030];
        looper(llist.get_root(), &arr);
        llist.swap();
        assert_eq!(format!("{llist}"), "lists=3 len[0]=0 *LEN[1]=0 len[2]=0");
    }

    #[test]
    fn test_list_single_node() {
        let mut llist = LinkedLists::new();
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=0 len[1]=0 len[2]=0");
        llist.add_node(10, 10);
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=1 len[1]=0 len[2]=0");
        llist.swap();
        assert_eq!(format!("{llist}"), "lists=3 len[0]=0 *LEN[1]=0 len[2]=0");
        llist.add_node(20, 20);
        assert_eq!(format!("{llist}"), "lists=3 len[0]=0 *LEN[1]=1 len[2]=0");
        llist.swap();
        assert_eq!(format!("{llist}"), "lists=3 *LEN[0]=0 len[1]=0 len[2]=0");
        llist.swap();
        assert_eq!(format!("{llist}"), "lists=3 len[0]=0 *LEN[1]=0 len[2]=0");
    }

    #[test]
    fn test_list_has_root() {
        let llist = LinkedLists::new();
        assert!(!llist.has_root(), "Does not return false if root is None");
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
    fn test_list_single_none() {
        let llist = LinkedLists::new();
        let _root = llist.get_root();
    }

    #[test]
    #[should_panic]
    fn test_list_single_none_mut() {
        let mut llist = LinkedLists::new();
        let _root = llist.get_root_mut();
    }

    #[test]
    fn test_nodes() {
        let mut root = Node::new(1, 2);
        assert_eq!(format!("{root}"), "(Node x:1, y:2 id:100000002)");

        let next = Node::new(4, 5);
        assert_eq!(format!("{next}"), "(Node x:4, y:5 id:400000005)");

        root.set_next(next);
        assert_eq!(format!("{root}"), "(Node x:1, y:2 id:100000002 +)");
        if root.has_next() {
            let m = root.get_next_mut();
            assert_eq!(format!("{m}"), "(Node x:4, y:5 id:400000005)");
            m.set_next(Node::new(99, 98));
            assert_eq!(format!("{m}"), "(Node x:4, y:5 id:400000005 +)");
        }

        if root.has_next() {
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
