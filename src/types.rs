use std::io::*;

fn types() {
    let i: isize = 0; //i8, i16, i32, i64, i128
    let u: usize = 0; //u8, u16, u32, u64, u128
    let bol: bool = true; //false
    let c: char = 'A';
    let s: &str = "String"; //String::from("String")
    let tuple: (isize, usize, bool, char, &str) = (i, u, bol, c, s);
    let arr: [isize; 1] = [1];
    let vec: Vec<isize> = vec![1];
    struct Person {
        name: String,
        age: usize,
    }

    let person = Person {
        name: String::from("John"),
        age: 20,
    };
    enum Colors {
        Red,
        Green,
        Blue,
    }
    let color = Colors::Red;

    let optional: Option<()> = Some(()); //() -> Value
}
