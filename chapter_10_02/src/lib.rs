use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Summary2 {
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Summary3 {
    fn sumarize_author(&self) -> String;

    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.sumarize_author())
    }
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

pub struct SocialMediaPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for SocialMediaPost {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

impl Summary2 for NewsArticle {}

impl Summary3 for SocialMediaPost {
    fn sumarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T){

}

pub fn notify3(item: &(impl Summary + Display)){

}

pub fn notify4<T: Summary + Display>(item: &T){

}

fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
    {0}

fn returns_summarizable() -> impl Summary {
    SocialMediaPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         SocialMediaPost {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}