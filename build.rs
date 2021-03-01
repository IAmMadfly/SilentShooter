use std::fs::File;
use std::io::Write;

use chrono;

struct Printer {
    file:       File
}

impl Printer {

    fn new(file_path: &str) -> Self {
        Printer {
            file:   File::create(&file_path).expect("Failed to open file!")
        }
    }

    fn println(&mut self, words: &str) {

        let instant  = chrono::offset::Local::now();

        self.file
            .write(instant.to_string().as_bytes())
            .expect("Failed to write DateTime!");

        self.file
            .write(": ".as_bytes())
            .expect("Failed to write DateTime!");

        self.file
            .write(words.as_bytes())
            .expect("Failed to write to output file!");

        self.file.write("\n".as_bytes()).expect("Failed to wrtite end line!");
    }
}


fn main() {
    println!("--- RUNNING BUILDER!! ---");

    let mut build_output_file = Printer::new("build_output.txt");

    build_output_file.println("--- STARTING BUILD ---");

}