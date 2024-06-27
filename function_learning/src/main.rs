fn main() {
    println!("Hello world!");
    // rust does not care where the function is defined
    another_function();
    function_variables(5, 4);

    let x = 6;

    // 函数式表达式 不能使用return 最后的一句不能是分号
    let y = {
        let x = 3;
        x + 3
    };
    function_variables(x, y);
    println!("add sum is {}", add(x, y));
}

// 函数名称通常使用蛇形命名法，即全小写用下划线连接
fn another_function() {
    println!("Hello runoob!");
}

fn function_variables(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

// 函数需要显式地声明变量的类型和返回值的类型
fn add(x: i32, y: i32) -> i32 {
    // 可以不写return
    x + y
}