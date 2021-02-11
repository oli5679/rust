extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::cmp;

pub fn get_counts(vec: Vec<u8>) -> [u8; 6] {
    let mut counts = [0; 6 as usize];
        for v in vec {
            counts[v as usize] += 1;
        }
    counts
}

pub fn get_colour_possibilities() -> Vec<[u8; 4]> {
    let mut colours = Vec::with_capacity(1296);
    for k1 in 0..6 {
        for k2 in 0..6 {
            for k3 in 0..6 {
                for k4 in 0..6 {
                    colours.push([k1, k2, k3, k4])
                }
            }
        }
    }
    colours.shuffle(&mut thread_rng()); // We need to keep our guesses random so we can't be exploited!
    colours
}

pub fn eval_guess(compare_colours: [u8;4], guess_colours: [u8; 4]) -> (u8, u8) {
    let guess_counts = get_counts(guess_colours.clone().to_vec());
    let compare_counts = get_counts(compare_colours.clone().to_vec());
    let mut hard_count = 0;
    for (k, guess_colour) in guess_colours.iter().enumerate() {
        if compare_colours[k] == *guess_colour {
            hard_count += 1;
        }
    }
    let mut soft_count = 0;
    for i in 0..6{
        soft_count += cmp::min(compare_counts[*&i as usize], guess_counts[*&i as usize]);
    }
    return (soft_count-&hard_count, hard_count)
}

pub struct Mastermind {
    hidden_colours: [u8;4],
    all_possibilities: Vec<[u8; 4]>,
    remaining_valid: Vec<[u8; 4]>,
}

impl Mastermind {
    pub fn new_game(hidden_colours: [u8; 4]) -> Self {
        let all_possibilities = get_colour_possibilities();
        Mastermind {
            hidden_colours,
            all_possibilities: all_possibilities.clone(),
            remaining_valid: all_possibilities.clone()
        }
    }
    pub fn simulate_perfect_move(&mut self) -> bool {
        let best_guess: [u8; 4];
        if self.remaining_valid.len() == 1 {
            best_guess = self.remaining_valid[0].clone();
        } else {
            best_guess = self.get_best_guess();
        }
        println!("  Best guess: {:?}, {:?}, {:?}, {:?}", best_guess[0], best_guess[1], best_guess[2], best_guess[3]);
        let guess_result = eval_guess(self.hidden_colours.clone(),best_guess.clone());
        println!("  Soft Matches: {}, Hard Matches: {}\n", guess_result.0, guess_result.1);
        return if guess_result.1 == 4 {
            // If we guessed the right answer
            true
        } else {
            self.cleanup_remaining_values(&best_guess, guess_result);
            false
        }
    }

    pub fn get_best_guess(&self) -> [u8; 4] {
        let mut best_guess = ([10,10,10,10],2000);
        for guess in self.all_possibilities.iter() {
            best_guess = self.check_with_remaining_values(guess, best_guess);
        }
        best_guess.0
    }

    pub fn cleanup_remaining_values(&mut self, guess: &[u8;4], guess_result: (u8, u8)){
        let mut filtered_remaining = Vec::with_capacity(self.remaining_valid.len());
        for remaining in self.remaining_valid.iter() {
            let (soft_count, hard_count) = eval_guess(remaining.clone(), guess.clone());
            if (soft_count == guess_result.0) && (hard_count == guess_result.1) {
                filtered_remaining.push(remaining.clone());
            }
        }
        self.remaining_valid = filtered_remaining;
    }

    fn check_with_remaining_values(&self, guess: &[u8; 4], best_guess: ([u8; 4], i32)) -> ([u8; 4], i32) {
        let mut counter = vec![0; 125];
        for possibility in self.remaining_valid.iter() {
            let (soft_count, hard_count) = eval_guess(possibility.clone(), guess.clone());
            let counter_index = 5*hard_count + soft_count;
            counter[counter_index as usize] += 1;
            if counter[counter_index as usize] >= best_guess.1 {
                return best_guess;
            }
        }
        let guess_maximum = counter.iter().max().unwrap();
        return if *guess_maximum < best_guess.1 {
            (guess.clone(), *guess_maximum)
        } else {
            best_guess
        }
    }
}

fn main() {
    let mut total = 0;
    for _i in 1..2 {
        let mut game = Mastermind::new_game([1, 1, 2, 3]);
        let mut game_iterations = 1;
        println!("Iteration {}:", game_iterations);
        while game.simulate_perfect_move() == false {
            game_iterations += 1;
            println!("Iteration {}:", game_iterations);
        }
        total += game_iterations;
        println!("Won the game in {} iterations!!", game_iterations)
    }

    println!("{}", &total )
}
