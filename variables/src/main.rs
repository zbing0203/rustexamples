
const THEREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


fn calculate_lenght(s: &String) -> usize {
    s.len()
}

//fn change(some_string: &String) {
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
fn main() {
    //let  x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of THEREE_HOURS_IN_SECONDS is: {THEREE_HOURS_IN_SECONDS}");

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let t = true;
    let f: bool = false;
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    let a = [1,2,3,4,5];
    println!("The value of a is: {:?}", a);
    let tup = (100, 2.4, 3);
    println!("The value of tup is: {:?}", tup);
    println!("The value of tup.0 is: {}", tup.0);

    let mut s1 = String::from("Hello, ");
    //let len = calculate_lenght(&s1);
    //println!("The length of '{}' is: {}", s1, len);
    change(&mut s1);
    let len2 = calculate_lenght(&s1);
    println!("The length of '{}' is: {}", s1, len2);

    //multiple references
    let mut s3 = String::from("hello");

    //let r1 = &s3; // 没问题
    //let r2 = &s3; // 没问题
    let r3 = &mut s3; // 没问题
    println!("{r3}");
    //println!("{r1}, {r2}, and {r3}");
    //println!("{}, {}, and {}", r1, r2, r3);
    let s4 = String::from("hello");
    let slice1 = &s4[0..2];
    let slice2 = &s4[3..4];
    println!("{slice1}, {slice2}");

    let s5 = String::from("hello world");
    let word = first_word(&s5);
    println!("{word}");
}

//&str is a slice of a string
//&[i32] is a slice of i32
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }