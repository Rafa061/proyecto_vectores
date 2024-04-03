use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let file = match File::open("src/clasificacion.txt") {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut contents: String = String::new();
    if let Err(why) = reader.read_to_string(&mut contents) {
        panic!("Could not read file: {}", why);
    }
    let mut vector_content: Vec<&str> = vec![""; 80];
    let mut i: usize = 0;
    for x in contents.split(",") {
        vector_content[i] = x.trim_end();
        i += 1;
    }
    let mut vector_enteros = Vec::new();
    for x in vector_content.iter() {
        match x.trim().parse::<i32>() {
            Ok(i) => vector_enteros.push(i),
            Err(_) => continue,
        }
    }
    for x in vector_enteros.iter() {
        print!("{x} ");
    }
}
