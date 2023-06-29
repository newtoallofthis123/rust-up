use std::fs;
use rand::Rng;

struct File {
    name: String,
    data: String,
}

impl File {
    fn read_file(&mut self) {
        println!("Reading file with path: {}", &self.name);
        let content = fs::read_to_string(&self.name)
            .expect("Failed to read file");
        self.data = content;
        println!("With text:\n{}", &self.data);
    }

    fn write_file(&mut self) {
        println!("Writing file with path: {}", &self.name);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        fs::write(&self.name, input.to_string())
            .expect("Failed to write file");
        println!("File written successfully");
    }

    fn delete_file(&mut self) {
        println!("Deleting file with path: {}", &self.name);
        fs::remove_file(&self.name)
            .expect("Failed to delete file");
        println!("File deleted successfully");
    }

    fn new_file(&mut self) {
        println!("Creating file with path: {}", &self.name);
        fs::File::create(&self.name)
            .expect("Failed to create file");
        println!("File created successfully");
    }
}

fn run() {
    let args: Vec<_> = std::env::args().collect();
    let filename = &args[1];
    let mut file = File {
        name: filename.to_string(),
        data: String::new(),
    };
    println!("Random 3 digit number: {}", rand::thread_rng().gen_range(10..100));
    file.read_file();
    let mut choice = String::new();
    println!("Do you want to write to the file? (y/n)");
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    if choice.trim() == "y" {
        file.write_file();
    }
}
