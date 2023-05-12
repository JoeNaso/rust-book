fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Val of x in the inner block is now {x}");
    }

    println!("Val of x in the outer block is {x}");

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{quotient}"); 
    println!("{truncated}");

    let f: bool = false;
    println!("{f}");

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let first = tup.0;
    println!("The value of y is: {y}");
    println!("First item is {first}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!(" {:?} ", a); // arrays dont have a display Trait, but this is a debug workaround
}
