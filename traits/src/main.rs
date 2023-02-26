trait Summary {
    fn get_summary(&self) -> String;
    fn default_impl_summarize(&self) -> String {
        format!("Read more....")
    }
}

struct BlogPost {
    title: String,
    content: String,
    author: String,
}

struct Tweet {
    content: String,
    author: String,
    likes: Vec<String>,
}

impl Summary for BlogPost {
    fn get_summary(&self) -> String {
        format!(
            "{} wrote a post with title {}. Its main content includes {}",
            self.author, self.title, self.content
        )
    }
}

impl Summary for Tweet {
    fn get_summary(&self) -> String {
        format!(
            "{} wrote a tweet. It contains {} and has {} likes.",
            self.author,
            self.content,
            self.likes.len()
        )
    }
}

fn main() {
    let post = BlogPost {
        author: String::from("Bob"),
        content: String::from("Bob"),
        title: String::from("Bob"),
    };

    let tweet = Tweet {
        author: String::from("John"),
        content: String::from("John"),
        likes: vec![String::from("Bob")],
    };

    println!("{}", post.get_summary());
    println!("{}", tweet.get_summary());

    println!("{}", tweet.default_impl_summarize());
    println!("{}", post.default_impl_summarize());

    notify(&post);
    notify(&tweet);

    all_notifications(&post, &tweet)
}

fn notify(item: &impl Summary) {
    println!("An new notification, {}", item.get_summary());
}

fn all_notifications<T: Summary>(item_1: &T, item_2: &impl Summary) {
    notify(item_1);
    notify(item_2);
}
