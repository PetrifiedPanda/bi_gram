mod bi_gram;

use rand::rngs::ThreadRng;
use std::{
    env,
    io::{stdin, stdout, Write},
};

use bi_gram::BiGramModel;

fn generate_n_words(model: &BiGramModel, rng: &mut ThreadRng, prompt: &str, n: u32) {
    let mut curr = prompt;
    for _ in 0..n {
        print!("{} ", curr);
        curr = model.get_next(curr, &mut *rng).unwrap();
    }
    println!("");
}

fn get_next(model: &BiGramModel, rng: &mut ThreadRng, prompt: &str) {
    match model.get_next(prompt, &mut *rng) {
        Some(next) => {
            println!("Next Word: {}", next);
        }
        None => {
            println!("Sorry, I don't know that word");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected file as argument");
        return;
    }
    let mut model = BiGramModel::new_multiple(&args[1..args.len()]);

    let mut input = String::new();
    loop {
        print!("Please enter a word:");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("What have you just brought upon this cursed land");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        let mut rng = rand::thread_rng();
        generate_n_words(&mut model, &mut rng, input.as_str(), 5);
        input.clear();
    }
}
