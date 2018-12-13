use std::collections::HashMap;
#[allow(unused_imports)]
use std::time::{Duration, Instant};

#[test]
fn test_day12_1_precompile() {
    let vec: Vec<u64> = vec![1,1,1,1,1];
    let mut calculated: Vec<u64> = vec![0,0,0,0,0];

    precompile(&vec, &mut calculated);
    assert_eq!(calculated, [0b00111,
                            0b01111,
                            0b11111,
                            0b11110,
                            0b11100]);
}

#[test]
fn test_day12_1_parse() {
    const INPUT: &str = "initial state: #...#\n\n...## => #\n..#.. => .";
    let (rules, vec) = read_contents(INPUT, 5, 5);
    println!("rules = {:?}", rules);
    assert_eq!(vec, [0,0,0,0,0,1,0,0,0,1,0,0,0,0,0]);
    assert_eq!(rules[&0b00011], 1);
    assert_eq!(rules[&0b00100], 0);
}

#[test]
fn test_day12_1_test1() {
    const INPUT: &str = "initial state: #####\n\n\n##### => #";
    assert_eq!(day12_1(INPUT, 1), 2);
}

#[test]
fn test_day12_1_test2() {
    const INPUT: &str = "initial state: #####\n\n\n##### => #\n...## => #";
    assert_eq!(day12_1(INPUT, 1), 1);
}

#[test]
fn test_day12_1_test() {
    const INPUT: &str = include_str!("../input/12_test.txt");
    assert_eq!(day12_1(INPUT, 20), 325);
}

#[test]
fn input_day12_1_input() {
    const INPUT: &str = include_str!("../input/12_input.txt");
    assert_eq!(day12_1(INPUT, 20), 325);
}

#[test]
fn input_day12_2_input() {
    const INPUT: &str = include_str!("../input/12_input.txt");
    let start = Instant::now();
    // check where patterns gets stable , let's say we first stable item is
    // at gen z and with difference x in values and initial value y
    // then at 50_000_000_000 iterations the sum will be
    // y + (50_000_000_000 - z) * y
    let val = day12_1(INPUT, 500);
    let duration = start.elapsed();
    println!("Time elapsed in test is: {:?}", duration);
    assert_eq!(val, 325);
}

fn day12_1(input: &str, gens: i32) -> i32 {

    let mut vecs: Vec<Vec<u64>> = Vec::new();
    let offset_back: i32 = gens;
    let offset_front: i32 = gens;
    let mut idx = 0;
    let mut next_idx = 1;

    let (rules, vec) = read_contents(input, offset_back, offset_front);
    let len = vec.len();
    vecs.push(vec.clone());
    vecs.push(vec);
    let mut calculated: Vec<u64> = (0..len).map(|_| 0).collect();
    //println!("{:?}", vecs[idx]);
    let mut result: i32 = 0;

    for cur_gen in 1..(gens+1) {
        result = 0;
        precompile(&vecs[idx], &mut calculated);
        //println!("calculated = {:?} and rules = {:?}", calculated, rules);
        for i in 0..len {
            let value = rules.get(&calculated[i]);
            //println!("value = {:?} and i = {:?} and calculated = {:?}",
                     //value, i, calculated[i]);
            if value.is_none() {
                vecs[next_idx][i] = 0;
            } else {
                vecs[next_idx][i] = value.unwrap().clone();
            }
            //println!("vec = {:?}", vecs[next_idx]);
        }

        idx = 1 - idx;
        next_idx = 1 - next_idx;
        //println!("{:?}", vecs[idx]);

        for i in 0..len {
            if vecs[idx][i] > 0 {
                result += (i as i32) - offset_back;
            }
        }

        println!("gen = {}, result = {}", cur_gen, result);
    }


    return result;
}

fn precompile(vec: &Vec<u64>, out: &mut Vec<u64>) {

    let len = vec.len();
    for idx in 0..len {
        let mut value = 0;
        let mut dec = 5;
        for i in (idx as i32)-2..(idx as i32) + 3 {
            dec -= 1;

            if i >= len as i32 {
                break;
            }

            if i >= 0 {
                value += vec[i as usize] << dec;
            }
        }

        out[idx] = value;
    }
}

fn read_contents(input: &str, offset_back: i32, offset_front: i32) -> (HashMap<u64,u64>, Vec<u64>) {
    let mut rules: HashMap<u64,u64> = HashMap::new();
    let mut bv:Vec<u64> = (0..offset_back).map(|_| 0).collect();

    for line in input.lines() {
        if line.len() > 10 {
            for c in line.chars() {
                if c == '#' {
                    bv.push(1);
                } else if c == '.' {
                    bv.push(0);
                }
            }
        } else if line.len() == 0 {
            continue;
        } else {
            let mut chars = line.chars();
            let mut key: u64 = 0;
            for i in 0..5 {
                let c = chars.next().unwrap();
                if c == '#' {
                    key += 1 << (4-i);
                }
            }

            for c in chars {
                if c == '#' {
                    rules.insert(key, 1);
                    break;
                } else if c == '.' {
                    rules.insert(key, 0);
                    break;
                }
            }
        }
    }

    for _ in 0..offset_front {
        bv.push(0);
    }

    return (rules, bv);
}
