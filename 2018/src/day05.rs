#[test]
fn day05_1_test_input() {
    const INPUT: &str = include_str!("../input/05_test.txt");

    assert_eq!(read_contents(INPUT), "dabAcCaCBAcCcaDA");
}

#[test]
fn day05_1_test_opposite() {
    let a: char = 'a';
    let mut b: char = 'b';
    assert_eq!(opposite_chars(a, b), false);

    b = 'A';
    assert_eq!(opposite_chars(a, b), true);
}

#[test]
fn day05_1_test_reduce() {
    const INPUT: &str = include_str!("../input/05_test.txt");

    assert_eq!(reduce_input(read_contents(INPUT).to_string()), "dabCBAcaDA");
}

#[test]
fn day05_1_test() {
    const INPUT: &str = include_str!("../input/05_test.txt");
    assert_eq!(day05_1(&INPUT), 10);
}

#[test]
fn day05_1_input() {
    const INPUT: &str = include_str!("../input/05_input.txt");
    assert_eq!(day05_1(&INPUT), 10);
}

#[test]
fn day05_2_test_reduction() {
    const INPUT: &str = include_str!("../input/05_test.txt");

    assert_eq!(eliminate(read_contents(INPUT).to_string(), 'a'), "dbcCCBcCcD");
}

#[test]
fn day05_2_test() {
    const INPUT: &str = include_str!("../input/05_test.txt");

    assert_eq!(day05_2(&INPUT), 4);
}

#[test]
fn day05_2_input() {
    const INPUT: &str = include_str!("../input/05_input.txt");

    assert_eq!(day05_2(&INPUT), 4);
}

fn day05_1(input: &str) -> i32 {
    return reduce_input(read_contents(input.into()).to_string()).len() as i32;
}

fn day05_2(input: &str) -> i32 {
    let mut min: i32 = 1_000_000_00;
    let v = "abcdefghhijklmnopqrstuvwzyz";
    let inp = read_contents(input.into()).to_string();
    for c in  v.chars() {
        let len = reduce_input(eliminate(inp.clone(), c)).len() as i32;
        if len < min {
            min = len;
        }
    }

    return min;
}

fn eliminate(s: String, c: char) -> String {
    return s.chars()
            .filter(|x| !x.eq_ignore_ascii_case(&c))
            .collect();
}

fn reduce_input(s: String) -> String {
    let mut chars = s.chars();
    let mut consumed: Vec<char> = Vec::new();

    let mut previous: char = chars.next().unwrap();
    let mut current: Option<char> = chars.next();

    while current.is_some() {
        let current_char = current.unwrap();

        if opposite_chars(previous, current_char) {
            if consumed.len() == 0 {
                previous = chars.next().unwrap();
                current = chars.next();
            } else {
                previous = consumed.pop().unwrap();
                current = chars.next();
            }
        } else {
            consumed.push(previous);
            previous = current_char;
            current = chars.next();
        }
    }

    consumed.push(previous);
    let result: String = consumed.into_iter().collect();

    return result;
}

fn opposite_chars(a: char, b: char) -> bool {
    if (a.is_ascii_lowercase() && !b.is_ascii_lowercase() ||
        b.is_ascii_lowercase() && !a.is_ascii_lowercase()) &&
        (a.to_ascii_lowercase() == b.to_ascii_lowercase()) {
            return true;
        }
    return false;
}

fn read_contents(contents: &str) -> &str {
    let input = contents.lines().next().unwrap();

    return input;
}
