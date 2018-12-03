use regex::Regex;
use std::collections::HashSet;


#[test]
fn day03_1_test() {
    const INPUT: &str = include_str!("../input/03_test.txt");
    assert_eq!(day03_1(&INPUT, 9, 8)[0], 4);
}

#[test]
fn day03_1_input() {
    const INPUT: &str = include_str!("../input/03_input.txt");
    assert_eq!(day03_1(&INPUT, 10000, 10000)[0], 2);
}

#[test]
fn day03_2_test() {
    const INPUT: &str = include_str!("../input/03_test.txt");
    assert_eq!(day03_1(&INPUT, 9, 8)[1], 3);
}

#[test]
fn day03_2_input() {
    const INPUT: &str = include_str!("../input/03_input.txt");
    assert_eq!(day03_1(&INPUT, 10000, 10000)[1], 2);
}

struct Rect {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

fn read_contents(contents: &str) -> Vec<Rect> {
    let mut v: Vec<Rect> = Vec::new();
    for line in contents.lines() {
        let re = Regex::new(r"(?x)
\#(?P<id>\d+)
\s@\s
(?P<x>\d+)
,
(?P<y>\d+)
:\s
(?P<w>\d+)
x
(?P<h>\d+)
").unwrap();

        let caps = re.captures(line).unwrap();

        let rect = Rect {
            id: caps["id"].parse::<i32>().unwrap(),
            x: caps["x"].parse::<i32>().unwrap(),
            y: caps["y"].parse::<i32>().unwrap(),
            w: caps["w"].parse::<i32>().unwrap(),
            h: caps["h"].parse::<i32>().unwrap()
        };

        v.push(rect);
    }

    return v;
}

fn day03_1(contents: &str, h: i32, w: i32) -> Vec<i32> {
    let changes: Vec<Rect> = read_contents(&contents);
    let mut area: Vec<i32> = (0..(h*w)).map(|_| 0).collect();
    let mut result: i32 = 0;
    let mut no_overlaps: HashSet<i32> = HashSet::new();
    let mut overlaps: HashSet<i32> = HashSet::new();

    for r in changes.iter() {
        for x in r.x..(r.x + r.w) {
            for y in r.y..(r.y + r.h) {
                let idx = (x + y * h) as usize;
                if area[idx] > 0 {
                    no_overlaps.remove(&area[idx]);
                    no_overlaps.remove(&r.id);
                    overlaps.insert(area[idx]);
                    overlaps.insert(r.id);
                    area[idx] = -1;
                    result += 1;
                } else if area[idx] != -1 {
                    area[idx] = r.id;
                    if !overlaps.contains(&r.id) {
                        no_overlaps.insert(r.id);
                    }
                }
            }
        }
    }

    println!("{:?}", no_overlaps);

    let ret = vec![result, *no_overlaps.iter().next().unwrap()];

    return ret;
}
