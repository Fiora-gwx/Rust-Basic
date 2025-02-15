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

/*************  ✨ Codeium Command ⭐  *************/
/// Prints out the contents of a `String` parameter.
/******  44f53e17-0fa9-4ec8-a4d3-5080fec8721a  *******/fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");  // 字符串字面量是 `&str`
    string("red".to_string());  // `to_string()` 生成 `String`
    string(String::from("hi")); // `String::from()` 生成 `String`
    string("rust is fun!".to_owned()); // `.to_owned()` 生成 `String`
    string("nice weather".into()); // `.into()` 生成 `String`
    string(format!("Interpolation {}", "Station")); // `format!()` 生成 `String`
    string_slice(&String::from("abc")[0..1]); // 切片 `&str`
    string_slice("  hello there ".trim()); // `.trim()` 返回 `&str`
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // `.replace()` 返回 `String`
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // `.to_lowercase()` 返回 `String`
}