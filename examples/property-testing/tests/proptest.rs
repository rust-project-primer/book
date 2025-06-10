use property_testing::{sort, sort_fixed};
use proptest::prelude::*;

proptest! {
    #[test]
    fn can_sort(input: Vec<u64>) {
        let sorted = sort(input);
    }
}
