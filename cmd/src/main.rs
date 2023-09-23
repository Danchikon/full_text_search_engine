use core::{PipelineBehaviour, Pipeline};
use engine::pipelines::{LowerCasePipelineBehaviour, StemmingPipelineBehaviour, RedundantTokensPipelineBehaviour};
use rust_stemmers::{Algorithm, Stemmer};
use std::{collections::HashSet, vec};

fn main() {
    let mut tokens = vec![
        String::from("vALUes"),
        "Muck".to_string(),
        "Duck".to_string(),
        "Currencies".to_string(),
        "running".to_string(),
        "swimming".to_string(),
        "ran".to_string(),
    ];

    let mut hash_set = HashSet::<String>::new();

    hash_set.insert("running".to_string());

    let stemmer = Stemmer::create(Algorithm::English);
    let stemming = StemmingPipelineBehaviour::new(stemmer);
    let lower_case = LowerCasePipelineBehaviour::new();
    let redundant_tokens = RedundantTokensPipelineBehaviour::new(hash_set);

    let behaviours: Vec<Box<dyn PipelineBehaviour>> = vec![
        Box::new(lower_case), 
        Box::new(redundant_tokens), 
        Box::new(stemming),
        ];

    let pipeline = Pipeline::new(behaviours);

    let result = tokens
        .iter()
        .map(|token| pipeline.execute(token))
        .filter_map(|token| token)
        .collect::<HashSet<String>>();

    println!("{:?}", result);
}
