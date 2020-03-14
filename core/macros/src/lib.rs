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

macro_rules! transaction_input_logic {
    () => {

    }
}

macro_rules! block_validation {
    () => {
    }
}

macro_rules! proposal_validation {
    () => {
    }
}

#[macro_export]
macro_rules! proposal_creator_election {
    // TODO: place PCE code in here
    ($peer_length: expr, $latest_block_id: expr) => {
        {
            ( ($latest_block_id + 1) % ( ($peer_length + 1) as i64) ) + 1
        }
    }
}

// @input
#[macro_export]
macro_rules! transaction_output_logic {
    /*
        @pattern StateJson, self
    */
    ($state: expr, $tx_hash: expr, $tx_data: expr) => {
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

    #[test]
    fn test_transaction_output_logic() -> (){
        let test_json: JsonValue = object!{
            "test" => "test"
        };

        let test_json_2: JsonValue = object!{
            "test" => "test",
            "test2" => "test2"
        };
        let test_string: String = String::from("test2");
        let result: JsonValue = transaction_output_logic!( test_json.clone(),
                                                           test_string,
                                                           test_string );
        assert_eq!(test_json_2, result);
    }
}
