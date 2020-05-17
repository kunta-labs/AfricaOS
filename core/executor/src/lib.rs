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
#[macro_use]
extern crate macros;

use json::{JsonValue};
use timestamp::{Timestamp};
use macros::{transaction_output_logic,
             proposal_creator_election,
             transaction_input_logic};

use signature::{DigitalSignature,
                Verifier,
                SignatureType,
                SignatureFormat,
                SignatureError,
                Signature};

use encode::{Encoder, RawBytesEncode, RawBytesDecode, Base64Decode, Base64Encode};


pub struct Executor {}
pub trait ExecuteMacro {
    /*
    @name execute_transaction_output_logic
    @desc macro for tx output
    */
    fn execute_transaction_output_logic(state: JsonValue,
                                        transaction_timestamp: Timestamp,
                                        transaction_sender: String,
                                        transaction_hash: String,
                                        transaction_data: String) -> JsonValue;


    fn execute_transaction_input_logic(state: JsonValue,
                                       transaction_timestamp: Timestamp,
                                       transaction_sender: String,
                                       transaction_hash: String,
                                       transaction_data: String) -> JsonValue;

    /*
    @name execute_proposal_creator_election
    @desc macro for proposal creator election
    */
    fn execute_proposal_creator_election(peer_length: usize,
                                         latest_block_id: i64) -> i64;
}

impl ExecuteMacro for Executor {

    fn execute_transaction_output_logic(state: JsonValue,
                                        transaction_timestamp: Timestamp,
                                        transaction_sender: String,
                                        transaction_hash: String,
                                        transaction_data: String) -> JsonValue {
        transaction_output_logic!(state.clone(),
                                  transaction_timestamp,
                                  transaction_sender,
                                  transaction_hash,
                                  transaction_data)
    }

    fn execute_transaction_input_logic(state: JsonValue,
                                       transaction_timestamp: Timestamp,
                                       transaction_sender: String,
                                       transaction_hash: String,
                                       transaction_data: String) -> JsonValue{

       transaction_input_logic!(state.clone(),
                                transaction_timestamp,
                                transaction_sender,
                                transaction_hash,
                                transaction_data)

    }

    fn execute_proposal_creator_election(peer_length: usize,
                                         latest_block_id: i64) -> i64 {
        proposal_creator_election!(peer_length,
                                   latest_block_id)
    }
}

#[cfg(test)]
mod tests {

    use super::{Executor, ExecuteMacro};

    use signature::{DigitalSignature,
                    Verifier,
                    SignatureType,
                    SignatureFormat,
                    SignatureError,
                    Signature};

    use encode::{Encoder, RawBytesEncode, RawBytesDecode, Base64Decode, Base64Encode};


    #[test]
    fn test_execute_proposal_creator_election() {
        let peer_length: usize = 3;
        let latest_block_id: i64 = 0;
        let new_creator_id: i64 = Executor::execute_proposal_creator_election{peer_length, latest_block_id};
        assert_
    }
}
