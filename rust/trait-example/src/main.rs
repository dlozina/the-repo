use crate::aggregator::Summary;

mod aggregator;

fn main() {
    let tweet = aggregator::Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("One new tweet: {}", tweet.summarize());
}

