use blockchainlib::*;

use p256::{
    ecdsa::{SigningKey,Signature, signature::Signer},
};
use rand_core::OsRng; 
use p256::ecdsa::{VerifyingKey, signature::Verifier};

use sha2::{Sha256, Digest};
use hex;

fn main () {
    let private_key = SigningKey::random(&mut OsRng);
    let  public_key = VerifyingKey::from(&private_key);
    let  mut hasher = Sha256::new();
    

    let mut difficulty = 0x000fffffffffffffffffffffffffffff;
    

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "PAVAN".to_owned(),
                    value: 50,
                },
                transaction::Output {
                    to_addr: "MANOJ".to_owned(),
                    value: 7,
                },
            ],
        },
    ], difficulty);
    let mut prev_timestamp=now();

    let  cliper=genesis_block.hash();
    hasher.update(cliper);
    let hashed_result = hasher.finalize();
    let hashe_hex = hex::encode(hashed_result.to_vec());

     let  message= hashe_hex.as_bytes();
    let  signature:Signature =private_key.sign(message);
    if (public_key.verify(message,&signature)).is_ok(){
    genesis_block.mine();
    }
    else{
        println!("FAKE BLOCK MINED");
    }
   
     if (now()-prev_timestamp)>1000
     {
       difficulty= 0x0000ffffffffffffffffffffffffffff;
     }
      prev_timestamp=genesis_block.timestamp;

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "VISHAL".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone()
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "PAVAN".to_owned(),
                    value: 36,
                },
                transaction::Output {
                    to_addr: "MANOJ".to_owned(),
                    value: 12,
                },
            ],
        },
    ], difficulty);
    let  mut hasher = Sha256::new();
    let  cliper1=block.hash();
    hasher.update(cliper1);
    let hashed_result = hasher.finalize();
    let hashe_hex = hex::encode(hashed_result.to_vec());

     let  message= hashe_hex.as_bytes();
    let  signature:Signature =private_key.sign(message);
    if (public_key.verify(message,&signature)).is_ok(){
       block.mine();
       }
       else{
             println!("FAKE BLOCK MINED");
         }
         if (now()-prev_timestamp)>1000
         {
           difficulty= 0x0000ffffffffffffffffffffffffffff;
         }
          prev_timestamp=block.timestamp;

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add block");
    let mut block1 = Block::new(2, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "UZAIR".to_owned(),
                    value: 536,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[1].transactions[1].outputs[0].clone()
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "UZAIR".to_owned(),
                    value: 36,
                },
                transaction::Output {
                    to_addr: "MANOJ".to_owned(),
                    value: 12,
                }
            ],
        },
        Transaction {
            inputs: vec![ blockchain.blocks[1].transactions[1].outputs[0].clone() ],
            outputs: vec![
                transaction::Output {
                    to_addr: "PAVAN".to_owned(),
                    value: 12,
                },
            ],
        }
    ], difficulty);

    block1.mine();
     if (now()-prev_timestamp)>1000
    {
      difficulty= 0x0000ffffffffffffffffffffffffffff;
    }
    // prev_timestamp=block1.timestamp;

    println!("Mined block {:?}", &block1);


}
