use crate::basics::traits_generics::domain::traits::summary::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Article Author: {}", self.author)
    }

    // Override summarize
    fn summarize(&self) -> String {
        format!("{}, by {} - {}", self.headline, self.author, self.location)
    }
}

/*
    One restriction to note with trait implementations is that we can 
    implement a trait on a type only if at least one of the trait or 
    the type is local to our crate. 
    For example, we can implement standard library traits like Display 
    on a custom type like Tweet as part of our aggregator crate functionality, 
    because the type Tweet is local to our aggregator crate. 
    We can also implement Summary on Vec<T> in our aggregator crate, 
    because the trait Summary is local to our aggregator crate.
*/