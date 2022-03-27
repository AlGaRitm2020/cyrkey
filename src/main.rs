use std::io;
use std::process::Command;
fn main() {
    let mut input_string = String::new();
    let mut output_string = String::new();

    io::stdin().read_line(&mut input_string)
        .expect("Input error");

    for i in input_string.chars() {
        
//абвгдеёжзийклмнопрстуфхцчшщъыьэюя
        let ch = match i {
            'f' => 'а',
            ',' => 'б',
            'd' => 'в',
            'u' => 'г',
            'l' => 'д',
            't' => 'е',
            '`' => 'ё',
            ';' => 'ж',
            'p' => 'з',
            'b' => 'и',
            'q' => 'й',
            'r' => 'к',
            'k' => 'л',
            'v' => 'м',
            'y' => 'н',
            'j' => 'о',
            'g' => 'п',
            'h' => 'р',
            'c' => 'с',
            'n' => 'т',
            'e' => 'у',
            'a' => 'ф',
            '[' => 'х',
            'w' => 'ц',
            'x' => 'ч',
            'i' => 'ш',
            'o' => 'щ',
            ']' => 'ъ',
            's' => 'ы',
            'm' => 'ь',
            '\'' => 'э',
            '.' => 'ю',
            'z' => 'я',
            
            
            _ => i,
        };
        
        output_string.push(ch)
    }
    println!("\n{}", output_string);
    
    //Command::new("ls")
    //.args(&["-l"])
    //                .output()
    //              .unwrap();

    // println!("Result was copied to clipboard");

}

