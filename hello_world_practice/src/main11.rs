use std::collections::HashMap;

struct Account{
    pubkey: String,
    balance: u64
}

struct TokenManager{
     accounts: HashMap<String, Account>
}

trait TokenActions{
    fn mint(&mut self, amount: u64, pubkey: &str);
    fn transfer(&mut self, amount: u64, from_pubkey: &str, to_pubkey: &str)->Result<(), String>;
    fn balance(&mut self, pubkey: &str);
}

impl TokenActions for TokenManager{
    fn mint(&mut self, amount: u64, pubkey: &str){
      let account =  &mut self.accounts.entry(pubkey.to_string()).or_insert(
        Account{
            pubkey: pubkey.to_string(),
            balance: 0
        }
      );

      account.balance += amount;
      println!("Success minted {} tokens to {}", amount, pubkey);
    }

    fn transfer(&mut self,amount: u64, from_pubkey: &str, to_pubkey: &str)->Result<(), String>{
        let  from_account = &mut self.accounts.entry(from_pubkey.to_string()).or_insert(
            Account{
                pubkey: from_pubkey.to_string(),
                balance: 0
            }
          );


        if *&from_account.balance < amount {
            Err("The balance is not enough to transfer that amount!".to_string())
        } else {
            from_account.balance -= amount;
            let clone_from_pubkey = from_account.pubkey.clone();
            let  to_account =  &mut self.accounts.entry(to_pubkey.to_string()).or_insert({
                Account{
                    pubkey: to_pubkey.to_string(),
                    balance: 0
                }
            });
            let clone_to_pubkey = to_account.pubkey.clone();
            to_account.balance += amount;
            println!("{} success end {} tokens to {}", clone_from_pubkey, clone_to_pubkey, amount);
            Ok(())
        }
    }

    fn balance(&mut self, pubkey: &str){
        let account = &mut self.accounts.entry(pubkey.to_string()).or_insert(
            Account{
                pubkey: pubkey.to_string(),
                balance: 0
            }
        );
            let balance = account.balance;
            println!("The balance of {} is {}", account.pubkey, balance);
    }

}


fn main(){
    let mut token_manager = TokenManager{
        accounts: HashMap::new()
    };

    token_manager.mint(20000, "0xKelwinshen");

    token_manager.balance("0xKelwinshen");
    token_manager.balance("0xEka");

    match token_manager.transfer(12000, "0xKelwinshen", "0xEka") {
        Ok(()) => {},
        Err(error) => {
            println!("{}", error);
        }
    }

    token_manager.balance("0xKelwinshen");
    token_manager.balance("0xEka");


 



}