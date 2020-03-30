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
extern crate lazy_static;

#[macro_use]
extern crate json;


use std::fs;
use std::io::{Write, Error, ErrorKind};
use lock::{Locker, FileLockWrite};
use json::{JsonValue};


pub struct DB {

}

pub trait NodeNameSetter {
    fn set_node_name(name: String) -> ();
}

impl NodeNameSetter for DB {
    fn set_node_name(name: String) -> () {
        lazy_static! {
            // TODO: variable node name
            //static ref NODENAME: String = name;
        }
    }
}

/*
    TODO: target node's names DB directory ONLY
    TODO: PROPOSALS_LOC needs to be relatively safe.
    Test Proposal by itsef throws an erorr since the entry point isnt the root
*/

/*
@desc PROPOSALS_LOC stores all proposals the network submits
*/
const PROPOSALS_LOC: &str = "storage/proposal/";//&(format!("{}", "storage/proposal/").as_str());
const PROPOSALS_DB_LOC: &str = "storage/proposals.db";

/*
@desc PROPOSALS_LOC stores all peer statuses on proposals
*/
const PROPOSALS_PEER_STATUS_LOC: &str = "storage/proposal/peer_status/";
const PROPOSALS_PEER_STATUS_DB_LOC: &str = "storage/peer_status.db";

/*
@desc TRANSACTIONS_LOC stores all transactions the network submits
*/
const TRANSACTIONS_LOC: &str = "storage/transaction/";
const TRANSACTIONS_DB_LOC: &str = "storage/transactions.db";

/*
@desc STATES_LOC stores all states through which the network progresses
*/
const STATES_LOC: &str = "storage/state/";
const STATES_DB_LOC: &str = "storage/states.db";

/*
@desc STATES_LOC stores all states through which the network progresses
*/
const BLOCKS_LOC: &str = "storage/chain/";
const BLOCKS_DB_LOC: &str = "storage/chain.db";

pub trait DBInit{
    fn create_sql_databases() -> Result<(), std::io::Error>;
}

impl DBInit for DB {
    fn create_sql_databases() -> Result<(), std::io::Error>{
        //TODO: create new proposals database
        //if not exists

        Ok(())
    }
}

pub trait DBWrite {
    fn write(content: String, location: String) -> Result<String,std::io::Error>;
}

impl DBWrite for DB {
    /*
    TODO: should take a type to store
    */
    fn write(content: String, location: String) -> Result<String,std::io::Error> {
        println!("DB write, Writing to DB");
        let file_location: String = format!("{}", location);
        //TODO:: invoke Lock::write(content, location)
        //let file_lock_write_result: Result<(), std::io::Error> = Locker::write(test_content,file_location);
        let file_lock_write_result: Result<(), std::io::Error> = Locker::write(content.clone(), location);
        //OLD
        //let mut file = fs::File::create(file_location.to_string())?;
        //file.write( content.as_bytes() )?;
        match file_lock_write_result {
            Ok(_) => {
                Ok(content)
            },
            Err(e) => {
                let db_lock_write_error = Error::new(ErrorKind::Other, "DBWrite ERROR, write(), could write with lock!");
                Err(db_lock_write_error)
            }
        }
    }
}

pub trait DBRead {
    fn read(file: String) -> Option<String>;
}

impl DBRead for DB {
    fn read(file: String) -> Option<String> {
        println!("DB Read File: {}", file);
        //TODO: read with lock as well
        let contents: Result<String, std::io::Error> = fs::read_to_string(file); //.expect("[DB Error reading file]");
        match contents {
            Ok(r) => {
                //println!("DBRead Text:\n{}", r);
                Some(r)
            },
            Err(e) => {
                None
            }
        }
    }
}

/*
@name DBReadProposalPeerStatus
@desc
*/

pub trait DBReadProposalPeerStatus{
    /*
    @name read_proposal_peer_status
    @desc read and return JSON proposal_status
    {"proposal_id": {"ip": "status", }}
    */
    fn read_proposal_peer_status(proposal_id: i32) -> Option<String>;
}

impl DBReadProposalPeerStatus for DB {
    fn read_proposal_peer_status(pid: i32) -> Option<String>{
        let file_location: String = format!("{}{}",
                                    PROPOSALS_PEER_STATUS_LOC,
                                    format!("proposal_{}.dat", pid));
        //let file_location: String = format!("{}",PROPOSALS_PEER_STATUS_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }
}


pub trait DBWriteProposalPeerStatus {
    /*
    @name write_proposal_peer_status
    @desc read and return JSON proposal_status
    */
    fn write_proposal_peer_status(pid: i32, proposal_string: String) -> Result<String, Error>;
}

impl DBWriteProposalPeerStatus for DB {
    fn write_proposal_peer_status(pid: i32, proposal_string: String) -> Result<String, Error>{
        let file_location: String = format!("{}{}",
                                    PROPOSALS_PEER_STATUS_LOC,
                                    format!("proposal_{}.prop", pid));
        //let file_location: String = format!("{}",PROPOSALS_PEER_STATUS_DB_LOC);
        Self::write(proposal_string, file_location);
        Ok(String::from("write_proposal_peer_status, Ok, Successfully wrote DB JSON index"))
    }
}

/*
@name DBReadProposal
@desc
*/
pub trait DBReadProposal {
    fn read_proposal_file_by_id(pid: i32) -> Option<String>;
    fn read_proposal_index() -> Option<String>;
    fn write_proposal_index(db_json_string: String) -> Result<String, Error>;
}

impl DBReadProposal for DB {

    /*
    @name read_proposal_file_by_id
    @desc read and return JSON DB PROPOSAL FILE
    */
    fn read_proposal_file_by_id(pid: i32) -> Option<String>{
        let file_location: String = format!("{}{}",
                                    PROPOSALS_LOC,
                                    format!("proposal_{}.prop", pid));
        //let file_location: String = format!("{}",PROPOSALS_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

    /*
    @name read_proposal_index
    @desc read and return JSON DB map
    */
    fn read_proposal_index() -> Option<String>{
        let file_location: String = format!("{}",PROPOSALS_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

    /*
    @name write_proposal_index
    @desc write JSON db map to disk
    */
    fn write_proposal_index(db_json_string: String) -> Result<String, Error> {
        println!("DB, write_proposal_index: Attempting to Write DB JSON INDEX");
        let file_location: String = format!("{}",PROPOSALS_DB_LOC);

        //TODO: purge proposal index
        let maximum_length: i32 = 10;

        // parse the db_json_string
        let parsed = json::parse( &format!(r#"{}"#, db_json_string) );
        match parsed {

            Ok(mut proposal_index) => {
                if proposal_index.clone().has_key("proposals") {
                    //clone proposal index after parse
                    //let parsed_result: JsonValue = proposal_index.clone();
                    //count number of proposals
                    let number_of_proposals: i32 = proposal_index.clone()["proposals"].len() as i32;
                    println!("DB, write_proposal_index, number_of_proposals: {}", number_of_proposals);

                    //check length of proposal index
                    if number_of_proposals > maximum_length {
                        println!("DB, write_proposal_index, number_of_proposals: number_of_proposals IS GREATER THAN maximum_length");
                        // remove length - window. Minus for genesis
                        let proposal_to_delete: i32 = (number_of_proposals - maximum_length) - 1;
                        //remove the proposal_id key
                        &proposal_index["proposals"].remove( &format!("{}",proposal_to_delete) );
                        &proposal_index["proposals"].remove( &format!("{}",proposal_to_delete+1) );
                        &proposal_index["proposals"].remove( &format!("{}",proposal_to_delete+2) );
                        &proposal_index["proposals"].remove( &format!("{}",proposal_to_delete+3) );
                        //proposal_index["proposals"] = parsed_result.clone();
                        //Self::write(proposal_index.dump(), file_location);
                        Self::write(proposal_index.dump(), file_location);
                        return Ok(String::from("Ok, Successfully wrote DB JSON proposal index, over max window"))
                    } else {
                        println!("DB, write_proposal_index, number_of_proposals: number_of_proposals IS NOT GREATER THAN maximum_length");
                        Self::write(proposal_index.dump(), file_location);
                        return Ok(String::from("Ok, Successfully wrote DB JSON proposal index, NOT over max window"))
                    }

                } else {
                    println!("DB, write_proposal_index, DBWrite ERROR, write_proposa_index, proposals key did not exists in proposal index!!!");
                    let db_proposals_key_is_missing_error = Error::new(ErrorKind::Other, "DBWrite ERROR, write_proposal_index, proposals key did not exists in proposal index!!!");
                    Err(db_proposals_key_is_missing_error)
                }
            },
            Err(_) => {
                println!("DB, write_proposal_index, DBWrite ERROR, write_proposal_index, cant parse proposal index!!!");
                let db_proposals_cant_parse = Error::new(ErrorKind::Other, "DBWrite ERROR, write_proposal_index, cant parse proposal index!!!");
                Err(db_proposals_cant_parse)
            }

        }

    }

}

/*
@name DBWriteProposal
@desc
*/
pub trait DBWriteProposal {
    fn write_proposal_to_sql(pid: i32, proposal_string: String) -> Result<String,std::io::Error>;
}

impl DBWriteProposal for DB {
    /*
    pass string to write into db, but alter json in proposal
    */
    fn write_proposal_to_sql(pid: i32, proposal_string: String) -> Result<String,std::io::Error>{
        println!("write_proposal_to_sql, Writing to DB");
        let file_location: String = format!("{}{}",
                                    PROPOSALS_LOC,
                                    format!("proposal_{}.prop", pid));
        let mut file = fs::File::create(file_location.to_string())?;
        //TODO: will fail if directory doesn't exist, but will fail gracefully
        file.write( proposal_string.as_bytes() )?;
        println!("Wrote Proposal");
        Ok(proposal_string)
    }
}

/*
@name DBStateManager
@desc
*/
pub trait DBStateManager {
    fn read_state() -> Option<String>;
    fn write_state(db_json_string: String) -> Result<String, Error>;
}

impl DBStateManager for DB {

    /*
    @name read_state
    @desc read and return JSON DB map
    */
    fn read_state() -> Option<String>{
        let file_location: String = format!("{}",STATES_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

    /*
    @name write_state
    @desc write JSON db map to disk
    */
    fn write_state(db_json_string: String) -> Result<String, Error> {
        println!("DB, write_state: Attempting to Write DB JSON INDEX FOR STATE");
        let file_location1: String = format!("{}",STATES_DB_LOC);
        let mut file = fs::File::create(file_location1.to_string())?;
        file.write( db_json_string.as_bytes() )?;
        let file_location2: String = format!("{}{}",
                                    STATES_LOC,
                                    format!("state_{}.prop", 0));
        Self::write(db_json_string, file_location2);
        Ok(String::from("Ok, Successfully wrote DB JSON index FOR STATE"))
    }

}

/*
@name DBReadBlock
@desc
*/
pub trait DBReadTransaction {
    fn read_transaction_index() -> Option<String>;
    fn write_transaction_index(db_json_string: String) -> Result<String, Error>;
    fn read_transaction(transaction_id: i64) -> Option<String>;
}

impl DBReadTransaction for DB {
    /*
    @name read_transaction_index
    @desc read and return JSON DB map
    */
    fn read_transaction_index() -> Option<String>{
        let file_location: String = format!("{}",TRANSACTIONS_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

    /*
    @name write_transaction_index
    @desc write JSON db map to disk
    */
    fn write_transaction_index(db_json_string: String) -> Result<String, Error> {
        println!("DB, write_transaction_index: Attempting to Write DB JSON INDEX for tx");
        let file_location: String = format!("{}",TRANSACTIONS_DB_LOC);
        //let mut file = fs::File::create(file_location.to_string())?;
        //file.write( proposal_string.as_bytes() )?;
        Self::write(db_json_string, file_location);
        Ok(String::from("Ok, Successfully wrote DB JSON index FOR TRANSACTION"))
    }

    fn read_transaction(transaction_id: i64) -> Option<String> {
        let file_location: String = format!("{}",TRANSACTIONS_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

}

/*
@name DBWriteTransaction
@desc
*/
pub trait DBWriteTransaction {
    fn write_transaction_to_sql(tid: i32, transaction_string: String) -> Result<String,std::io::Error>;
}

impl DBWriteTransaction for DB {
    /*
    pass string to write into db
    */
    fn write_transaction_to_sql(tid: i32, transaction_string: String) -> Result<String,std::io::Error>{
        //write to transactions file
        println!("Writing TRANSACTION to DB");
        let file_location: String = format!("{}{}",
                                    TRANSACTIONS_LOC,
                                    format!("transaction_{}.dat", tid));
        let mut file = fs::File::create(file_location.to_string())?;
        file.write( transaction_string.as_bytes() )?;
        println!("Wrote TRANSACTION");
        Ok(transaction_string)
    }
}

/*
@name DBReadBlock
@desc
*/
pub trait DBReadBlock {
    fn read_block_index() -> Option<String>;
    fn write_block_index(db_json_string: String) -> Result<String, Error>;
    fn read_block(block_id: i64) -> Option<String>;
}

impl DBReadBlock for DB {

    /*
    @name read_block_index
    @desc read and return JSON DB map
    */
    fn read_block_index() -> Option<String>{
        let file_location: String = format!("{}",BLOCKS_DB_LOC);
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

    /*
    @name write_block_index
    @desc write JSON db map to disk
    */
    fn write_block_index(db_json_string: String) -> Result<String, Error> {
        println!("DB, write_block_index: Attempting to Write DB JSON INDEX FOR BLOCK");
        let file_location: String = format!("{}",BLOCKS_DB_LOC);

        //TODO: purge block index
        let maximum_length: i32 = 10;

        // parse the db_json_string
        let parsed = json::parse( &format!(r#"{}"#, db_json_string) );
        match parsed {

            Ok(mut block_index) => {
                if block_index.clone().has_key("blocks") {
                    println!("DB, write_block_index, number_of_blocks: block index has blocks key");
                    //let parsed_result: JsonValue = block_index.clone();
                    let number_of_blocks: i32 = block_index.clone()["blocks"].len() as i32;
                    if number_of_blocks > maximum_length {
                        println!("DB, write_block_index, number_of_proposals: number_of_proposals IS GREATER THAN maximum_length");
                        // remove length - window
                        let block_to_delete: i32 = (number_of_blocks - maximum_length) - 1;
                        &block_index["blocks"].remove(&format!("{}",block_to_delete));
                        &block_index["blocks"].remove(&format!("{}",block_to_delete+1));
                        &block_index["blocks"].remove(&format!("{}",block_to_delete+2));
                        &block_index["blocks"].remove(&format!("{}",block_to_delete+3));
                        //block_index["blocks"] = parsed_result.clone();
                        Self::write(block_index.dump(), file_location);
                        return Ok(String::from("Ok, Successfully wrote DB JSON block index, over max window"))
                    } else {
                        println!("DB, write_block_index, number_of_blocks: number_of_blocks IS NOT GREATER THAN maximum_length");
                        Self::write(block_index.dump(), file_location);
                        return Ok(String::from("Ok, Successfully wrote DB JSON block index, NOT over max window"))
                    }
                } else {
                    println!("DB, write_block_index, number_of_blocks: number_of_blocks IS GREATER THAN maximum_length");
                    let db_proposals_key_is_missing_error = Error::new(ErrorKind::Other, "DBWrite ERROR, write_block_index, blocks key did not exists in block index!!!");
                    Err(db_proposals_key_is_missing_error)
                }
            },
            Err(_) => {
                let db_blocks_cant_parse = Error::new(ErrorKind::Other, "DBWrite ERROR, write_blocks_index, cant parse block index!!!");
                Err(db_blocks_cant_parse)
            }

        }


    }

    fn read_block(block_id: i64) -> Option<String> {
        let file_location: String = format!("{}{}",
                                            BLOCKS_LOC,
                                            format!("block_{}.dat", block_id));
        match Self::read(file_location) {
            Some(p) => Some(p),
            None => None
        }
    }

}

/*
@name DBWriteBlock
@desc
*/
pub trait DBWriteBlock {
    fn write_block_to_sql(bid: i64, block_string: String) -> Result<String,std::io::Error>;
}

impl DBWriteBlock for DB {
    /*
    pass string to write into db, but alter json in block
    */
    fn write_block_to_sql(bid: i64, block_string: String) -> Result<String,std::io::Error>{
        println!("Writing BLOCK to DB");
        let file_location: String = format!("{}{}",
                                    BLOCKS_LOC,
                                    format!("block_{}.dat", bid));
        let mut file = fs::File::create(file_location.to_string())?;
        file.write( block_string.as_bytes() )?;
        println!("Wrote block");
        Ok(block_string)
    }
}

/*
@name FileDirectoryReader
@desc this trait handles all disk-bound file directory reading
*/

pub trait FileDirectoryReader {
    fn read_proposals_directory() -> Vec<String>;
    fn read_transactions_directory() -> Vec<String>;
    fn read_states_directory() -> Vec<String>;
    fn read_blocks_directory() -> Vec<String>;
}

impl FileDirectoryReader for DB {
    fn read_proposals_directory() -> Vec<String>{
        println!("Reading Proposals Directory from DB");
        let mut file_vector: Vec<String> = Vec::new();
        let paths = fs::read_dir(PROPOSALS_LOC).unwrap();
        for path in paths {
            file_vector.push(path.unwrap().path().display().to_string());
        }
        file_vector
    }

    fn read_transactions_directory() -> Vec<String> {
        println!("Reading Transactions Directory from DB");
        let mut file_vector: Vec<String> = Vec::new();
        let paths = fs::read_dir(TRANSACTIONS_LOC).unwrap();
        for path in paths {
            file_vector.push(path.unwrap().path().display().to_string());
        }
        file_vector
    }

    fn read_states_directory() -> Vec<String> {
        println!("Reading States Directory from DB");
        let mut file_vector: Vec<String> = Vec::new();
        let paths = fs::read_dir(STATES_LOC).unwrap();
        for path in paths {
            file_vector.push(path.unwrap().path().display().to_string());
        }
        file_vector
    }

    fn read_blocks_directory() -> Vec<String> {
        println!("Reading Blocks Directory from DB");
        let mut file_vector: Vec<String> = Vec::new();
        let paths = fs::read_dir(BLOCKS_LOC).unwrap();
        for path in paths {
            file_vector.push(path.unwrap().path().display().to_string());
        }
        file_vector
    }
}

#[cfg(test)]
mod tests {

}
