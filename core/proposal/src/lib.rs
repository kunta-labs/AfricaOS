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

use db::{DB,
         DBWriteProposal,
         DBReadProposal,
         FileDirectoryReader};

use block::{Block,
            CreateNewBlock,
            BlockToJson,
            BlockFromString,
            JsonToBlock,
            ReadBlock,
            CommitBlock};

use timestamp::{Timestamp, NewTimestamp, StringToTimestamp};
use hash::{Hasher, CalculateSHA256Hash};

/*
@name Proposal
@desc proposal struct for info
*/
#[derive(Clone,Debug, PartialEq)]
pub struct Proposal {
    pub proposal_id: i32,
    pub proposal_status: ProposalStatus,
    pub proposal_hash: String,
    pub proposal_time: Timestamp,
    pub proposal_sender: String,
    pub proposal_block: Block
}

/*
@name ProposalStatus
@desc enum for the different proposal statuses
*/
#[derive(Clone,Debug,PartialEq)]
pub enum ProposalStatus {
    Pending,
    Created,
    Accepted,
    AcceptedBroadcasted,
    AcceptedByNetwork,
    Rejected,
    RejectedBroadcasted,
    RejectedByNetwork,
    Committed,
    NotValid,
    NotValidIncorrectNextBlockIndex,
    NotValidIncorrectProposalHash,
    ProposalStatusError
}

pub trait StatusToString {
    fn string_from_status(status: ProposalStatus) -> &'static str;
}

impl StatusToString for Proposal {
    fn string_from_status(status: ProposalStatus) -> &'static str {
        match status {
            ProposalStatus::Pending     => "pending",
            ProposalStatus::Created     => "created",
            ProposalStatus::Accepted    => "accepted",
            ProposalStatus::AcceptedBroadcasted    => "accepted_broadcasted",
            ProposalStatus::AcceptedByNetwork    => "accepted_by_network",
            ProposalStatus::Rejected    => "rejected",
            ProposalStatus::RejectedBroadcasted    => "rejected_broadcasted",
            ProposalStatus::RejectedByNetwork    => "rejected_by_network",
            ProposalStatus::Committed    => "committed",
            ProposalStatus::NotValid    => "notvalid",
            ProposalStatus::NotValidIncorrectNextBlockIndex => "not_valid_incorrect_next_block_id",
            ProposalStatus::NotValidIncorrectProposalHash => "not_valid_incorrect_proposal_hash",
            _ => "proposal_status_error"
        }
    }
}

/*
@name StringToStatus
@desc convert to status from string
*/
pub trait StringToStatus {
    fn status_from_string(string_representation: &str) -> ProposalStatus;
}

/*
@name status_from_string
@desc
*/
impl StringToStatus for Proposal {
    fn status_from_string(string_representation: &str) -> ProposalStatus {
        match string_representation {
            "pending" =>    ProposalStatus::Pending,
            "created" =>    ProposalStatus::Created,
            "accepted" =>   ProposalStatus::Accepted,
            "accepted_broadcasted" =>   ProposalStatus::AcceptedBroadcasted,
            "accepted_by_network" =>   ProposalStatus::AcceptedByNetwork,
            "rejected" =>   ProposalStatus::Rejected,
            "rejected_broadcasted" =>   ProposalStatus::RejectedBroadcasted,
            "rejected_by_network" =>   ProposalStatus::RejectedByNetwork,
            "committed" =>   ProposalStatus::Committed,
            "notvalid" =>            ProposalStatus::NotValid,
            "not_valid_incorrect_next_block_id" => ProposalStatus::NotValidIncorrectNextBlockIndex,
            "not_valid_incorrect_proposal_hash" => ProposalStatus::NotValidIncorrectProposalHash,
            _ =>            ProposalStatus::ProposalStatusError
        }
    }
}

/*
@name JsonConverter
@desc
*/
pub trait JsonConverter {
    /*
    @name to_json
    @desc convert proposal into JSON String
    */
    fn to_json(proposal: Proposal) -> String;

    /*
    @name from_json
    @desc create a proposal from a JSONValue
    */
    fn from_json(payload: JsonValue) -> Result<Proposal, String>;

    /*
    @name from_json_string
    @desc create a proposal from a JSON string
    */
    fn from_json_string(json_string: String) -> Result<Proposal, String>;
}

/*
@name JsonConverter for Proposal
@desc
*/
impl JsonConverter for Proposal {
    fn to_json(proposal: Proposal) -> String {
        let data = object!{
            "proposal_id" => proposal.proposal_id,
            "proposal_status" => Self::string_from_status(proposal.proposal_status),
            "proposal_hash" => proposal.proposal_hash,
            "proposal_time" => proposal.proposal_time.timestamp,
            "proposal_sender" => proposal.proposal_sender,
            "proposal_block" => Block::to_json(proposal.proposal_block),
        };
        println!("Proposal, to_json, data.dump(): {}", data.dump());
        String::from(data.dump())
    }

    fn from_json(payload: JsonValue) -> Result<Proposal, String> {
        println!("From JSON: {}", payload);
        let proposal_id_from_json_option: Option<i32> = payload["proposal_id"].as_i32();
        match proposal_id_from_json_option {
            Some(proposal_id_from_json) => {
                let proposal_status_from_json: ProposalStatus = Self::status_from_string( payload["proposal_status"].as_str().unwrap() );
                let proposal_hash: String = payload["proposal_hash"].to_string();
                let unchecked_proposal_timestamp: String = payload["proposal_time"].to_string();
                let proposal_time: Option<Timestamp> = Timestamp::string_to_timestamp(unchecked_proposal_timestamp);
                if proposal_time.is_some() {
                    let proposal_sender: String = payload["proposal_sender"].to_string();
                    let block_string_to_check: Result<Block, String> = Block::from_string( payload["proposal_block"].to_string() );
                    match block_string_to_check {
                        Ok(block) => {
                            let proposal_block: Block = block;
                            Ok(Proposal {
                                proposal_id: proposal_id_from_json,
                                proposal_status: proposal_status_from_json,
                                proposal_hash: proposal_hash,
                                proposal_time: proposal_time.unwrap(),
                                proposal_sender: proposal_sender,
                                proposal_block: proposal_block
                            })
                        },
                        Err(err) => {
                            Err(String::from(format!("Proposal, ERROR: from_json, Block Format is invalid: {}", err)))
                        }
                    }
                } else {
                    Err(String::from("Proposal, ERROR: from_json, proposal_time is invalid"))
                }
            },
            None => {
                Err(String::from("Proposal, ERROR: from_json, proposal_id could not be parsed as i32!"))
            }
        }

    }

    fn from_json_string(json_string: String) -> Result<Proposal, String> {
        let parsed = json::parse( &format!(r#"{}"#, json_string) ).unwrap();
        Self::from_json(parsed)
    }

}


/*
@name CreateProposalIndex
@desc
*/
pub trait CreateProposalIndex {
    fn create_proposal_index() -> ();
}

/*
@name CreateProposalIndex
@desc to initially create the proposal index
*/
impl CreateProposalIndex for Proposal {
    fn create_proposal_index() -> (){
        let new_proposal_index = object!{
            "proposals" => object!{}
        };
        let index_to_write: String = json::stringify(new_proposal_index);
        match DB::write_proposal_index(index_to_write) {
            Ok(_) => {
                println!("Successfully wrote proposal index");
            },
            Err(_) => {
                println!("Failure writing proposal index");
            }
        }
    }
}


/*
@name UpdateProposalInDB
*/
pub trait UpdateProposalInDB {
    /*
        @name add_peer_status_to_proposal
        @desc add a key to the proposal DB
    */
    fn add_peer_status_to_proposal(proposal: Proposal, status: ProposalStatus, peer: String) -> Result<String, String>;

    /*
        @name update_proposal
        @desc
    */
    fn update_proposal(proposal: Proposal, status: &str) -> Result<String,String> ;
}

/*
@name UpdateProposalInDB
@desc
*/
impl UpdateProposalInDB for DB {

    fn add_peer_status_to_proposal(proposal: Proposal,
                                   status: ProposalStatus,
                                   peer: String) -> Result<String, String> {

        println!("Inside add_node_status_to_proposal_json");
        let mut proposal_index_option: Option<JsonValue> = Self::get_proposal_index_as_json();
        match proposal_index_option {
            Some(mut proposal_index) => {
                let all_proposals = &mut proposal_index["proposals"];
                let stringed_status = Proposal::string_from_status(status);
                all_proposals[proposal.proposal_id.to_string()][peer] = JsonValue::from(stringed_status);
                let proposal_write_result: Result<String,String> = match Self::write_proposal_index(proposal_index.dump()) {
                    Ok(result) => {
                        Ok(result)
                    },
                    Err(err) => {
                        Err( String::from("add_peer_status_to_proposal ERROR: Writing proposal index failed") )
                    }
                };
                proposal_write_result
            },
            None => {
                Err(String::from("proposal_index_option is NONE"))
            }
        }
    }

    fn update_proposal(proposal: Proposal, status: &str) -> Result<String,String> {
        println!("Inside update proposal");
        let mut proposal_index_option: Option<JsonValue> = Self::get_proposal_index_as_json();
        match proposal_index_option {
            Some(mut proposal_index) => {
                let all_proposals = &proposal_index["proposals"];
                let new_proposal_status: ProposalStatus = Proposal::status_from_string( status.clone() );
                let altered_proposal_block: Result<Block, String> = Block::from_json(all_proposals[ proposal.proposal_id.to_string() ]["proposal_block"].clone());
                match altered_proposal_block {
                    Ok(block) => {
                        let altered_proposal: Proposal = Proposal {
                            proposal_id: all_proposals[proposal.proposal_id.to_string()]["proposal_id"].as_i32().unwrap(),
                            proposal_status: new_proposal_status.clone(),
                            proposal_hash: String::from( all_proposals[proposal.proposal_id.to_string()]["proposal_hash"].as_str().unwrap() ),
                            proposal_time: Timestamp::string_to_timestamp(String::from(all_proposals[proposal.proposal_id.to_string()]["proposal_time"].as_str().unwrap())).unwrap(),
                            proposal_sender: String::from( all_proposals[proposal.proposal_id.to_string()]["proposal_sender"].as_str().unwrap() ),
                            proposal_block: block
                        };
                        let parsed = json::parse( &format!(r#"{}"#, Proposal::to_json(altered_proposal.clone()) ) );
                        if parsed.is_ok() {
                            proposal_index
                            ["proposals"]
                            [proposal.proposal_id.to_string()]
                            ["proposal_status"] = JsonValue::from(status);
                            let proposal_write_result: Result<String,String> = match Self::write_proposal_index(proposal_index.dump()) {
                                Ok(result) => {
                                    match Self::write_proposal(altered_proposal.clone(), new_proposal_status.clone()){
                                        Ok(result) => {
                                            Ok( String::from("update_proposal SUCCESS: Successful write of proposal") )
                                        },
                                        Err(_) => {
                                            Err( String::from("update_proposal ERROR: Writing to disk failed") )
                                        }
                                    }
                                },
                                Err(err) => {
                                    Err( String::from("update_proposal ERROR: Writing proposal index failed") )
                                }
                            };
                            proposal_write_result
                        } else {
                            println!("update_proposal ERROR: parsed is not okay");
                            Err( String::from("update_proposal ERROR: Could not parse proposal into JSON") )
                        }
                    }
                    Err(err) => {
                        Err(String::from("update_proposal ERROR: Block::from_json failed/is invalid"))
                    }
                }
            },
            None => {
                Err(String::from("update_proposal ERROR, proposal_index_option is NONE"))
            }
        }
    }
}

/*
@name ReadProposalFromDB
@desc trait to read a proposal from a db
*/
pub trait ReadProposalFromDB {
    fn get_proposal_index_as_json() -> Option<JsonValue>;
    fn get_latest_proposal() -> Option<Proposal>;
    fn get_all_proposals() -> Option<Vec<Proposal>>;
    fn get_last_n_proposals() -> Option<Vec<Proposal>>;
}

/*
@name ReadProposalFromDB for DB
@desc
*/
impl ReadProposalFromDB for DB {
    /*
    @name get_proposal_index_as_json
    @desc return the proposal index as a json object
    */
    fn get_proposal_index_as_json() -> Option<JsonValue> {
        let proposal_index: String = match DB::read_proposal_index() {
            Some(i) => {
                i
            },
            None => String::from("NO INDEX")
        };
        println!("Proposal index: {}", proposal_index);
        let parsed_result: Result<JsonValue, json::Error> = json::parse( &format!(r#"{}"#, proposal_index) );
        match parsed_result {
            Ok(parsed) => {
                println!("proposal index parsed: {}", parsed["proposals"]);
                println!("PI parse example 0 {}", parsed["proposals"]["0"]);
                Some(parsed)
            },
            Err(_) => {
                None
            }
        }
    }

    /*
    @name get_latest_proposal
    @desc get the latest proposal
    */
    fn get_latest_proposal() -> Option<Proposal> {
        let last_n_proposals_option: Option<Vec<Proposal>> = Self::get_last_n_proposals();
        match last_n_proposals_option {
            Some(mut last_n_proposals) => {
                last_n_proposals.pop()
            },
            None => {
                None
            }
        }
    }

    /*
    @name get_all_proposals
    @desc get all proposals from the proposals directory
    */
    fn get_all_proposals() -> Option<Vec<Proposal>> {
        let parsed_option: Option<JsonValue> = Self::get_proposal_index_as_json();
        match parsed_option {
            Some(parsed) => {
                let mut all_proposals_vector: Vec<Proposal> = Vec::new();
                let proposals_iter = parsed["proposals"].entries();
                for (id, proposal) in proposals_iter {
                    let parsed_proposal: Result<Proposal, String> = Proposal::from_json(proposal.clone());
                    match parsed_proposal {
                        Ok(proposal) => {
                            all_proposals_vector.push(proposal);
                        },
                        Err(err) => {
                            println!("get_all_proposals ERROR parsed_proposal: {:?}", err);
                        }
                    }
                }
                Some(all_proposals_vector)
            },
            None => {
                None
            }
        }
    }

    /*
    @name get_last_n_proposals
    @desc get last n proposal from the proposals directory
    */
    fn get_last_n_proposals() -> Option<Vec<Proposal>> {
        let proposal_index_option: Option<JsonValue> = Self::get_proposal_index_as_json();
        match proposal_index_option {
            Some(proposal_index) => {
                let next_proposal_id_option: Option<i32> = Proposal::get_next_proposal_id();
                match next_proposal_id_option {
                    Some(next_proposal_id) => {
                        let mut all_proposals_vector: Vec<Proposal> = Vec::new();
                        let mut highest_proposal_to_fetch: i32 = ( format!("{}", proposal_index["proposals"].clone().len() ).parse::<i32>().unwrap() );//next_proposal_id + 5;
                        let mut furthest_proposal_to_fetch: i32 = highest_proposal_to_fetch - 5;//next_proposal_id;
                        if furthest_proposal_to_fetch < 0 {
                            furthest_proposal_to_fetch = 0;
                        } else {}
                        println!("further_proposal_to_fetch: {} - {} - {}", furthest_proposal_to_fetch,
                                                                        highest_proposal_to_fetch,
                                                                        proposal_index.to_string());
                        for proposal_id in furthest_proposal_to_fetch..highest_proposal_to_fetch {
                            let stringed_proposal_id: String = format!("{}", proposal_id);
                            let json_proposal_representation: JsonValue = proposal_index["proposals"][ stringed_proposal_id.as_str() ].clone();
                            println!("json_proposal_representation: {} : {}", json_proposal_representation, proposal_id);
                            let converted_proposal: Result<Proposal, String> = Proposal::from_json(json_proposal_representation);
                            match converted_proposal {
                                Ok(proposal_result) => {
                                    all_proposals_vector.push(proposal_result);
                                },
                                Err(_) => {}
                            }
                        }
                        Some(all_proposals_vector)
                    },
                    None => None
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
pub trait WriteProposalToDB {
    fn write_proposal(proposal: Proposal, new_status: ProposalStatus) -> Result<String,std::io::Error>;
}

/*
@name WriteProposalToDB
@desc implementation for Writing Proposals To DB for DB
*/
impl WriteProposalToDB for DB {
    /*
    @name write_proposal
    @desc write proposal to DB
    */
    fn write_proposal(mut proposal: Proposal, new_status: ProposalStatus) -> Result<String,std::io::Error> {
        println!("inside write_proposal new_status: ProposalStatusn Proposal, DB trait");
        let parsed_option: Option<JsonValue> = Self::get_proposal_index_as_json();
        match parsed_option {
            Some(mut parsed) => {
                proposal.proposal_status = new_status.clone();
                let proposal_string: String = Proposal::to_json(proposal.clone());
                if parsed.has_key( &(format!("{}", proposal.proposal_id).to_string()) ) {
                    let stringed_status = Proposal::string_from_status(new_status);
                    parsed["proposals"]
                          [&(format!("{}", proposal.proposal_id).to_string())]
                          ["proposal_status"] = JsonValue::from(stringed_status);
                    println!("write_proposal, UPDATE Proposal JSON: {}", parsed.dump());
                    let db_index_write_result: Result<String, Error> = Self::write_proposal_index(parsed.dump());
                    match db_index_write_result {
                        Ok(result) => {
                            let proposal_string: String = Proposal::to_json(proposal.clone());
                            let db_write_result: Result<String, std::io::Error> = Self::write_proposal_to_sql(proposal.proposal_id, proposal_string.clone());
                            match db_write_result {
                                Ok(r) => {
                                    Ok(r)
                                },
                                Err(err) => {
                                    Err(err)
                                }
                            }
                        },
                        Err(err) => {
                            Err(err)
                        }
                    }
                } else {
                    let new_proposal_entry = object!{
                        "proposal_id" => proposal.proposal_id,
                        "proposal_status" => Proposal::string_from_status(new_status),
                        "proposal_hash" => proposal.proposal_hash,
                        "proposal_time" => proposal.proposal_time.timestamp,
                        "proposal_sender" => proposal.proposal_sender,
                        "proposal_block" => Block::to_json(proposal.proposal_block)
                    };
                    let pindex_insert_result: Result<String, Error> = match parsed["proposals"]
                          .insert( &(format!("{}", proposal.proposal_id).to_string() ),
                                   new_proposal_entry) {
                        Ok(_) => {
                            println!("New Proposal JSON: {}", parsed.dump());
                            let db_index_write_result = Self::write_proposal_index(parsed.dump());
                            if db_index_write_result.is_ok() {
                                let db_write_result: Result<String, std::io::Error> = Self::write_proposal_to_sql(proposal.proposal_id, proposal_string.clone());
                                db_write_result
                            } else {
                                let proposal_db_write_error = Error::new(ErrorKind::Other, "Couldn't write Proposal to DB");
                                Err(proposal_db_write_error)
                            }
                        },
                        Err(r) => {
                            println!("Failed adding new Proposal to Proposal_index: {}", parsed.dump());
                            let proposal_index_insert_error = Error::new(ErrorKind::Other, "Could not add proposal to proposal_index");
                            Err(proposal_index_insert_error)
                        }
                    };
                    pindex_insert_result
                }
            },
            None => {
                let get_proposal_index_as_json_error = Error::new(ErrorKind::Other, "write_proposal() [Error], Problem getting proposal index as json");
                Err(get_proposal_index_as_json_error)
            }
        }
    }
}

/*
@name WriteNewProposalToDB
@desc trait to write a new proposal to DB
*/
trait WriteNewProposalToDB {
    fn write_new_proposal(proposal: Proposal) -> Result<String,std::io::Error>;
}

/*
@name WriteNewProposalToDB for Proposal
@desc implementation to write a new proposal to the DB
*/
impl WriteNewProposalToDB for Proposal {
    fn write_new_proposal(proposal: Proposal) -> Result<String,std::io::Error> {
        DB::write_proposal(proposal, ProposalStatus::Pending)
    }
}

/*
@name StoreProposal
@desc trait to store a proposal to DB trait
*/
pub trait StoreProposal {
    fn store_proposal(proposal: Proposal, proposal_status: ProposalStatus) -> Result<String,std::io::Error>;
}

/*
@name StoreProposal for Proposal
@desc implementation to store a proposal
*/
impl StoreProposal for Proposal {
    fn store_proposal(proposal: Proposal, proposal_status: ProposalStatus) -> Result<String,std::io::Error> {
        DB::write_proposal(proposal, proposal_status)
    }
}

/*
    @name GetProposals
    @desc get proposals without exposing the DB struct
*/
pub trait GetProposals {
    fn get_all_proposals() -> Option<Vec<Proposal>>;
    fn get_last_n_proposals() -> Option<Vec<Proposal>>;
    fn get_latest_proposal() -> Option<Proposal>;
}

impl GetProposals for Proposal {
    fn get_all_proposals() -> Option<Vec<Proposal>> {
        DB::get_all_proposals()
    }

    fn get_last_n_proposals() -> Option<Vec<Proposal>> {
        DB::get_last_n_proposals()
    }

    fn get_latest_proposal() -> Option<Proposal>{
        DB::get_latest_proposal()
    }

}

/*
    @name UpdateProposal
    @desc update proposals without exposing the DB struct
*/
pub trait UpdateProposal {
    /*
        @name add_peer_status_to_proposal
        @desc add a key to the proposal DB
    */
    fn add_peer_status_to_proposal(proposal: Proposal, status: ProposalStatus, peer: String) -> Result<String, String>;

    fn update_proposal(proposal: Proposal, status: &str) -> Result<String,String> ;
}

impl UpdateProposal for Proposal {
    fn add_peer_status_to_proposal(proposal: Proposal, status: ProposalStatus, peer: String) -> Result<String, String> {
        DB::add_peer_status_to_proposal(proposal, status, peer )
    }

    fn update_proposal(proposal: Proposal, status: &str) -> Result<String,String>  {
        DB::update_proposal(proposal, status)
    }
}


/*
    @name HashProposal
    @desc
*/
trait HashProposal {
    fn hash_proposal(calculated_proposal_id: i32, new_proposal_sender: String, ts: Timestamp) -> String;
}

impl HashProposal for Proposal {
    fn hash_proposal(calculated_proposal_id: i32, new_proposal_sender: String, ts: Timestamp) -> String {
        let raw_str: String = format!("{}{}{}", calculated_proposal_id, new_proposal_sender, ts.timestamp);
        let str_to_hash: &str = raw_str.as_str();
        let string_to_hash: String = String::from( str_to_hash );
        let new_proposal_hash: String = Hasher::calculate_sha256( string_to_hash );
        new_proposal_hash
    }
}

/*
@name NewProposal
@desc trait to create a new proposal
*/
pub trait NewProposal {
    fn create(request_origin: String) -> Option<Proposal>;
}

/*
@name NewProposal for Proposal
@desc create a new proposal
*/
impl NewProposal for Proposal {
    /*
    @name advance
    @desc ping all peers
    */
    fn create(request_origin: String) -> Option<Proposal> {
        println!("Creating New Proposal...");
        let new_proposal_id:i32 = match Self::get_next_proposal_id(){
            Some(pid) => pid,
            None => -1
        };
        println!("New Proposal ID for Newly Created Proposal: {}", new_proposal_id);
        let calculated_proposal_id: i32 = new_proposal_id + 1;
        let new_proposal_status: ProposalStatus = ProposalStatus::Pending;
        let new_proposal_timestamp: Option<Timestamp> = Timestamp::new();
        match new_proposal_timestamp {
            Some(ts) => {
                let new_proposal_sender: String = request_origin;
                let new_proposal_hash: String = Self::hash_proposal(calculated_proposal_id.clone(), new_proposal_sender.clone(), ts.clone());
                let new_proposal_block: Result<Block, String> = Block::new(new_proposal_hash.clone());
                match new_proposal_block {
                    Ok(block) => {
                        let new_proposal: Proposal = Proposal {
                            proposal_id: calculated_proposal_id,
                            proposal_status: new_proposal_status,
                            proposal_hash: new_proposal_hash,
                            proposal_time: ts,
                            proposal_sender: new_proposal_sender,
                            proposal_block: block
                        };
                        Self::write_new_proposal(new_proposal.clone()).unwrap();
                        Some(new_proposal)
                    },
                    Err(_) => {
                        println!();
                        None
                    }
                }
            },
            None => None
        }
    }
}


/*
@name ProposalIDGenerator
@desc trait f generating a next proposal_id
*/
trait ProposalIDGenerator {
    fn parse_filename_for_proposal_id(filename: &str) -> Option<i32>;
    fn get_next_proposal_id() -> Option<i32>;
}

impl ProposalIDGenerator for Proposal {
    /*
    @name parse_filename_for_proposal_id
    @desc parse a i32 from a string proposal filename
    */
    fn parse_filename_for_proposal_id(filename: &str) -> Option<i32> {
        let filename_proposal_id:Vec<&str> = filename.split("_").collect::<Vec<_>>();
        match filename_proposal_id[0] {
            "proposal" => {
                if filename_proposal_id.len() == 2 {
                    let proposal_filename_section: &str = filename_proposal_id[1];
                    let last_split_section: Vec<&str> = proposal_filename_section.split(".").collect::<Vec<_>>();
                    let filename_proposal_id: i32 = last_split_section[0].parse::<i32>().unwrap();
                    println!("last_split_section: {}", filename_proposal_id);
                    Some(filename_proposal_id)
                }else{
                    println!("no proposal_id in filename, Proposal Filename length is != 2: {}", filename_proposal_id.len());
                    None
                }
            },
            _ => None
        }
    }

    /*
    @name get_next_proposal_id
    @desc generate the next proposal_id from all proposals on disk
    */
    fn get_next_proposal_id() -> Option<i32> {
        let files:Vec<String> = DB::read_proposals_directory();
        let mut iter = (&files).into_iter();
        let mut highest_proposal_index: i32 = -1;
        while let Some(v) = iter.next(){
            println!("Filename Iter: {}", v);
            let filename_split_vector = v.split("/").collect::<Vec<_>>();
            let last_split_section: &str = filename_split_vector[filename_split_vector.len() - 1];
            let parsed_proposal_id: Option<i32> = Self::parse_filename_for_proposal_id(last_split_section);
            match parsed_proposal_id {
                Some(pid) => {
                    if pid > highest_proposal_index {
                        highest_proposal_index = pid;
                    }
                },
                None => ()
            }
        }
        println!("highest_proposal_index: {}", highest_proposal_index);
        match highest_proposal_index {
            -1 => None,
            _ => Some(highest_proposal_index),

        }
    }
}

#[derive(Clone,Debug,PartialEq)]
pub enum ProposalValidationResult {
    Valid,
    NotValid,
    NotValidIncorrectNextBlockIndex,
    NotValidIncorrectProposalHash
}

pub trait ProposalValidator {
    fn is_accepted_broadcasted_already(submitted_proposal: Proposal) -> bool;
    fn validate_proposal(submitted_proposal: Proposal) -> Result<ProposalValidationResult, std::io::Error>;
}

impl ProposalValidator for Proposal {

    fn is_accepted_broadcasted_already(submitted_proposal: Proposal) -> bool {
        let all_proposals: Option<Vec<Proposal>> = Proposal::get_last_n_proposals();
        let already_commited: Option<Proposal> = match all_proposals {
            Some(proposals) => {
                let mut commited_proposal: Option<Proposal> = None;
                for proposal in proposals {
                    if proposal.clone().proposal_block.block_id == submitted_proposal.proposal_block.block_id {
                        println!("is_commited_already, proposal.block_id matches submitted block_id");
                        if
                           proposal.clone().proposal_status == ProposalStatus::AcceptedBroadcasted
                           ||
                           proposal.clone().proposal_status == ProposalStatus::AcceptedByNetwork
                           ||
                           proposal.clone().proposal_status == ProposalStatus::Committed
                           ||
                           proposal.clone().proposal_status == ProposalStatus::Accepted
                           {
                            println!("is_commited_already, proposal status IS INDEED AcceptedBroadcasted, DO NOT VALIDATE ANOTHER VERSION");
                            commited_proposal = Some(proposal);
                        } else {
                            println!("is_commited_already, ERROR proposal STATUS IS NOT COMMITED, DON'T RESPOND WITH IT");
                        }
                    } else {
                        println!("is_commited_already, proposal.block_id does not match requested block_id");
                    }
                }
                commited_proposal
            },
            None => {
                None
            }
        };

        if already_commited.is_some() {
            true
        } else {
            false
        }
    }

    //NOTE: validate proposal is called from the "created endpoint"
    // this should only be invoked from one node for a given block
    fn validate_proposal(submitted_proposal: Proposal) -> Result<ProposalValidationResult, std::io::Error> {
        println!("validate_proposal(), Submitted Proposal: {}", submitted_proposal.proposal_id);
        if Self::is_accepted_broadcasted_already(submitted_proposal.clone()) {
            let is_commited_already_error = Error::new(ErrorKind::Other, "validate_proposal() [ERROR] WE COMMITED THE BLOCK ALREADY, PROBABLY AWAITING RESOLUTION");
            return Err(is_commited_already_error)
        } else {
            let proposal_index_parsed_option: Option<JsonValue> = DB::get_proposal_index_as_json();
            let proposal_index_parsed = match proposal_index_parsed_option {
                Some(p) => p,
                None => {
                    let get_proposal_index_as_json_error = Error::new(ErrorKind::Other, "validate_proposal() [ERROR] Problem getting proposal index as JSON");
                    return Err(get_proposal_index_as_json_error)
                }
            };
            let all_proposals = &proposal_index_parsed["proposals"];
            let proposal_id_string: String = format!("{}", submitted_proposal.proposal_id);
            let proposal_id_check = &all_proposals[proposal_id_string.clone()];
            if !proposal_id_check.has_key( proposal_id_string.clone().as_str() ) {
                    let current_block_id: Option<i64> = Block::get_latest_block_id();
                    let current_block_id_result: i64 = match current_block_id {
                        Some(block_id) => {
                            println!("validate_proposal(), current_block_id, block_id: {}", block_id);
                            block_id
                        },
                        None => {
                            -1
                        }
                    };
                    if ( (current_block_id_result + 1) == submitted_proposal.proposal_block.block_id ) {

                    } else {
                        return Ok(ProposalValidationResult::NotValidIncorrectNextBlockIndex)
                    }

                    let string_to_hash: String = String::from( format!("{}{}{}", submitted_proposal.proposal_id,
                                                                                 submitted_proposal.proposal_sender,
                                                                                 submitted_proposal.proposal_time.timestamp).as_str() ) ;

                    let expected_hash: String = submitted_proposal.proposal_hash;
                    let submitted_proposal_hash: String = Hasher::calculate_sha256( string_to_hash );

                    match submitted_proposal_hash {
                        _ if submitted_proposal_hash == expected_hash => {
                            println!("HASH SUCCESS: proposal hash IS CORRECT: {}{}", expected_hash, submitted_proposal_hash);
                        },
                        _ => {
                            println!("ERROR: proposal hash not valid: {}{}", expected_hash, submitted_proposal_hash);
                            return Ok(ProposalValidationResult::NotValidIncorrectProposalHash)
                        }
                    }
            } else {

            }

            let proposal_validation_error = Error::new(ErrorKind::Other, "Couldn't validate proposal");
            Ok(ProposalValidationResult::Valid)
        }
    }
}


/*
    @name CompareWithoutStatus
    @desc without a status, compare two proposals
*/
pub trait CompareWithoutStatus {
    fn compare_without_status(proposal_left: Proposal, proposal_right: Proposal) -> bool;
}

impl CompareWithoutStatus for Proposal {
    fn compare_without_status(proposal_left: Proposal, proposal_right: Proposal) -> bool {
        match proposal_left.proposal_id == proposal_right.proposal_id {
            true => {

            },
            false => {
                return false
            }
        }

        match proposal_left.proposal_hash == proposal_right.proposal_hash {
            true => {

            },
            false => {
                return false
            }
        }

        match proposal_left.proposal_time == proposal_right.proposal_time {
            true => {

            },
            false => {
                return false
            }
        }

        match proposal_left.proposal_sender == proposal_right.proposal_sender {
            true => {

            },
            false => {
                return false
            }
        }

        match proposal_left.proposal_block == proposal_right.proposal_block {
            true => {

            },
            false => {
                return false
            }
        }

        true

    }
}

/*
    @name ValidateProposalBlock
    @desc validate whether a successfully network-accepted proposal has a validate block to commit
*/
pub trait ValidateProposalBlock {
    fn validate_proposal_block(&mut self) -> Result<(), String>;
}

impl ValidateProposalBlock for Proposal {
    fn validate_proposal_block(&mut self) -> Result<(), String> {
        println!("validate_proposal_block: check if we commited already");
        Block::commit_if_valid(self.clone().proposal_block)
    }
}



/*
    @desc upon a resolution proposal received, provided it is accepted-broadcast,
    attempt to finalize the proposal, this includes commiting it
*/
pub trait ProposalResolutionAccepted {
    /*
        @name proposal_resolution_decision
        @desc check if the received proposal was accepted_by_network,or not... for resolution purposes
    */
    fn validate_proposal_resolution(local_proposal: Proposal, received_proposal: Proposal) -> Result<(), ()>;
}

impl ProposalResolutionAccepted for Proposal {
    fn validate_proposal_resolution(local_proposal: Proposal, received_proposal: Proposal) -> Result<(), ()> {
        match received_proposal.proposal_status {
            ProposalStatus::AcceptedByNetwork => {
                println!("invoke_action(), proposal_resolution - received_proposal STATUS IS AcceptedByNetwork");
                if received_proposal
                   .clone()
                   .validate_proposal_block()
                   .is_ok() {
                       println!("invoke_action(), proposal_resolution - validate_proposal_block SUCCESS");
                       Ok(())
                } else {
                       println!("invoke_action(), proposal_resolution - validate_proposal_block FAILED");
                       Err(())
                }
            },

            ProposalStatus::RejectedByNetwork => {
                println!("invoke_action(), proposal_resolution - decoded_proposal STATUS IS RejectedByNetwork");
                Err(())
            },


            ProposalStatus::Committed => {
                println!("invoke_action(), proposal_resolution - received_proposal STATUS IS Commited");
                if received_proposal
                   .clone()
                   .validate_proposal_block()
                   .is_ok() {
                       println!("invoke_action(), proposal_resolution - validate_proposal_block SUCCESS");
                       Ok(())
                } else {
                       println!("invoke_action(), proposal_resolution - validate_proposal_block FAILED");
                       Err(())
                }
            },
            _ => {
                Err(())
            }
        }
    }
}

pub trait CalculateProposalCreatorID {
    /*
        @name calculate_next_proposal_creator_id
        @desc to determine which node is gets to create the next proposal:
        @example node_is is CONGRUENT to current_block_id % peer_set.len()
    */
    fn calculate_next_proposal_creator_id(peer_length: usize, latest_block_id: i64) -> i64;
}

impl CalculateProposalCreatorID for Proposal {
    fn calculate_next_proposal_creator_id(peer_length: usize, latest_block_id: i64) -> i64 {
        println!("calculate_next_proposal_creator_id: peer_length: {} latest_block_id: {}",
                 peer_length,
                 latest_block_id);
        return ( (latest_block_id + 1) % ( (peer_length + 1) as i64) ) + 1;

    }
}

#[cfg(test)]
mod tests {
    use super::{Proposal,
                ProposalStatus,
                JsonConverter,
                ProposalValidator,
                ProposalValidationResult,
                CalculateProposalCreatorID};

    use block::{Block, CreateNewBlock};
    use timestamp::{Timestamp, NewTimestamp};

    #[test]
    fn test_proposal_created_reception() {
        let starting_string: String = String::from("{\"proposal_id\": 0,
        \"proposal_status\": \"pending\",
        \"proposal_hash\": \"test hash\",
        \"proposal_time\": 0,
        \"proposal_block\": {
            \"block_id\": 0,
            \"block_hash\": \"test block hash\",
            \"block_parent_hash\": \"block parent hash\",
            \"block_time\": 0,
            \"proposal_hash\": \"proposal hash\",
            \"block_data\": \"block data\"
        }}");
        let proposal: Result<Proposal, String> = Proposal::from_json_string(starting_string);
        assert_eq!(proposal.unwrap().proposal_status, ProposalStatus::Pending);
    }

    #[test]
    fn test_calculate_next_proposal_creator_id_3_nodes_block_0() {
        let number_of_peers: usize = 2;
        let next_block_id: i64 = 0;
        assert_eq!(Proposal::calculate_next_proposal_creator_id(number_of_peers,
                                                                next_block_id), 1);
    }

    #[test]
    fn test_calculate_next_proposal_creator_id_3_nodes_block_1() {
        let number_of_peers: usize = 2;
        let next_block_id: i64 = 1;
        assert_eq!(Proposal::calculate_next_proposal_creator_id(number_of_peers,
                                                                next_block_id), 2);
    }

    #[test]
    fn test_calculate_next_proposal_creator_id_3_nodes_block_2() {
        let number_of_peers: usize = 2;
        let next_block_id: i64 = 2;
        assert_eq!(Proposal::calculate_next_proposal_creator_id(number_of_peers,
                                                                next_block_id), 3);
    }

}
