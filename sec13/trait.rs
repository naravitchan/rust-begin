//Trait
trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more {} ...)", self.summarize_auth())
    }
    fn summarize_auth(&self) -> String;
}
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_auth(&self) -> String {
        format!("@ {}", self.author)
    }
    // fn fun() {}
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_auth(&self) -> String {
        format!("@ {}", self.username)
    }
}

fn main() {
    let news = NewsArticle {
        headline: String::from("We Won"),
        location: String::from("Australia"),
        author: String::from("Diwakar"),
        content: String::from("We Won World Cup"),
    };
    println!("News Article\n{}", news.summarize());

    let tweet = Tweet {
        username: String::from("Diwakar"),
        content: String::from("We Won the match"),
        reply: false,
        retweet: false,
    };
    // println!("New Tweet\n{}", tweet.summarize());
}
