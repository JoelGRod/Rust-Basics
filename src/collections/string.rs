

pub fn string_examples() -> () {
    // Create a String
    let mut my_string: String = String::new();
    my_string.push_str("this is a String");
    my_string.push('h');

    let first_string: String = "this is a String".to_string();

    let second_string: String = "this is a String".to_owned();      // Best Way

    let third_string: String = "this is a String".into();

    let final_string: String = String::from("this is a String");

    let fifth_string: String = format!("{}", "this is a String");   // Worst way



}