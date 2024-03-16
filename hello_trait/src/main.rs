use hello_trait::{Post, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("Warun"),
        content: String::from("Hello, world!"),
    };

    println!("1 new tweet: {}", tweet.summarize());

    let post = Post {
        author: String::from("Warun"),
        conten: String::from("Hello, world!"),
    };

    println!("1 new post: {}", post.summarize());

    notify(&tweet);
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
