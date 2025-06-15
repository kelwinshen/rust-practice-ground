use std::collections::HashMap;

// Step 1: Enum to represent the state of the NFT
#[derive(Debug, Clone)]
enum RevealState {
    Placeholder,
    Revealed,
}

// Step 2: Struct to represent an NFT
#[derive(Debug, Clone)]
struct NFT {
    name: String,
    owner: String,
    metadata_uri: String,
    state: RevealState,
}

// Step 3: Define a trait for NFT behaviors, including minting and revealing
trait NFTActions {
    fn mint_nft(&mut self, name: &str, owner: &str, placeholder_uri: &str) -> NFT;
    fn reveal(&mut self, nft: &mut NFT, new_metadata_uri: String);
}

// Step 4: Implement the NFTActions trait for NFTManager
struct NFTManager {
    // A collection of NFTs managed by the manager
    nfts: HashMap<String, NFT>,
}

impl NFTActions for NFTManager {
    // Minting an NFT and adding it to the manager's collection
    fn mint_nft(&mut self, name: &str, owner: &str, placeholder_uri: &str) -> NFT {
        let nft = NFT {
            name: name.to_string(),
            owner: owner.to_string(),
            metadata_uri: placeholder_uri.to_string(),
            state: RevealState::Placeholder,
        };
        // Add the newly minted NFT to the collection
        self.nfts.insert(name.to_string(), nft.clone());
        println!("NFT '{}' minted for owner '{}'", name, owner);
        nft
    }

    // Revealing an NFT's real metadata
    fn reveal(&mut self, nft: &mut NFT, new_metadata_uri: String) {
        match nft.state {
            RevealState::Placeholder => {
                nft.metadata_uri = new_metadata_uri;
                nft.state = RevealState::Revealed;
                println!("NFT '{}' is now revealed!", nft.name);
            }
            RevealState::Revealed => {
                println!("NFT '{}' is already revealed!", nft.name);
            }
        }
    }
}

fn main() {
    // Create an NFT manager that will handle minting and revealing
    let mut manager = NFTManager {
        nfts: HashMap::new(),
    };

    // Step 5: Mint a new NFT using the manager
    let mut nft1 = manager.mint_nft("Mystery Box #1", "Alice", "ipfs://placeholder/mystery.json");

    // Display NFT information before reveal
    println!("NFT before reveal: {:?}", nft1);

    // Step 6: Reveal the NFT using the manager
    manager.reveal(&mut nft1, "ipfs://final_metadata/real_image.json".to_string());

    // Display NFT information after reveal
    println!("NFT after reveal: {:?}", nft1);

    // Attempt to reveal again (to test behavior)
    manager.reveal(&mut nft1, "ipfs://another_try.json".to_string());

    
}
