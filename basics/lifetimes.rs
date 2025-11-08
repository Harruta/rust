use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Anouncement! {ann}");
    if x.len() > y.len() { x } else { y }
}
fn main() {
    let string1 = String::from("short");
    let string2 = String::from("this one is definitely longer");

    let result = longest_with_an_announcement(&string1, &string2, "Rust is memory safe!");
    println!("Longest string: \" {}\"", result);
}
