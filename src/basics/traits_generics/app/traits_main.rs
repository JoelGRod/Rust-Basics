use crate::basics::traits_generics::domain::{
    structs::{
        news_article::NewsArticle, 
        tweet::Tweet
    }, 
    traits::summary::Summary
};

pub fn traits_examples() {
    println!("---------- Generics ----------");

    println!("----------- Traits -----------");
    let new_article: NewsArticle = NewsArticle { 
        headline: "Elden Ring is a must!!".to_owned(), 
        location: "Santa Cruz de Tenerife".to_owned(), 
        author: "Jeremy Irons".to_owned(), 
        content: "Officia cillum adipisicing laborum magna 
                  in elit enim exercitation voluptate. 
                  Do aliquip eiusmod amet reprehenderit et 
                  Lorem nulla ex elit minim. Eiusmod aute 
                  est et eiusmod irure veniam ullamco occaecat 
                  ad voluptate reprehenderit exercitation. 
                  Consectetur pariatur consequat veniam nostrud 
                  enim anim exercitation ut aliqua. 
                  Labore aute anim dolor eu veniam duis ex 
                  pariatur labore elit ullamco minim Lorem.".to_owned(),
    };

    println!("New Article available! {}", new_article.summarize());
    println!("{}", new_article.summarize_author());

    let tweet: Tweet = Tweet { 
        username: "Fernando_Her".to_owned(), 
        content: "New vue course 4 U!!".to_owned(), 
        reply: false, 
        retweet: false 
    };

    println!("1 new Tweet: {}", tweet.summarize());
    println!("{}", tweet.summarize_author());


}