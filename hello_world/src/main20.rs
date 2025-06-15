//Traits are like interfaces in other programming languages. They are used to define shared behavior that types can implement.

enum Category {
    FungibleToken,
    NonFungibleToken,
}

struct Token {
    name: String,
    price: f64,
    category: Category,
}

trait GetInfo {
    fn get_name(&self) -> String;
    fn get_price(&self) -> String;
}

impl GetInfo for Token {
    fn get_name(&self) -> String {
        let name_info = match &self.category {
            Category::FungibleToken => format!("Name of the FT: {}", &self.name),
            Category::NonFungibleToken => format!("Name of the NFT: {}", &self.name),
        };
        name_info
    }

    fn get_price(&self) -> String {
        let price_info = match &self.category {
            Category::FungibleToken => format!("Price of FT: {}", &self.price),
            Category::NonFungibleToken => format!("Price of NFT: {}", &self.price),
        };
        price_info
    }
}

fn main() {
    let token = Token {
        name: String::from("BTC"),
        price: 65000.00,
        category: Category::FungibleToken,
    };
    println!("{}", token.get_name());
    println!("{}", token.get_price());

    let nft = Token {
        name: String::from("ApeYatchClub"),
        price: 1000.00,
        category: Category::NonFungibleToken,
    };

    println!("{}", nft.get_name());
    println!("{}", nft.get_price());
}
