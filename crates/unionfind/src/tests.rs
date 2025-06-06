use crate::HashUnionFindByRank;

#[test]
pub fn grow() {
    macro_rules! grow_test {
        ($ty: path) => {{
            type T = $ty;
            let mut uf = T::new([0, 1, 2]).unwrap();
            uf.union_by_rank(&0, &2).unwrap();

            assert_eq!(uf.find(&0), uf.find(&2));
            assert_eq!(uf.find(&1), Some(1));

            uf.add(3).unwrap();

            assert_eq!(uf.find(&0), uf.find(&2));
            assert_eq!(uf.find(&1), Some(1));

            uf.union_by_rank(&3, &1).unwrap();

            assert_eq!(uf.find(&0), uf.find(&2));
            assert_eq!(uf.find(&1), uf.find(&3));
        }};
    }

    grow_test!(HashUnionFindByRank::<usize>);
}

#[test]
pub fn grow_non_consecutive() {
    macro_rules! grow_test {
        ($ty: path) => {{
            type T = $ty;
            let mut uf = T::new([8, 1, 2]).unwrap();
            uf.union_by_rank(&8, &2).unwrap();

            assert_eq!(uf.find(&8), uf.find(&2));
            assert_eq!(uf.find(&1), Some(1));

            uf.add(9).unwrap();

            assert_eq!(uf.find(&8), uf.find(&2));
            assert_eq!(uf.find(&1), Some(1));

            uf.union_by_rank(&9, &1).unwrap();

            assert_eq!(uf.find(&8), uf.find(&2));
            assert_eq!(uf.find(&1), uf.find(&9));
        }};
    }

    grow_test!(HashUnionFindByRank::<usize>);
}

#[test]
pub fn union_by_rank() {
    macro_rules! by_rank_test {
        ($ty: path) => {{
            type T = $ty;
            let mut uf = T::new(0..20).unwrap();
            uf.union_by_rank(&0, &1).unwrap();
            uf.union_by_rank(&2, &0).unwrap();
            uf.union_by_rank(&0, &3).unwrap();

            assert_eq!(uf.find(&1), uf.find(&3));
            assert_ne!(uf.find(&2), uf.find(&8));
            assert_ne!(uf.find(&6), uf.find(&8));

            uf.union_by_rank(&5, &6).unwrap();
            uf.union_by_rank(&7, &8).unwrap();
            uf.union_by_rank(&5, &7).unwrap();

            assert_eq!(uf.find(&8), uf.find(&6));

            uf.union_by_rank(&10, &11).unwrap();
            uf.union_by_rank(&12, &13).unwrap();
            uf.union_by_rank(&11, &13).unwrap();

            assert_eq!(uf.find(&10), uf.find(&12));

            uf.union_by_rank(&14, &15).unwrap();
            uf.union_by_rank(&16, &17).unwrap();
            uf.union_by_rank(&14, &17).unwrap();

            assert_eq!(uf.find(&15), uf.find(&16));
        }};
    }

    by_rank_test!(HashUnionFindByRank::<usize>);
}
