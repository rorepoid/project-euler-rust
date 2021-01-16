pub fn run(limit: u32) -> u32 {
    multiple_3_and_5(limit).iter().sum()
}

fn multiple_3_and_5(limit: u32) -> Vec<u32> {
    let mut results: Vec<u32> = vec![];

    for x in 1..limit {
        if (x % 3 == 0) || (x % 5 == 0) {
            results.push(x);
        }
    }

    results
}

#[test]
fn multiple_of_3_are() {
    assert_eq!(multiple_3_and_5(10), [3, 5, 6, 9]);
}

#[test]
fn multiple_of_3_are_with_limit_of_10_is_23() {
    assert_eq!(run(10), 23);
}