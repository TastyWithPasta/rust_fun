pub fn pig_latin(text: &str) -> String {
    let mut iterator = text.chars();
    let ch = iterator.next().unwrap();
    let rest = iterator.as_str();
    let res = String::from(text);
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => format!("{}hay", res),
        _ => format!("{}{}ay", rest, ch),    }
}

pub fn example() {
    let text = "fred";
    let pig_latin = pig_latin(text);
    println!("Pig latin for {} is {}", text, pig_latin);
}
