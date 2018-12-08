use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn day07_1_test_input() {
    const INPUT: &str = "Step F must be finished before step E can begin.\nStep B must be finished before step E can begin.";
    assert_eq!(read_contents(INPUT), [['F', 'E'], ['B', 'E']]);
}

#[test]
fn day07_1_test_sort() {
    let mut vec = vec!['E', 'Z', 'A'];
    vec.sort_unstable_by(|a, b| a.cmp(&b));

    assert_eq!(vec, ['A', 'E', 'Z']);
}

#[test]
fn day07_1_test() {
    const INPUT: &str = include_str!("../input/07_test.txt");
    assert_eq!(day07_1(&INPUT), "CABDFE");
}

#[test]
fn day07_1_test2() {
    const INPUT: &str = include_str!("../input/07_test2.txt");
    assert_eq!(day07_1(&INPUT), "XIJACQ");
}

#[test]
fn day07_1_input() {
    const INPUT: &str = include_str!("../input/07_input.txt");
    assert_eq!(day07_1(&INPUT), "CABDFE");
}

#[test]
fn day07_2_work_time() {
    assert_eq!(compute_delay('A', 60), 61);
    assert_eq!(compute_delay('Z', 60), 86);
}

#[test]
fn day07_2_test() {
    const INPUT: &str = include_str!("../input/07_test.txt");
    assert_eq!(day07_2(&INPUT, 0, 2), 15);
}

#[test]
fn day07_2_input() {
    const INPUT: &str = include_str!("../input/07_input.txt");
    assert_eq!(day07_2(&INPUT, 60, 5), 15);
}

fn day07_2(input: &str, initial: i32, workers_num: i32) -> i32 {
    let mut elapsed_time = 0;
    let mut time_to_work: Vec<i32> = (0..workers_num).map(|_| 0).collect();
    let mut chars_in_progress: HashMap<usize, char> = HashMap::new();

    let inp_vec = read_contents(input);

    let mut straight: HashMap<char, Vec<char>> = HashMap::new();
    let mut reverse: HashMap<char, Vec<char>> = HashMap::new();

    for p in inp_vec.iter() {
        let mut svec: Vec<char>;
        let mut rvec: Vec<char>;

        if straight.get(&p[0]).is_some() {
            svec = straight.get(&p[0]).unwrap().to_vec();
        } else {
            svec = Vec::new();
        }

        svec.push(p[1]);
        straight.insert(p[0], svec);

        if reverse.get(&p[1]).is_some() {
            rvec = reverse.get(&p[1]).unwrap().to_vec();
        } else {
            rvec = Vec::new();
        }

        rvec.push(p[0]);
        reverse.insert(p[1], rvec);
    }

    let mut result: Vec<char> = Vec::new();
    let mut expanded: Vec<char> = Vec::new();
    let mut to_process: Vec<char> = Vec::new();
    let mut expanded_set: HashSet<char> = HashSet::new();

    for k in straight.keys() {
        if !reverse.contains_key(&k) {
            to_process.push(k.clone());
            //expanded.push(k.clone());
            //expanded_set.insert(k.clone());
        }
    }

    to_process.sort_unstable_by(|a, b| a.cmp(&b));
    to_process.reverse();

    while expanded.len() > 0 || chars_in_progress.keys().len() > 0 || to_process.len() > 0 {
        if chars_in_progress.keys().len() == time_to_work.len()
            || (chars_in_progress.keys().len() > 0 && expanded.len() == 0)
        {
            let mi = min_index(&time_to_work, false);
            let t = time_to_work[mi].clone();
            println!(
                "mi = {:?}, time_to_work = {:?}, chars_in_progress = {:?}",
                mi, time_to_work, chars_in_progress
            );
            elapsed_time += t;
            for i in 0..workers_num {
                let u = i as usize;
                println!(
                    "update worker {:?}, {:?} - {:?} = {:?}",
                    i,
                    time_to_work[u],
                    t,
                    time_to_work[u] - t
                );
                if time_to_work[u] > 0 {
                    time_to_work[u] = time_to_work[u] - t;

                    if time_to_work[u] == 0 {
                        let c = chars_in_progress.get(&u).unwrap().clone();
                        chars_in_progress.remove(&u);

                        expanded.push(c.clone());
                        expanded_set.insert(c.clone());
                    }
                }
            }
            println!(
                "mi = {:?}, time_to_work = {:?}, chars_in_progress = {:?}",
                mi, time_to_work, chars_in_progress
            );
            expanded.sort_unstable_by(|a, b| a.cmp(&b));
            expanded.reverse();
            continue;
        }

        while expanded.len() > 0 {
            let c = expanded.pop().unwrap();
            expanded_set.remove(&c);
            let mut deps: Vec<char> = Vec::new();
            if straight.get(&c).is_some() {
                let unfiltered_deps = straight.get(&c).unwrap().clone();
                for dep in unfiltered_deps.iter() {
                    if !expanded_set.contains(&dep)
                        && chars_in_progress.values().position(|d| d == dep).is_none()
                    {
                        deps.push(dep.clone());
                    }
                }
            }

            result.push(c);

            //println!("before expanded {:?}", expanded);
            //println!("before result {:?}", result);

            if deps.len() > 0 {
                straight.remove(&c);
                for dep in deps.iter() {
                    let rev_deps = reverse.get(&dep).unwrap();
                    //println!("reverse deps for {:?} are {:?} and expanded_set is {:?}", dep, rev_deps, expanded_set);

                    if rev_deps
                        .iter()
                        .position(|rd| {
                            expanded_set.contains(&rd)
                                || chars_in_progress.values().position(|d| d == dep).is_some()
                                || straight.get(&rd).is_some()
                                || deps.iter().position(|dd| rd == dd).is_some()
                        })
                        .is_none()
                    {
                        to_process.push(dep.clone());
                    }
                }

                // could not figure out how to do reverse sort
                //expanded.sort_unstable_by(|a, b| a.cmp(&b));
                //expanded.reverse();

                //println!("after expanded {:?}", expanded);
                //println!("after result {:?}", result);
            }
        }

        to_process.sort_unstable_by(|a, b| a.cmp(&b));
        to_process.reverse();
        println!("to_process {:?}", to_process);

        while chars_in_progress.keys().len() < time_to_work.len() && to_process.len() > 0 {
            let mi = min_index(&time_to_work, true);
            let c: char = to_process.pop().unwrap();
            time_to_work[mi] += compute_delay(c, initial);
            chars_in_progress.insert(mi, c);
        }

        println!("elapsed_time = {:?} expanded = {:?} to_process {:?} chars_in_progress = {:?}, time_to_work = {:?}",
                 elapsed_time, expanded, to_process, chars_in_progress, time_to_work);

        //return elapsed_time;
    }

    return elapsed_time;
}

fn min_index(array: &Vec<i32>, include_zero: bool) -> usize {
    let mut i = 0;
    let mut min = 1_000_000;

    for (j, &value) in array.iter().enumerate() {
        if (include_zero || value != 0) && value < min {
            i = j;
            min = value.clone();
        }
    }

    i
}

fn compute_delay(c: char, initial: i32) -> i32 {
    let mut chrs = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let position: usize = chrs.position(|cc| c == cc).unwrap();
    let delay: i32 = initial + (position as i32) + 1;

    return delay;
}

fn day07_1(input: &str) -> String {
    let inp_vec = read_contents(input);

    let mut straight: HashMap<char, Vec<char>> = HashMap::new();
    let mut reverse: HashMap<char, Vec<char>> = HashMap::new();

    for p in inp_vec.iter() {
        let mut svec: Vec<char>;
        let mut rvec: Vec<char>;

        if straight.get(&p[0]).is_some() {
            svec = straight.get(&p[0]).unwrap().to_vec();
        } else {
            svec = Vec::new();
        }

        svec.push(p[1]);
        straight.insert(p[0], svec);

        if reverse.get(&p[1]).is_some() {
            rvec = reverse.get(&p[1]).unwrap().to_vec();
        } else {
            rvec = Vec::new();
        }

        rvec.push(p[0]);
        reverse.insert(p[1], rvec);
    }

    let mut result: Vec<char> = Vec::new();
    let mut expanded: Vec<char> = Vec::new();
    let mut expanded_set: HashSet<char> = HashSet::new();

    for k in straight.keys() {
        if !reverse.contains_key(&k) {
            expanded.push(k.clone());
            expanded_set.insert(k.clone());
        }
    }

    expanded.sort_unstable_by(|a, b| a.cmp(&b));
    expanded.reverse();

    while expanded.len() > 0 {
        let c = expanded.pop().unwrap();
        expanded_set.remove(&c);
        let mut deps: Vec<char> = Vec::new();
        if straight.get(&c).is_some() {
            let unfiltered_deps = straight.get(&c).unwrap().clone();
            for dep in unfiltered_deps.iter() {
                if !expanded_set.contains(&dep) {
                    deps.push(dep.clone());
                }
            }
        }

        result.push(c);

        //println!("before expanded {:?}", expanded);
        //println!("before result {:?}", result);

        if deps.len() > 0 {
            straight.remove(&c);
            for dep in deps.iter() {
                let rev_deps = reverse.get(&dep).unwrap();
                //println!("reverse deps for {:?} are {:?} and expanded_set is {:?}", dep, rev_deps, expanded_set);

                if rev_deps
                    .iter()
                    .position(|rd| {
                        expanded_set.contains(&rd)
                            || straight.get(&rd).is_some()
                            || deps.iter().position(|dd| rd == dd).is_some()
                    })
                    .is_none()
                {
                    expanded.push(dep.clone());
                    expanded_set.insert(dep.clone());
                }
            }

            expanded.dedup_by(|a, b| a == b);

            // could not figure out how to do reverse sort
            expanded.sort_unstable_by(|a, b| a.cmp(&b));
            expanded.reverse();

            //println!("after expanded {:?}", expanded);
            //println!("after result {:?}", result);
        }
    }

    let s: String = result.into_iter().collect();

    return s;
}

fn read_contents(contents: &str) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let re = Regex::new(
        r"(?x)
Step\s
(?P<aa>[A-Z])
\smust\sbe\sfinished\sbefore\sstep\s
(?P<bb>[A-Z])
",
    )
    .unwrap();

    for line in contents.lines() {
        let caps = re.captures(line).unwrap();

        let ret = vec![
            caps["aa"].chars().next().unwrap(),
            caps["bb"].chars().next().unwrap(),
        ];

        result.push(ret);
    }

    return result;
}
// FDHKBRSEGJLBPHKNHRTCBVYAMOAMQIUHTVWZBXB
// FDHKMRSEGJLPKNHRTCVWYOAMQIUHTVZXB
// FDHKMRSEGJLPKNHRTCVWYOAMQIUHTVZXB
// FD

/*
[F -> [E]], [E -> [F]]
[F -> [E], B -> [E]], [E -> [B,F]]
[A -> [B], F -> [E], B -> [E]], [E -> [B,F], B -> [A]]
[A -> [B], C -> [F], F -> [E], B -> [E]], [E -> [B,F], B -> [A], F -> [C]]
[A -> [B], C -> [F], D -> [E],  F -> [E], B -> [E]], [E -> [B, D, F], B -> [A], F -> [C]]
[A -> [B, D], C -> [F], D -> [E],  F -> [E], B -> [E]], [E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
[A -> [B, D], C -> [A, F], D -> [E],  F -> [E], B -> [E]], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
, [C] [A -> [B, D], C -> [A, F], D -> [E],  F -> [E], B -> [E]], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
C, [A,F] [A -> [B, D], D -> [E],  F -> [E], B -> [E]], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
CA, [BDF] [D -> [E],  F -> [E], B -> [E]], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
CAB, [DF] [D -> [E],  F -> [E]], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
CABD, [F] [F -> [E]], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
CABDF, [E] [], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
CABDFE, [] [], [A -> [C] E -> [B, D, F], B -> [A], D -> [A],  F -> [C]]
*/
