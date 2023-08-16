mod bi_gram;

use std::{
    env,
    io::{stdin, stdout, Write},
};

use bi_gram::BiGramModel;

fn generate_n_words(model: &mut BiGramModel, prompt: &str, n: u32) {
    let mut curr = String::from(prompt);
    for _ in 0..n {
        print!("{} ", curr);
        curr = String::from(model.get_next(curr.as_str()).unwrap());
    }
    println!("");
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

        generate_n_words(&mut model, input.as_str(), 5);
        /*
        match model.get_next(input.as_str()) {
            Some(next) => {
                println!("Next Word: {}", next);
            }
            None => {
                println!("Sorry, I don't know that word");
            }
        }
        */
        input.clear();
    }
}
