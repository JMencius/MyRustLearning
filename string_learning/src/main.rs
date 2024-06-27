fn main() {
    // 有初始值用String::from
    let mut s1 = String::from("hello world");
    // 没有初始值的时候用String::new
    let empty = String::new();
    println!("{:?}", empty);

    println!("{:?}", s1);
    s1.push_str("bruh");
    println!("{:?}", s1);
    let a = s1.remove(4);
    println!("{:?}", s1);
    println!("{:?}", a);

    let mut s2 = "hello bruh";
    println!("{:?}", s2);
    let s2 = "hhhh";
    println!("{:?}", s2);

}
