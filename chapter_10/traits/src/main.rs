pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Must implement given trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Long form that must implement trait -- Only way to enforce multiple params to have the exact same type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

fn main() {}
