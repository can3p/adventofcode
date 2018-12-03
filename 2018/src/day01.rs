use std::collections::HashMap;

#[test]
fn day01_1_test() {
    const INPUT: &str = include_str!("../input/01_test.txt");
    assert_eq!(day01_1(&INPUT), 2)
}

#[test]
fn day01_1_input() {
    const INPUT: &str = include_str!("../input/01_input.txt");
    assert_eq!(day01_1(&INPUT), 2)
}

#[test]
fn day01_2_input() {
    const INPUT: &str = include_str!("../input/01_input.txt");
    assert_eq!(day01_2(&INPUT).unwrap(), 2)
}

fn read_contents(contents: &str) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();

    for line in contents.lines() {
        v.push(line.parse::<i32>().unwrap());
    }

    return v;
}

fn day01_1(contents: &str) -> i32 {
    let changes: Vec<i32> = read_contents(&contents);
    let mut result: i32 = 0;
    for val in changes.iter() {
        result += val;
    }

    return result;
}

fn day01_2(contents: &str) -> Option<i32> {
    let changes: Vec<i32> = read_contents(&contents);
    let mut result: i32 = 0;
    let mut seen = HashMap::new();

    loop {
        for val in changes.iter() {
            result += val;
            if seen.contains_key(&result) {
                return Some(result);
            }

            seen.insert(result, 1);
        }
    }
}
