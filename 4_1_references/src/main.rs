fn main() {
    let s1 = String::from("hello");

    // & used to indicate a reference to an object
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}
// & used to indicate a reference to an object
fn calculate_length(s: &String) -> usize {
    s.len()
}
