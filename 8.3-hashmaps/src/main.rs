fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team {team_name}'s score is {score}!");

    println!("All team scores:");

    for (team, score) in &scores {
        println!("{team}: {score}");
    }

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Orange");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{map:?}");
    // println!("{field_name}: {field_value}"); <-- can't use field_name and field_value here

    scores.insert(String::from("Blue"), 25);

    println!("All team scores (after blue was updated):");

    for (team, score) in &scores {
        println!("{team}: {score}");
    }

    scores.entry(String::from("Yellow")).or_insert(500);
    scores.entry(String::from("Red")).or_insert(499);

    println!("All team scores (after more updates):");

    for (team, score) in &scores {
        println!("{team}: {score}");
    }

    let text = "the quick brown fox jumps over the lazy dog";

    let mut wordmap = HashMap::new();

    for word in text.split_whitespace() {
        let count = wordmap.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{wordmap:?}")
}
