pub trait Summary {
    fn sum_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more...), {}", self.sum_author()) // default implementation
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn sum_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{} tweeted {}", self.username, self.content)
    }
}

pub struct Post {
    pub author: String,
    pub conten: String,
}

impl Summary for Post {
    fn sum_author(&self) -> String {
        format!("by {}", self.author)
    }
}
