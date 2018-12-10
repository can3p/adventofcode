#[test]
fn day09_1_test1() {
    //let mut game = Game::new(9, 3);
    let mut game = Game::new(9, 25);

    assert_eq!(game.run(), 32);
}

#[test]
fn day09_1_test2() {
    let mut game = Game::new(10, 1618);

    assert_eq!(game.run(), 8317);
}

#[test]
fn day09_1_input() {
    let mut game = Game::new(419, 71052);

    assert_eq!(game.run(), 8317);
}

#[test]
fn day09_2_input() {
    let mut game = Game::new(419, 71052 * 100);

    assert_eq!(game.run(), 8317);
}

struct Marble {
    value: i32,
    next: usize,
    prev: usize,
}

struct Game {
    marbles: Vec<Marble>,
    current_idx: usize,
    current_player: usize,
    next_marble: i32,
    scores: Vec<i64>,
    max_marble: i32,
}

impl Game {
    fn new(num_players: i32, max_marble: i32) -> Game {
        let scores: Vec<i64> = (0..num_players).map(|_| 0).collect();
        let mut marbles: Vec<Marble> = Vec::with_capacity((max_marble + 1) as usize);
        marbles.push(Marble {
            value: 0,
            prev: 0,
            next: 0,
        });

        let game = Game {
            marbles: marbles,
            current_idx: 0,
            current_player: scores.len() - 1, // to start first move from the first player
            next_marble: 1,
            scores: scores,
            max_marble: max_marble
        };

        return game;
    }

    fn is_finished(&self) -> bool {
        return self.next_marble > self.max_marble;
    }

    fn run(&mut self) -> i64 {
        while !self.is_finished() {
            self.next_move();
            //self.print();
            //println!("current_idx = {:?}, marbles = {:?}", self.current_idx, self.marbles);
        }

        return self.scores.iter().max().unwrap().clone();
    }

    fn print(&self) {
        let mut current_idx = self.marbles[0].next;
        print!("{:?} ", self.marbles[0].value);

        while current_idx != 0 {
            print!("{:?} ", self.marbles[current_idx].value);
            current_idx = self.marbles[current_idx].next;
        }
        println!("");
    }

    fn next_move(&mut self) {
        self.current_player = (self.current_player + 1 + self.scores.len()).wrapping_rem(self.scores.len());
        let marble = self.next_marble;
        self.next_marble += 1;

        if marble.wrapping_rem(23) != 0 {
            self.move_current(1);
            self.insert_after(marble);
        } else {
            self.add_to_score(marble);
            self.move_current(-7);
            let marble = self.eject_current();
            self.add_to_score(marble);
        }
    }

    fn insert_after(&mut self, value: i32) {
        let idx: usize = self.marbles.len();
        let next = self.marbles[self.current_idx].next;

        let marble = Marble {
            value: value,
            next: next,
            prev: self.current_idx,
        };

        self.marbles.push(marble);
        self.marbles[self.current_idx].next = idx;
        self.marbles[next].prev = idx;
        self.current_idx = idx;
    }

    fn eject_current(&mut self) -> i32 {
        let next = self.marbles[self.current_idx].next;
        let prev = self.marbles[self.current_idx].prev;
        let value = self.marbles[self.current_idx].value;

        self.marbles[prev].next = self.marbles[self.current_idx].next;
        self.marbles[next].prev = self.marbles[self.current_idx].prev;
        self.current_idx = next;

        return value;
    }

    fn move_current(&mut self, delta: i32) {
        if delta < 0 {
            for _ in 0 .. delta.abs() {
                self.current_idx = self.marbles[self.current_idx].prev;
            }
        } else {
            for _ in 0 .. delta.abs() {
                self.current_idx = self.marbles[self.current_idx].next;
            }
        }
    }

    fn add_to_score(&mut self, marble: i32) {
        self.scores[self.current_player] += marble as i64;
    }
}
