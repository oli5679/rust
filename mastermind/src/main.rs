use std::cmp;
use rand::{seq::IteratorRandom, thread_rng};

fn print_vector(vector: &Vec<u16>) {
    println!("[{}]", vector.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "));
}

fn get_counts(vec: Vec<u16>,guess_counts:u16) -> Vec<u16> {
    let mut counts = vec![0; guess_counts as usize];
        for v in vec {
            counts[v as usize] += 1;
        }
    counts
}

fn get_combinations(num_cells:u16, num_values:u16) -> Vec<Vec<u16>> {
    let mut combinations = vec![];
    if num_cells == 1 {
        for i in 0..num_values {
            combinations.push(vec![i]);
        }
    }
    else {
        let subcombinations = get_combinations(num_cells-1, num_values);
        for s in subcombinations {
            for i in 0..num_values {
                let mut combined = vec![i];
                let mut s_clone = s.clone();
                combined.append(&mut s_clone);
                combinations.push(combined);
            }
        }
    }
    combinations
}

pub fn eval_guess(compare_colours: Vec<u16>, guess_colours: Vec<u16>,num_colours:u16 ) -> (u16, u16) {
    let guess_counts = get_counts(guess_colours.clone().to_vec(),num_colours);
    let compare_counts = get_counts(compare_colours.clone().to_vec(),num_colours);
    let mut hard_count = 0;
    for (k, guess_colour) in guess_colours.iter().enumerate() {
        if compare_colours[k] == *guess_colour {
            hard_count += 1;
        }
    }
    let mut soft_count = 0;
    for i in 0..num_colours{
        soft_count += cmp::min(compare_counts[*&i as usize], guess_counts[*&i as usize]);
    }
    return (soft_count-&hard_count, hard_count)
}

pub struct Mastermind {
    hidden_colours: Vec<u16>,
    all_possibilities: Vec<Vec<u16>>,
    remaining_valid: Vec<Vec<u16>>,
    num_colours: u16,
    num_cells: u16
}

impl Mastermind {
    pub fn new_game(hidden_colours: Vec<u16>,num_colours:u16, num_cells: u16) -> Self {
        let all_possibilities =  get_combinations(num_cells,num_colours);
        Mastermind {
            hidden_colours,
            all_possibilities:all_possibilities.clone(),
            remaining_valid: all_possibilities.clone(),
            num_colours: num_colours,
            num_cells: num_cells
        }
    }
    fn simulate_perfect_move(&mut self) -> bool {
        let best_guess: Vec<u16>;
        if self.remaining_valid.len() == 1 {
            best_guess = self.remaining_valid[0].clone();
        } else {
            best_guess = self.get_best_guess();
        }
        print_vector(&best_guess);

        let guess_result = eval_guess(self.hidden_colours.clone(),best_guess.clone(),self.num_colours);
        println!("  Soft Matches: {}, Hard Matches: {}\n", guess_result.0, guess_result.1);
        return if guess_result.1 == self.num_cells {
            // If we guessed the right answer
            true
        } else {
            self.filter_valid_remaining(&best_guess, guess_result);
            false
        }
    }

    fn get_best_guess(&self) -> Vec<u16> {
        let mut best_guess = (vec![],20000000);
        for guess in self.all_possibilities.iter() {
            best_guess = self.check_with_remaining_values(guess, best_guess);
        }
        best_guess.0
    }

    fn filter_valid_remaining(&mut self, guess: &Vec<u16>, guess_result: (u16, u16)){
        let mut filtered_remaining = Vec::with_capacity(self.remaining_valid.len());
        for remaining in self.remaining_valid.iter() {
            let (soft_count, hard_count) = eval_guess(remaining.clone(), guess.clone(),self.num_colours);
            if (soft_count == guess_result.0) && (hard_count == guess_result.1) {
                filtered_remaining.push(remaining.clone());
            }
        }
        self.remaining_valid = filtered_remaining;
    }

    fn check_with_remaining_values(&self, guess: &Vec<u16>, best_guess: (Vec<u16>, i32)) -> (Vec<u16>, i32) {
        let counter_len = (self.num_cells+1).pow(self.num_cells as u32);
        let mut counter = vec![0; counter_len as usize];
        for possibility in self.remaining_valid.iter() {
            let (soft_count, hard_count) = eval_guess(possibility.clone(), guess.clone(),self.num_colours);
            let counter_index = ((self.num_cells+1)*hard_count) + soft_count;
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
    let num_colours = 6;
    let num_cells = 4;
    let all_possibilities =  get_combinations(num_cells,num_colours);
    let sample = all_possibilities.iter().choose_multiple(&mut thread_rng(), 50);
    for s in sample {
        print_vector(&s);
        let mut game = Mastermind::new_game(s.to_vec(),num_colours,num_cells);
        let mut game_iterations = 1;
    
        while game.simulate_perfect_move() == false {
            game_iterations += 1;
            println!("Iteration {}:", game_iterations);
            println!("Won the game in {} iterations!!", game_iterations)
        }
    }

}
