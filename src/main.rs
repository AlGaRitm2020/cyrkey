use std::io;
use std::process::Command;
use std::env;
use std::fs::File;
use std::io::Write;




fn main() {
    while true {

        let mut input_string = String::new();
        let mut output_string = String::new();

        io::stdin().read_line(&mut input_string)
        .expect("Input error");

        for i in input_string.chars() {
        
        //абвгдеёжзийклмнопрстуфхцчшщъыьэюя
            let ch = match i {
                'f' => 'а',
            ','     => 'б',
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
    
        // create file /tmp/target
        let temp_directory = env::temp_dir();
        let temp_file = temp_directory.join("target");
        let mut file = File::create(temp_file).unwrap();
        writeln!(&mut file,"{}", output_string.as_str()).unwrap();




        let command = Command::new("clear")
            //.args(&["/tmp/target", "|", "xclip", "-sel", "clip"])
            .output()
            .expect("Failed to execute shell command");


    //.args(&["-l"])
    //                .output()
    //              .unwrap();

    // println!("Result was copied to clipboard");
    



    // Write a byte string.
    // file.write(b"Bytes\n").unwrap();


    }

}



