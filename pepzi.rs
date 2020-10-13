use std::collections::HashMap;

pub fn print_scores(scores: &HashMap<String, i32>) {
    for (k, v) in scores {
            println!("{}: {}", k, v);
    }
}
