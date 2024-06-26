static MY_STATIC : i32 = 42;
static mut MY_MUT_STATIC : i32 = 42;

fn main() {
    // const
    const SECOND_HOUR : usize = 3_600;
    const SECOND_DAY : usize = 24 * SECOND_HOUR;
    {
        const SE : usize = 1_100;
        println!("{SE}");
    }
    
    println!("{}", SECOND_DAY);
    println!("{MY_STATIC}");
    unsafe {
        MY_MUT_STATIC = 32;
        println!("{MY_MUT_STATIC}");
    }
    // Can not print MY_MUT_STATIC outside unsafe
    // println!("{MY_MUT_STATIC}");
    
}
