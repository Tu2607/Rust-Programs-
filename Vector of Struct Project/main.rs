//Struct Playground with Vector of Struct
use std::io::{self,Write};

struct Info {
    name: String,
    age: i32,
    sex: String,
}

fn main () {
    let mut list: Vec<Info> = vec![]; //This is a vector of struct
    
    println!("Do you want to add some information? Type yes if you do");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer)
        .expect("Can't read it!");
    /******There is a a problem in this segment of code for the match statement */
    match &answer as &str {
        "Yes" => {list.push(addon())}, 
        "No" => {println!("Thank you for your time")},
        _ => println!("Type something else"),
    }
    /********End of Segment********/
    display(list);
}

//This function create a new struct of type info and return it
fn addon () -> Info {
    let mut Tu = Info {
        name: u_name(),
        age: u_age(),
        sex: u_sex(),
    };
    Tu 
}

//This function read in a string for the name and return that string
fn u_name () -> String {
    println!("Please type in your name");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Can't read name");
    io::stdout().flush().unwrap(); 
    input
}

//Read in the age and return it
fn u_age () -> i32 {
    println!("Please type in your age");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Can't read name");
    io::stdout().flush().unwrap(); 
    let input :i32 = input.trim().parse()
        .expect("Can't read age");
    input
}

//Read in the gender and return the string
fn u_sex () -> String {
    println!("Please type in your sex");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Can't read your gender :D");
    io::stdout().flush().unwrap(); 
    input
}

//Looping through the vector to display each struct
fn display(List: Vec<Info>) {
    let mut counter = 0;
    while counter < List.len() {
        println!("Name: {}", List[counter].name);
        println!("Age: {}", List[counter].age);
        println!("Sex: {}", List[counter].sex);
        println!("", );
        counter = counter + 1;
    }
}
