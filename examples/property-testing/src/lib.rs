#[cfg(not(feature = "fixed"))]
// ANCHOR: sort_broken
pub fn sort(mut input: Vec<u16>) -> Vec<u16> {
    let mut output = Vec::new();
    while let Some(value) = input.iter().min().copied() {
        input.retain(|v| v != &value);
        output.push(value);
    }
    output
}
// ANCHOR_END: sort_broken

#[cfg(feature = "fixed")]
// ANCHOR: sort_fixed
pub fn sort(mut input: Vec<u16>) -> Vec<u16> {
    let mut output = Vec::new();
    while let Some(value) = input.iter().min().copied() {
        let count = input.iter().filter(|v| *v == &value).count();
        input.retain(|v| v != &value);
        for _ in 0..count {
            output.push(value);
        }
    }
    output
}
// ANCHOR_END: sort_fixed

// ANCHOR: unit_test
#[test]
fn test_sort() {
    assert_eq!(sort(vec![]), vec![]);
    assert_eq!(sort(vec![2, 1, 3]), vec![1, 2, 3]);
}
// ANCHOR_END: unit_test
