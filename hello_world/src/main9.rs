use std::io;

struct Identity{
    firstname: String,
    lastname: String
}

fn main(){

    //Initialze the data first
    let kelwin_identity: Identity = Identity {
        firstname: String::from("Kelwin"),
        lastname: String::from("Shen")
    };


    println!("Please select this menu");
    println!("1. Print out your first name\n2. Print out your first name\n3. Print out your fullname");
    
    let mut options: String = String::new();
    io::stdin().read_line(&mut options).expect("Make sure select right options");

    let options: u32 = match options.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please make sure you select the right options");
            return;
        }
    };

    // match options {
    //     1 => {
    //         println!("{}", kelwin_identity.firstname);
    //     } ,
    //     2 => {
    //         println!("{}", kelwin_identity.lastname);
    //     }, 
    //     3 => {
    //         println!("{} {}", kelwin_identity.firstname, kelwin_identity.lastname);
    //     },
    //     _ => {
    //         println!("Something wrong with your options!");
    //     }
    // };
    //Conditional Statements
    if options == 1 {
        println!("{}", kelwin_identity.firstname);
    } else if options == 2 {
        println!("{}", kelwin_identity.lastname);
    } else if options == 3 {
        println!("{}, {}", kelwin_identity.firstname, kelwin_identity.lastname);
    } else {
        println!("Something wrong with your options!");
    }
}