use std::collections::HashSet;

#[test]
fn day06_1_test_input() {
    const INPUT: &str = include_str!("../input/06_test.txt");

    assert_eq!(read_contents(INPUT),[[1,1],[1,6],[8,3],[3,4],[5,5],[8,9]]);
}

#[test]
fn day06_1_test_area_size() {
    const INPUT: &str = include_str!("../input/06_test.txt");

    assert_eq!(area_size(&read_contents(INPUT)),[[1,1], [8,9]]);
}

#[test]
fn day06_1_test_distances() {
    const INPUT: &str = include_str!("../input/06_test.txt");

    assert_eq!(distances(vec![0,0], &vec![1,1], &read_contents(INPUT)),
               [[0,0], [1,5], [3,5], [4,8], [2,9], [5, 15]]);
}

#[test]
fn day06_1_test_edges() {
    assert_eq!(is_edge(0, 3, 3), true);
    assert_eq!(is_edge(1, 3, 3), true);
    assert_eq!(is_edge(3, 3, 3), true);
    assert_eq!(is_edge(5, 3, 3), true);
    assert_eq!(is_edge(4, 3, 3), false);
    assert_eq!(is_edge(8, 3, 3), true);
}

#[test]
fn day06_1_test() {
    const INPUT: &str = include_str!("../input/06_test.txt");

    assert_eq!(day06_1(read_contents(INPUT)), 17);
}

#[test]
fn day06_1_input() {
    const INPUT: &str = include_str!("../input/06_input.txt");

    assert_eq!(day06_1(read_contents(INPUT)), 17);
}

#[test]
fn day06_2_test() {
    const INPUT: &str = include_str!("../input/06_test.txt");

    assert_eq!(day06_2(read_contents(INPUT), 32), 16);
}

#[test]
fn day06_2_input() {
    const INPUT: &str = include_str!("../input/06_input.txt");

    assert_eq!(day06_2(read_contents(INPUT), 10_000), 16);
}

fn day06_1(points: Vec<Vec<i32>>) -> i32 {
    let area_corners = area_size(&points);
    let tl: Vec<i32> = area_corners[0].clone();
    let w = area_corners[1][0] - area_corners[0][0] + 1;
    let h = area_corners[1][1] - area_corners[0][1] + 1;

    let mut area: Vec<i32> = Vec::new();
    let mut edges: HashSet<i32> = HashSet::new();
    let mut areas: Vec<i32> = (0..points.len()).map(|_| 0).collect();

    for p in 0..(w*h) {
        let x = p.wrapping_rem(w);
        let y = p.wrapping_div(w);
        let dsts = distances(vec![x,y], &tl, &points);
        let mut idx;

        if dsts[0][1] == dsts[1][1] {
            idx = -1;
        } else {
            idx = dsts[0][0];

            if is_edge(p, w, h) {
                edges.insert(idx);
            }
        }
        area.push(idx);
    }

    for idx in area.iter() {
        if idx > &-1 && !edges.contains(idx) {
            areas[*idx as usize] += 1;
        }
    }

    return areas.iter().max().unwrap().clone();
}

fn day06_2(points: Vec<Vec<i32>>, treshold: i32) -> i32 {
    let area_corners = area_size(&points);
    let tl: Vec<i32> = area_corners[0].clone();
    let w = area_corners[1][0] - area_corners[0][0] + 1;
    let h = area_corners[1][1] - area_corners[0][1] + 1;

    let mut count = 0;

    for p in 0..(w*h) {
        let x = p.wrapping_rem(w);
        let y = p.wrapping_div(w);
        let dsts = distances(vec![x,y], &tl, &points);

        let total: i32 = dsts.iter().map(|x| x[1]).sum();
        if total < treshold {
            count += 1;
        }
    }

    return count;
}

fn is_edge(p: i32, w: i32, h: i32) -> bool {
    let x = p.wrapping_rem(w);
    let y = p.wrapping_div(w);

    if y == 0 { return true }
    if x == 0 { return true }
    if x == w - 1 { return true }
    if y == h - 1 { return true }

    return false;
}

fn distances(p: Vec<i32>, tl: &Vec<i32>,  points: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for (i, v) in points.iter().enumerate() {
        result.push(vec![i as i32, (p[0] - v[0] + tl[0]).abs() +
                    (p[1] - v[1] + tl[1]).abs()]);
    }

    result.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

    return result;
}

fn area_size(points: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut min = vec![1_000_000, 1_000_000];
    let mut max = vec![0, 0];

    for v in points.iter() {
        if v[0] < min[0] { min[0] = v[0]; }
        if v[1] < min[1] { min[1] = v[1]; }
        if v[0] > max[0] { max[0] = v[0]; }
        if v[1] > max[1] { max[1] = v[1]; }
    }

    return vec![min, max];
}

fn read_contents(contents: &str) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let mut v: Vec<i32> = Vec::new();

        for part in line.split(", ") {
            v.push(part.parse::<i32>().unwrap());
        }

        result.push(v);
    }

    return result;
}
