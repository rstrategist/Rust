/*A trait defines the functionality a particular type has and
can share with other types.

Trait definitions are a way to group method signatures together
to define a set of behaviours necessary to accomplish some purpose.*/

use aggregator::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // Calling the summarize (default) method on an instance of NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // The notify function accepts any type for the item parameter
    // that implements the Summary trait. We can therefore call
    // notify on anz instance of NewsArticle or Tweet as these implement
    // the Summary trait.
    pub fn notify(item: &impl Summary) {
        // We can call the summarize method of the Summary trait on item
        println!("Breaking news! {}", item.summarize());
    }
}
