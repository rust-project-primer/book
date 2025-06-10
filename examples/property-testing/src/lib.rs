

pub fn sort(mut input: Vec<u64>) -> Vec<u64> {
    let mut output = Vec::new();
    while let Some(value) = input.iter().min().copied() {
        input.retain(|v| v != &value);
        output.push(value);
    }
    output
}

pub fn sort_fixed(mut input: Vec<u64>) -> Vec<u64> {
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
