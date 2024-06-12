fn main() {
    // 进制的字面量
    let a1 = -125;
    let a2 = 0xFF;
    let a3 = 0o13;
    let a4 = 0b10;

    println!("{a1} {a2} {a3} {a4}");

    // Max Min
    println!("u32 max : {}", u32::MAX);
    println!("u32 min : {}", u32::MIN);
    println!("i32 max : {}", i32::MAX);
    println!("i32 max : {}", i32::MIN);
    println!("i64 max : {}", i64::MAX);
    println!("i64 max : {}", i64::MIN);
    println!("usize max : {}", usize::MAX);
    println!("usize min : {}", usize::MIN);

    println!("isize is {} bytes", std::mem::size_of::<isize>());
    println!("usize is {} bytes", std::mem::size_of::<usize>());

    println!("u64 is {} bytes", std::mem::size_of::<u64>());
    println!("i64 is {} bytes", std::mem::size_of::<i64>());
    println!("i32 is {} bytes", std::mem::size_of::<i32>());

    // float
    let f1 : f32 = 1.234324;
    let f2 : f64 =  9.8858888;
    // 在第二位进行四舍五入
    println!("Float are {:.2} {:.2}", f1, f2);

    // bool
    let is_ok = true;
    let can_ok = false;
    println!("is ok ? {is_ok} can ok? {can_ok}");
    println!("is ok or can ok ?{}, can ok and is ok? {}", is_ok || can_ok, is_ok && can_ok);

    // char
    let char_c: char = 'A';
    let emoji_char: char = '😀';
    println!("You get {char_c} feel {emoji_char}");
    println!("{}", emoji_char as usize);
    println!("{}", emoji_char as i32);
}
