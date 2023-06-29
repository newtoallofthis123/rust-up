// mod guess_game;

struct Person{
        name: String,
        age: u8,
        job: String,
    }

fn print_struct(person: &Person){
    println!("{:?}", (person.name.clone(), person.age, person.job.clone()));
}

fn build_struct(name:String, age:u8, job:String)->Person{
    return Person{
        name,
        age,
        job
    };
}

fn increment_age(person: &mut Person){
    person.age += 1;
}

pub fn run() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to Read Line");
    let info: Vec<&str> = input.trim().split(',').collect();
    let age:u8 = info[1].trim().parse().expect("Age not an integer");
    let info:(String, u8, String) = (
        String::from(info[0].trim()), 
        age, 
        String::from(info[2].trim())
    );
    let mut person:Person = build_struct(
        info.0, 
        info.1, 
        info.2
    );
    increment_age(&mut person);
    print_struct(&person);
}