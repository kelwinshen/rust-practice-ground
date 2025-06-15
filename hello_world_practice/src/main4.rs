fn main(){
    let mut name: String = String::from("Kelwin");

    let name1 = &name;
    let name2 = &name;
    println!("{}, {}", name1, name2);

    let name3 = &mut name;
    appendix_suffix(name3);
    println!("{}", name);

}

fn appendix_suffix(x: &mut String){
    let a = " Jr.";
    x.push_str(a);
}