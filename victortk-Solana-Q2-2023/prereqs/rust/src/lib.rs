use solana_client::rpc_client::RpcClient;
use solana_program::{
    system_instruction::transfer, 
    system_program,
};
use solana_sdk::{message::Message,signature::{Keypair, Signer, read_keypair_file, write_keypair_file}, pubkey::Pubkey,transaction::Transaction} ;
use std::str::FromStr;
mod programs;
use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};
//Create a new keypair 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keygen() {
        let  kp = Keypair::new() ;
        println!("Youve generated a new Solana wallet: {}",5);
        println!("To save your wallet , copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
        println!("The secret key is {:?}", kp.pubkey());
        write_keypair_file(&kp, "dev-wallet.json").expect("Could not write key-pair");
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        //Connected to Solana Devnet RPC Client 
        let client = RpcClient::new(RPC_URL);

        println!("The secret key is {:?}", keypair.pubkey());
        let y  =  client.request_airdrop_with_blockhash(&keypair.pubkey(), 2_000_000_000u64,&client.get_latest_blockhash().unwrap()) ;

        println!(" the result ----> > {:?} ",y );
        match y {
            Ok(s) => {
                println!("Success! Check out your TX here:") ;
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {} " , e.to_string())
        };
    }

    #[test]
    fn transfer_sol(){
        // import our keypair 
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        const RPC_URL: &str = "https://api.devnet.solana.com";

        //Define our WBA public key 

        let to_pubkey = Pubkey::from_str("").unwrap(); //jjjjjjj

        //Create a Solana devnet connection 

        let rpc_client = RpcClient::new(RPC_URL);

        // Get the recent blockhash 

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(),
                &to_pubkey,
                1_000_000
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash 
        );

        // Send the transaction 
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");

        //Print out transaction out 
        println!("
        Success! Check out your TX here: 
        https://explorer.solana.com/tx/{}/?cluster=devnet", signature)


    }
//     #[test]
//     fn transfer_empty() {
//          // import our keypair 
//         let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
//         const RPC_URL: &str = "https://api.devnet.solana.com";

//         //Define our WBA public key 

//         let to_pubkey = Pubkey::from_str("").unwrap();

//         //Create a Solana devnet connection 

//         let rpc_client = RpcClient::new(RPC_URL);

//         // Get the recent blockhash 

//         let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");
//         //Get balance of dev wallet 
//         let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

//         //Create a test transaction to calculate fees 
//         let message = Message::new_with_blockhash(
//             &[transfer(
//                 &keypair.pubkey(),
//                 &to_pubkey, 
//                 balance,
//             )],
//             Some(&keypair.pubkey()),
//             &recent_blockhash
//         );

//         // Calculate exact fee rate to transafer entire SOL amount out of account minud fees 
//         let fee = rpc_client 
//         .get_fee_for_message(&message)
//         .expect("Failed to get fee calculator");

//         //Deduct fee from lamports amount and create a TX with correct balance 
//         let transaction  = Transaction::new_signed_with_payer(
//             &[transfer(
//                 &keypair.pubkey(),
//                 &to_pubkey,
//                 balance - fee, 
//             )],
//             Some(&keypair.pubkey()),
//             &vec![&keypair],
//             recent_blockhash
//         );
//     }

//     #[test]
//    fn submit_prereq() {
//     const RPC_URL: &str = "https://api.devnet.solana.com";
//     //Create a Solana devenet connection 
//     let rpc_client = RpcClient::new(RPC_URL);
//     // Let's define our accounts 
//     let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");

//     let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
//     //Define our instruction data 
//     let args = CompleteArgs {
//         github: b"testaccount".to_vec()
//     };

//     //Get recent blockhash 
//     let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

//     //Now we can invoke the "complete " function 
//     let transaction = WbaPrereqProgram::complete(
//         &[&signer.pubkey(), &prereq, &system_program::id()],
//         &args, 
//         Some(&signer.pubkey()),
//         &[&signer],
//         blockhash
//     );

//     //Send the transaction 
//     let signature = rpc_client.send_and_confirm_transaction(&transaction)
//     .expect("Failed to send transaction");

// //Print our transaction out 
// println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
//    }
}
