use crate::basics::traits_generics::{domain::{
    structs::{
        news_article::NewsArticle, 
        tweet::Tweet, 
        generics::{Point, Pair}
    }, 
    traits::summary::Summary
}, infrastructure::generics };

pub fn traits_examples() {
    println!("---------- Generics ----------");
    let p1: Point<i32, f32> = Point {x: 5, y: 10.8};
    let p2: Point<&str, char> = Point {x: "hello", y: 'c'};
    let p3: Point<f32, f32> = Point { x: 5.5, y: 8.8 };

    let mix_p4: Point<i32, char> = p1.mixup(p2);

    let pair: Pair<i32> = Pair::new(32, 56);
    pair.cmp_display();

    println!("mix_p4.x = {}, mix_p4.y = {}", mix_p4.x, mix_p4.y);
    println!("Distance from origin: {}", p3.distance_from_origin());

    let largest_number: i32 = generics::largest(&[1,2,3,4,5,6,7,0,89,34,123,345,67,44,56]);
    println!("Largest number: {}", largest_number);

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

    println!("---------- Generics Pt.2 ----------");
    generics::notify(&new_article, &tweet);
    generics::notify_2(&new_article, &tweet);


}