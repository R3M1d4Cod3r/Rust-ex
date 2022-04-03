fn main() {
    let stringa = "èasjhvxsj".to_string();
    println!("{}",capitalize(&stringa));
}

fn capitalize(s: &str) -> String {
    if ! s.chars().any(|c| c.is_ascii_alphanumeric()) { return s.to_string();}

    let  split  = s.split(' ');
    let mut result = String::new();
    let mut str_temp:String;

    for word in split{
        str_temp=word.to_string();
        let  tmp=str_temp.remove(0);
        str_temp.insert(0, tmp.to_ascii_uppercase());
        str_temp.push_str(" ");
        result.push_str(&str_temp);
    }

    return result;
}