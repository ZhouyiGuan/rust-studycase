fn main() {
    let string1 = String::from("abcdefg");
    let mut string2 = & string1;
    string2.push_str("11");
    let mut string3 = & string1;


    // {
    //     let string2 = String::from("abcde");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // // println!("string1{}",string1);
    // println!("The longest string is {}", result);
}

// fn longest(x: & str, y: & str) -> & str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
