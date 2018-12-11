extern crate rand;

use rand::Rng;
use std::collections::VecDeque;

fn main() {

	let mut vals: VecDeque<i32> = VecDeque::with_capacity(100);

	let total_trials: i32 = 1000;

	let mut sum_trials = 0;

	for _trial in 0..total_trials{  //  1000 trials

		//  refill vector queue
		for val in 0..99 {
			vals.push_back(val);
		}
		
		//  current number of trials
		let mut cur_trial_count: i32 = 0;

		//  loop until vector is empty
		loop {

			if vals.is_empty(){
				break;
			}
			else{
				//  get the current value in the queue
				let cur_val = vals.pop_front();

				//  generate probability of keeping or throwing away
				let secret_number = rand::thread_rng().gen_range(1, 101);

				//  if the random value is over 28 (which there is a 72% chance of) then put it back in front
				if secret_number > 28 {
					vals.push_front(cur_val.unwrap());
				}

				//  increment trial count
				cur_trial_count = cur_trial_count + 1;
			}

		} 

		sum_trials = sum_trials + cur_trial_count;

	}

	println!("sum trials: {}", sum_trials);

	println!("Here is the average number it took to complete: {}", sum_trials / total_trials);
}