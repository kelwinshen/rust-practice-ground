fn main(){
    //let's learn some mutable reference
    let mut name: String = String::from("Komala");
   

    marry_to_kelwin(&mut name);
    println!("{}", name);

   
    let name1 = &mut name;
    // let name2 = &mut name;  //this is not allowed due to mutable reference is done at the same time, have to be drop the first operation first, then we can borrow the mutable reference to the other
    println!("{}", name1);

    let name3 = &mut name; //this is allowed because name1 is already dropped
    println!("{}", name3);


   

}

fn marry_to_kelwin(x: &mut String){
    x.push(' ');
    x.push_str("Shen");
}

