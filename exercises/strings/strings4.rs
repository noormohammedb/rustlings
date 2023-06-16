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
    String::from("blue");
    String::from("red".to_string());
    String::from(String::from("hi"));
    String::from("rust is fun!".to_owned());
    String::from_utf8("nice weather".into());
    String::from(format!("Interpolation {}", "Station"));
    String::from(&String::from("abc")[0..1]);
    String::from("  hello there ".trim());
    String::from("Happy Monday!".to_string().replace("Mon", "Tues"));
    String::from("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
