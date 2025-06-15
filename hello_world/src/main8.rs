struct Identity{
    name: String,
    age: u32,
    gpa: f64,
    enroll: bool
}

struct Rect{
    width: f64,
    height: f64
}

struct Color(i32, i32, i32);

fn area(rect: &Rect)-> f64{
   let total_area = rect.height * rect.width;
   total_area
}


enum Transactions {
    TokenTransfer(String),
    NFTTransfer(String)
}


fn calculate_transaction_fee (tx: &Transactions)-> u32 {
    match tx {
        Transactions::TokenTransfer(token) => {
            if token == "BTC" {
                10
            } else if token == "ETH" {
                5
            } else {
                1
            }
        },
        Transactions::NFTTransfer(nft) => {
            if nft == "APE" {
                10
            } else {
                5
            }
        }
    }
}   

fn main(){
   let identity_of_kelwin = Identity {
        name: String::from("Kelwin Shen"),
        age: 24,
        gpa: 3.88,
        enroll: false
   };

   println!("name: {}", identity_of_kelwin.name);
   println!("age: {}", identity_of_kelwin.age);
   println!("gpa: {}", identity_of_kelwin.gpa);
   println!("enroll: {}", identity_of_kelwin.enroll);

   let small_rect = Rect {
        height: 10.0,
        width: 20.0
   };

   let small_rect_area = area(&small_rect);

   println!("The small rect area is {}", small_rect_area);

   
   let color_design = Color(2, 3, 4);
   println!("Color design is: {}, {}, {}", color_design.0, color_design.1, color_design.2);


   let tx = Transactions::TokenTransfer(String::from("ETH"));

   let tx_fee = calculate_transaction_fee(&tx);
   println!("The transaction fee is: {}", tx_fee);


   let tx = Transactions::NFTTransfer(String::from("APE"));
   let tx_fee = calculate_transaction_fee(&tx);
   println!("The transaction fee is: {}", tx_fee);



 
}