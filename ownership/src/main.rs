fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}


fn main() {
    /*
    let mut s = String::from("hello");
    println!("{}", s);
    s.push_str(", world");
    println!("{}", s);
     */

    /*
    let x = 5;
    println!("x : {x}");
    let y = x;
    println!("x : {x}");
    println!("y : {y}");
    */

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

}
