use property_testing::sort;
use proptest::prelude::*;

proptest! {
    #[test]
    fn output_is_sorted(input: Vec<u16>) {
        let sorted = sort(input.clone());
        let is_sorted = sorted
            .iter()
            .zip(sorted.iter().skip(1))
            .all(|(left, right)| left <= right);
        assert!(is_sorted);
    }

    #[test]
    fn output_same_contents(input: Vec<u16>) {
        let mut sorted = sort(input.clone());
        for value in input {
            let index = sorted.iter().position(|element| *element == value).unwrap();
            sorted.remove(index);
        }
        assert!(sorted.is_empty());
    }
}
