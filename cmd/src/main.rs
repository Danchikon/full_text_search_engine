use core::{trie, Pipeline, PipelineBehaviour};
use engine::pipeline::behaviours::{
    LowerCasePipelineBehaviour, RedundantTokensPipelineBehaviour, StemmingPipelineBehaviour,
};
use rust_stemmers::{Algorithm, Stemmer};
use std::{collections::HashSet, vec};

fn main() {
    let tokens = vec![
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
        .filter_map(|token| pipeline.execute(token.clone()))
        .collect::<HashSet<String>>();

    let mut tree = trie::Node::root();

    let v = "value".to_string();
    tree.insert(v);
    tree.insert("var".to_string());
    tree.insert("val".to_string());
    tree.insert("swimming".to_string());
    tree.insert("swap".to_string());

    println!("var - {}", tree.exists(&"var".to_string()));
    println!("vay - {}", tree.exists(&"vay".to_string()));

    println!("all - {:?}", tree.values());
    println!("count - {:?}", tree.count());
    println!("height - {:?}", tree.height());

    println!("{:?}", result);
}
