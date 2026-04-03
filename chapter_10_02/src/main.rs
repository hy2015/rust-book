use aggregator::{SocialMediaPost, Summary, Summary2, Summary3, notify};

fn main() {
    let post = SocialMediaPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = aggregator::NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("John Doe"),
        content: String::from("The Pittsburgh Penguins have won the Stanley Cup Championship!"),
    };
    println!("1 new article: {}", article.summarize());
    println!("1 new article: {}", article.summarize2());

    let post2 = SocialMediaPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new post: {}", post2.summarize3());

    notify(&post2)
}

