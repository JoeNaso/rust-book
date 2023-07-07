use rand::Rng;
use num_traits::pow;

fn main() {
    println!("Hello, world!");
    other_func(10);
    print_labeled_measurement(100, 'h');

    let x = five_plus();
    println!("The value of x is: {x}");

    let num = rand::thread_rng().gen_range(1..=6);
    // println!("{num}");
    conditional(num);

    // String literal considered to be &str, not of type String ()
    let val:&str = if 1 > 0 {"hello"} else {"alternate reality"};
    println!("{val}");
    // But this makes them a String
    let other:String= if 1 > 0 {"hello".to_string()} else {"alternate reality".to_string()};
    println!("{other}");
    let mut counter = 0;
    let res = loop {
        println!("ayyy");
        counter += 1;
        if counter == 4 {
            break pow(counter, 2)
        }
    };
    println!("counter:  {counter}");
    println!("res:      {res}");

    // Loops labels
    let mut cnt = 0;
    'a_loop: loop {
        println!("count: {cnt}");
        let mut inner_cnt = 10;
        loop {
            println!("inner count:  {inner_cnt}");
            if inner_cnt == 9 {
                break;
            }
            if cnt == 2 {
                break 'a_loop;
            }
            inner_cnt -= 1;
        }
        cnt += 1;
    }
    println!("End count = {cnt}");

    // .map results in a lazy iterator, so final values need to be usage 
    // similarly, .fold() produces a single output (ala reduce) so can be printed
    let x: Vec<_> = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter()
        .map(|x| x + 3)
        .collect();
        // .fold(0, |x, y| x + y);
    println!("{:?}", &x)

}

fn other_func(val: u32) {
    println!("The value of val is {val}");
}

fn print_labeled_measurement(val: u32, label: char) {
    println!("The value is {val}{label}");
}

fn five_plus() -> i32 {
    // implicit return
    5 + 1
}

fn conditional(x: i32) -> () {
    if x > 5 {
        return 
    } else {
        println!("Greetings and salutations. Ya number is small");
    }
}
