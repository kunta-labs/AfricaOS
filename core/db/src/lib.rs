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

use std::fs;
use std::io::{Write, Error};

pub struct DB {

}

pub trait NodeNameSetter {
    fn set_node_name(name: String) -> ();
}

impl NodeNameSetter for DB {
    fn set_node_name(name: String) -> () {
        lazy_static! {
            //static ref NODENAME: String = name;
        }
    }
}

/*
@desc PROPOSALS_LOC stores all proposals the network submits
*/
const PROPOSALS_LOC: &str = "storage/proposal/";//&(format!("{}", "storage/proposal/").as_str());
const PROPOSALS_DB_LOC: &str = "storage/proposals.db";

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
        Ok(())
    }
}

pub trait DBWrite {
    fn write(content: String, location: String) -> Result<String,std::io::Error>;
}

impl DBWrite for DB {
    fn write(content: String, location: String) -> Result<String,std::io::Error> {
        println!("DB write, Writing to DB");
        let file_location: String = format!("{}", location);
        let mut file = fs::File::create(file_location.to_string())?;
        file.write( content.as_bytes() )?;
        Ok(content)
    }
}

pub trait DBRead {
    fn read(file: String) -> Option<String>;
}

impl DBRead for DB {
    fn read(file: String) -> Option<String> {
        println!("DB Read File: {}", file);
        let contents: Result<String, std::io::Error> = fs::read_to_string(file);
        match contents {
            Ok(r) => {
                println!("DBRead Text:\n{}", r);
                Some(r)
            },
            Err(e) => {
                None
            }
        }
    }
}

/*
@name DBReadProposal
@desc
*/
pub trait DBReadProposal {
    fn read_proposal_index() -> Option<String>;
    fn write_proposal_index(db_json_string: String) -> Result<String, Error>;
}

impl DBReadProposal for DB {

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
        Self::write(db_json_string, file_location);
        Ok(String::from("Ok, Successfully wrote DB JSON index"))
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
        Self::write(db_json_string, file_location);
        Ok(String::from("Ok, Successfully wrote DB JSON index FOR BLOCK"))
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
