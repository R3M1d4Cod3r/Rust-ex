fn main(){
    //isbn13 978-88-430-2534-3
    //isbn10 3-598-21507-X
    let a = String::from("978-88-430-2534-3");
    if isbn13(a)==0 {
        println!("Valido");
    }
    else {
        println!("Non Valido");
    }
}

fn isbn10(s : String) -> u32{
    let a = &s;
    if s.len() < 10 {
        return 1;
    }
    let mut tmp;
    let mut i=10;
    let mut result = 0;
    for character in a.chars() {

        if character=='x' || character == 'X'{
            tmp=10;
            result += tmp * i;
            i-=1;
        }
        else if character.is_digit(10){
            tmp=character.to_digit(32).unwrap();
            result += tmp * i;
            i-=1;
        }
        else if character != '-' {
            return 1;
        }
    }
    println!("result :{}", result % 11);
    return result % 11;
}

fn isbn13(s : String) -> u32{
    let a = &s;
    if s.len() < 13 {
        return 1;
    }
    let mut tmp;
    let mut i=-1;
    let mut result = 0;
    for character in a.chars() {

        if character=='x' || character == 'X'{
            tmp=10;
            i+=1;
            if i % 2 == 0 {
                result += tmp * 1;
            }
            else {
                result += tmp * 3;
            }
        }

        else if character.is_digit(10) {
            tmp=character.to_digit(32).unwrap();
            i+=1;
            if i % 2 == 0 {
                result += tmp * 1;
            }
            else {
                result += tmp * 3;
            }
        }

        else if character != '-' {
            return 1;
        }
    }
    println!("result :{}", result % 11);
    return result % 10;
}