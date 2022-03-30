

pub fn string_examples() {

    /* ----------- Create a String ----------- */
    let mut string_1: String = String::new();
    
    let string_2: String = "World!!".to_string();

    let string_3: String = "this is a String".to_owned();      // Best Way

    let string_4: String = "this is a String".into();

    let string_5: String = String::from("this is a String");

    let string_6: String = format!("{}", "this is a String");   // Worst way

    /* ----------- Update Strings ----------- */
    // push_str()   -> concatenate chars chain
    string_1.push_str("this is a String");
    println!("push_str: {}", string_1);
    
    // push()       -> concatenate char
    string_1.push('h');
    println!("push: {}", string_1);

    // + operator
    let lost_ownership: String = "Hello ".to_owned();
    let concatenated: String = lost_ownership + &string_2 + &string_3;
    println!("+ operator: {}", concatenated);

    // format! macro (references, ownership remains)
    let format_string_1: String = "Hello".to_owned();
    let format_string_2: String = "World!!".to_owned();
    let format_string_3: String = "Friends".to_owned();
    let formated_string: String = format!("{} {} {}", format_string_1, format_string_2, format_string_3);
    println!("formated string: {}", formated_string);

    /* ---------- Slicing Strings (Bad idea) ---------- */ 
    let slice_example = "abcdef";
    let sliced = &slice_example[0..3];
    println!("Sliced: {}", sliced);

    /* ---------- Iterating over Strings ---------- */ 
    // Chars
    for c in slice_example.chars() {
        println!("{}", c);
    }

    // Bytes
    for b in slice_example.bytes() {
        println!("{}", b);
    }
}