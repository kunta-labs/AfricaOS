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
use timestamp::{Timestamp, NewTimestamp};


macro_rules! block_validation {
    () => {
    }
}

macro_rules! proposal_validation {
    () => {
    }
}

/*
@name proposal_creator_election
@description
*/
#[macro_export]
macro_rules! proposal_creator_election {
    // TODO: place PCE code in here
    // ((B + 1) mod |N|) + 1
    // fix: (B mode |N| + 1)
    ($peer_length: expr, $latest_block_id: expr) => {
        {
            //( ($latest_block_id + 1) % ( ($peer_length + 1) as i64) ) + 1
            ($latest_block_id % $peer_length as i64) + 1
        }
    }
}

/*
@name transaction_input_logic
@description
*/
macro_rules! transaction_input_logic {
    () => {

    }
}

/*
@name transaction_output_logic
@description
*/
#[macro_export]
macro_rules! transaction_output_logic {
    /*
        @pattern StateJson, self

        TODO:
        //// [SIG] [HASHPUBK] [CHECKSIG] [PUBKHASH]
        TODO: [SIG] [PUBKEY] [CHECKSIG] [PUBKHASH]
        F - [CHECKSIG] [PUBKHASH]
        F - CHECK SIGNATURE USING SIG AND HASHPUBK, HASH SUBMITTED PUBLIC KEY

        M - [SIG] [PUBKEY]
        M - CHECK SIGNATURE USING SIG AND HASHPUBK, public_key_raw

        if this succeeds,the transaction is valid

        BTC:
        sender:
        ScriptPubKey= OP_DUP OP_HASH160 <Public KeyHash> OP_EQUAL OP_CHECKSIG

        recipient:
        ScriptSig= <Signature> <Public Key>

        combined:
        <sig> <pubKey> OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
        M + F
    */
    ($state: expr, $tx_timestamp: expr, $tx_sender: expr, $tx_hash: expr, $tx_data: expr) => {
        {
            println!("TX execute TX Output BEFORE: {} : ", $state.clone() );
            let mut state_as_json: JsonValue = $state;
            // insert a new account into the state db
            match &state_as_json.insert( &( format!("{}", $tx_hash).to_string() ),
                                            format!("{}", $tx_data) ) {
                 Ok(_) => {
                     //TODO: after we insert the initial state for the sender
                     //current_state_buffer
                     println!("TX execute TX Output AFTER: {} : ",  state_as_json.clone()  );
                     state_as_json
                 },
                 Err(_) => {
                     // error on inserting, return current state
                     println!("TX execute ERROR: State::to_json is NOT okay: {} ", $state.clone() );
                     $state
                 }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    //use transaction::{State};
    //#[macro_uses]
    //extern crate json;

    use json::{JsonValue};
    use timestamp::{Timestamp, NewTimestamp};

    #[test]
    fn test_transaction_output_logic() -> (){


        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_sender: String = String::from("test_sender");
        let test_hash_string: String = String::from("HASHTEST");
        let test_tx_data_string: String = String::from("100,200,test_string");


        let test_state_json: JsonValue = object!{
            "test" => "test"
        };

        let test_json_2: JsonValue = object!{
            "test" => "test",
            "HASHTEST" => "100,200,test_string"
        };


        let result: JsonValue = transaction_output_logic!( test_state_json.clone(),
                                                           test_timestamp.unwrap(),
                                                           test_sender,
                                                           test_hash_string,
                                                           test_tx_data_string );
        assert_eq!(test_json_2, result);
    }

    #[test]
    fn test_transaction_input_logic() -> (){


        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_sender: String = String::from("test_sender");
        let test_hash_string: String = String::from("HASHTEST");
        let test_tx_data_string: String = String::from("100,200,test_string");


        let test_state_json: JsonValue = object!{
            "test" => "test"
        };

        let test_json_2: JsonValue = object!{
            "test" => test_sender.as_str(),
            "HASHTEST" => "100,200,test_string"
        };

    }
}
