use regex::Regex;
use std::collections::HashMap;

#[test]
fn day04_1_test_sort() {
    const INPUT: &str = "[1518-11-05 00:55] wakes up\n[1518-11-01 00:30] falls asleep";

    assert_eq!(
        read_contents(INPUT),
        [
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-05 00:55] wakes up"
        ]
    );
}

#[test]
fn day04_1_test() {
    const INPUT: &str = include_str!("../input/04_test.txt");
    assert_eq!(day04_1(&INPUT), 240);
}

#[test]
fn day04_1_input() {
    const INPUT: &str = include_str!("../input/04_input.txt");
    assert_eq!(day04_1(&INPUT), 240);
}

#[test]
fn day04_2_test() {
    const INPUT: &str = include_str!("../input/04_test.txt");
    assert_eq!(day04_2(&INPUT), 4455);
}

#[test]
fn day04_2_input() {
    const INPUT: &str = include_str!("../input/04_input.txt");
    assert_eq!(day04_2(&INPUT), 4455);
}

fn day04_1(input: &str) -> i32 {
    let parsed = parse_sequence(read_contents(input));
    let mut counts = Vec::new();

    for key in parsed.keys() {
        let counted = parsed
            .get(key)
            .unwrap()
            .iter()
            .fold(0i32, |acc, v| acc + v[1] - v[0] + 1);

        let mut v = Vec::new();
        v.push(*key);
        v.push(counted);
        counts.push(v);
    }

    let mut id = 0;
    let mut max = 0;

    for count in counts.iter() {
        if count[1] > max {
            id = count[0].clone();
            max = count[1].clone();
        }
    }

    let minute_idx = find_max_minute(parsed.get(&id).unwrap().to_vec())[0];

    return id * minute_idx;
}

fn day04_2(input: &str) -> i32 {
    let parsed = parse_sequence(read_contents(input));
    let mut counts = Vec::new();

    for key in parsed.keys() {
        let max_info = find_max_minute(parsed.get(key).unwrap().to_vec());

        let mut v = Vec::new();
        v.push(*key);
        v.push(max_info[0]);
        v.push(max_info[1]);
        counts.push(v);
    }

    let mut id = 0;
    let mut max = 0;
    let mut minute = 0;

    for count in counts.iter() {
        if count[2] > max {
            id = count[0].clone();
            minute = count[1].clone();
            max = count[2].clone();
        }
    }

    return id * minute;
}

fn find_max_minute(strips: Vec<Vec<i32>>) -> Vec<i32> {
    let mut minutes: Vec<i32> = (0..60).map(|_| 0).collect();

    for strip in strips.iter() {
        for minute in ((strip[0]) as usize)..((strip[1] + 1) as usize) {
            minutes[minute] += 1;
        }
    }

    let mut minute_idx: i32 = 0;
    let mut max_minute = 0;

    for minute in 0..60 {
        if minutes[minute] > max_minute {
            minute_idx = minute as i32;
            max_minute = minutes[minute];
        }
    }

    return vec![minute_idx, max_minute];
}

fn parse_new_guard(line: &str) -> Option<Vec<i32>> {
    let re = Regex::new(
        r"(?x)
(?P<hh>\d+)
:
(?P<mm>\d+)
\]\sGuard\s\#
(?P<num>\d+)
",
    )
    .unwrap();

    let caps = re.captures(line);

    if caps.is_some() {
        let unwrapped = caps.unwrap();

        let ret = vec![
            unwrapped["hh"].parse::<i32>().unwrap(),
            unwrapped["mm"].parse::<i32>().unwrap(),
            unwrapped["num"].parse::<i32>().unwrap(),
        ];

        return Some(ret);
    }

    return None;
}

fn parse_wakes_up(line: &str) -> Option<Vec<i32>> {
    let re = Regex::new(
        r"(?x)
(?P<hh>\d+)
:
(?P<mm>\d+)
\]\swakes
",
    )
    .unwrap();

    let caps = re.captures(line);

    if caps.is_some() {
        let unwrapped = caps.unwrap();

        let ret = vec![
            unwrapped["hh"].parse::<i32>().unwrap(),
            unwrapped["mm"].parse::<i32>().unwrap(),
        ];

        return Some(ret);
    }

    return None;
}

fn parse_falls_asleep(line: &str) -> Option<Vec<i32>> {
    let re = Regex::new(
        r"(?x)
(?P<hh>\d+)
:
(?P<mm>\d+)
\]\sfalls
",
    )
    .unwrap();

    let caps = re.captures(line);

    if caps.is_some() {
        let unwrapped = caps.unwrap();

        let ret = vec![
            unwrapped["hh"].parse::<i32>().unwrap(),
            unwrapped["mm"].parse::<i32>().unwrap(),
        ];

        return Some(ret);
    }

    return None;
}

fn parse_sequence(lines: Vec<String>) -> HashMap<i32, Vec<Vec<i32>>> {
    let mut result: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();
    let mut current_guard = 0;
    let mut current_vec: Vec<Vec<i32>> = Vec::new();
    let mut current_minute = 0;
    //println!("lines {:?}", lines);

    for line in lines.iter() {
        //println!("line {:?}", line);
        let new_guard = parse_new_guard(line);

        if new_guard.is_some() {
            if current_guard > 0 {
                result.insert(current_guard, current_vec);
            }

            let g = new_guard.unwrap();
            current_guard = g[2];
            //println!("new guard {:?}", g);
            if g[0] == 0 {
                current_minute = g[1];
            } else {
                current_minute = 0;
            }

            if result.contains_key(&current_guard) {
                current_vec = result.get(&current_guard).unwrap().to_vec();
            } else {
                current_vec = Vec::new();
            }

            continue;
        }

        let falls_asleep = parse_falls_asleep(line);

        if falls_asleep.is_some() {
            let g = falls_asleep.unwrap();
            current_minute = g[1];
            //println!("falls asleep {:?}", g);

            continue;
        }

        let wakes_up = parse_wakes_up(line);

        if wakes_up.is_some() {
            let g = wakes_up.unwrap();
            //println!("wakes up {:?}", g);
            let v = vec![current_minute, g[1] - 1];
            current_minute = g[1] - 1;

            current_vec.push(v);

            continue;
        }
    }

    if current_guard > 0 {
        result.insert(current_guard, current_vec);
    }

    return result;
}

fn read_contents(contents: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for line in contents.lines() {
        v.push(line.into());
    }

    v.sort_unstable();

    return v;
}
