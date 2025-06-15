use std::collections::HashMap;


//Create enum for RevelState
#[derive(Debug, Clone)]
enum RevealState{
    Placeholder,
    Revealed
}

//Struct of the NFT
#[derive(Debug, Clone)]
struct NFT{
    name: String,
    owner: String,
    metadata_uri: String,
    state: RevealState
}

//Struct of the NFT Manager
#[derive(Debug, Clone)]
struct NFTManager{
    nfts: HashMap<String, NFT>
}

//Define NFT Actions Trait:
trait NFTActions {
    fn mint_nft(&mut self, name:&str, owner:&str, metadata_uri:String)->NFT;

    fn reveal_nft(&mut self,  nft: &mut NFT, new_metadata_uri:String);
}

impl NFTActions for NFTManager{

    fn mint_nft(&mut self, name:&str, owner:&str, metadata_uri:String)->NFT{
        let nft = NFT{    
            name: name.to_string(),
            owner: owner.to_string(),
            metadata_uri: metadata_uri,
            state: RevealState::Placeholder
        };
        self.nfts.insert(String::from(&nft.name), nft.clone());
        nft
    }

    fn reveal_nft(&mut self,  nft: &mut NFT, new_metadata_uri:String){
        match nft.state {
            RevealState::Placeholder => {
                nft.state = RevealState::Revealed;
                nft.metadata_uri = new_metadata_uri;
                nft.name = String::from("King of The Hills");
            println!("The NFT is successful being reveal!");
            },
            RevealState::Revealed=>{
                println!("The NFT is already revealed!");
            }
        }
    }

}


fn main(){
    let mut manager = NFTManager{
        nfts: HashMap::new()
    };

    let mut nft1 = manager.mint_nft( "#1 MYSTERY BOX", "KELWIN SHEN", String::from("IPFS://MysteryBox") );
    println!("Name of the NFT: {}", nft1.name);
    println!("State of the NFT: {:?}", nft1.state);

    manager.reveal_nft(&mut nft1,  String::from("IPFS://KingOfTheHill"));
    println!("Name of the NFT: {}", nft1.name);
    println!("State of the NFT: {:?}", nft1.state);

   
}