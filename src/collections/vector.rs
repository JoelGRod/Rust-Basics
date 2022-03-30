

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vector_examples() -> () {
    // Create a Vector
    let mut first_vector: Vec<i32> = Vec::new();
    let mut second_vector: Vec<i32> = vec![1,2,3,4,5];

    // Update a Vector
    first_vector.push(1);
    first_vector.push(2);
    first_vector.push(3);
    first_vector.pop();

    // Reading Vector elements
    let element = &first_vector[0];
    println!("The elementos of first vector is {}", element);

    match second_vector.get(2) {
        Some(element) => println!("The element of second vector is {}", element),
        None => println!("There is no element in vector")
    }

    // Multiple types in vector
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Iterate vector elements
    for i in &first_vector {
        println!("element {}", i);
    };

    for i in &mut second_vector {
        *i += 50;
        println!("element modified {}", i);
    }

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
        }
    }
}

