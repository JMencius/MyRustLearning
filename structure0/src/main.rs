fn main() {
    // no need to add ;
    struct User {
        username : String,
        email : String,
        sign_in_count : u64,
        active : bool,
    };

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

}
