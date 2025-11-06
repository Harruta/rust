pub fn main() {
    let v = vec![1, 2, 3];
    let v2 = vec![String::from("Sengmonkham"), String::from("Kawkoi")];
    let v3 = vec![1.0, 2.0, 3.0];
    println!("{}", first_element(v.clone()).unwrap());
    println!("{}", first_element(v2).unwrap());
    println!("{}", first_element(v3).unwrap());

    println!("{}", does_exist(v, 1));
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    v.into_iter().next()
}

fn does_exist<T: PartialEq>(v: Vec<T>, element: T) -> bool {
    for value in v.iter() {
        if value == &element {
            return true;
        }
    }
    false
}
