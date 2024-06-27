fn main() {
    let num = 3;
    if num < 5 {
        println!("Smaller than 5");
    } else {
        println!("Greater than 5");
    }

    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }
    else if a < 0{
        b = -1;
    }
    else {
        b = 0;
    }
    println!("b is {b}");

    // 条件表达式必须是bool类型
    /*
    if num {
        println!("Yes");
    }
    */

    let triple = if num > 0 { 1 } else { -1 };
    println!("triple is {triple}");
}
