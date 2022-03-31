use std::collections::HashMap;

pub fn hashmap_examples() {
    /* ------------- Create a hashmap ------------- */
    // Basic
    let mut classifier: HashMap<String, i32> = HashMap::new();
    classifier.insert("Blue".to_owned(), 50);
    classifier.insert("Red".to_owned(), 100);

    // With a set of tuples (from vec) -> Tuple iterator
    let teams: Vec<String> = vec!["Blue".to_owned(), "Red".to_owned()];
    let scores: Vec<i32> = vec![50, 100];
    let mut another_classifier: HashMap<String, i32> = 
        teams.into_iter().zip(scores.into_iter()).collect();
    
    /* ------------- Ownership ------------- */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    /* ------------- Reading HashMap Elements ------------- */
    let wanted_key: String = "Blue".to_owned();
    let score: Option<&i32> = classifier.get(&wanted_key);
    match score {
        Some(value) => println!("The {} has an score of {}", wanted_key, value),
        None => println!("No key founded")
    }

    /* ------------- Iterating HashMap Elements ------------- */
    for (key, value) in &classifier {
        println!("Team {}: {}", key, value);
    }

    /* ------------- Updating HashMap Elements ------------- */
    // Overwriting a value
    classifier.insert("Orange".to_owned(), 100);
    classifier.insert("Orange".to_owned(), 150);
    println!("{:?}", classifier);

    // Only insert a value if the key has no value
    // Note: entry() and or_insert() returns a mutable reference of Value!!
    classifier.entry("Yellow".to_owned()).or_insert(200);
    classifier.entry("Blue".to_owned()).or_insert(200);
    println!("{:?}", classifier);

    // Updating a value based on the old value
    // Note: entry() and or_insert() returns a mutable reference of Value!!
    /* 
    This code will print {"world": 2, "hello": 1, "wonderful": 1}. 
    The split_whitespace method iterates over sub-slices, separated by 
    whitespace, of the value in text. The or_insert method returns a 
    mutable reference (&mut V) to the value for the specified key. 
    Here we store that mutable reference in the count variable, so 
    in order to assign to that value, we must first dereference count 
    using the asterisk (*). The mutable reference goes out of scope 
    at the end of the for loop, so all of these changes are safe and 
    allowed by the borrowing rules. 
    */
    let text: &str = "hello world world hello example";
    let mut words: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut i32 = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", classifier);
}