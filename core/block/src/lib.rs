/*
Copyright 2018-Present The AfricaOS Authors
This file is part of the AfricaOS library.
The AfricaOS Platform is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
The AfricaOS Platform is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with the AfricaOS Platform. If not, see <http://www.gnu.org/licenses/>.
*/

#[macro_use]
extern crate json;
use json::{JsonValue};
use std::io::{Error, ErrorKind};
use timestamp::{Timestamp, NewTimestamp, StringToTimestamp};
use transaction::{Transaction,
                  ReadTransactionFromDB,
                  ExecuteTransactions,
                  JsonConverter,
                  ClearTransactionIndex};

use db::{DB,
         DBWriteBlock,
         DBReadBlock,
         FileDirectoryReader};

use hash::{Hasher, CalculateSHA256Hash};


#[derive(Clone,Debug,PartialEq)]
pub struct Block {
  pub block_id: i64,
  pub block_hash: String,
  pub block_parent_hash: String,
  pub block_time: Timestamp,
  pub proposal_hash: String,
  pub block_data: String,
  pub transactions: Vec<Transaction>
}


/*
@name CreateBlockIndex
@desc
*/
pub trait CreateBlockIndex {
    fn create_block_index() -> ();
}

/*
@name CreateBlockIndex
@desc to initially create the block index
*/
impl CreateBlockIndex for Block {
    fn create_block_index() -> (){
        let new_block_index = object!{
            "blocks" => object!{}
        };
        let index_to_write: String = json::stringify(new_block_index);
        match DB::write_block_index(index_to_write) {
            Ok(_) => {
                println!("Successfully wrote block index");
            },
            Err(_) => {
                println!("Failure writing block index");
            }
        }
    }
}

/*
@name ReadBlockFromDB
@desc trait to read a proposal from a db
*/
pub trait ReadBlockFromDB {
    fn get_block_index_as_json() -> Option<JsonValue>;
    fn get_latest_block_id() -> Option<i64>;
    fn get_all_blocks() -> Option<Vec<Block>>;
    fn get_block_by_block_id(block_id: i64) -> Option<Block>;
}

/*
@name ReadBlockFromDB for DB
@desc
*/
impl ReadBlockFromDB for DB {

    /*
    @name get_block_index_as_json
    @desc return the block index as a json object
    */
    fn get_block_index_as_json() -> Option<JsonValue> {
        let block_index: String = match DB::read_block_index() {
            Some(i) => {
                //TODO: parse/verify proposal index
                i
            },
            None => String::from("NO INDEX")
        };
        println!("Block index: {}", block_index);
        //TODO: convert DB json string to json
        let parsed_result: Result<JsonValue, json::Error> = json::parse( &format!(r#"{}"#, block_index) );
        match parsed_result {
            Ok(parsed) => {
                println!("block index parsed: {}", parsed["blocks"]);
                println!("BI parse example 0 {}", parsed["blocks"]["0"]);
                Some(parsed)
            },
            Err(_) => {
                println!("get_block_index_as_json(), could not parse ::: {}", block_index);
                None
            }
        }
    }

    /*
    @name get_latest_block_id
    @desc get the block id
    @fix - without the loop
    @problem references length of block index
    */
    fn get_latest_block_id() -> Option<i64> {
        let block_index_parsed_option: Option<JsonValue> = Self::get_block_index_as_json();

        match block_index_parsed_option {
            Some(block_index_parsed) => {
                let all_blocks = &block_index_parsed["blocks"];

                if all_blocks.is_empty() {
                    //None
                    Some(-1)
                } else {
                    // TODO: dont calculate number of blocks by length in block index
                    //let mut amount_of_blocks: i64 = all_blocks.len() as i64;
                    //TODO: invoke get_next_block_id_from_index() instead
                    //match Block::get_next_block_id() {
                    match Block::get_next_block_id_from_index() {
                        Some(block_id) => {
                            println!("get_latest_block_id, Block::get_next_block_id(), get_next_block_id_from_index() is SOME");
                            Some(block_id)
                        },
                        None => {
                            println!("get_latest_block_id, Block::get_next_block_id() is NONE");
                            Some(-1) // return -1 for default, could be error
                        }
                    }
                }
            },
            None => None
        }


    }


    /*
    @name get_all_proposals
    @desc get all proposals from the proposals directory
    */
    fn get_all_blocks() -> Option<Vec<Block>> {
        //TODO: read proposal index
        let parsed: Option<JsonValue> = Self::get_block_index_as_json();
        match parsed {
            Some(parsed) => {
                let mut all_blocks_vector: Vec<Block> = Vec::new();
                let blocks_iter = parsed["blocks"].entries();
                //let blocks_iter = parsed.members();
                for (id, block) in blocks_iter {
                    //println!("get_all_blocks(), block: {}:{}", id, block);
                    let parsed_block: Result<Block, String> = Block::from_json(block.clone());
                    match parsed_block {
                        Ok(block) => {
                            //TODO: DB::index_block_sync_check(block)
                            all_blocks_vector.push(block);
                        },
                        Err(err) => {
                            println!("get_all_blocks ERROR parsed_block: {:?}", err);
                        }
                    }
                }
                Some(all_blocks_vector)
            },
            None => {
                None
            }
        }
    }

    /*
    @name get_block_by_block_id
    @desc return block by block id
    */
    fn get_block_by_block_id(block_id: i64) -> Option<Block> {
        let block_string_result: Option<String> = Self::read_block(block_id);
        match block_string_result {
            Some(block_string) => {
                let block: Result<Block, String> = Block::from_string(block_string);
                if block.is_ok() {
                    Some(block.unwrap())
                } else {
                    None
                }
            },
            None => {
                None
            }
        }
    }

}


/*
@name BlockIDGenerator
@desc trait for generating a next block_id
@fix fixes get_latest_block_id problem of counting length of block index, this counts files
*/
pub trait BlockIDGenerator {
    fn parse_filename_for_block_id(filename: &str) -> Option<i64>;
    fn get_next_block_id() -> Option<i64>;
    fn get_next_block_id_from_index() -> Option<i64>;
}

impl BlockIDGenerator for Block {
    /*
    @name parse_filename_for_block_id
    @desc parse a i64 from a string block filename
    */
    fn parse_filename_for_block_id(filename: &str) -> Option<i64> {
        let filename_block_id:Vec<&str> = filename.split("_").collect::<Vec<_>>();
        match filename_block_id[0] {
            "block" => {
                if filename_block_id.len() == 2 {
                    let block_filename_section: &str = filename_block_id[1];
                    let last_split_section: Vec<&str> = block_filename_section.split(".").collect::<Vec<_>>();
                    let filename_block_id: i64 = last_split_section[0].parse::<i64>().unwrap();
                    println!("last_split_section: {}", filename_block_id);
                    Some(filename_block_id)
                }else{
                    println!("no block_id in filename, block Filename length is != 2: {}", filename_block_id.len());
                    None
                }
            },
            _ => None
        }
    }

    /*
    @name get_next_block_id
    @desc generate the next block_id from all blocks on disk
    @deprecated due to counting files, instead of accessing index
    */
    fn get_next_block_id() -> Option<i64> {
        //read all directories
        let files:Vec<String> = DB::read_blocks_directory();
        let mut iter = (&files).into_iter();
        let mut highest_block_index: i64 = -1;
        //iterate over all proposal files
        while let Some(v) = iter.next(){
            //println!("Filename Iter: {}", v);
            //parse file name for proposal id
            let filename_split_vector = v.split("/").collect::<Vec<_>>();
            let last_split_section: &str = filename_split_vector[filename_split_vector.len() - 1];
            let parsed_block_id: Option<i64> = Self::parse_filename_for_block_id(last_split_section);
            //TODO: parse to a block type, and check status
            // should not count a none commited block in the highest_block_id calculation
            match parsed_block_id {
                Some(bid) => {
                    //Could keep this in memory globally?
                    if bid > highest_block_index {
                        highest_block_index = bid;
                    }
                },
                None => ()
            }
        }
        println!("highest_block_index: {}", highest_block_index);
        match highest_block_index {
            -1 => None,
            _ => Some(highest_block_index),

        }
    }

    /*
    @name get_next_block_id_from_index
    @desc generate next block id from index
    */
    fn get_next_block_id_from_index() -> Option<i64> {
        let parsed_option: Option<JsonValue> = DB::get_block_index_as_json();
        match parsed_option {
            Some(mut block_index) => {

                let blocks_iter = &block_index.clone()["blocks"];
                let last_block_option = blocks_iter.entries().last().clone();
                match last_block_option {
                    Some(last_block) => {
                        let last_block_json = last_block.1;

                        // TODO: error handling
                        Some(last_block_json["block_id"].as_i64().unwrap())

                    },
                    None => {
                        None
                    }
                }

            },
            None => None
        }
    }
}


/*
@name WriteProposalToDB
@desc trait to write a proposal to the DB
*/
pub trait WriteBlockToDB {
    fn write_block(block: Block) -> Result<String,std::io::Error>;
}

impl WriteBlockToDB for Block {
    fn write_block(block: Block) -> Result<String,std::io::Error>{
        let block_json: JsonValue = Self::to_json(block.clone());
        DB::write_block_to_sql(block.clone().block_id, block_json.to_string())
    }
}

/*
    @name HashBlock
*/
trait HashBlock {
    fn hash_block(block_id: i64, ts: Timestamp) -> String;
}

impl HashBlock for Block {
    fn hash_block(block_id: i64, ts: Timestamp) -> String {

        //TODO URGENT: include other properties when hashing block

        let raw_str: String = format!("{}{}", block_id, ts.timestamp);
        let str_to_hash: &str = raw_str.as_str();
        let string_to_hash: String = String::from( str_to_hash ) ;
        let submitted_proposal_hash: String = Hasher::calculate_sha256( string_to_hash );
        submitted_proposal_hash
    }
}

/*
    @name CreateNewBlock
    @desc create a new block
*/
pub trait CreateNewBlock {
    fn new(proposal_hash: String) -> Result<Block, String>;
}

impl CreateNewBlock for Block {
    fn new(proposal_hash: String) -> Result<Block, String> {
        let new_block_time: Option<Timestamp> = Timestamp::new();
        let latest_block_id: i64 = DB::get_latest_block_id().unwrap();
        let new_block_id: i64 = latest_block_id + 1;
        let parent_hash: String = match latest_block_id.clone() {
            -1 => String::from("00000000000000000"),
            _ => {
                let current_block_by_id: Option<Block> = DB::get_block_by_block_id(latest_block_id);
                if current_block_by_id.is_some() {
                    current_block_by_id.unwrap().block_hash
                } else {
                    return Err( String::from( &format!("Block error: current_block_by_id failed when finding parent hash, latest_block_id = {}", latest_block_id) ));
                }
            }
        };
        //TODO: Retrieve Transactions from TX pool
        let transactons_from_pool: Vec<Transaction> = DB::get_all_transactions();
        println!("CreateNewBlock, transactons_from_pool, tx count: {}", transactons_from_pool.len());
        match new_block_time {
            Some(ts) => {
                Ok(Block {
                    block_id: new_block_id,
                    // TODO: Transaction::json_from_tx_vec()
                    block_hash: Self::hash_block(new_block_id, ts.clone()),
                    block_parent_hash: parent_hash,
                    block_time: ts,
                    proposal_hash: proposal_hash,
                    block_data: String::from("TEST DATA"),
                    transactions: transactons_from_pool
                })
            },
            None => {
                Err(String::from("Block error: new block time failed"))
            }
        }
    }
}

/*
@name BlockToJson
@desc convert a block to json
*/
pub trait BlockToJson {
    fn to_json(block: Block) -> JsonValue;
}

impl BlockToJson for Block {
    fn to_json(block: Block) -> JsonValue {
        let block_object: JsonValue = object!{
            "block_id" => block.block_id,
            "block_hash" => block.block_hash,
            "block_parent_hash" => block.block_parent_hash,
            "block_time" => block.block_time.timestamp,
            "proposal_hash" => block.proposal_hash,
            "block_data" => block.block_data,
            "transactions" => Transaction::json_from_tx_vec(block.transactions).unwrap()
        };
        println!("Block to_json, data.dump(): {}", block_object.dump());
        block_object
    }
}

/*
@name BlockFromString
@desc convert a string to a block, optionally
*/
pub trait BlockFromString {
    fn from_string(stringed_block: String) -> Result<Block, String>;
}

impl BlockFromString for Block {
    fn from_string(stringed_block: String) -> Result<Block, String> {
        let block_option: Result<JsonValue, json::Error> = json::parse( &format!(r#"{}"#, stringed_block) );
        if block_option.is_ok() {
            let parsed: JsonValue = block_option.unwrap();
            println!("BlockFromString, from_string, parsed: {}", parsed.dump());
            println!("BlockFromString, from_string, block_time: {}", parsed["block_time"]);
            let block_time_string: String = parsed["block_time"].to_string();
            let block_time_parse: String = block_time_string;
            let parsed_timestamp: Option<Timestamp> = Timestamp::string_to_timestamp( String::from( format!("{}", block_time_parse) ) );
            match parsed_timestamp.clone() {
                Some(ts) => {
                    println!("BlockFromString, from_string, Parsed timestamp, Some: {}", parsed_timestamp.clone().unwrap().timestamp);
                    let tx_vec: Result<Vec<Transaction>, String> = Transaction::tx_vec_from_json( parsed["transactions"].clone() );
                    if tx_vec.clone().is_ok() {
                        println!("BlockFromString, from_string, tx_vec count: {}", tx_vec.clone().unwrap().len());
                        let parsed_block: Block = Block {
                              block_id: parsed["block_id"].as_i64().unwrap(),
                              block_hash: String::from(parsed["block_hash"].as_str().unwrap()),
                              block_parent_hash: String::from( parsed["block_parent_hash"].as_str().unwrap() ),
                              block_time: ts,
                              proposal_hash: String::from( parsed["proposal_hash"].as_str().unwrap() ),
                              block_data: String::from( parsed["block_data"].as_str().unwrap() ),
                              transactions: tx_vec.unwrap()
                        };
                        Ok(parsed_block)
                    } else {
                        Err(String::from("BlockFromString, from_string ERROR, tx_vec is NOT OKAY"))
                    }
                }
                None => {
                    Err(String::from("ERROR: BlockFromString, from_string, parsed_timestamp is None"))
                }
            }
        } else {
            Err(String::from("ERROR: BlockFromString, from_string, block_option is NOT OK"))
        }
    }
}

/*
    @name JsonToBlock
*/
pub trait JsonToBlock {
    fn from_json(payload: JsonValue) -> Result<Block, String>;
}

impl JsonToBlock for Block {
    fn from_json(payload: JsonValue) -> Result<Block, String> {
        println!("BLOCK, FROM_JSON: {}", json::stringify( payload.clone() ));
        let block_id: i64 = payload["block_id"].as_i64().unwrap();
        let block_hash: String = String::from(payload["block_hash"].as_str().unwrap());
        let block_parent_hash: String = String::from(payload["block_parent_hash"].as_str().unwrap());
        let block_time: Option<Timestamp> = Timestamp::string_to_timestamp(String::from(payload["block_time"].as_str().unwrap()));
        let proposal_hash: String = String::from(payload["proposal_hash"].as_str().unwrap());
        let block_data: String = String::from(payload["block_data"].as_str().unwrap());
        match block_time {
            Some(bt) => {
                Ok(Block{
                    block_id: payload["block_id"].as_i64().unwrap(),
                    block_hash: String::from(payload["block_hash"].as_str().unwrap()),
                    block_parent_hash: String::from(payload["block_parent_hash"].as_str().unwrap()),
                    block_time: bt,
                    proposal_hash: String::from(payload["proposal_hash"].as_str().unwrap()),
                    block_data: String::from(payload["block_data"].as_str().unwrap()),
                    transactions: Transaction::tx_vec_from_json( payload["transactions"].clone() ).unwrap()
                })
            },
            None => {
                Err(String::from("JsonToBlock, from_json, block_time is not valid"))
            }
        }
    }
}



/*
    @name VerifyBlockAnscestry
    @desc return a result regarding if the submitted block adheres to the anscestry rules
*/
trait VerifyBlockAnscestry {
    fn verify_block_anscestry(current_block: Block, proposed_block: Block) -> bool;
}

impl VerifyBlockAnscestry for Block {
    fn verify_block_anscestry(current_block: Block, proposed_block: Block) -> bool {
        match current_block {
            _ if current_block.block_hash == proposed_block.block_parent_hash => {
                true
            },
            _ => {
                false
            }
        }

    }
}

/*
    @name ValidateAcceptedProposalBlock
    @desc perform all validation steps necessary to commit block to ledger
*/
pub trait ValidateAcceptedProposalBlock {
    fn validate_block(block: Block) -> bool;
}

impl ValidateAcceptedProposalBlock for Block {
    fn validate_block(block: Block) -> bool {
        let current_block_id: Option<i64> = DB::get_latest_block_id();
        match current_block_id {
            Some(block_id) => {
                println!("validate_block, after current_block_id, block_id: {}", block_id);
                //TODO: CHECK IF GENESIS Block
                if block_id == 0 {
                    Self::process_genesis_block(block)
                } else if block_id > 0 {
                    Self::process_nongenesis_block(block)
                } else {
                    false
                }
            }
            None => {

                false
            }
        }
    }
}

/*
    @name process block
*/

trait ProcessBlock {
    fn process_genesis_block(submitted_block: Block) -> bool;
    fn process_nongenesis_block(submitted_block: Block) -> bool;
}

impl ProcessBlock for Block {
    fn process_genesis_block(submitted_block: Block) -> bool {
        println!("PROCESSING GENESIS BLOCK, submitted_block_id: {}", submitted_block.block_id);
        //TODO: COMPARE WITH BLOCKCHAIN CONFIG
        //TODO: CALL CHAIN LOGIC BLOCKVALIDATION
        Transaction::execute_block_transactions(submitted_block.transactions);
        true
    }

    fn process_nongenesis_block(submitted_block: Block) -> bool {
        println!("PROCESSING NONGENESIS BLOCK, submitted_block_id: {}", submitted_block.block_id);
        //TODO: PROBLEM
        //if we get the block with block id - 1, we just get the block "before" the submitted block
        // might want to just get the latest block....?
        //let current_block_id: Option<i64> = DB::get_latest_block_id();
        //let current_block_by_id: Option<Block> = DB::get_block_by_block_id( current_block_id.unwrap() );
        //TODO: validate the block getting added is the child of the most recent block
        let previous_block_by_id: Option<Block> = DB::get_block_by_block_id(submitted_block.block_id - 1);
        if previous_block_by_id.clone().is_some() {
            //TODO: verify parent hash
            match Self::verify_block_anscestry(previous_block_by_id.clone().unwrap(),
                                               submitted_block.clone()) {
                true => {
                    println!("process_nongenesis_block, verify_block_anscestry, SUCCESS");
                },
                false => {
                    println!("process_nongenesis_block, verify_block_anscestry, ERROR");
                    return false
                }
            }

            //TODO: Self.verify_block_id(submitted_block, current_block_by_id);
            match submitted_block.clone().block_id
                  ==
                  (previous_block_by_id
                   .clone()
                   .unwrap().block_id + 1) {
                true => {
                    println!("process_nongenesis_block, SUBMITTED_BLOCK ID IS EQUAL TO MY BLOCK ID + 1, SUCCESS");
                    //
                    Transaction::execute_block_transactions(submitted_block.transactions);
                },
                false => {
                    println!("process_nongenesis_block, SUBMITTED_BLOCK ID IS [NOT] EQUAL TO MY BLOCK ID + 1, ERROR");
                    //TODO modularize out into a is_valid_current_block(submitted_block)
                    let current_block_id_option: Option<i64> = DB::get_latest_block_id();  // Get my latest block
                    match current_block_id_option {
                        Some(current_block_id) => {
                            let current_block_by_id_option: Option<Block> = DB::get_block_by_block_id(current_block_id);
                            match current_block_by_id_option {
                                Some(current_block_by_id) => {

                                      // if block ids are the same
                                      // match ( submitted_block.clone().block_id == (current_block_by_id.clone().block_id) )
                                      //       &&
                                      //       // if the parent hashes are the same
                                      //       ( submitted_block.clone().block_parent_hash == current_block_by_id.clone().block_parent_hash )
                                      //       &&
                                      //       // different block hashes
                                      //       ( submitted_block.clone().block_hash != current_block_by_id.clone().block_hash ) {
                                      //     true => {
                                      //         println!("process_nongenesis_block, BLOCK ID MATCH MY TOP BLOCK, AND PARENT HASHES MATCH, and BLOCK HASHES ARE NOT THE SAME - SUCCESS");
                                      //     },
                                      //     false => {
                                      //         println!("process_nongenesis_block ERROR!, BLOCK ID MATCH MY TOP, BLOCK AND PARENT HASHES DONT MATCH, and BLOCK HASHES ARE NOT THE SAME - ERROR");
                                      //         return false
                                      //     }
                                      // }

                                      /*
                                      match current_block {
                                          _ if current_block.block_hash == proposed_block.block_parent_hash => {
                                              true
                                          },
                                          _ => {
                                              false
                                          }
                                      }
                                      */
                                      //return false


                                },
                                None => {
                                    println!("process_nongenesis_block, current_block_by_id_option is NONE");
                                    return false
                                }
                            }
                        },
                        //current_block_id_option is NONE
                        None => {
                            println!("process_nongenesis_block, current_block_id_option is NONE");
                            return false
                        }

                    }
                }
            }

            //TODO: MACRO: CUSTOM_BLOCK_VALIDATION!()
            // AFTER CHECKING IF BLOCK ID IS RIGHT SEQUENCE
            //TODO: verify block hash
            //TODO: CALL CHAIN LOGIC BLOCKVALIDATION
            //TODO: error handling
            //TODO: BUG THIS EXECUTE TX SITS OUTSIDE OF
            //Transaction::execute_block_transactions(submitted_block.transactions);
            true
        } else {
            //current block by block id is NOT SOME
            false
        }
    }
}

/*
    @name CommitBlock
    @desc Attempt to commit the block to the ledger, called after block validation
*/
pub trait CommitBlock {
    fn commit_if_valid(block: Block) -> Result<(),String>;
    fn commit_block(block: Block) -> Result<(), ()>;
}

impl CommitBlock for Block {
    fn commit_if_valid(block: Block) -> Result<(),String> {
        match Self::validate_block(block.clone()) {
            true => {
                match Self::commit_block(block.clone()) {
                    Ok(_) => {
                        println!("[BLOCK, CRITICAL] COMMIT BLOCK SUCCESSFUL");

                        // TODO: only clear if proposals[block_parent_hash].
                        // This clears all transactions waiting to get into chain
                        // upon every new block commit, regardless of who commited it
                        Transaction::clear_transaction_index();

                        Ok(())
                    },
                    Err(_) => {
                        println!("[BLOCK, CRITICAL] COMMIT BLOCK NOT SUCCESSFUL!");
                        Err(String::from("[BLOCK, CRITICAL] COMMIT BLOCK NOT SUCCESSFUL!"))
                    }
                }
            },
            false => {
                if block.clone().block_id == 0 {
                    match Self::commit_block(block.clone()) {
                        Ok(_) => {
                            println!("[BLOCK, CRITICAL] COMMIT BLOCK SUCCESSFUL");
                            Ok(())
                        },
                        Err(_) => {
                            println!("[BLOCK, CRITICAL] COMMIT BLOCK NOT SUCCESSFUL!");
                            Err(String::from("[BLOCK, CRITICAL] COMMIT BLOCK NOT SUCCESSFUL!"))
                        }
                    }
                } else {
                    Err(String::from("ERROR: commit_if_valid, block_id is NOT 0"))
                }
            }
        }
    }

    fn commit_block(block: Block) -> Result<(), ()> {
        //TODO: get higher block id
        //TODO: verify the ledger history (within window) is valid
        //TODO: if it is valid, actually write block to blocks directory, and alter block_index
        let mut block_index_option: Option<JsonValue> = DB::get_block_index_as_json();
        match block_index_option {
            Some(mut block_index) => {
                match block_index["blocks"].insert( &(format!("{}", block.clone().block_id).to_string() ),
                                                  Self::to_json(block.clone()) ) {
                    Ok(_) => {
                        match DB::write_block_index( block_index.clone().to_string() ) {
                            Ok(_) => {
                                match Self::write_block(block.clone()) {
                                    Ok(_) => {
                                        Ok(())
                                    },
                                    Err(_) => {
                                        Err(())
                                    }
                                }
                            },
                            Err(_) => {
                                Err(())
                            }
                        }
                    },
                    Err(_) => {
                        //couldnt insert new block into index
                        Err(())
                    }
                }
            },
            None => Err(())
        }
    }
}

//TODO: change access to DB directly for modules calling ReadBlockFromDB - later
pub trait ReadBlock {
    fn get_latest_block_id() -> Option<i64>;
    fn get_block_by_block_id(block_id: i64) -> Option<Block>;
}

impl ReadBlock for Block {
    fn get_latest_block_id() -> Option<i64> {
        DB::get_latest_block_id()
    }
    fn get_block_by_block_id(block_id: i64) -> Option<Block> {
        DB::get_block_by_block_id(block_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, BlockFromString, JsonToBlock};
    use timestamp::{Timestamp, StringToTimestamp};

    #[test]
    fn test_from_string() {
        let expected_block: Block = Block {
          block_id: 0,
          block_hash: String::from("test block hash"),
          block_parent_hash: String::from("test parent hash"),
          block_time: Timestamp::string_to_timestamp(String::from("0")).unwrap(),
          proposal_hash: String::from("test proposal hash"),
          block_data: String::from("test block data"),
          transactions: Vec::new()
        };

        let stringed_block: &str = "{
            \"block_id\": 0,
            \"block_hash\": \"test block hash\",
            \"block_parent_hash\": \"test parent hash\",
            \"block_time\": \"0\",
            \"proposal_hash\": \"test proposal hash\",
            \"block_data\": \"test block data\"
        }";
        let actual_block: Result<Block, String> = Block::from_string( String::from(stringed_block) );
        assert_eq!(actual_block.unwrap(), expected_block);
    }

    #[test]
    fn test_from_json(){
        let data = object!{
            "block_id" => 0,
            "block_hash" => "hash",
            "block_parent_hash" => "hash",
            "block_time" => "0",
            "proposal_hash" => "hash",
            "block_data" => "data",
        };
        let expected_block: Block = Block {
            block_id: 0,
            block_hash: String::from("hash"),
            block_parent_hash: String::from("hash"),
            block_time: Timestamp::string_to_timestamp(String::from("0")).unwrap(),
            proposal_hash: String::from("hash"),
            block_data: String::from("data"),
            transactions: Vec::new()
        };
        let actual_block: Result<Block, String> = Block::from_json(data);
        assert_eq!(expected_block, actual_block.unwrap());
    }
}
