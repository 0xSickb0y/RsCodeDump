use aggregator::*;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    aggregator::notify(&tweet);
    aggregator::some_function(&String::from("String"), &20);

    let tweet2 = aggregator::returns_summarizable();
    println!("{}", tweet2.summarize());

    let pair = aggregator::Pair::new(10, 20);
    pair.cmp_display();
}

