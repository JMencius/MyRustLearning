fn get_length(s: String) -> usize {
    println!("String: {}", s);
    // 函数结束之后main::s3也销毁了
    s.len()  // 可以不写分号
}

fn main() {
    // copy ove
    // copy
    let c1 = 1;
    let c2 = c1;
    println!("{}", c1);

    let s1 = String::from("value");
    let s2 = s1; // s1的所有权转移给s2
    // println!("{s1}");  //报错
    println!("{s2}");

    let s3 = String::from("value3");
    let s4 = s3.clone(); //深拷贝
    println!("{}", s3);

    // println!("{}", s3); // borrow of moved value
    let len: usize = get_length(s3);
    println!("{}", len);

    let back = first_word("hello world");
    println!("{}", back);
    let back = first_word("we are the wolrd");
    println!("{}", back);
    
}

// fn dangle() -> String {
//   "hello".to_owned();
// }

// 静态的声明周期，污染了全局作用域
fn dangle_static() -> &'static str {
    "jkdjalfd"
}

// String 与 &str vec u8 ref
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // 传出也是切片引用
    &s[..]
}