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

use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::io::{Write, Read};
use http::{Request, Response, StatusCode};
use std::collections::HashMap;
use proposal::{Proposal,
               NewProposal,
               JsonConverter,
               StatusToString,
               StringToStatus,
               ProposalValidator,
               ProposalValidationResult,
               StoreProposal,
               ProposalStatus,
               GetProposals,
               UpdateProposal,
               CompareWithoutStatus,
               ValidateProposalBlock,
               ProposalResolutionAccepted};
use url::Url;
use reqwest::header::{USER_AGENT, CONTENT_TYPE, ORIGIN};
use encode::{Encoder, Base64Encode, Base64Decode};
use transaction::{Transaction, CreateNewOuputTransaction};
use block::{Block, ReadBlock, BlockToJson};


pub trait PayloadParser {
    /*
    @name get_query_from_payload
    @desc parse out query from TCP payload buffer
    */
    fn get_query_from_payload(payload_split: Vec<&str>) -> &str;

    /*
    @name get_key_from_header
    @desc after matched on the header, retrieve the actual key from the header
    */
    fn get_key_from_header(sections: &str, header_sections: Vec<&str>) -> Result<String, String>;

    /*
    @name get_data_from_payload
    @desc parse out data from TCP payload buffer
    */
    fn get_header_from_payload(payload_split: Vec<&str>, header: &str) -> Result<String, String>;
}

impl PayloadParser for Server {
    /*
    @name get_query_from_payload
    @desc parse out query from TCP payload buffer
    */
    fn get_query_from_payload(payload_split: Vec<&str>) -> &str {
        payload_split[1]
    }

    fn get_key_from_header(section: &str, header_sections: Vec<&str>) -> Result<String, String> {
        if header_sections.len() == 2 {
            let data_second_element: &str = header_sections[1];
            println!("Data data_second_element: {}", data_second_element);
            let string_to_trunc: String = String::from(data_second_element);
            let mut header_data: &str = string_to_trunc.trim();
            let final_header_data: String = String::from(header_data);
            Ok(final_header_data)
        } else if header_sections.len() == 3 {
            let data_second_element: &str = header_sections[1];
            let data_third_element: &str = header_sections[2];
            let string_to_trunc: String = format!("{}:{}", data_second_element, data_third_element);
            println!("Data SECOND AND THIRD ELEMENT: {}", string_to_trunc);
            let mut header_data: &str = string_to_trunc.trim();
            let final_header_data: String = String::from(header_data);
            Ok(final_header_data)
        } else {
            println!("Error Splitting relevant header, header_sections.LEN() IS NOT 2 OR 3: {}", section);
            Err(String::from("Error Splitting relevant header, header_sections.LEN() IS NOT 2 OR 3"))
        }
    }

    /*
    @name get_data_from_payload
    @desc parse out data from TCP payload buffer
    */
    fn get_header_from_payload(payload_split: Vec<&str>, header: &str) -> Result<String, String> {
        let mut header_result: Result<String, String> = Err(String::from("N/A"));
        'outer: for section in payload_split.clone() {
            let header_sections: Vec<&str> = section.split(":").collect();
            header_sections.iter().for_each(|v| println!("Header Section: {}", v));
            let header_name: String = header_sections[0].to_lowercase();
            println!("HEADER NAME: {}", header_name);
            match header_name.as_str() {
                "user-agent" => {
                    match header {
                        "user-agent" => {
                            println!("Found Data Index: {}", header_sections[0]);
                            header_result = Self::get_key_from_header(section, header_sections);
                            break 'outer
                        },
                        _ => {
                            header_result = Err(String::from("get_header_from_payload, Error: user-agent not found"))
                        }
                    }
                },
                "origin" => {
                    match header {
                        "origin" => {
                            println!("Found Origin Index: {}", header_sections[0]);
                            header_result = Self::get_key_from_header(section, header_sections);
                            break 'outer
                        },
                        _ => {
                            header_result = Err(String::from("get_header_from_payload, Error: origin not found"))
                        }
                    }
                },
                _ => {
                    println!("NOT CORRECT Data Index: {}", header_sections[0]);
                }
            };
        }

        if header_result.is_ok() {
            header_result
        } else {
            Err(String::from("Relevant Header NOT FOUND"))
        }
    }
}

/*
@name Server
@desc this contains server information
*/
#[derive(Debug, Clone)]
pub struct Server {
    pub port: i32,
}

/*
@name Transmitter
@desc this trait enables our server to transmit data over the network
*/
pub trait Transmitter{

    /*
    @name broadcast_proposal_created
    @desc broadcast the proposal that we just
          create to the network
    */
    fn broadcast_proposal_created(proposal: Proposal, peer_location: String, ip: String) -> Result<(), String>;

    /*
    @name broadcast_proposal_response
    @desc broadcast the proposal response to
          the peer who sent the proposal
    */
    fn broadcast_proposal_response(proposal: Proposal, peer_location: String, ip: String, proposal_status: ProposalStatus) -> Result<(), String>;

    /*
    @name broadcast_proposal_resolution
    @desc broadcast resolution to peers who responded to proposal id
    */
    fn broadcast_proposal_resolution(proposal: Proposal, peer_location: String, ip: String) -> Result<(), Box<std::error::Error>>;

    /*
    @name broadcast_block_query
    @desc broadcast query to fetch block for sync purposes
    */
    fn broadcast_block_query(block_id_requested: i64, peer_location: String, ip: String) -> Result<(), Box<std::error::Error>>;

    /*
    @name broadcast_block_query_response
    @desc respond to a block query from another node
    */
    fn broadcast_block_query_response(proposal: Proposal, peer_location: String) -> Result<(), String> ;

}

impl Transmitter for Server {

        /*
        @name broadcast_proposal_created
        @desc broadcast the proposal that we just
              create to the network
        */
        fn broadcast_proposal_created(proposal: Proposal, peer_location: String, ip: String) -> Result<(), String> {
            println!("Broadcasting After Proposal is Created... TO {}", peer_location);
            let mut map = HashMap::new();
            let proposal_id_to_insert: &str = &format!("{}", proposal.clone().proposal_id);
            map.insert("proposal_id",  proposal_id_to_insert);
            let proposal_status_to_insert: &str = Proposal::string_from_status(proposal.clone().proposal_status);
            map.insert("proposal_status",  proposal_status_to_insert);
            let peer_location_url: &str = &format!("http://{}/proposal/created/", peer_location).to_string();
            let url_object = url::Url::parse( peer_location_url );
            let client = reqwest::Client::new();
            let proposal_to_json: String = Proposal::to_json(proposal.clone()).to_string();
            let b64_stringed_proposal: Result<String,String> = Encoder::encode_base64(proposal_to_json);
            //TODO: alter a meaningful header, not user agent...
            if b64_stringed_proposal.is_ok() {
                let resp = client.get(peer_location_url)
                                 .header(ORIGIN, ip.as_str())
                                 .header(USER_AGENT, b64_stringed_proposal.unwrap())
                                 .send();
                 match resp {
                    Ok(result) => {
                        println!("broadcast_proposal_created, Broadcast Sent Successfully: {:#?}", result);
                        Ok(())
                    },
                    Err(err) => {
                        println!("broadcast_proposal_created, Broadcast Sent Failed: {:#?}", err);
                        Ok(())
                    }
                 }
            } else {
                Ok(())
            }
        }

        /*
        @name broadcast_proposal_response
        @desc broadcast the proposal response to
              the peer who sent the proposal
        */
        fn broadcast_proposal_response(proposal: Proposal, peer_location: String, ip: String, proposal_status: ProposalStatus) -> Result<(), String>{
            println!("Broadcasting Response to a proposal received... TO {}", peer_location);
            let mut map = HashMap::new();
            let proposal_id_to_insert: &str = &format!("{}", proposal.clone().proposal_id);
            map.insert("proposal_id",  proposal_id_to_insert);
            let proposal_status_to_insert: &str = Proposal::string_from_status(proposal.clone().proposal_status);
            map.insert("proposal_status",  proposal_status_to_insert);
            let peer_location_url: &str = &format!("http://{}/proposal/response/", peer_location).to_string();
            let url_object = url::Url::parse( peer_location_url );
            let client = reqwest::Client::new();
            let proposal_to_json: String = Proposal::to_json(proposal.clone()).to_string();
            let b64_stringed_proposal: Result<String,String> = Encoder::encode_base64(proposal_to_json);
            //TODO: alter a meaningful header, not user agent...
            if b64_stringed_proposal.is_ok() {
                let resp = client.get(peer_location_url)
                                 .header(ORIGIN, ip.as_str())
                                 .header(USER_AGENT, b64_stringed_proposal.unwrap())
                                 .send();
                 match resp {
                    Ok(result) => {
                        println!("broadcast_proposal_response, Broadcast Sent Successfully: {:#?}", result);
                        Ok(())
                    },
                    Err(err) => {
                        println!("broadcast_proposal_response, Broadcast Sent Failed: {:#?}", err);
                        Ok(())
                    }
                 }
            } else {
                Ok(())
            }
        }

        /*
        @name broadcast_proposal_resolution
        @desc broadcast resolution to peers who responded to proposal id
        */
        fn broadcast_proposal_resolution(proposal: Proposal, peer_location: String, ip: String) -> Result<(), Box<std::error::Error>> {
            println!("Broadcasting Resolution after proposal, and responses... TO {}", peer_location);
            let mut map = HashMap::new();
            let proposal_id_to_insert: &str = &format!("{}", proposal.clone().proposal_id);
            map.insert("proposal_id",  proposal_id_to_insert);
            let proposal_status_to_insert: &str = Proposal::string_from_status(proposal.clone().proposal_status);
            map.insert("proposal_status",  proposal_status_to_insert);
            let peer_location_url: &str = &format!("http://{}/proposal/resolution/", peer_location).to_string();
            let url_object = url::Url::parse( peer_location_url );
            let client = reqwest::Client::new();
            let proposal_to_json: String = Proposal::to_json(proposal.clone()).to_string();
            let b64_stringed_proposal: Result<String,String> = Encoder::encode_base64(proposal_to_json);
            //TODO: alter a meaningful header, not user agent...
            if b64_stringed_proposal.is_ok() {
                let resp = client.get(peer_location_url)
                                 .header(ORIGIN, ip.as_str())
                                 .header(USER_AGENT, b64_stringed_proposal.unwrap())
                                 .send();
                 match resp {
                    Ok(result) => {
                        println!("broadcast_proposal_resolution, Broadcast Sent Successfully: {:#?}", result);
                        Ok(())
                    },
                    Err(err) => {
                        println!("broadcast_proposal_resolution, Broadcast Sent Failed: {:#?}", err);
                        Ok(())
                    }
                 }
            } else {
                Ok(())
            }
        }

        /*
            @name broadcast_block_query
            @desc
        */
        fn broadcast_block_query(block_id_requested: i64, peer_location: String, ip: String) -> Result<(), Box<std::error::Error>> {
            println!("Broadcasting to attempt to sync chain... TO {}, fetching: {}", peer_location, block_id_requested);
            let peer_location_url: &str = &format!("http://{}/block/query/", peer_location).to_string();
            let url_object = url::Url::parse( peer_location_url );
            let client = reqwest::Client::new();
            let resp = client.get(peer_location_url)
                             .header(ORIGIN, ip.as_str())
                             .header(USER_AGENT, format!("{}", block_id_requested))
                             .send();
            match resp {
                Ok(result) => {
                    println!("broadcast_block_query, Broadcast Sent Successfully: {:#?}", result);
                    Ok(())
                },
                Err(err) => {
                    println!("broadcast_block_query, Broadcast Sent Failed: {:#?}", err);
                    Ok(())
                }
            }
        }

        /*
            @name broadcast_block_query_response
        */
        fn broadcast_block_query_response(proposal: Proposal, peer_location: String) -> Result<(), String> {
            println!("Broadcasting Block Query response to a block query... TO {}", peer_location);
            let mut map = HashMap::new();
            let proposal_id_to_insert: &str = &format!("{}", proposal.clone().proposal_id);
            map.insert("proposal_id",  proposal_id_to_insert);
            let peer_location_url: &str = &format!("http://{}/block/response/", peer_location).to_string();
            let url_object = url::Url::parse( peer_location_url );
            let client = reqwest::Client::new();
            let proposal_to_json: String = Proposal::to_json(proposal.clone()).to_string();
            let b64_stringed_proposal: Result<String,String> = Encoder::encode_base64(proposal_to_json);
            //TODO: alter a meaningful header, not user agent...
            if b64_stringed_proposal.is_ok() {
                println!( "broadcast_block_query_response(), b64_stringed_proposal{}", b64_stringed_proposal.clone().unwrap() );
                let resp = client.get(peer_location_url)
                                 .header(ORIGIN, "127.0.0.1")
                                 .header(USER_AGENT, b64_stringed_proposal.unwrap())
                                 .send();
                 match resp {
                    Ok(result) => {
                        println!("broadcast block query response, Broadcast Sent Successfully: {:#?}", result);
                        Ok(())
                    },
                    Err(err) => {
                        println!("broadcast block query response, Broadcast Sent Failed: {:#?}", err);
                        Ok(())
                    }
                 }
            } else {
                Ok(())
            }
        }
}


/*
@name Receiver
@desc this trait enables TCP receiving for the server
*/
pub trait Receiver {
    fn start(&self) -> Result<String, String>;
    fn handle_client(stream: TcpStream) -> Result<String, String>;
    fn handle_read(stream: &TcpStream) -> Result<String, String>;
    fn handle_write(stream: TcpStream, result: String) -> Result<String, String>;
}

/*
TODO: convert this to using trait bounds?
fn f<T: Display + Clone)>(t: T) -> i32
*/
impl Receiver for Server {
    /*
    @name start
    @desc this starts the TCP server
    */
    fn start(&self) -> Result<String, String> {
        let server_prefix = String::from("0.0.0.0:");
        let port = self.port;
        let server_complete_address = format!("{}{}", server_prefix, port);
        let listener = TcpListener::bind(server_complete_address).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        let client_handle_result: Result<String, String> = Self::handle_client(stream);
                        match client_handle_result {
                            Ok(s) => Ok(s),
                            Err(e) => Err(e)
                        }
                    });
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            };
        }
        Ok(String::from("Default start result"))
    }

    /*
    @name handle_client
    @desc for every TcpListener.incoming instance, this handles
          the reading and writing of data for the request
    */
    fn handle_client(stream: TcpStream) -> Result<String, String> {
        let read_result = match Self::handle_read(&stream) {
            Ok(read_result) => {
                println!("handle_client, read_result success: {}",read_result);
                Ok(read_result)
            },
            Err(_) => {
                println!("handle_client ERROR: read_result failed...");
                Err(String::from("ERROR: read_result failed..."))
            }
        };

        if read_result.is_ok() {
            let write_result = match Self::handle_write(stream, read_result.unwrap() ) {
                Ok(write_result) => {
                    println!("handle_client, write_result: {}",write_result);
                    Ok(String::from("write_result successs"))
                },
                Err(_) => {
                    println!("handle_client, write result has failed ");
                    Err(String::from("write_result error"))
                }
            };
            write_result
        } else {
            Err(String::from("ERROR: read_result was NOT ok"))
        }
    }

    /*
    @name handle_read
    @desc per every TCPStream instance, this handles
          the reading of data from the requestor
    */
    fn handle_read(mut stream: &TcpStream) -> Result<String, String> {
        let mut buf: [u8; 4096] = [0u8 ; 4096];
        println!("handle_read, Stream IP: {}", stream.local_addr().unwrap());
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("handle_read, REQ STR: {}", req_str);
                let split_payload_for_query: Vec<&str> = req_str.split(" ").collect();
                let query: &str = Self::get_query_from_payload(split_payload_for_query);
                println!("handle_read, Query: {}", query);
                let split_payload_for_data: Vec<&str> = req_str.split("\n").collect();
                let data: Result<String, String> = Self::get_header_from_payload(split_payload_for_data.clone(), "user-agent");
                //TODO: state which header to return
                if data.is_ok() {
                    //TODO: get node IP address to send to receiver to pass to invoke action
                    let request_origin: Result<String, String> = Self::get_header_from_payload(split_payload_for_data, "origin");
                    if request_origin.is_ok() {
                        println!("request_origin success: {}", request_origin.clone().unwrap());
                        //TODO: pass request_origin to invoke_action()
                        let invoked_action_result: Result<String, String> = Self::invoke_action( query, &(data.unwrap().to_string()), request_origin.clone().unwrap() );
                        match invoked_action_result {
                            Ok(r) => {
                                Ok(String::from(r))
                            },
                            Err(_) => {
                                Ok(String::from(query))
                            }
                        }
                    } else {
                        println!("ERROR: request_origin couldn't fetch origin from payload: {:?}", request_origin);
                        Err(String::from("Request Origin not found in payload"))
                    }
                } else {
                    Err(String::from("Data Parse was not successful"))
                }
            },
            Err(e) => {
                println!("Unable to read stream: {}", e);
                Err(String::from("error with stream"))
            }
        }
    }

    /*
    @name handle_write
    @desc per every TCPStream instance, this handles the
          writing of data back to the requestor
    */
    fn handle_write(mut stream: TcpStream, result: String) -> Result<String, String> {

        //TODO: set CONTENT-LENGTH dynamically
        let response_result = format!( "HTTP/1.1 200 OK
                                       \r\nContent-Length: 1000
                                       \r\nContent-Type: application/json; charset=UTF-8
                                       \r\n\r\n{}
                                       \r\n", result );
        //a.iter().cloned().collect();
        //let c = &a[..]; // c: &[u8]

        let response_string: String = String::from(response_result);

        //TODO: WRITE BACK THE RESULT PASSED
        //let data_to_write: String = format!("{}");
        match stream.write(response_string.as_bytes()) {
            Ok(_) => {
                println!("handle_write, Stream Write Success");
                Ok(String::from("Response Sent"))
            },
            Err(e) => {
                println!("handle_write, Stream Write FAILURE: {}", e);
                Ok(String::from("Response error"))
            },
        }
    }
}


/*
@name API for Server
@desc invoked actions by implementing this trait
*/
pub trait API {
    /*
    @name invoke_action
    @desc interpret which API endpoint action to invoke
    */
    fn invoke_action(command: &str, data: &str, request_origin: String) -> Result<String, String>;
}


/*
@name API for Server
@desc invoked actions by implementing this trait
*/
impl API for Server {
    /*
    @name invoke_action
    @desc interpret which API endpoint action to invoke
    */
    fn invoke_action(command: &str, data: &str, request_origin: String) -> Result<String, String> {
        match command {

            // TODO:
            /*
            @endpoint /API/block/height/
            @desc get the top block
            */
            "/API/block/height/" => {
                println!("API Block Height: {}, {}, {}", command, data, request_origin);
                // get latest block id
                let top_block_id: Option<i64> = Block::get_latest_block_id();
                match top_block_id {
                    Some(block_id) => {
                        let block_to_return: Option<Block> = Block::get_block_by_block_id(block_id);
                        match block_to_return {
                            Some(block) => {
                                Ok( String::from(Block::to_json(block).dump()) )
                            },
                            None => {
                                Err( String::from("API Block block by ID: Block Option was null") )
                            }
                        }
                    },
                    None => {
                        Err( String::from("API Block height: NO TOP BLOCK...") )
                    }
                }

            }

            // TODO:
            /*
            @endpoint /API/block/get/
            @desc get the latest proposal
            */
            "/API/block/get/" => {
                println!("API Block by ID get: {}, {}, {}", command, data, request_origin);
                // get block by id
                let block_id = data.parse::<i64>().unwrap();
                let block_to_return: Option<Block> = Block::get_block_by_block_id(block_id);
                match block_to_return {
                    Some(block) => {
                        Ok( String::from(Block::to_json(block).dump()) )
                    },
                    None => {
                        Err( String::from("API Block by ID: Block Option was null") )
                    }
                }

            }

            /* TODO:
            @endpoint /API/proposal/latest/
            @desc get the latest proposal
            */
            "/API/proposal/latest/" => {
                // get latest proposal id
                println!("API Proposal Latest: {}, {}, {}", command, data, request_origin);
                let latest_proposal: Option<Proposal> = Proposal::get_latest_proposal();
                match latest_proposal {
                    Some(proposal) => {
                        Ok( String::from(Proposal::to_json(proposal)) )
                    },
                    None => {
                        Err( String::from("API latest proposal: Proposal Option was null") )
                    }
                }
            }

            /*
            @endpoint /transaction/submit/output/
            @desc for an external submission of a transaction
            */
            "/transaction/submit/output" => {
                println!("Transaction Submit: {}, {}, {}", command, data, request_origin);
                let new_transaction: Option<Transaction> = Transaction::new_output(request_origin.clone(), String::from(data.clone()) );
                match new_transaction {
                    Some(tx) => {
                        println!("Transaction Made: {}", tx.transaction_id);
                        let create_tx_result: String = format!("Transaction Received {}", tx.transaction_hash);
                        Ok( String::from(create_tx_result) )
                    },
                    None => {
                        println!("ERROR Transaction NOT Made");
                        Err( String::from("Transaction ERROR, NEW TX FAILED TO BE MADE") )
                    }
                }
            },

            /*
            @endpoint /proposal/create/
            @desc create a proposal, NOTE: should we expose this externally?
            peerid, proposalid
            "INVOKES" A -> B
            */
            "/proposal/create/" => {
                println!("Invocation to create new proposal: {}", data);
                let proposal_created: Option<Proposal> = Proposal::create(request_origin);
                match proposal_created {
                    Some(proposal) => {
                        println!("Proposal Created, at endpoint: {}", Proposal::to_json(proposal));
                        Ok(String::from("Proposal Created"))
                    },
                    None => Err(String::from("network ERROR: invoke action, Proposal created is nont"))
                }
            },

            /*
            @endpoint /proposal/created/
            @desc receive a proposal created by someone else
            peerid, proposalid?
            A -> B, AS B
            */
            "/proposal/created/" => {
                let decoded_proposal_string: Result<String, String> = Encoder::decode_base64(String::from(data));
                if decoded_proposal_string.clone().is_ok() {
                    println!("invoke_action() - Success: Received a proposal created by another node: {}::{}", data, decoded_proposal_string.clone().unwrap());
                    println!("Decoded Proposal String: {:?}", decoded_proposal_string);
                    let decoded_proposal: Result<Proposal, String> = Proposal::from_json_string(decoded_proposal_string.unwrap());
                    match decoded_proposal.clone() {
                        Ok(proposal) => {
                            println!("invoke_action, proposal_created: successful proposal decoding, proposal_id: {}", decoded_proposal.unwrap().proposal_id);
                            //TODO: Check current block ID against the proposal block_id to see if network's chain is ahead of the node's chain
                            //proposal verdict
                            match Proposal::validate_proposal(proposal.clone()) {
                                //NOTE: ONLY DO SOMETHING IF YOU CAN SAFELY PARSE THE PROPOSAL, OTHERWISE ERROR
                                Ok(verdict) => {
                                    match verdict {
                                        ProposalValidationResult::Valid => {
                                            //TODO: check if we already stored the proposal, we may be receiving it because author may be re-broadcasting it
                                            Proposal::store_proposal(proposal.clone(), ProposalStatus::Accepted);
                                            Ok(String::from("Proposal Valid"))
                                        },
                                        ProposalValidationResult::NotValid => {
                                            Proposal::store_proposal(proposal.clone(), ProposalStatus::Rejected);
                                            Ok(String::from("Proposal Not Valid"))
                                        },
                                        ProposalValidationResult::NotValidIncorrectNextBlockIndex => {
                                            Proposal::store_proposal(proposal.clone(), ProposalStatus::NotValidIncorrectNextBlockIndex);
                                            Ok(String::from("Proposal Not Valid - incorrect next block id"))
                                        },
                                        ProposalValidationResult::NotValidIncorrectProposalHash => {
                                            Proposal::store_proposal(proposal.clone(), ProposalStatus::NotValidIncorrectProposalHash);
                                            Ok(String::from("Proposal Not Valid - incorrect proposal hash"))
                                        }
                                    }
                                },
                                Err(_) => {
                                    println!("invoke_action(), ERROR, could not decide on proposal");
                                    Err(String::from("invoke_action(), ERROR, could not decide on proposal"))
                                }
                            }
                        },
                        Err(string) => {
                            let err_msg: &str = "Error: invoke_action, proposal_created: FAILED proposal decoding";
                            println!("{}", err_msg);
                            Err(String::from(err_msg))
                        }
                    }
                } else {
                    println!("invoke_action() - Error: could not decode proposal in proposal_created: {}", data);
                    Err(String::from("invoke_action() - Error: could not decode proposal in proposal_created"))
                }
            },

            /*
            @endpoint /proposal/response/
            @desc get responses to a proposal request from peers
            A <- B, B back to A
            */
            "/proposal/response/" => {
                println!("Proposal response received: {}", data);
                let decoded_proposal_string: Result<String, String> = Encoder::decode_base64( String::from(data) );
                if decoded_proposal_string.clone().is_ok() {
                    println!("invoke_action(), proposal_response - Success: Received a proposal RESPONDED by another node: {}::{}", data, decoded_proposal_string.clone().unwrap());
                    println!("Decoded Proposal String: {:?}", decoded_proposal_string);
                    let decoded_proposal: Result<Proposal, String> = Proposal::from_json_string(decoded_proposal_string.unwrap());
                    //TODO: check if we have a proposal with that id
                    let all_proposals: Option<Vec<Proposal>> = Proposal::get_last_n_proposals();
                    if decoded_proposal.is_ok() {
                        //SYNC CHECK

                        //TODO: search for proposal
                        //TODO: Breakout into Proposal::find_proposal
                        let found_proposal: Option<Proposal> = match all_proposals {
                            Some(proposals) => {
                                let mut same_proposal: Option<Proposal> = None;
                                for proposal in proposals {
                                    if Proposal::compare_without_status(proposal.clone(), decoded_proposal.clone().unwrap() ) {
                                        println!("/proposal/response/, proposals are equal");
                                        //if we have the proposal, check its status
                                        same_proposal = Some(proposal);
                                        // TODO: can safely break for proposal in proposals iteration
                                    } else {
                                        println!("/proposal/response/, proposals are NOT equal");
                                    }
                                }
                                same_proposal
                            },
                            None => {
                                None
                            }
                        };

                        //TODO: IF WE FOUND THE SUBMITTED PROPOSAL IN OUR LOCAL SET
                        if found_proposal.is_some() {
                            match found_proposal.clone().unwrap().proposal_status {
                                //TODO: IF WE CREATED THE PROPOSAL
                                //TODO: CHECK IF THIS NODE CREATED THE PROPOSALS
                                ProposalStatus::Created => {
                                    //TODO: update how many votes the proposal has
                                    //TODO CHECK IF THE AMOUNT OF VOTES IS ENOUGH TO SAY "ACCEPTED"
                                    match decoded_proposal.clone().unwrap().proposal_status {
                                        ProposalStatus::Accepted | ProposalStatus::AcceptedBroadcasted => {
                                            // TODO: set the proposal db
                                            Proposal::add_peer_status_to_proposal(found_proposal.clone().unwrap(),
                                                                                  ProposalStatus::Accepted,
                                                                                  request_origin);
                                            //TODO: WE CREATED IT AND WE JUST RECEIVED AN ACCEPTANCE
                                            //TODO: DO NOT SET TO ACCEPTED BY NETWORK HERE
                                        },
                                        ProposalStatus::Rejected | ProposalStatus::RejectedBroadcasted => {
                                            Proposal::add_peer_status_to_proposal(found_proposal.clone().unwrap(),
                                                                                  ProposalStatus::Rejected,
                                                                                  request_origin);
                                            //TODO: WE CREATED IT AND WE JUST RECEIVED A REJECTION
                                            //TODO: DO NOT SET TO ACCEPTED BY NETWORK HERE
                                            //Proposal::update_proposal(found_proposal.clone().unwrap(), "rejected_by_network");
                                        },
                                        _ => {

                                        }
                                    }
                                    Ok(String::from("Proposal response: Successfully parsed"))
                                },
                                // TODO ProposalStatus::AcceptedBroadcasted
                                ProposalStatus::AcceptedByNetwork => {
                                    //TODO: update how many votes the proposal has
                                    //TODO CHECK IF THE AMOUNT OF VOTES IS ENOUGH TO SAY "ACCEPTED"
                                    match decoded_proposal.clone().unwrap().proposal_status {
                                        ProposalStatus::Accepted => {
                                            //TODO: WE CREATED IT AND WE JUST RECEIVED AN ACCEPTANCE
                                        },
                                        ProposalStatus::Rejected => {
                                            //TODO: WE CREATED IT AND WE JUST RECEIVED A REJECTION
                                        },
                                        _ => {

                                        }
                                    }
                                    Ok(String::from("Proposal response: Successfully parsed"))
                                },
                                ProposalStatus::Committed => {
                                    //TODO: update how many votes the proposal has
                                    //TODO CHECK IF THE AMOUNT OF VOTES IS ENOUGH TO SAY "ACCEPTED"
                                    match decoded_proposal.clone().unwrap().proposal_status {
                                        ProposalStatus::Accepted => {
                                            //TODO: WE CREATED IT AND WE JUST RECEIVED AN ACCEPTANCE
                                        },
                                        ProposalStatus::Rejected => {
                                            //TODO: WE CREATED IT AND WE JUST RECEIVED A REJECTION
                                        },
                                        _ => {

                                        }
                                    }
                                    Ok(String::from("Proposal response: Successfully parsed"))
                                },
                                _ => {
                                    Err(String::from("Proposal response: Error: found_proposal is not the correct status! expected to be ProposalStatus::Created"))
                                }
                            }
                        } else {
                            Err(String::from("Proposal response: Error: all_proposals option was none"))
                        }
                    } else {
                        Err(String::from("Proposal response: Error: decoded_proposal is NOT OK"))
                    }

                } else {
                    println!("invoke_action() - Error: could not decode proposal in proposal_response: {}", data);
                    Err(String::from(""))
                }
                //TODO 1: check DB for proposal ID, and status
                //TODO 2: store response in DB
                //TODO 3: if responses proposal is valid
                    //verify if the proposal's response received completes "round"
            },

            /*
            @endpoint /proposal/resolution/
            @desc notify peers that you have commited
                  to a resolution.
            */
            "/proposal/resolution/" => {
                println!("Resolution received: {}", data);
                //TODO: resolve only if our consensus goal is met
                let decoded_proposal_string: Result<String, String> = Encoder::decode_base64(String::from(data));
                if decoded_proposal_string.clone().is_ok() {
                    println!("invoke_action(), proposal_resolution - Success: Received a proposal RESOLUTION by another node: {}::{}", data, decoded_proposal_string.clone().unwrap());
                    //TODO: check if we have a proposal with that id
                    let decoded_proposal: Result<Proposal, String> = Proposal::from_json_string(decoded_proposal_string.unwrap());
                    let all_proposals: Option<Vec<Proposal>> = Proposal::get_last_n_proposals();
                    if decoded_proposal.is_ok() {
                        //TODO: search for proposal
                        //TODO: Breakout into Proposal::find_proposal
                        let found_proposal: Option<Proposal> = match all_proposals {
                            Some(proposals) => {
                                let mut same_proposal: Option<Proposal> = None;
                                for proposal in proposals {
                                    if Proposal::compare_without_status(proposal.clone(), decoded_proposal.clone().unwrap() ) {
                                        println!("/proposal/resolution/, proposals are equal");
                                        same_proposal = Some(proposal);
                                        // TODO: can safely break for proposal in proposals iteration
                                    } else {
                                        println!("/proposal/resolution/, proposals are NOT equal");
                                    }
                                }
                                same_proposal
                            },
                            None => {
                                None
                            }
                        };
                        //TODO: IF WE FOUND THE SUBMITTED PROPOSAL IN OUR LOCAL SET
                        //TODO: maybe change to same_proposal.is_some()?
                        if found_proposal.is_some() {

                            println!("invoke_action(), proposal_resolution - WE FOUND A LOCAL PROPOSAL MATCH");
                            match found_proposal.clone().unwrap().proposal_status {
                                //TODO: CHECK IF THIS NODE ACCEPTED THE PROPOSAL AND BROADCASTED ALREADY
                                ProposalStatus::AcceptedBroadcasted => {
                                    println!("invoke_action(), proposal_resolution - FOUND PROPOSAL STATUS IS ACCEPTEDBROADCASTED");
                                    match Proposal::validate_proposal_resolution(found_proposal.clone().unwrap(), decoded_proposal.clone().unwrap()){
                                        Ok(_) => {
                                            //TODO CHECK IF THE AMOUNT OF VOTES IS ENOUGH TO SAY "COMMITTED"
                                            Proposal::update_proposal(found_proposal.clone().unwrap(),
                                            "committed");
                                            Ok(String::from("Proposal resolution: Successfully parsed"))
                                        },
                                        Err(_) => {
                                            Err(String::from("Proposal resolution ERROR: FAILED parsed"))
                                        }
                                    }
                                },
                                ProposalStatus::RejectedBroadcasted => {
                                    println!("Proposal resolution: PROPOSAL REJECTED BY ME, DO NOTHING");
                                    Ok(String::from("Proposal resolution: PROPOSAL REJECTED BY ME, DO NOTHING"))
                                },
                                _ => {
                                    Err(String::from("Proposal resolution: Error: found_proposal is not the correct status! expected to be ProposalStatus::AcceptedBroadcasted"))
                                }
                            }

                        } else {
                            Err(String::from("Proposal resolution: Error: all_proposals option was none"))
                        }
                    } else {
                        Err(String::from("Proposal resolution: Error: decoded_proposal is NOT OK"))
                    }
                } else {
                    println!("invoke_action() - Error: could not decode proposal in proposal_resolution: {}", data);
                    Err(String::from(""))
                }
            },

            /*
                @endpoint /state/get/
                @desc get the state from the DB
            */
            "/state/get/" => {
                println!("Get State from State db");
                Ok(String::from(""))
            },

            /*
            @endpoint /block/query/
            @desc when a node requests a specific block
            */
            "/block/query/" => {
                // TODO: another node asked for a block by its ID, respond with proposal with block id, and commited
                println!("block query received: {} | {} | {}", command, data, request_origin);
                let all_proposals: Option<Vec<Proposal>> = Proposal::get_last_n_proposals();
                //TODO: Breakout into Proposal::find_proposal
                let found_proposal: Option<Proposal> = match all_proposals {
                    Some(proposals) => {
                        let mut same_proposal: Option<Proposal> = None;
                        for proposal in proposals {
                            //TODO: data parse here could be broken out of line to be checked against
                            if proposal.clone().proposal_block.block_id == data.parse::<i64>().unwrap() {
                                println!("/block/query/, proposal.block_id matches requested block_id");
                                if proposal.clone().proposal_status == ProposalStatus::Committed {
                                    println!("/block/query/, proposal status IS INDEED COMMITED, RESPOND WITH IT!");
                                    same_proposal = Some(proposal);
                                    // TODO: can safely break for proposal in proposals iteration
                                } else {
                                    println!("/block/query/, ERROR proposal STATUS IS NOT COMMITED, DON'T RESPOND WITH IT");
                                }
                            } else {
                                println!("/block/query/, proposal.block_id does not match requested block_id");
                            }
                        }
                        same_proposal
                    },
                    None => {
                        None
                    }
                };
                if found_proposal.is_some() {
                    Self::broadcast_block_query_response(found_proposal.clone().unwrap(), request_origin);
                    Ok(String::from("RESPONDING TO BLOCK QUERY"))
                } else {
                    Err(String::from("ERROR RESPONDING TO BLOCK QUERY, FOUND_PROPOSAL IS NONE!"))
                }
            },

            /*
            @endpoint /block/response/
            @desc after a node requests a block, the block is sent to this endpoint in response

            */
            "/block/response/" => {
                // TODO: the response is a proposal, containing a block
                println!("Received Block from a peer AFTER QUERYING FOR IT");
                let decoded_proposal_string: Result<String, String> = Encoder::decode_base64(String::from(data));
                if decoded_proposal_string.clone().is_ok() {
                    println!("invoke_action(), block received AFTER QUERING FOR IT - Success: queryied for block: {}", data);
                    //TODO: check if we have a proposal with that id
                    let decoded_proposal: Result<Proposal, String> = Proposal::from_json_string(decoded_proposal_string.unwrap());
                    match decoded_proposal {
                        Ok(mut proposal) => {
                            if proposal.validate_proposal_block().is_ok() {
                                Ok(String::from(""))
                            } else {
                                Err(String::from("Block response, proposal.validate_proposal_block() FAILED"))
                            }
                        },
                        Err(msg) => {
                            println!("invoke_action(), ERROR, decoded_proposal FAILED ON BLOCK RESPONSE");
                             Err(String::from("invoke_action(), ERROR, decoded_proposal FAILED ON BLOCK RESPONSE"))
                        }
                    }
                } else {
                    Err(String::from("Block response, proposal.decode_base64() FAILED"))
                }
            },
            // default case
            _ => Err(String::from("API endpoint not correct"))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::{Server, PayloadParser, API};

    #[test]
    fn test_parse_payload_for_header() {
        let payload: String = String::from("HTTP/1.1 200 OK
                                           \r\nContent-Length: 10
                                           \r\nContent-Type: application/json; charset=UTF-8
                                           \r\nUser-Agent: example
                                           \r\n\r\nRESPONSE FROM NODE
                                           \r\n");
        let payload_split: Vec<&str> = payload.split("\n").collect();
        let data: Result<String, String> = Server::get_header_from_payload(payload_split, "user-agent");
        assert_eq!(data, Ok(String::from("example")));
    }


    #[test]
    fn test_parse_origin_for_header() {
        let payload: String = String::from("HTTP/1.1 200 OK
                                           \r\nContent-Length: 10
                                           \r\nContent-Type: application/json; charset=UTF-8
                                           \r\nUser-Agent: example
                                           \r\nOrigin: 127.0.0.1
                                           \r\n\r\nRESPONSE FROM NODE
                                           \r\n");
        let payload_split: Vec<&str> = payload.split("\n").collect();
        let data: Result<String, String> = Server::get_header_from_payload(payload_split, "origin");
        assert_eq!(data, Ok(String::from("127.0.0.1")));
    }
}
