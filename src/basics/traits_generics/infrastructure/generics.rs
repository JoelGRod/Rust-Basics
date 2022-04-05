use crate::basics::traits_generics::domain::traits::summary::Summary;

/* ------------------------------------------------------------ */
/* 
    1 - Traits as parameters
    Restricting type parameter 'T'
    Limit the number of types a generic can take based on 
    the traits those types implements
*/

// Method 1 - Trait Bound Syntax
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Method 1 - Trait Bound Syntax
pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

// Method 2 - Traits as parameters (same as above)
pub fn notify_2(item1: &impl Summary, item2: &impl Summary) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

// Extras
// pub fn notify(item: &(impl Summary + Display)) {}

// pub fn notify<T: Summary + Display>(item: &T) {}

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

/* ------------------------------------------------------------ */
/* 
    2 - Returning items that implements traits 
*/

// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }

/* ------------------------------------------------------------ */
/* 
    3 - Using Trait Bounds to Conditionally Implement Methods
    See Structs
    Generics example 2 
*/

/* ------------------------------------------------------------ */
/* 
    4 - Implement a trait for any type that implements another 
    trait - Important!! 
*/

// impl<T: Display> ToString for T {
//      --snip--
// }




