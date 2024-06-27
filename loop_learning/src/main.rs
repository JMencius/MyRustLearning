fn main() {
    // while循环
    let mut number = 1;
    while number < 8 {
        println!("{}", number);
        number += 1;
    }
    println!("EXIT");
    print!("{}", '\n');

    // for循环
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("Value is {}", i);
    }

    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }

    for i in (31..34).rev() {
        println!("{}", i);
    }

    // loop循环
    let s = ['a', 'p', 'p', 'l', 'e'];
    let mut i = 0;
    let _location = loop {
        let ch = s[i];
        if ch == 'l' {
            break i;
        }
        println!("\'{}\'", ch);
        i += 1;
    };
    println!("the index of l is {}", i);

    /*
    // 没有break会一直循环下去
    loop {
        println!("again");
    }
    */

}
