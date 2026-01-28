fn current_favorite_color() -> &'static str {
    "blue"
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
