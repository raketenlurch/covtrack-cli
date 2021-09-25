#![allow(unused)]
use std::env;
use std::fs;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = "test.txt";

    write_vector_to_file(arguments, filename.to_string());
}

// zwei Argumente für die Funktion: Vector mit Argumenten u. Pfad zur Datei in die reingeschrieben wird
fn write_vector_to_file(arguments: Vec<String>, filename: String) {
    let mut file = fs::OpenOptions::new().write(true).append(true).open(filename);

    for argument in arguments {
        println!("{}", argument);
    }
}