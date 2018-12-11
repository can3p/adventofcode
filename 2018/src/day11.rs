use rayon::prelude::*;

#[test]
fn day11_1_cell() {
    assert_eq!(calc_level(3,5,8), 4);
    assert_eq!(calc_level(122,79,57), -5);
    assert_eq!(calc_level(217,196,39), 0);
    assert_eq!(calc_level(101,153,71), 4);
    assert_eq!(calc_level(33,45,18), 4);
    assert_eq!(calc_level(34,45,18), 4);
    assert_eq!(calc_level(35,45,18), 4);
    assert_eq!(calc_level(33,46,18), 3);
    assert_eq!(calc_level(34,46,18), 3);
    assert_eq!(calc_level(35,46,18), 4);
    assert_eq!(calc_level(33,47,18), 1);
    assert_eq!(calc_level(34,47,18), 2);
    assert_eq!(calc_level(35,47,18), 4);
}

#[test]
fn day11_1_test() {
    assert_eq!(day11_1(18, 3), (33, 45, 29));
    assert_eq!(day11_1(42, 3), (21, 61, 30));
}

#[test]
fn day11_1_input() {
    assert_eq!(day11_1(5034, 3), (33, 45,29));
}

#[test]
fn day11_2_test() {
    assert_eq!(day11_2(18), (90,269,16));
    //assert_eq!(day11_2(42), (232,251,12));
}

#[test]
fn day11_2_input() {
    assert_eq!(day11_2(5034), (90,269,16));
    //assert_eq!(day11_2(42), (232,251,12));
}


fn day11_2(sn: i32) -> (i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut val = -1_000_000;
    let mut max = 0;

    let sizes: Vec<usize> = (1..301).map(|x| x).collect();
    let pairs: Vec<(i32,i32,i32)> = sizes
        .par_iter()
        .map(|x| day11_1(sn, *x))
        .collect();

    for (j, &size) in sizes.iter().enumerate() {
        let (mx,my,mval) = pairs[j];
        if mval > val {
            val = mval;
            max = size;
            x = mx;
            y = my;
        }
    }

    return (x,y,max as i32);
}


/*
fn day11_2(sn: i32) -> (i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut val = -1_000_000;
    let mut max = 0;

    for size in 1..30 {
        let (mx,my,mval) = day11_1(sn, size);
        if mval > val {
            val = mval;
            max = size;
            x = mx;
            y = my;
        }
    }

    return (x,y,max as i32);
}
*/


fn day11_1(sn: i32, size: usize) -> (i32, i32, i32) {
    let mut max = -1_000_000;
    let mut maxx = 0;
    let mut maxy = 0;
    let mut rolling;

    let mut v: Vec<Vec<i32>> = Vec::new();

    for y in 1..301 {
        v.push(Vec::with_capacity(300));
        for x in 1..301 {
            v[y-1].push(calc_level(x,y as i32,sn));
        }
    }

    for y in 0..(300 - size) {
        rolling = 0;
        for my in 0..size {
            for mx in 0..size {
                rolling += v[y+my][mx];
            }
        }

        if rolling > max {
            max = rolling;
            maxx = 1;
            maxy = y + 1;
        }

        for x in 1..(300 - size) {
            for my in 0..size {
                rolling += v[y+my][x + size - 1] - v[y+my][x - 1];
            }

            if rolling > max {
                max = rolling;
                maxx = x + 1;
                maxy = y + 1;
            }
        }
    }

    return (maxx as i32,maxy as i32, max);
}

fn calc_level(x: i32, y: i32, sn: i32) -> i32 {
    let rack_id = x + 10;
    let mut level = (rack_id * y + sn) * rack_id;

    if level < 100 {
        level = 0;
    } else {
        level = (level % 1000 - level % 100) / 100
    }

    level -= 5;

    return level;
}
