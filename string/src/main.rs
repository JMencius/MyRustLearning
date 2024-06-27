struct Person {
    name: String,
    color: String,
    age: i32,
}

// function
// &String &str
fn print(data: &str) {
    println!("{}", data);
}

// 只能String
fn print_string_borrow(data: &String) {
    println!("{}", data);
}

fn main() {
    // 3 ways to convert "test" to string
    // String::from
    // .to_string()
    // .to_owned()
    let name = String::from("Value C++");
    let test = "Yep".to_string();
    let course = "Rust".to_owned();
    let new_name = name.replace("C++", "CPP");
    println!("{name} {test} {course} {new_name}");

    let rust = "\x52\x75\x73\x74"; // ASCII
    println!("{rust}");

    // string &str
    let color = "green".to_string();
    let name = "John".to_string();
    let people = Person { 
        name: name, 
        color: color,
        age: 89,s
    };

    // func
    let value = "value".to_owned();
    print(&value);
    print("data");
    // print_string_borrow("dd"); //error
    print_string_borrow(&value);
}
