mod block;
mod chain;

use chain::Chain;
use std::io;

fn main() {
    let mut blockchain = Chain::new();

    // Listen block creation
    loop {
        println!("--------------------");
        let block = &blockchain.getLastBlock();
        println!("New block added to the chain:\n {:?}", block);
        println!("--------------------");

        // Allow user to create block for poc
        println!("Enter new data to the blockchain, or type 'quit'/'q' to exit:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input = input.trim().to_owned();

        if input == "quit" || input == "q" {
            break;
        }

        blockchain.add_block(input);
    }
}
