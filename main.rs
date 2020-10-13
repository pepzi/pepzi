pub use std::collections::HashMap;
use crate::pepzi::*;

mod pepzi;

fn main() {

    // Simple way to  fill a HashMap
    // =============================
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let blue_score = scores.get_key_value(&String::from("Blue"));
    match &blue_score {
        Some(t) => println!("Team {} scored {} points.", t.0, t.1),
        None => println!("Unable to find Blue team"),
    };

    // Overwriting a value:
    scores.insert(String::from("Blue"), 25);
    print_scores(&scores);

    // Only insert if Key has no Value
    scores.entry(String::from("Yellow")).or_insert(500);
    scores.entry(String::from("Red")).or_insert(500);
    println!("- After:");
    print_scores(&scores);

    // Update based on the old value
    println!("- Remove 350 from Reds score");
    *scores.get_mut("Red").unwrap() -= 350;
    print_scores(&scores);

    println!("- Add 88 to Yellows score");
    let newscore = scores.entry(String::from("Yellow")).or_insert(0);
    *newscore += 88;
    print_scores(&scores);

    #[allow(unused_variables, unused_mut)]
    let mut number = 7;

}
