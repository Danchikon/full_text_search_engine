use core::{
    pipeline::{Behaviour, Pipeline},
    tokenization::{Token, Tokenizer},
};
use engine::pipeline::behaviours::{
    LowerCaseBehaviour, RedundantTokensBehaviour, StemmingBehaviour,
};
use rust_stemmers::{Algorithm, Stemmer};
use std::{collections::HashSet, fs::File, io::Read, time::Instant, vec};

fn main() {
    let mut hash_set = HashSet::<String>::new();

    hash_set.insert("running".to_string());

    let stemmer = Stemmer::create(Algorithm::English);
    let stemming = StemmingBehaviour::new(stemmer);
    let lower_case = LowerCaseBehaviour::new();
    let redundant_tokens = RedundantTokensBehaviour::new(hash_set);

    let behaviours: Vec<Box<dyn Behaviour>> = vec![
        Box::new(lower_case),
        Box::new(redundant_tokens),
        Box::new(stemming),
    ];

    let pipeline = Pipeline::new(behaviours);
    let mut delimeters = HashSet::<char>::new();
    delimeters.insert(' ');
    delimeters.insert('"');
    delimeters.insert('”');
    delimeters.insert(')');
    delimeters.insert('(');
    delimeters.insert('“');
    delimeters.insert('\'');
    delimeters.insert('.');
    delimeters.insert('’');
    delimeters.insert('`');
    delimeters.insert(',');
    delimeters.insert('\n');

    let tokenizer = Tokenizer::new(delimeters, 4);

    let mut file = File::open("resources/sample-2mb-text-file.txt").unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    let text = String::from_utf8(buffer).unwrap();

    let now = Instant::now();
    let result = tokenizer.tokenize(&text).collect::<Vec<Token>>();

    let elapsed = now.elapsed();

    println!("{:?}", elapsed);

    for token in result.iter().take(7) {
        println!("{:?}", token)
    }
    println!("{}", result.len());
}
