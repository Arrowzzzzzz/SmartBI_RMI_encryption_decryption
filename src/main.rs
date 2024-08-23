extern crate percent_encoding;  
use percent_encoding::percent_decode;  
use std::collections::HashMap;
use std::string::String;
use once_cell::sync::Lazy;  
use clap::{Arg, AppSettings, App};

//请求加密字典
static REQUEST_ENCRYPTION_FIELD: Lazy<HashMap<char, char>> = Lazy::new(|| {  
    let mut map = HashMap::new();  
    map.insert('0', '7');  
    map.insert('1', '1');  
    map.insert('2', 'u');  
    map.insert('3', 'N');  
    map.insert('4', 'K');  
    map.insert('5', 'J');  
    map.insert('6', 'M');  
    map.insert('7', '9');  
    map.insert('8', '\'');  
    map.insert('9', 'm');  
    map.insert('!', 'P');  
    map.insert('%', '/');  
    map.insert('\'', 'n');  
    map.insert('(', 'A');  
    map.insert(')', 'E');  
    map.insert('*', 's');  
    map.insert('+', '+');  
    map.insert('-', 'f');  
    map.insert('.', 'q');  
    map.insert('A', 'O');  
    map.insert('B', 'V');  
    map.insert('C', 't');  
    map.insert('D', 'T');  
    map.insert('E', 'a');  
    map.insert('F', 'x');  
    map.insert('G', 'H');  
    map.insert('H', 'r');  
    map.insert('I', 'c');  
    map.insert('J', 'v');  
    map.insert('K', 'l');  
    map.insert('L', '8');  
    map.insert('M', 'F');  
    map.insert('N', '3');  
    map.insert('O', 'o');  
    map.insert('P', 'L');  
    map.insert('Q', 'Y');  
    map.insert('R', 'j');  
    map.insert('S', 'W');  
    map.insert('T', '*');  
    map.insert('U', 'z');  
    map.insert('V', 'Z');  
    map.insert('W', '!');  
    map.insert('X', 'B');  
    map.insert('Y', ')');  
    map.insert('Z', 'U');  
    map.insert('a', '(');  
    map.insert('b', '~');  
    map.insert('c', 'i');  
    map.insert('d', 'h');  
    map.insert('e', 'p');  
    map.insert('f', '_');  
    map.insert('g', '-');  
    map.insert('h', 'I');  
    map.insert('i', 'R');  
    map.insert('j', '.');  
    map.insert('k', 'G');  
    map.insert('l', 'S');  
    map.insert('m', 'd');  
    map.insert('n', '6');  
    map.insert('o', 'w');  
    map.insert('p', '5');  
    map.insert('q', '0');  
    map.insert('r', '4');  
    map.insert('s', 'D');  
    map.insert('t', 'k');  
    map.insert('u', 'Q');  
    map.insert('v', 'g');  
    map.insert('w', 'b');  
    map.insert('x', 'C');  
    map.insert('y', '2');  
    map.insert('z', 'X');  
    map.insert('~', 'e');  
    map.insert('_', 'y');  
    map
});

//请求解密字典
static REQUEST_DECRYPTION_FIELD: Lazy<HashMap<char, char>> = Lazy::new(|| {  
    let mut map = HashMap::new();  
    for (key, value) in REQUEST_ENCRYPTION_FIELD.iter() {  
        map.insert(*value, *key);  
    }  
    map  
}); 

// 响应解密字典
const RESPONSE_DECRYPTION_FIELD: [u8; 128] = [  
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  
    0, 0, 0, 80, 0, 0, 0, 47, 0, 110,  
    65, 69, 115, 43, 0, 102, 113, 0, 55, 49,  
    117, 78, 75, 74, 77, 57, 39, 109, 0, 0,  
    0, 0, 0, 0, 0, 79, 86, 116, 84, 97,  
    120, 72, 114, 99, 118, 108, 56, 70, 51, 111,  
    76, 89, 106, 87, 42, 122, 90, 33, 66, 41,  
    85, 0, 0, 0, 0, 121, 0, 40, 126, 105,  
    104, 112, 95, 45, 73, 82, 46, 71, 83, 100,  
    54, 119, 53, 48, 52, 68, 107, 81, 103, 98,  
    67, 50, 88, 0, 0, 0, 101, 0,  
];


// 请求包加解密
fn request(code: &str,proces: &str) -> String {
    let mut out = String::new();
    let dictionary = if proces == "Encryption" {
        &*REQUEST_ENCRYPTION_FIELD
    }else if proces == "Decryption" {
        &*REQUEST_DECRYPTION_FIELD
    }else {
        println!("退出程序，因为加解密参数不对请输入Encryption or Decryption。");  
        std::process::exit(0); // 使用0表示正常退出  
    };
    for item in code.chars() {
        if let Some(data) = dictionary.get(&item) {
            out.push(*data);
        } else {
            out.push(item);
        }
    }
    if *dictionary == *REQUEST_DECRYPTION_FIELD { 
        let out = percent_decode(out.as_bytes())  
            .decode_utf8_lossy()  
            .into_owned();  
        out
    }else{
        out
    }
    
}

pub fn response(encoded_string: &str) -> String {  
    let mut decrypted_string = String::new();  
    for char in encoded_string.chars() {  
        let c = char as u8; // Assuming the input is ASCII  
        if usize::from(c) < RESPONSE_DECRYPTION_FIELD.len() && RESPONSE_DECRYPTION_FIELD[c as usize] != 0 {  
            let decrypted_char = RESPONSE_DECRYPTION_FIELD[c as usize] as char;  
            if char != '%' {  
                decrypted_string.push(decrypted_char);  
            } else {  
                decrypted_string.push(char);  
            }  
        } else {  
            decrypted_string.push(char);  
        }  
    }  
    decrypted_string  
}  


fn main() {
    let matches = App::new("SmartBI RMI Tool")
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Author: Arrowzzzzzz")
        .about("SmartBI RMI encryption or decryption tool for data in request and response packets")
        .arg(Arg::with_name("Encryption")
            .short('e')
            .long("encryption")
            .value_name("Encryption")
            .takes_value(true)
            .help("SmartBI RMI encryption or data in request packets"))
        .arg(Arg::with_name("Decryption")
            .short('d')
            .long("decryption")
            .value_name("Decryption")
            .takes_value(true)
            .help("SmartBI RMI decryption or data in request packets"))
        .arg(Arg::with_name("RE_Decryption")
            .short('r')
            .long("response")
            .value_name("RE_Decryption")
            .takes_value(true)
            .help("SmartBI RMI decryption or data in response packets"))
        .get_matches();
    if matches.is_present("Encryption")==true {
        println!("--------------------------------Encryption data------------------------------------------------");
        println!("Encryption data:{}", request(&matches.value_of("Encryption").unwrap(), "Encryption"));
    }else if matches.is_present("Decryption")==true {
        println!("---------------------------------Decryption data-----------------------------------------------");
        println!("Decryption data:{}", request(&matches.value_of("Decryption").unwrap(), "Decryption"));
    }else if matches.is_present("RE_Decryption")==true {
        println!("---------------------------------Decryption data-----------------------------------------------");
        println!("Decryption data:{}", response(&matches.value_of("RE_Decryption").unwrap()));
    }else{
        eprintln!("Invalid option.");
    }
}

