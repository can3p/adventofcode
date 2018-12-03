use std::collections::HashMap;

#[test]
fn day02_1_test() {
    const INPUT: &str = include_str!("../input/02_test.txt");
    assert_eq!(day02_1(&INPUT), 12);
}

#[test]
fn day02_1_input() {
    const INPUT: &str = include_str!("../input/02_input.txt");
    assert_eq!(day02_1(&INPUT), 2);
}

#[test]
fn day02_2_input() {
    const INPUT: &str = include_str!("../input/02_input.txt");
    assert_eq!(day02_2(&INPUT).unwrap(), "abcd");
}

fn read_contents(contents: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for line in contents.lines() {
        v.push(line.into());
    }

    return v;
}

fn day02_1(contents: &str) -> i32 {
    let changes: Vec<String> = read_contents(&contents);
    let mut res: Vec<i32> = vec![0, 0];

    for s in changes.iter() {
        let found = find_seqs(&s);
        res[0] += found[0];
        res[1] += found[1];
    }

    return res[0] * res[1];
}

fn day02_2(contents: &str) -> Option<String> {
    let changes: Vec<String> = read_contents(&contents);

    for idx in 0..changes.len() {
        let similar = compare_input(idx, &changes);

        if similar.is_some() {
            return similar;
        }
    }

    return None;
}

fn find_seqs(s: &String) -> Vec<i32> {
    let mut counts = HashMap::new();
    let mut res: Vec<i32> = vec![0, 0];

    for c in s.chars() {
        let count = match counts.get(&c) {
            Some(count) => count + 1,
            None => 1
        };

        counts.insert(c, count);
    }

    for val in counts.values() {
        if *val == 2 {
            res[0] = 1;
        }

        if *val == 3 {
            res[1] = 1;
        }
    }

    return res;
}


fn find_similariry(str1: &String, str2: &String) -> Option<String> {
    let mut err_cnt = 0;
    let mut res = String::new();

    for idx in 0..str1.len() {
        let sub1: String = str1.get(idx..(idx+1)).unwrap().to_string();
        let sub2: String = str2.get(idx..(idx+1)).unwrap().to_string();
        if sub1 != sub2 {
            err_cnt += 1;
            if err_cnt > 1 {
                return None;
            }
        } else {
            res.push_str(&sub1);
        }
    }

    return Some(res);
}

fn compare_input(current: usize, strs: &Vec<String>) -> Option<String> {
    for idx in 0..strs.len() {
        if idx != current {
            let diff = find_similariry(&strs[idx], &strs[current]);
            if diff.is_some() {
                return diff;
            }
        }
    }

    return None;
}
