use regex::Regex;
use std::collections::HashMap;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

#[test]
fn test_day10_1_parse() {
    const INPUT: &str = "position=<-43576, -32684> velocity=< 4,  3>\nposition=< 33043,  22029> velocity=<-3, -2>\n";

    let stars: Vec<Star> = read_contents(INPUT);

    assert_eq!(stars[0].x, -43576);
    assert_eq!(stars[0].y, -32684);
    assert_eq!(stars[0].vx, 4);
    assert_eq!(stars[0].vy, 3);

    assert_eq!(stars[1].x, 33043);
    assert_eq!(stars[1].y, 22029);
    assert_eq!(stars[1].vx, -3);
    assert_eq!(stars[1].vy, -2);
}

#[test]
fn test_day10_1_print() {
    let stars: Vec<Star> = vec![
        Star { x: -1, y: -1, vx: 0, vy: 0 },
        Star { x: 1, y: 1, vx: 0, vy: 0 },
    ];

    let out:String = print_stars(&stars, 1);
    println!("{}", out);

    let test:&str =  ".....\n.*...\n.....\n...*.\n.....";
    println!("{}", test);

    assert_eq!(out, test);
}

#[test]
fn test_day10_1_test() {
    const INPUT: &str = include_str!("../input/10_test.txt");
    let mut stars: Vec<Star> = read_contents(INPUT);

    let sol = day10_1(&mut stars, 5);

    println!("{}", print_stars(&stars, 2));

    assert_eq!(sol, Some(3));
}

#[test]
fn test_day10_1_input() {
    const INPUT: &str = include_str!("../input/10_input.txt");
    let mut stars: Vec<Star> = read_contents(INPUT);

    let sol = day10_1(&mut stars, 11_000);
    let write_file = File::create("day10_1_out.txt").unwrap();
    let mut writer = BufWriter::new(&write_file);
    write!(&mut writer, "{}", print_stars(&stars, 2));
    //println!("{}", print_stars(&stars, 2));

    assert_eq!(sol, Some(4));
}

fn day10_1(stars: &mut Vec<Star>, max_turns: i32) -> Option<i32> {
    let mut current_second = 0;
    let mut dim = 1_000_000_000;

    while current_second < max_turns {
        current_second += 1;

        let mut minx = 1_000_000;
        let mut miny = 1_000_000;
        let mut maxx = -1_000_000;
        let mut maxy = -1_000_000;
        for star in stars.iter_mut() {
            star.x += star.vx;
            star.y += star.vy;
            if star.x < minx { minx = star.x }
            if star.y < miny { miny = star.y }
            if star.x > maxx { maxx = star.x }
            if star.y > maxy { maxy = star.y }
        }

        let new_dim = (maxx - minx).abs();

        if new_dim > dim {
            for star in stars.iter_mut() {
                star.x -= star.vx;
                star.y -= star.vy;
            }
            return Some(current_second - 1);
        }

        dim = new_dim;
    }

    return None;
}


struct Star {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn print_stars(stars: &Vec<Star>, margin: i32) -> String {
    let mut minx = 1_000_000;
    let mut miny = 1_000_000;
    let mut maxx = -1_000_000;
    let mut maxy = -1_000_000;

    for star in stars.iter() {
        if star.x < minx { minx = star.x }
        if star.y < miny { miny = star.y }
        if star.x > maxx { maxx = star.x }
        if star.y > maxy { maxy = star.y }
    }

    let mut s: Vec<Vec<char>> = (0..(maxy - miny + 2 * margin + 1))
                        .map(|_| Vec::with_capacity((maxx - minx + 2*margin + 1) as usize))
                        .collect();

    for _ in 0 .. (maxx - minx + 2*margin + 1) as usize {
        for y in 0 .. (maxy - miny + 2*margin + 1) as usize {
            s[y].push('.');
        }
    }

    for star in stars.iter() {
        s[(star.y - (miny - margin)) as usize][(star.x - (minx - margin)) as usize] = '*';
    }

    let mut strs: Vec<String> = Vec::new();
    for line in s.iter() {
        strs.push(line.into_iter().collect());
    }

    return strs.join("\n");
}

fn read_contents(contents: &str) -> Vec<Star> {
    let mut v: Vec<Star> = Vec::new();
    for line in contents.lines() {
        let re = Regex::new(
            r"(?x)
position=<\s*(?P<x>-?\d+),\s*(?P<y>-?\d+)>\s+
velocity=<\s*(?P<vx>-?\d+),\s*(?P<vy>-?\d+)>
",
        )
        .unwrap();

        let caps = re.captures(line).unwrap();

        let star = Star {
            x:   caps["x"].parse::<i32>().unwrap(),
            y:   caps["y"].parse::<i32>().unwrap(),
            vx: caps["vx"].parse::<i32>().unwrap(),
            vy: caps["vy"].parse::<i32>().unwrap(),
        };

        v.push(star);
    }

    return v;
}
