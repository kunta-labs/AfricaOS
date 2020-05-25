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

use signature::{DigitalSignature,
                Verifier,
                SignatureType,
                SignatureFormat,
                SignatureError,
                Signature};

use encode::{Encoder, RawBytesEncode, RawBytesDecode, Base64Encode, Base64Decode};


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
@name transaction_output_logic
@description
*/
#[macro_export]
macro_rules! transaction_output_logic {
    /*
        @pattern StateJson, self
        default output, to be customized
    */
    ($state: expr, $tx_timestamp: expr, $tx_sender: expr, $tx_hash: expr, $tx_data: expr) => {
        {
            println!("TX execute TX Output BEFORE: {} : ", $state.clone() );
            let mut state_as_json: JsonValue = $state;

            // TODO: prepend the senders account for partner lookup
            // PROBLEM: this^ step breaks the base64 encoding on execution of an input
            //let new_tx_data: String = format!("{} {}", $tx_sender, $tx_data);
            let new_tx_data: String = format!("{}", $tx_data); // do not prepend

            // insert a new account into the state db
            if state_as_json.has_key( &(format!("{}", $tx_sender).to_string()) ) {
                //state already has the account

                // check if code stored at the tx hash
                if state_as_json.has_key( &(format!("{}", $tx_hash).to_string()) ) {

                    // just return state
                    $state

                }else{

                    // check if code stored at the tx hash - specifically if the tx hash exists in state
                    if state_as_json.has_key( &(format!("{}", $tx_hash).to_string()) ) {
                        $state
                    }else{


                        // state doesnt have tx_sender account, so insert
                        match &state_as_json.insert( &( format!("{}", $tx_hash).to_string() ),
                                                        format!("{}", new_tx_data) ) {
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

            }else{

                // state doesnt have tx_sender account, so insert
                match &state_as_json.insert( &( format!("{}", $tx_sender).to_string() ),
                                                format!("{}", 0) ) {
                     Ok(_) => {
                         //TODO: after we insert the initial state for the sender
                         //current_state_buffer
                         println!("TX execute TX Output AFTER insert sender: {} : ",  state_as_json.clone()  );

                         // check if code stored at the tx hash
                         if state_as_json.has_key( &(format!("{}", $tx_hash).to_string()) ) {
                             $state
                         }else{
                             // state doesnt have tx_sender account, so insert
                             match &state_as_json.insert( &( format!("{}", $tx_hash).to_string() ),
                                                             format!("{}", new_tx_data) ) {
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


                     },
                     Err(_) => {
                         // error on inserting, return current state
                         println!("TX execute ERROR: State::to_json is NOT okay: {} ", $state.clone() );
                         $state
                     }
                } // end insert tx sender

            }

        }
    }
}



/*
@name transaction_input_logic
@description
*/
#[macro_export]
macro_rules! transaction_input_logic {
    ($state: expr, $tx_timestamp: expr, $tx_sender: expr, $tx_hash: expr, $tx_data: expr) => {
        {
            println!("TX execute TX INput BEFORE: {} : ", $state.clone() );

            let mut state_as_json: JsonValue = $state;

            let b64_decoded: Result<String, String> = Encoder::decode_base64($tx_data);

            match b64_decoded {
                Ok(decoded) => {

                    ///////////////////
                    println!("decoded 1: {}", decoded);


                    let tx_sections: Vec<&str> = decoded.split(" ").collect::<Vec<_>>();
                    let partner_tx_sender: String = String::from(tx_sections[0]);
                    let tx_partner_hash: String = String::from(tx_sections[1]);
                    let tx_signature: String = String::from(tx_sections[2]);
                    let tx_public_key: String = String::from(tx_sections[3]);

                    // id state has the sender key
                    if state_as_json.has_key( &(format!("{}", tx_partner_hash).to_string()) ) {

                        let partner_tx: Option<&str> = state_as_json[ tx_partner_hash ].as_str();

                        // if partner hash exists
                        if partner_tx.clone().is_some() {

                            println!("partner tx: {}", partner_tx.unwrap());

                            //let b64_decoded_partner: Result<String, String> = Encoder::decode_base64($tx_data);
                            let b64_decoded_partner: String = Encoder::decode_base64( String::from( partner_tx.clone().unwrap() ) ).unwrap();

                            println!("b64_decoded_partner: {}", b64_decoded_partner);

                            let partner_tx_sections: Vec<&str> = b64_decoded_partner.split(" ").collect::<Vec<_>>();
                            // let partner_tx_sender: String = String::from(partner_tx_sections[0]);
                            let public_key_hash_section: String = String::from(partner_tx_sections[0]);
                            let partner_amount_section: String = String::from(partner_tx_sections[1]);


                            //TODO: VERIFY RECEIVERS SUBMITTED HASH HASHES TO BE WHAT THE RECEIVER SUBMITTED
                            //

                            // TODO: check signature
                            let digital_signature: DigitalSignature = signature::DigitalSignature {
                                signature_type: signature::SignatureType::RSA,
                                format_type: signature::SignatureFormat::DISK
                            };

                            // TODO: sign transaction has instead of static str
                            let content: &str = "TEST";
                            let signature_result: &str = tx_signature.as_str();
                            let public_key_base64: &str = tx_public_key.as_str();

                            //convert base64 public key to bytes
                            match encode::Encoder::base64_to_bytes( String::from(public_key_base64) ) {
                                Ok(public_key_bytes) => {

                                    let verification_result: Result<String, signature::SignatureError> = Signature::verify(digital_signature, String::from(signature_result), content);

                                    match verification_result {

                                        Ok(result) => {

                                            // if input sender has account
                                            if state_as_json.has_key( &(format!("{}", $tx_sender.clone()).to_string()) ) {

                                                // grab amount for sender
                                                // parse
                                                //let state_account_amount: Option<i32> = state_as_json[ $tx_sender.clone() ].as_i32();
                                                let state_account_amount: &JsonValue = &state_as_json[ $tx_sender.clone() ];
                                                let parsed_string_state_account_amount: String = state_account_amount.to_string();
                                                println!("parsed_string_state_account_amount: {}", parsed_string_state_account_amount);
                                                let amount_i32_account: Result<i32, std::num::ParseIntError> = parsed_string_state_account_amount.parse::<i32>();

                                                //parse partner amount
                                                let partner_tx_amount_parse_result: Result<i32, std::num::ParseIntError> = partner_amount_section.parse::<i32>();

                                                // parse partner sender amount
                                                //let state_partner_amount: Option<i32> = state_as_json[ partner_tx_sender.clone() ].as_i32();

                                                ////////////
                                                let state_partner_amount: &JsonValue = &state_as_json[ partner_tx_sender.clone() ];
                                                let parsed_string_state_partner_amount: String = state_partner_amount.to_string();
                                                println!("parsed_string_state_partner_amount: {}", parsed_string_state_partner_amount);
                                                let amount_i32_partner: Result<i32, std::num::ParseIntError> = parsed_string_state_partner_amount.parse::<i32>();
                                                ///////////


                                                println!("input: {} {} {} {} {} {}",
                                                                            state_as_json.dump(),
                                                                            partner_tx_sender,
                                                                            $tx_sender,
                                                                            // state_account_amount.is_some(),
                                                                            // partner_tx_amount_parse_result.is_ok(),
                                                                            // state_partner_amount.is_some()
                                                                            amount_i32_account.is_ok(),
                                                                            partner_tx_amount_parse_result.is_ok(),
                                                                            amount_i32_partner.is_ok()
                                                                        );


                                                // if state_account_amount.is_some()
                                                //    & partner_tx_amount_parse_result.is_ok()
                                                //    & state_partner_amount.is_some() {
                                                if amount_i32_account.is_ok()
                                                   & partner_tx_amount_parse_result.is_ok()
                                                   & amount_i32_partner.is_ok() {

                                                    // TODO: error handling on amount adjustments
                                                    // update senders balance to what they had, plus the partner tx value
                                                    // state_as_json[$tx_sender] = JsonValue::from( state_account_amount.unwrap() + partner_tx_amount_parse_result.clone().unwrap() );
                                                    // state_as_json[partner_tx_sender] = JsonValue::from( state_partner_amount.unwrap() - partner_tx_amount_parse_result.unwrap() );

                                                    match &state_as_json.insert( &( format!("{}", $tx_sender ).to_string() ),
                                                                                    //format!("{}", JsonValue::from( state_account_amount.unwrap() + partner_tx_amount_parse_result.clone().unwrap() ) ) ) {
                                                                                    format!("{}", JsonValue::from( amount_i32_account.unwrap() + partner_tx_amount_parse_result.clone().unwrap() ) ) ) {
                                                         Ok(_) => {

                                                             match &state_as_json.insert( &( format!("{}", partner_tx_sender ).to_string() ),
                                                                                             //format!("{}", JsonValue::from( state_partner_amount.unwrap() - partner_tx_amount_parse_result.unwrap() ) ) ) {
                                                                                             format!("{}", JsonValue::from( amount_i32_partner.unwrap() - partner_tx_amount_parse_result.unwrap() ) ) ) {
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

                                                         },
                                                         Err(_) => {
                                                             // error on inserting, return current state
                                                             println!("TX execute ERROR: State::to_json is NOT okay: {} ", $state.clone() );
                                                             $state
                                                         }
                                                    }

                                                }else{
                                                    println!("transaction_input_logic: first condition else");
                                                    $state
                                                }

                                            }else{

                                                // parse
                                                //let state_account_amount: Option<i32> = state_as_json[$tx_sender.clone()].as_i32();

                                                //parse partner amount
                                                let partner_tx_amount_parse_result: Result<i32, std::num::ParseIntError> = partner_amount_section.parse::<i32>();

                                                println!("state: {} {} {}", state_as_json.dump(),
                                                                            $tx_sender.clone(),
                                                                            partner_tx_sender.clone());

                                                // parse partner sender amount
                                                let state_partner_amount: &JsonValue = &state_as_json[ partner_tx_sender.clone() ];

                                                let parsed_string: String = state_partner_amount.to_string();

                                                println!("parsed_string: {}", parsed_string);

                                                let amount_i32: Result<i32, std::num::ParseIntError> = parsed_string.parse::<i32>();

                                                println!("TX SENDER DOESNT HAVE ACCOUNT: {} - {} - {} - {}", partner_tx_amount_parse_result.is_ok(),
                                                                                                    state_partner_amount.dump(),
                                                                                                    partner_tx_sender,
                                                                                                    amount_i32.clone().unwrap());

                                                // TODO: check if sender exists
                                                if partner_tx_amount_parse_result.is_ok() & amount_i32.is_ok() {

                                                    // update senders balance to the partner tx value
                                                    // state_as_json[$tx_sender] = JsonValue::from( partner_tx_amount_parse_result.clone().unwrap() );
                                                    // state_as_json[partner_tx_sender] = JsonValue::from( state_partner_amount.unwrap() - partner_tx_amount_parse_result.unwrap() );

                                                    match &state_as_json.insert( &( format!("{}", $tx_sender ).to_string() ),
                                                                                    format!("{}", JsonValue::from( partner_tx_amount_parse_result.clone().unwrap() ) ) ) {
                                                         Ok(_) => {

                                                             match &state_as_json.insert( &( format!("{}", partner_tx_sender ).to_string() ),
                                                                                             format!("{}", JsonValue::from( amount_i32.unwrap() - partner_tx_amount_parse_result.unwrap() ) ) ) {
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

                                                         },
                                                         Err(_) => {
                                                             // error on inserting, return current state
                                                             println!("TX execute ERROR: State::to_json is NOT okay: {} ", $state.clone() );
                                                             $state
                                                         }
                                                    }

                                                }else{

                                                    println!("partner_amount_parse_result is not ok");
                                                    $state

                                                }

                                            }

                                        },

                                        Err(_) => {
                                            println!("SIGNATURE VERIFICATION FAILED");
                                            $state
                                        }

                                    }

                                },
                                Err(error_message) => {
                                    println!("ERROR: {}", error_message);
                                    $state
                                }
                            }

                        } else {
                            // partner tx doesnt exist in state
                            $state
                        }



                    }else{
                        $state
                    }
                    //////////////////////

                },
                Err(_) => {
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
    use encode::{Encoder, RawBytesEncode, RawBytesDecode, Base64Encode, Base64Decode};
    use signature::{DigitalSignature,
                    Verifier,
                    SignatureType,
                    SignatureFormat,
                    SignatureError,
                    Signature};

    #[test]
    fn test_transaction_output_public_key_new_account() -> (){

        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_sender: String = String::from("test_sender");
        let test_hash_string: String = String::from("e2a8aca48c5b24df14c6e0ab0b30df7ed50fa97bc22fd706c71a7eebe96a8b67");
        let test_tx_data_string: String = format!("{}", "TESTDATA");


        // female transaction will hold the public key hash of the recipient
        let test_state_json: JsonValue = object!{
            "test" => "test"
        };

        let test_json_2: JsonValue = object!{
            "test" => "test",
            test_sender.as_str() => "0",
            //test_hash_string.as_str() => format!("{} {}", test_sender, test_tx_data_string.clone()),
            test_hash_string.as_str() => format!("{}", test_tx_data_string.clone()),
        };


        let result: JsonValue = transaction_output_logic!( test_state_json.clone(),
                                                           test_timestamp.unwrap(),
                                                           test_sender,
                                                           test_hash_string,
                                                           test_tx_data_string );
        assert_eq!(test_json_2, result);

    }

    #[test]
    fn test_transaction_output_public_key_existing_account() -> (){

        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_tx_string: String = String::from("TXTESTHASH");
        let test_sender: String = String::from("test_sender");
        let test_hash_string: String = String::from("e2a8aca48c5b24df14c6e0ab0b30df7ed50fa97bc22fd706c71a7eebe96a8b67");
        let test_tx_data_string: String = format!("{}", test_hash_string);


        // female transaction will hold the public key hash of the recipient
        let test_state_json: JsonValue = object!{
            test_sender.as_str() => "100"
        };

        let test_json_2: JsonValue = object!{
            test_sender.as_str() => "100",
            //test_hash_string.as_str() => format!("{} {}", test_sender, test_tx_data_string.clone()),
            test_hash_string.as_str() => format!("{}", test_tx_data_string.clone()),
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
        let test_partner_sender: String = String::from("alice");
        let test_sender: String = String::from("test_sender");
        let test_tx_hash_string: String = String::from("TESTTXHASH");
        let test_pubkhash_string: String = String::from("e2a8aca48c5b24df14c6e0ab0b30df7ed50fa97bc22fd706c71a7eebe96a8b67");
        let test_amount: i32 = 10;

        let test_out_tx_data_string: String = format!("{} {}", test_pubkhash_string, test_amount);
        println!("be641: {}", Encoder::encode_base64(test_out_tx_data_string.clone()).unwrap());

        let test_out_tx_data_string: String = Encoder::encode_base64( test_out_tx_data_string.clone() ).unwrap();

        // the signature of the transaction
        let test_bob_signature: String = String::from("opEssZ7CaoYvtZJErFPqiB0L+lxwFm1/YT3tLZ+07fCnwWvuRcXtpwmo4esdNs05OItDBK6SZaxVPO+tKG22NC8R64DQj4J6CXpt4XMxtGJSUeY9MyZB6eyW8qYye7zascGv5+Eht4VJ5Zu9TX8Xl2+oyZA+3RYw5QKHvMgHyN0mpPU8PYpBDdVKg5Nglh4WOjqrvJF/EAdyqfeLN0CNJHeFwwjlkDaOz1x9LBOBf8c5HhDulgblSd4tlJ9zRA97SbnxmQtip/XDLweTtCx9vmjFd0tw/JTcfl2V87r+JgxL0r9EgEoFsexs3XkdqKZ2LzypPMvp0XqeoNEJ03g96A==");

        //the public key to check the signature, and to hash
        let test_bob_public_key_base64: &str = "MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB";

        let test_in_tx_data_string2: String = format!("{} {} {} {}", test_partner_sender,
                                                                    test_tx_hash_string,
                                                                    test_bob_signature,
                                                                    test_bob_public_key_base64);

        //let b64_test_data: Result<String,String> = Encoder::decode_base64(test_in_tx_data_string2.clone());
        let b64_test_data: Result<String,String> = Encoder::encode_base64(test_in_tx_data_string2.clone());

        println!("be64: {}", b64_test_data.clone().unwrap());

        let test_in_tx_data_string: String = b64_test_data.unwrap();

        let test_state_json: JsonValue = object!{
            //test_tx_hash_string.as_str() => test_out_tx_data_string.clone(),
            test_tx_hash_string.as_str() => test_out_tx_data_string.clone(),
            test_partner_sender.as_str() => "100"
        };

        let test_json_2: JsonValue = object!{
            // test_tx_hash_string.as_str() => format!("{} {}", test_sender, test_out_tx_data_string.clone()),
            test_tx_hash_string.as_str() => format!("{}", test_out_tx_data_string.clone()),
            test_partner_sender.as_str() => "90",
            test_sender.as_str() => "10"
        };

        let result: JsonValue = transaction_input_logic!( test_state_json.clone(),
                                                          test_timestamp.unwrap(),
                                                          test_sender,
                                                          test_tx_hash_string,
                                                          test_in_tx_data_string
                                                      );
        assert_eq!(test_json_2, result);

    }

    #[test]
    fn test_transaction_input_logic_existing_account() -> (){

        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_partner_sender: String = String::from("alice");
        let test_sender: String = String::from("test_sender");
        let test_tx_hash_string: String = String::from("TESTTXHASH");
        let test_pubkhash_string: String = String::from("e2a8aca48c5b24df14c6e0ab0b30df7ed50fa97bc22fd706c71a7eebe96a8b67");
        let test_amount: i32 = 10;

        let test_out_tx_data_string: String = format!("{} {}", test_pubkhash_string, test_amount);
        println!("be641: {}", Encoder::encode_base64(test_out_tx_data_string.clone()).unwrap());

        let test_out_tx_data_string: String = Encoder::encode_base64( test_out_tx_data_string.clone() ).unwrap();

        // the signature of the transaction
        let test_bob_signature: String = String::from("opEssZ7CaoYvtZJErFPqiB0L+lxwFm1/YT3tLZ+07fCnwWvuRcXtpwmo4esdNs05OItDBK6SZaxVPO+tKG22NC8R64DQj4J6CXpt4XMxtGJSUeY9MyZB6eyW8qYye7zascGv5+Eht4VJ5Zu9TX8Xl2+oyZA+3RYw5QKHvMgHyN0mpPU8PYpBDdVKg5Nglh4WOjqrvJF/EAdyqfeLN0CNJHeFwwjlkDaOz1x9LBOBf8c5HhDulgblSd4tlJ9zRA97SbnxmQtip/XDLweTtCx9vmjFd0tw/JTcfl2V87r+JgxL0r9EgEoFsexs3XkdqKZ2LzypPMvp0XqeoNEJ03g96A==");

        //the public key to check the signature, and to hash
        let test_bob_public_key_base64: &str = "MIIBCgKCAQEAuJ3hhGpo+nInkqHpgBx3E5eihx0IGNVit5u0UlvLHmnW2PJ3HqFyafr/eYaas7VetW5Ss5kAWZmH3oED7n4xVlXrUeFlSwShSDVMKyT3iK1et1KQIX5cqp2CiFSs+xi5eJOEEWZxoexYbSTg0Rg34gzob+VqHZzRtRN9eTja7ZCE3/m3cMlWfb7yL2jyN521ZL02QG9PZ4EYenDTM2xcWvCZrWtKUFLahCWWQ1H8ZpoWg/y/tenvUy5YnWLrbhbSZqMJQYfnUwr8FRWZsiI2RWRtXlx9X6bxvaOoG/h4IOhV/G52Xas2WxxUbZ5rgRPziA/mvkJxytHkZPeEnpRcawIDAQAB";

        let test_in_tx_data_string2: String = format!("{} {} {} {}", test_partner_sender,
                                                                    test_tx_hash_string,
                                                                    test_bob_signature,
                                                                    test_bob_public_key_base64);

        //let b64_test_data: Result<String,String> = Encoder::decode_base64(test_in_tx_data_string2.clone());
        let b64_test_data: Result<String,String> = Encoder::encode_base64(test_in_tx_data_string2.clone());

        println!("be64: {}", b64_test_data.clone().unwrap());

        let test_in_tx_data_string: String = b64_test_data.unwrap();

        let test_state_json: JsonValue = object!{
            //test_tx_hash_string.as_str() => test_out_tx_data_string.clone(),
            test_tx_hash_string.as_str() => test_out_tx_data_string.clone(),
            test_partner_sender.as_str() => "90",
            test_sender.as_str() => "10",
        };

        let test_json_2: JsonValue = object!{
            // test_tx_hash_string.as_str() => format!("{} {}", test_sender, test_out_tx_data_string.clone()),
            test_tx_hash_string.as_str() => format!("{}", test_out_tx_data_string.clone()),
            test_partner_sender.as_str() => "80",
            test_sender.as_str() => "20"
        };

        let result: JsonValue = transaction_input_logic!( test_state_json.clone(),
                                                          test_timestamp.unwrap(),
                                                          test_sender,
                                                          test_tx_hash_string,
                                                          test_in_tx_data_string
                                                      );
        assert_eq!(test_json_2, result);

    }
}
