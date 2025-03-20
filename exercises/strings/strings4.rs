// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // "blue" 是 &str 类型
    string("red".to_string()); // "red".to_string() 是 String 类型
    string(String::from("hi")); // String::from("hi") 是 String 类型
    string("rust is fun!".to_owned()); // "rust is fun!".to_owned() 是 String 类型
    string("nice weather".into()); // "nice weather".into() 是 String 类型
    string(format!("Interpolation {}", "Station")); // format! 宏返回 String 类型
    string_slice(&String::from("abc")[0..1]); // &String::from("abc")[0..1] 是 &str 类型
    string_slice("  hello there ".trim()); // "  hello there ".trim() 是 &str 类型
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // 替换操作后是 String 类型
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // 转换为小写后是 String 类型
}
