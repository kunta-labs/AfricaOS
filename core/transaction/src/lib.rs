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
use db::{DB,
         DBReadTransaction,
         FileDirectoryReader,
         DBWriteTransaction,
         DBStateManager};
use std::io::{Error, ErrorKind};
use timestamp::{Timestamp, NewTimestamp, StringToTimestamp};
use hash::{Hasher, CalculateSHA256Hash};
use encode::{Encoder, Base64Encode, Base64Decode};
use executor::{Executor, ExecuteMacro};

/*
@name Transaction
@desc structure for Transaction
*/
#[derive(Clone, Debug, PartialEq)]
pub struct Transaction {
    pub transaction_id: i32,
    pub transaction_timestamp: Timestamp,
    pub transaction_type: TransactionType,
    pub transaction_sender: String,
    //pub transaction_recipient: String,
    pub transaction_data: String,
    pub transaction_hash: String
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransactionType {
    Output,
    Input,
    TxTypeError
}

trait TransactionTypeToString {
    fn type_to_string(transaction_type: TransactionType) -> &'static str {
        match transaction_type {
            TransactionType::Output => "output",
            TransactionType::Input => "input",
            TransactionType::TxTypeError => "tx_type_error"
        }
    }
}

trait StringToTransactionType {
    fn string_to_type(transaction_type: &str) -> TransactionType {
        match transaction_type {
            "output" => TransactionType::Output,
            "input" => TransactionType::Input,
            _ => TransactionType::TxTypeError
        }
    }
}

impl TransactionTypeToString for Transaction {}
impl StringToTransactionType for Transaction {}


/*
    @name HashTransaction
    @desc hash the contents of a transaction
*/
trait HashTransaction {
    fn hash_transaction(transaction_id: i32, ts: Timestamp, data: String) -> String;
}

impl HashTransaction for Transaction {
    fn hash_transaction(transaction_id: i32, ts: Timestamp, data: String) -> String {
        let raw_str: String = format!("{}{}{}", transaction_id, ts.timestamp, data);
        let string_to_hash: String = String::from( raw_str.as_str() ) ;
        let transaction_hash: String = Hasher::calculate_sha256( string_to_hash );
        transaction_hash
    }
}

/*
@name JsonConverter
@desc
*/
pub trait JsonConverter {
    /*
    @name to_json
    @desc convert transaction into JSON String
    */
    fn to_json(transaction: Transaction) -> String;

    /*
    @name from_json
    @desc create a transaction from a JSONValue
    */
    fn from_json(payload: JsonValue) -> Result<Transaction, String>;

    /*
    @name from_json_string
    @desc create a transaction from a JSON string
    */
    fn from_json_string(json_string: String) -> Result<Transaction, String>;

    /*
    @name tx_vec_from_json
    */
    fn tx_vec_from_json(payload: JsonValue) -> Result<Vec<Transaction>, String>;

    /*
    @name tx_vec_from_json
    */
    fn json_from_tx_vec(transactions: Vec<Transaction>) -> Result<JsonValue, String>;
}

/*
@name JsonConverter for Transaction
@desc
*/
impl JsonConverter for Transaction {
    fn to_json(transaction: Transaction) -> String {
        let data = object!{
            "transaction_id" => transaction.transaction_id,
            "transaction_timestamp" => transaction.transaction_timestamp.timestamp,
            "transaction_type" => Self::type_to_string(transaction.transaction_type),
            "transaction_sender" => transaction.transaction_sender,
            //"transaction_recipient" => transaction.transaction_recipient,
            "transaction_data" => transaction.transaction_data,
            "transaction_hash" => transaction.transaction_hash
        };
        println!("Transaction, to_json, data.dump(): {}", data.dump());
        data.to_string()
    }

    fn from_json(payload: JsonValue) -> Result<Transaction, String> {
        println!("TX From JSON: {} id: {}", payload, payload["transaction_id"]);
        let transaction_id_from_json: Option<i32> = payload["transaction_id"].as_i32();
        if transaction_id_from_json.is_some() {
            let transaction_timestamp_from_json: Option<Timestamp> = Timestamp::string_to_timestamp( payload["transaction_timestamp"].to_string() );
            match transaction_timestamp_from_json {
                Some(ts) => {
                    let parsed_tx: Transaction = Transaction {
                        transaction_id: transaction_id_from_json.unwrap(),
                        transaction_timestamp: ts,
                        transaction_type: Transaction::string_to_type( payload["transaction_type"].as_str().unwrap() ),
                        transaction_sender: payload["transaction_sender"].to_string(),
                        //transaction_recipient: payload["transaction_recipient"].to_string(),
                        transaction_data: payload["transaction_data"].to_string(),
                        transaction_hash: payload["transaction_hash"].to_string()
                    };
                    Ok(parsed_tx)
                },
                None => {
                    Err(String::from("Timestamp from string failed in TX"))
                }
            }
        } else {
            Err(String::from("transaction_id_from_json IS NONE"))
        }



    }

    fn from_json_string(json_string: String) -> Result<Transaction, String> {
        let json_parsed_tx = json::parse( &format!(r#"{}"#, json_string) ).unwrap();
        Self::from_json(json_parsed_tx)
    }

    fn tx_vec_from_json(payload: JsonValue) -> Result<Vec<Transaction>, String> {
        let mut transactions_vector: Vec<Transaction> = Vec::new();
        let transactions_iter = payload.entries();
        for (id, transaction) in transactions_iter {
            let json_parsed_tx = json::parse( &format!(r#"{}"#, transaction) ).unwrap();
            match Self::from_json( json_parsed_tx.clone() ) {
                Ok(tx) => {
                    transactions_vector.push(tx);
                },
                Err(msg) => {
                    println!("tx_vec_from_json, transaction from_json FAILED: {}{}", transaction.clone(), msg);
                    return Err(String::from("tx_vec_from_json ERROR from json failed"));
                }
            }
        }
        Ok(transactions_vector)
    }

    fn json_from_tx_vec(transactions: Vec<Transaction>) -> Result<JsonValue, String> {
        let mut new_transaction_index = object!{};
        for tx in transactions {
            match new_transaction_index.insert( &( format!("{}", tx.transaction_id).to_string() ),
                                                Self::to_json(tx) ) {
                 Ok(_) => {

                 },
                 Err(_) => {

                 }
            }
        }
        Ok(new_transaction_index)
    }
}

pub trait ReadTransactionFromDB {
    fn get_all_transactions() -> Vec<Transaction>;
    fn get_latest_transaction_id() -> Option<i32>;
}

impl ReadTransactionFromDB for DB {
    fn get_all_transactions() -> Vec<Transaction> {
        let parsed: JsonValue = Self::get_transaction_index_as_json();
        let mut all_transactions_vector: Vec<Transaction> = Vec::new();
        let transactions_iter = parsed["transactions"].entries();
        for (id, transaction) in transactions_iter {
            println!("get_all_transactions(), transaction: {}:{}", id, transaction);
            let json_parsed_tx = json::parse( &format!(r#"{}"#, transaction) ).unwrap();
            let parsed_transaction: Result<Transaction, String> = Transaction::from_json(json_parsed_tx.clone());
            match parsed_transaction {
                Ok(transaction) => {
                    all_transactions_vector.push(transaction);
                },
                Err(err) => {
                    println!("get_all_transactions ERROR parsed_transaction: {:?}", err);
                }
            }
        }
        all_transactions_vector
    }

    /*
    @name get_latest_transaction_id
    @desc get the latest proposal
    */
    fn get_latest_transaction_id() -> Option<i32> {
        let transaction_index_parsed: JsonValue = Self::get_transaction_index_as_json();
        let all_transactions = &transaction_index_parsed["transactions"];
        if all_transactions.is_empty() {
            Some(-1)
        } else {
            let mut highest_transaction_id: i32 = -1;
            let transactions_iter = all_transactions.entries();
            for (id, transaction_iter) in transactions_iter {
                println!("get_latest_transaction_id(), tx: iter {}:{}", id, transaction_iter);
                let transaction_from_json: Result<Transaction, String> = Transaction::from_json( (*transaction_iter).clone() );
                match transaction_from_json {
                    Ok(tx) => {
                        if tx.transaction_id > highest_transaction_id {
                            highest_transaction_id = tx.transaction_id;
                        } else {
                            println!("get_latest_transaction_id(), transaction id not higher than highest_transaction_id: {}", tx.transaction_id);
                        }
                    },
                    Err(_) => {
                        println!("Couldn't convert JSON tx to tx type");
                    }
                }
            }
            Some(highest_transaction_id)
        }
    }
}

trait TransactionIndexReader {
    fn get_transaction_index_as_json() -> JsonValue;
}

impl TransactionIndexReader for DB {
    fn get_transaction_index_as_json() -> JsonValue {
        let transaction_index: String = match Self::read_transaction_index() {
            Some(i) => {
                //TODO: parse/verify proposal index
                i
            },
            None => String::from("NO TRANSACTION INDEX")
        };
        println!("Transaction index: {}", transaction_index);
        //TODO: convert DB json string to json
        let parsed = json::parse( &format!(r#"{}"#, transaction_index) ).unwrap();
        println!("get_transaction_index_as_json(), transaction index parsed: {}", parsed["transactions"]);
        parsed
    }
}

/*
@name CreateTransactionIndex
@desc
*/
pub trait CreateTransactionIndex {
    fn create_transaction_index() -> ();
}

/*
@name CreateTransactionIndex
@desc to initially create the transaction index
*/
impl CreateTransactionIndex for Transaction {
    fn create_transaction_index() -> (){
        let new_transaction_index = object!{
            "transactions" => object!{}
        };
        let index_to_write: String = json::stringify(new_transaction_index);
        match DB::write_transaction_index(index_to_write) {
            Ok(_) => {
                println!("Successfully wrote transaction index");
            },
            Err(_) => {
                println!("Failure writing transaction index");
            }
        }
    }
}

/*
    @name ClearTransactionIndex
    @desc make the transaction index empty again after block commitment
*/
pub trait ClearTransactionIndex {
    fn clear_transaction_index() -> ();
}

impl ClearTransactionIndex for Transaction {
    fn clear_transaction_index() -> () {
        let empty_transaction_string: JsonValue = object!{
            "transactions" => object!{}
        };
        let db_index_write_result = DB::write_transaction_index(empty_transaction_string.dump());
    }
}

/*
@name WriteTransactionToDB
@desc trait to write a transaction to the DB
*/
pub trait WriteTransactionToDB {
    fn write_transaction(transaction: Transaction) -> Result<String,std::io::Error>;
}

/*
@name WriteTransactionToDB
@desc implementation for Writing Transaction To DB for DB
*/
impl WriteTransactionToDB for DB {
    /*
    @name write_transaction
    @desc write transaction to DB
    */
    fn write_transaction(transaction: Transaction) -> Result<String,std::io::Error> {
        println!("inside write_transaction, DB trait");
        //TODO: convert from Proposal to JSON
        let transaction_string: String = Transaction::to_json(transaction.clone());
        //TODO: Read transaction index JSON
        //TODO: pass Node Peer name
        let mut parsed: JsonValue = Self::get_transaction_index_as_json();
        //TODO: alter proposal index json object
        let new_transaction_entry = object!{
            "transaction_id" => transaction.transaction_id,
            "transaction_timestamp" => transaction.transaction_timestamp.timestamp,
            "transaction_type" => Transaction::type_to_string( transaction.transaction_type ),
            "transaction_sender" => transaction.transaction_sender,
            //"transaction_recipient" => transaction.transaction_recipient,
            "transaction_data" => transaction.transaction_data,
            "transaction_hash" => transaction.transaction_hash
        };

        let tindex_insert_result: Result<String, Error> = match parsed["transactions"]
              .insert( &(format!("{}", transaction.transaction_id).to_string() ),
                       new_transaction_entry) {
            Ok(_) => {
                println!("New Transaction JSON: {}", parsed.dump());
                //TODO: commit proposal to DB
                let db_write_result: Result<String, std::io::Error> = Self::write_transaction_to_sql(transaction.transaction_id, transaction_string.clone());
                if db_write_result.is_ok() {
                    //TODO: commit proposal index to DB
                    let db_index_write_result = Self::write_transaction_index(parsed.dump());
                    db_index_write_result
                } else {
                    let transaction_db_write_error = Error::new(ErrorKind::Other, "Couldn't write Transaction to DB");
                    Err(transaction_db_write_error)
                }
            },
            Err(r) => {
                println!("Failed adding new Transaction to Transaction_index: {}", parsed.dump());
                let transaction_index_insert_error = Error::new(ErrorKind::Other, "Could not add transaction to transaction_index");
                Err(transaction_index_insert_error)
            }
        };
        tindex_insert_result
    }
}

/*
@name WriteNewTransactionToDB
@desc trait to write a new transaction to DB
*/
trait WriteNewTransactionToDB {
    fn write_new_transaction(transaction: Transaction) -> Result<String,std::io::Error>;
}

/*
@name WriteNewProposalToDB for Transaction
@desc implementation to write a new proposal to the DB
*/
impl WriteNewTransactionToDB for Transaction {
    fn write_new_transaction(transaction: Transaction) -> Result<String,std::io::Error> {
        DB::write_transaction(transaction) //write transaction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    tree: Vec<String>
}

trait StateToJson {
    fn to_json(state: State) -> JsonValue;
}

impl StateToJson for State {
    fn to_json(state: State) -> JsonValue {
        let mut state_object: JsonValue = object!{};
        for sub_state in state.tree.iter() {
            let state_sections: Vec<&str> = sub_state.split(":").collect::<Vec<_>>();
            let first_section: String = String::from(state_sections[0]);
            let second_section: String = String::from(state_sections[1]);
            let state_insert_result: Result<String, Error> = match state_object.insert( &( format!("{}", first_section).to_string() ), format!("{}", second_section).as_str() ) {
                  Ok(_) => {
                      Ok(String::from(""))
                  },
                  Err(_) => {
                      Ok(String::from(""))
                  },
            };
        }
        state_object
    }
}

trait JsonToState {
    fn to_state(state_string: JsonValue) -> State;
}

impl JsonToState for State {
    fn to_state(state_string: JsonValue) -> State {
        let mut state_vec: Vec<String> = Vec::new();
        let states_iter = state_string.entries();
        for (address, state) in states_iter {
            state_vec.push(format!("{}:{}", address.to_string(), state.to_string()));
        }
        //TODO: change from a single string to a pair
        State{
            tree: state_vec
        }
    }
}

/*
@name CreateStateDB
@desc
*/
pub trait CreateStateDB {
    fn create_state_db() -> ();
}

/*
@name CreateStateDB
@desc to initially create the state db
*/
impl CreateStateDB for State {
    fn create_state_db() -> () {
        let new_state_index = object!{};
        let index_to_write: String = json::stringify(new_state_index);
        // TODO: SPECIFY WHICH STATE INDEX TO WRITE
        match DB::write_state(index_to_write) {
            Ok(_) => {
                println!("Successfully wrote/created state DB");
            },
            Err(_) => {
                println!("Failure writing/creating state db");
            }
        }

    }

}

trait WriteState {
    fn write(state: State) -> Result<State, String>;
}

impl WriteState for State {
    fn write(state: State) -> Result<State, String> {
        let new_state_index: JsonValue = State::to_json(state.clone());
        let index_to_write: String = json::stringify(new_state_index);
        // TODO: SPECIFY WHICH STATE INDEX TO WRITE
        match DB::write_state(index_to_write) {
            Ok(_) => {
                println!("Successfully wrote/created state DB");
                Ok(state)
            },
            Err(_) => {
                println!("Failure writing/creating state db");
                Err(String::from("WriteState, write faile at write_state(index_to_write)"))
            }
        }
    }
}

trait ReadState {
    fn read() -> Option<State>;
}

impl ReadState for State{
    fn read() -> Option<State>{
        //TODO: read json string for state
        let current_state_string: Option<String> = DB::read_state();
        match current_state_string {
            Some(state) => {
                //TODO: parse as JSONValue
                let parsed = json::parse( &format!(r#"{}"#, state) ).unwrap();
                //TODO: STATE::from_json
                let state_from_json: State = State::to_state(parsed);
                Some(state_from_json)
            },
            None => {
                println!("READSTATE ERROR: read() is None");
                None
            }
        }
    }
}


////// New Transaction
pub trait CreateNewOuputTransaction {
    fn new_output(sender: String, data: String) -> Option<Transaction>;
}

impl CreateNewOuputTransaction for Transaction {
    //TODO: convert to return an Option instead of only Transaction
    fn new_output(sender: String, data: String) -> Option<Transaction> {
        let latest_transaction_id: Option<i32> = DB::get_latest_transaction_id();
        //TODO: condition on successful latest_transaction_id
        let new_transaction_id: i32 = latest_transaction_id.unwrap() + 1;
        let new_timestamp: Timestamp = Timestamp::new().unwrap();
        let b64_encoded_data: Result<String,String> = Encoder::encode_base64(data);
        match b64_encoded_data {
            Ok(data) => {
                let new_transaction_hash: String = Self::hash_transaction(new_transaction_id.clone(), new_timestamp.clone(), data.clone());
                let new_tx = Transaction {
                    transaction_id: new_transaction_id,
                    transaction_timestamp: new_timestamp,
                    transaction_type: TransactionType::Output,
                    transaction_sender: sender.clone(),
                    //transaction_recipient: sender,
                    transaction_data: data,
                    transaction_hash: new_transaction_hash
                };
                match DB::write_transaction( new_tx.clone() ){
                    Ok(msg) => {
                        println!("CreateNewTransaction SUCCESS: {}", msg);
                        Some(new_tx)
                    },
                    Err(msg) => {
                        println!("CreateNewTransaction FAILED: {}", msg);
                        None
                    }
                }
            },
            Err(_) => {
                None
            }
        }
    }
}

//New Transaction
pub trait CreateNewInputTransaction {
    fn new_input(sender: String, data: String) -> Transaction;
}

/*
@name ExecuteTransactions
@desc trait for Executable behavior on transactions
*/
pub trait ExecuteTransactions {
    fn execute_block_transactions(transactions: Vec<Transaction>) -> ();
}

/*
@name ExecuteTransactions for Transaction
@desc implements the executable behavior for a transaction
*/
impl ExecuteTransactions for Transaction {
    fn execute_block_transactions(mut transactions: Vec<Transaction>) -> () {
        //TODO: READ CURRENT STATE
        let current_state: Option<State> = State::read();
        match current_state {
            Some(state) => {
                println!( "execute_block_transactions(), current_state: {}", State::to_json( state.clone() ) );
                let mut json_state_buffer: JsonValue = ( State::to_json( state.clone() ) );
                // iterate over each transaction
                transactions.iter().for_each( | tx | {
                    println!( "execute_block_transactions(), BEFORE json_state_buffer OVERWRITE: {}", json_state_buffer.clone() );
                    json_state_buffer = tx.execute( &Some( State::to_state( json_state_buffer.clone() ) ) );
                    println!("execute_block_transactions(),  AFTER json_state_buffer OVERWRITE: {}", json_state_buffer.clone() );
                });
                let state_to_write: String = json::stringify( json_state_buffer.clone() );
                // TODO: SPECIFY WHICH STATE INDEX TO WRITE
                match DB::write_state(state_to_write) {
                    Ok(_) => {
                        println!("execute_block_transactions(), Successfully wrote/created state DB AFTER TX EXECUTION");
                    },
                    Err(_) => {
                        println!("execute_block_transactions(), Failure writing/creating state db AFTER TX EXECUTION");
                    }
                }
            },
            None => {
                println!("execute_block_transactions(), current_state is NONE");
            }
        }
    }
}

/*
@name Executable
@desc trait for Executable behavior on transactions
*/
trait Executable {
    fn execute(&self, current_state_buffer: &Option<State>) -> JsonValue;
}

impl Executable for Transaction {
    fn execute(&self, current_state_buffer: &Option<State>) -> JsonValue {
        format!("Executing Transaction {}", self.transaction_id);
        match &self.transaction_type {
            TransactionType::Output => {
                // TODO MACRO USE!!!! CUSTOM_TRANSACTION_OUTPUT_LOGIC!()
                println!("TX execute TX Output BEFORE: {} : ",  State::to_json( current_state_buffer.clone().unwrap() ) );

                // TODO: create new address in state
                let mut state_as_json: JsonValue = State::to_json(current_state_buffer.clone().unwrap());
                Executor::execute_transaction_output_logic(state_as_json,
                                                           self.transaction_timestamp.clone(),
                                                           self.transaction_sender.clone(),
                                                           self.transaction_hash.clone(),
                                                           self.transaction_data.clone())
            },
            TransactionType::Input => {
                // TODO MACRO USE!!!! CUSTOM_TRANSACTION_INPUT_LOGIC!()
                println!("TX execute TX Input");
                State::to_json(current_state_buffer.clone().unwrap())
            },
            TransactionType::TxTypeError => {
                println!("TX execute ERROR: TxTypeError");
                State::to_json(current_state_buffer.clone().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Transaction,
                TransactionType,
                JsonValue,
                Executable,
                ExecuteTransactions,
                State};
    use timestamp::{Timestamp, NewTimestamp};

    #[test]
    fn test_create_tx_output_execution() {
        let new_timestamp: Timestamp = Timestamp::new().unwrap();
        let state: State = State {
            tree: Vec::new()
        };
        let test_state: &Option<State> = &Some(state);
        let new_tx = Transaction {
            transaction_id: 1,
            transaction_timestamp: new_timestamp,
            transaction_type: TransactionType::Output,
            transaction_sender: String::from("test sender"),
            //transaction_recipient: String::from("test recipient"),
            transaction_data: String::from("test data"), //data
            transaction_hash: String::from("test hash")
        };
        let execution_result: JsonValue = new_tx.execute( test_state );
        let expected_json: JsonValue = object!{
            "test hash" => "test data"
        };
        assert_eq!(expected_json, execution_result);
    }
}
