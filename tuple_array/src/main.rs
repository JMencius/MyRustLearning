fn main() {
    // tuple
    let tup = (0, "hi", 3.14);
    println!("tup elements {} {} {}", tup.0, tup.1, tup.2);
    
    // mutable tuple
    let mut tup2 = (0, "hi", 3.14);
    tup2.1 = "f";
    println!("tup2 elements {} {} {}", tup2.0, tup2.1, tup2.2);

    // empty tupe
    let tup3 = ();
    println!("tup3 {:?}", tup3);
    // println!("tup3 {}", tup3);

    // array
    let mut arr = [11, 12, 34];
    println!("arr len {} first element is {}", arr.len(), arr[0]);
    arr[0] = 999;
    println!("arr len {} first element is {}", arr.len(), arr[0]);

    for elem in arr {
        println!("{}", elem);
    }

    let arr2 = [42; 3];
    for i in arr2 {
        println!("{}", i);
    }

    // ownership
    let arr_item = [1, 2, 3];
    let tup_item = (2, "ff");
    println!("arr : {:?}", arr_item);
    println!("tup : {:?}", tup_item);

    let arr_ownership = arr_item;
    let tup_ownership = tup_item;
    println!("arr : {:?}", arr_item);
    println!("tup : {:?}", tup_item);

    let a = 3;
    let b = a;
    println!("{a}");

}
