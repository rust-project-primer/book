use property_testing::{sort, sort_fixed};
use proptest::prelude::*;

fn is_sorted(input: Vec<u64>) -> bool {
    input.iter().zip(input.iter().skip(1)).all(|(left, right)| left <= right)
}

proptest! {
    #[test]
    fn can_sort(input: Vec<u64>) {
        prop_assert!(is_sorted(sort(input)));
    }
}
