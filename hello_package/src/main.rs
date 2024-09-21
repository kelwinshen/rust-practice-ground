// In src/main.rs
mod front_of_house;


// FILL in the blank and FIX the errors
fn main() {
    assert_eq!(hello_package::front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
    println!("{}", hello_package::front_of_house::hosting::seat_at_table());
    println!("{}", hello_package::eat_at_restaurant());
}