use std::io;

#[derive(Debug)]
struct Block{
    previous_hash : String,
    block_no : u128,
    data : Vec<Transaction>
}

#[derive(Debug)]
struct Transaction {
    to: String,
    from: String,
    amount: u32
}

fn main(){
    let genesis_transaction = Transaction{
        to: String::from("A"),
        from: String::from("B"),
        amount: 50
    };

    let genesis_block = Block{
        previous_hash: String::from("0".repeat(64)),
        block_no : 0,
        data:  vec![genesis_transaction]
    };

    let mut choice_of_block_info = String::new();

    println!("Please select the options you want to show!\n1. Block Info\n2. The Previous Block Hash\n3. The Block Number\n4. The Transactions Details");


    io::stdin().read_line(&mut choice_of_block_info).expect("The selection not found!");

    let choice: u32 = match choice_of_block_info.trim().parse() {
        Ok(num) =>  num,
        Err(error)=>{
            println!("Selection not found, the error: {}", error);
            return;
        }
    };


    match choice {
        1 => {
            println!("The genesis block details: {:?}", genesis_block);
        },
        2 => {
            println!("The previous hash of the block: {}", genesis_block.previous_hash);
        },
        3 => {
            println!("The block number of the block: {}", genesis_block.block_no);
        },
        4 => {
            println!("The transaction details of the block: {:?}", genesis_block.data)
        },
        _ => {
            println!("Not found!")
        }
    }


    


}