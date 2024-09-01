// Implementing the Summary trait on the NewsArticle and Tweet types

pub trait Summary {
    //fn summarize(&self) -> String; // Here we do not provide a default
    // implementation for the summarize method.

    // Each type implementing this trait
    // must provide its own custom behaviour for the body of this method.

    fn summarize(&self) -> String {
        String::from("(Read more...)") // Default behaviour implemented.
    }
}

// New articles
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implement Summary for news articles
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// impl Summary for NewsArticle {} // If we were using the default impl.

// Tweets
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implement Summary for tweets
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
