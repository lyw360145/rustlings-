// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    // Equivalent to `String::from("blue")`
    let a = "blue".to_string();
    let b = a.clone();
     println!("My current favorite color is {b}");
    a
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
    let a = Vec::from(["a", "b", "c"]);
    println!("My current favorite color is {a:?}");
}
