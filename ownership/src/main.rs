fn main() {
    let mut s = String::from("hola");
    println!("{}", s);
    s.push_str(", amigos");
    println!("{}", s);

    // bind and make a copy of the fixed size integer
    let x = 5;
    let y = x;

    // this doesnt make a copy; s2 points to the same data as s1
    // The second assignment here frees s1, even though the vars
    // are pointed to the same memory. This will not compile if referencing s1
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    // But this version is fine, bc we clone it
    let new_s = String::from("yoooo");
    let another_s = new_s.clone();
    println!("new: {}, another: {}", new_s, another_s);

    // We can call this though, bc the integer size is known
    println!("x = {}, y = {}", x, y);


    let z = String::from("hello");  // z comes into scope

    takes_ownership(z);             // z's value moves into the function...
                                    // ... and so is no longer valid here

    let c = 5;                      // c comes into scope

    makes_copy(c);                  // c would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use c afterward
    
    // Here, c goes out of scope, then z. But because s's value was moved, nothing
    // special happens.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
