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