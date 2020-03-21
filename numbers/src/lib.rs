pub fn print_nums(limit: u8) {
    let nums = generate_sequence(limit);
    // let nums = [1, 2, 3, 4, 5];
    output_sequence(&nums)
}

fn output_sequence(numbers: &[u8]) {
    for number in numbers {
        println!("{}", number)
    }
}

fn generate_sequence(limit: u8) -> Vec<u8> {
    (1..=limit).collect();
}

#[test]
fn generate_sequence_should_work() {
    let result = generate_sequence(3);
    assert_eq!(result, &[1, 2, 3]);
}