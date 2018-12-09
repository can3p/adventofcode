// this solution is damn slow. As was pointed in reddit thread,
// the proper way would be to avoid inserting in the middle
// of the list at all and keep track of prev/next indexes instead
use std::collections::VecDeque;

#[test]
fn day09_1_test1() {
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

struct Game {
    //marbles: Vec<i32>,
    marbles: VecDeque<i32>,
    current_idx: usize,
    current_player: usize,
    next_marble: i32,
    scores: Vec<i64>,
    max_marble: i32,
}

impl Game {
    fn new(num_players: i32, max_marble: i32) -> Game {
        let scores: Vec<i64> = (0..num_players).map(|_| 0).collect();
        let mut marbles: VecDeque<i32> = VecDeque::with_capacity(max_marble as usize);
        marbles.push_back(0);
        //marbles.push(0);

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
            //println!("current_idx = {:?}, marbles = {:?}", self.current_idx, self.marbles);
        }

        return self.scores.iter().max().unwrap().clone();
    }

    fn next_move(&mut self) {
        self.current_player = (self.current_player + 1 + self.scores.len()).wrapping_rem(self.scores.len());
        let marble = self.next_marble;
        self.next_marble += 1;

        if marble.wrapping_rem(23) != 0 {
            self.move_current(1);
            self.insert_after(marble);
            self.move_current(1);
        } else {
            self.add_to_score(marble);
            self.move_current(-7);
            let marble = self.eject_current();
            self.add_to_score(marble);
        }
    }

    fn insert_after(&mut self, marble: i32) {
        if self.current_idx == self.marbles.len() -1 {
            self.marbles.push_back(marble);
            //self.marbles.push(marble);
        } else {
            self.marbles.insert(self.current_idx + 1, marble);
        }
    }

    fn eject_current(&mut self) -> i32 {
        let marble = self.marbles.remove(self.current_idx).unwrap();
        if self.current_idx == self.marbles.len() {
            self.current_idx = 0;
        }

        return marble;
    }

    fn move_current(&mut self, delta: i32) {
        self.current_idx = self.current_idx + self.marbles.len();
        if delta < 0 {
            self.current_idx -= delta.abs() as usize;
        } else {
            self.current_idx += delta as usize;
        }

        self.current_idx = self.current_idx.wrapping_rem(self.marbles.len());
    }

    fn add_to_score(&mut self, marble: i32) {
        self.scores[self.current_player] += marble as i64;
    }
}
