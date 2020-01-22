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

use network::{Server,Receiver,Transmitter};
use transaction::{Transaction};
use db::{DB, NodeNameSetter};

use proposal::{Proposal,
               NewProposal,
               ReadProposalFromDB,
               CreateProposalIndex,
               ProposalStatus,
               UpdateProposalInDB,
               JsonConverter,
               GetProposals,
               ProposalValidator,
               ProposalValidationResult,
               StringToStatus,
               UpdateProposal,
               CalculateProposalCreatorID};

use block::{Block,
            CreateBlockIndex,
            CommitBlock,
            ValidateAcceptedProposalBlock,
            ReadBlockFromDB};

use transaction::{CreateTransactionIndex,
                  CreateStateDB,
                  State};

use std::thread;

/*
@name Peer
@desc struct to represent Peer
*/
#[derive(Debug, Clone)]
pub struct Peer {
    pub location: String
}

/*
@name Peers
@desc struct to hold the Peer type
@desc uses Debug, so we can print it, and Clone to handle copying the data type
*/
#[derive(Debug, Clone)]
pub struct Peers {
    peer_set: Vec<Peer>
}

pub trait PeerManager {
    fn set_peers(&mut self, peer_list_string: String) -> ();
    fn new_peers() -> Peers;
    fn add_peer(&mut self, peer_to_add: Peer) -> ();
    fn get_peers(&mut self) -> &Peers;
    fn peers_to_location_vector(&mut self) -> Vec<String>;
}

impl PeerManager for Node {
    fn set_peers(&mut self, peer_list_string: String) -> () {
        let peer_vec: Vec<&str> = peer_list_string.split(",").collect();
        let mut peer_iter = (&peer_vec).into_iter();
        while let Some(p) = peer_iter.next(){
            let new_peer = Peer {
                    location: p.to_string()
            };
            self.add_peer(new_peer);
        }
        println!("Peers set")
    }

    fn new_peers() -> Peers {
        let new_peer_set: Vec<Peer> = Vec::new();
        Peers {
            peer_set: new_peer_set
        }
    }

    fn add_peer(&mut self, peer_to_add: Peer) -> () {
        self.peers.peer_set.push(peer_to_add);
    }

    fn get_peers(&mut self) -> &Peers {
        &self.peers
    }

    fn peers_to_location_vector(&mut self) -> Vec<String>{
        let mut peer_location_vector: Vec<String> = Vec::new();
        let mut iter = (&self.peers.peer_set).into_iter();
        while let Some(v) = iter.next(){
            peer_location_vector.push(v.clone().location);
        }
        peer_location_vector
    }
}


#[derive(Debug, Clone)]
pub struct Node {
    node_name: String,
    node_id: i32,
    peers: Peers,
    server: Server,
    ip: String,
}

pub trait Initiate {
    fn new(node_name: String, node_id: i32, port: i32, ip: String) -> Node;
    fn init(&self);
    fn set_node_name(&mut self, name: String) -> ();
    fn set_node_id(&mut self, id: i32) -> ();
    fn set_port(&mut self, port: i32) -> ();
    fn set_initial_peers(&mut self, peer_list_string: String) -> ();
    fn set_node_ip(&mut self, ip: String) -> ();
}

impl Initiate for Node {
    fn new(node_name: String, node_id: i32, port: i32, ip: String) -> Node {
        let server = Server{
           port: port
        };

        Proposal::create_proposal_index();
        Block::create_block_index();
        Transaction::create_transaction_index();
        State::create_state_db();

        Node {
            node_name: node_name,
            node_id: node_id,
            peers: Self::new_peers(),
            server: server,
            ip: ip
        }
    }

    fn init(&self) {
        println!("Node Name: {}", self.node_name);
        println!("Node Port: {}", self.server.port);
        println!("Node IP: {}", self.ip);
        for peer in &self.peers.peer_set {
            println!("Peer IP: {}", peer.location);
        }

        match self.server.start() {
            Ok(_) => {
                println!("Server successfully started");
            },
            Err(_) => {
                println!("Issue starting server");
            }
        }

    }

    fn set_node_name(&mut self, name: String) -> (){
        println!("Setting node name: {}", name);
        let name_set_handler = || {
            DB::set_node_name(name.clone());
        };
        self.node_name = name;
    }

    fn set_node_id(&mut self, id: i32) -> (){
        println!("Setting node id: {}", id);
        self.node_id = id;
    }

    fn set_port(&mut self, port: i32) -> (){
        println!("Setting port number: {}", port);
        self.server.port = port;
    }

    fn set_initial_peers(&mut self, peer_list_string: String) -> (){
        self.set_peers(peer_list_string)
    }

    fn set_node_ip(&mut self, ip: String) -> (){
        println!("Setting node ip: {}", ip);
        self.ip = ip;
    }
}

pub trait StateTransition {
    fn transition(&mut self) -> ();
    fn determine_transition_step(&mut self, proposal: Proposal, proposal_index: JsonValue) -> ();
}

impl StateTransition for Node {
    fn transition(&mut self) -> (){
        let proposals: Result<Vec<Proposal>, ()> = match Proposal::get_last_n_proposals(){
            Some(proposals) => {
                println!("proposals length: {}", proposals.len());
                Ok(proposals)
            },
            None => {
                println!("No Proposals");
                Err(())
            }
        };

        let delay: u32 = 3000;

        match proposals {
            Ok(p) => {
                for proposal_iterator in p.into_iter() {
                   let proposal_index_option: Option<JsonValue> = DB::get_proposal_index_as_json();
                    match proposal_index_option {
                        Some(proposal_index) => {
                            let proposal_id_string: &str = &format!("{}", proposal_iterator.proposal_id);
                            let proposal_index_version_of_proposal: JsonValue = proposal_index["proposals"][ proposal_id_string ].clone();
                            let proposal_from_json: Result<Proposal, String> = Proposal::from_json(proposal_index_version_of_proposal);
                            match proposal_from_json {
                                Ok(proposal) => {
                                    println!("Proposal ID: {}", proposal.proposal_id);
                                    let proposal_status: ProposalStatus = proposal.clone().proposal_status;
                                    let local_block_id_option: Option<i64> = DB::get_latest_block_id();
                                    match local_block_id_option {
                                        Some(local_block_id) => {
                                            println!("[transition] local_block_id: {}", local_block_id);
                                            let current_block_by_id_option: Option<Block> = DB::get_block_by_block_id(local_block_id);
                                            match current_block_by_id_option {
                                                Some(current_block_by_id) => {
                                                    let proposal_window: i64 = current_block_by_id.block_id - 5;
                                                    if proposal.proposal_block.block_id > proposal_window {
                                                        self.determine_transition_step(proposal.clone(), proposal_index);
                                                        thread::sleep_ms(delay);
                                                    }
                                                    else {
                                                        // DO NOT TRANSITION on proposals from a "lomg time ago"
                                                    }
                                                },
                                                None => {
                                                    if local_block_id == -1 {
                                                        self.determine_transition_step(proposal.clone(), proposal_index);
                                                        thread::sleep_ms(delay);
                                                    } else {}

                                                }
                                            }
                                        },
                                        None => {

                                        }
                                    }
                                },
                                Err(_) => {

                                }
                            }
                        },
                        None => {

                        }
                    }
                } //end for loop
            },
            Err(_) => {
                //TODO: perform Err logic
                ()
            }
        }

        let latest_proposal_option: Option<Proposal> = Proposal::get_latest_proposal();
        match latest_proposal_option {
            Some(latest_proposal) => {
                println!("[transition] - latest_proposal_option is some");
                match latest_proposal.proposal_status {
                    ProposalStatus::Committed |
                    ProposalStatus::RejectedByNetwork => {
                        // get latest block id
                        let latest_block_id_option: Option<i64> = DB::get_latest_block_id();
                        match latest_block_id_option {
                            Some(block_id) => {
                                let calculated_proposal_creator_id: i32 = Proposal::calculate_next_proposal_creator_id(self.peers.peer_set.len(), block_id) as i32;
                                println!("calculated_proposal_creator_id: {} latest block_id: {}", calculated_proposal_creator_id, block_id);
                                if calculated_proposal_creator_id == self.node_id {
                                    Proposal::create( self.clone().ip );
                                } else {

                                }
                            },
                            None => {
                                //SHOULD NEVER HAPPEN, AT WORST IS -1
                                println!("[transition] ERROR, NO LATEST BLOCK ID");
                            }
                        }
                    },
                    _ => {
                        println!("[transition] latest_proposal.proposal_status IS NOT COMMITED");
                    }
                }
            },
            None => {
                println!("[transition] ERROR - latest_proposal_option is NONE")
            }
        }

        println!("[Done with state transition]")
    }

    fn determine_transition_step(&mut self, proposal: Proposal, proposal_index: JsonValue) -> (){
        println!("Performing Transition for proposal_id: {}", proposal.clone().proposal_id);
        let node_ip: String = self.ip.to_string();
        Self::sync_check(&mut self.clone(), proposal.clone(), node_ip.clone());
        match proposal.clone().proposal_status {
            ProposalStatus::Pending => {
                println!("[determine_transition_step], pending...");
                for peer in self.peers.clone().peer_set {
                    if Server::broadcast_proposal_created(proposal.clone(),
                                                          peer.clone().location,
                                                          node_ip.clone()).is_ok() {
                        println!("[determine_transition_step], broadcast_proposal_created SUCCESS...");
                        DB::update_proposal(proposal.clone(), "created");
                    } else {
                        println!("[determine_transition_step], broadcast_proposal_created FAILED...");
                    }
                }
            },

            ProposalStatus::Created => {
                let mut missing_peer_vote: bool = false;
                let mut at_least_one_peer_rejected: bool = false;
                let all_proposals: &JsonValue = &proposal_index["proposals"];
                println!("[determine_transition_step], ProposalStatus::Created, all_proposals.dump(): {}", all_proposals.dump());
                for peer in self.peers.clone().peer_set {
                    match all_proposals.has_key( proposal.clone().proposal_id.to_string().as_str() ) {
                        true => {
                            let proposal_object: &JsonValue = &all_proposals[ proposal.clone().proposal_id.to_string().as_str() ];
                            if proposal_object.has_key( peer.clone().location.as_str() ) {
                                let string_value: Option<&str> = proposal_object[ peer.clone().location.as_str() ].as_str();
                                match string_value {
                                    Some(value) => {
                                        match Proposal::status_from_string( value ) {
                                            ProposalStatus::Accepted => {

                                            },
                                            ProposalStatus::Rejected => {
                                                at_least_one_peer_rejected = true;
                                            },
                                            _ => {
                                                println!("[determine_transition_step], NEITHER ACCEPTED OR REJECTED STORED IN PROPOSAL PEER STATUS");
                                                at_least_one_peer_rejected = true;
                                            }
                                        }
                                    },
                                    None => {
                                        println!("[determine_transition_step], CREATED, STRING VALUE FOR PEER KEY IS NONE");
                                    }
                                }
                            } else {
                                println!("[determine_transition_step], CREATED, PEER KEY DOESN'T EXIST");
                                at_least_one_peer_rejected = true;
                            }
                        },
                        false => {
                            println!("[determine_transition_step], proposal doesnt exist in proposal index");
                            missing_peer_vote = true;
                        }
                    }
                }

                if missing_peer_vote {
                    println!("[determine_transition_step], Created, missing vote");
                } else if at_least_one_peer_rejected  {
                    println!("[determine_transition_step], Created, a peer rejected it");
                } else {
                    println!("[determine_transition_step], Created, all good to update proposal");
                    Proposal::update_proposal(proposal.clone(), "accepted_by_network");
                }
            },

            ProposalStatus::Accepted => {
                for peer in self.peers.clone().peer_set {
                    //TODO: decide who we should broadcast to
                    if Server::broadcast_proposal_response(proposal.clone(),
                                                           peer.clone().location,
                                                           node_ip.clone(),
                                                           ProposalStatus::Accepted).is_ok() {

                        println!("[determine_transition_step], broadcast_proposal_accepted SUCCESS...");
                        DB::update_proposal(proposal.clone(), "accepted_broadcasted");
                    } else {
                        println!("[determine_transition_step], broadcast_proposal_accepted FAILED...");
                    }
                }
            },
            ProposalStatus::AcceptedBroadcasted => {
                let local_block_id_option: Option<i64> = DB::get_latest_block_id();
                match local_block_id_option {
                    Some(local_block_id) => {
                        for peer in self.peers.clone().peer_set {
                            if Server::broadcast_block_query( ( local_block_id ),
                                                               peer.clone().location,
                                                               node_ip.clone()).is_ok() {
                                println!("[determine_transition_step], broadcast_block_query SUCCESS 1...");
                            } else {
                                println!("[determine_transition_step], broadcast_block_query FAILED 1...");
                            }
                        }
                    },
                    None => {

                    }
                }
            },
            ProposalStatus::AcceptedByNetwork => {
                println!("[determine_transition_step], accepted_by_network...");
                let block_commit_result: Result<(),String> = Block::commit_if_valid(proposal.clone().proposal_block);
                if block_commit_result.is_ok() {
                    for peer in self.peers.clone().peer_set {
                        if Server::broadcast_proposal_resolution(proposal.clone(),
                                                              peer.clone().location,
                                                              node_ip.clone()).is_ok() {
                            println!("[determine_transition_step], broadcast_proposal_resolution SUCCESS...");
                        } else {
                            println!("[determine_transition_step], broadcast_proposal_resolution FAILED...");
                        }
                    }
                    DB::update_proposal(proposal.clone(), "committed");
                } else {
                    println!("[ERROR] Block commit result is NOT OKAY!");
                }
            },
            ProposalStatus::Rejected => {
                for peer in self.peers.clone().peer_set {
                    if Server::broadcast_proposal_response(proposal.clone(),
                                                           peer.clone().location,
                                                           node_ip.clone(),
                                                           ProposalStatus::Rejected).is_ok() {
                        println!("[determine_transition_step], broadcast_proposal_rejected SUCCESS...");
                        DB::update_proposal(proposal.clone(), "rejected_broadcasted");
                    } else {
                        println!("[determine_transition_step], broadcast_proposal_rejected FAILED...");
                    }
                }
            },
            ProposalStatus::RejectedBroadcasted => {

            },
            ProposalStatus::RejectedByNetwork => {

            },
            ProposalStatus::Committed => {
                println!("[determine_transition_step], committed, only broadcast so others waiting can get it...");
                for peer in self.peers.clone().peer_set {
                    if Server::broadcast_proposal_resolution(proposal.clone(),
                                                          peer.clone().location,
                                                          node_ip.clone()).is_ok() {
                        println!("[determine_transition_step], broadcast_proposal_resolution SUCCESS...");
                    } else {
                        println!("[determine_transition_step], broadcast_proposal_resolution FAILED...");
                    }
                }
            },
            ProposalStatus::NotValid => {
                //TODO: throw error
            },
            ProposalStatus::NotValidIncorrectNextBlockIndex => {
                match Proposal::validate_proposal(proposal.clone()) {
                    //NOTE: ONLY DO SOMETHING IF YOU CAN SAFELY PARSE THE PROPOSAL, OTHERWISE ERROR
                    Ok(verdict) => {
                        match verdict {
                            ProposalValidationResult::Valid => {
                                DB::update_proposal(proposal.clone(), "accepted");
                            },
                            ProposalValidationResult::NotValid => {
                                DB::update_proposal(proposal.clone(), "rejected");
                            },
                            ProposalValidationResult::NotValidIncorrectNextBlockIndex => {

                            },
                            ProposalValidationResult::NotValidIncorrectProposalHash => {
                                DB::update_proposal(proposal.clone(), "not_valid_incorrect_proposal_hash");
                            }
                        }
                    },
                    Err(_) => {
                        println!("determine_transition_step(), ERROR, could not decide on proposal");
                    }
                }

                for peer in self.peers.clone().peer_set {
                    //TODO: decide who we should broadcast to
                    if Server::broadcast_proposal_response(proposal.clone(),
                                                           peer.clone().location,
                                                           node_ip.clone(),
                                                           ProposalStatus::Rejected).is_ok() {

                        println!("[determine_transition_step], broadcast_proposal_rejected SUCCESS...");
                        DB::update_proposal(proposal.clone(), "rejected_broadcasted");

                    } else {
                        println!("[determine_transition_step], broadcast_proposal_rejected FAILED...");
                        //DB::update_proposal(proposal.proposal_id, "accepted_broadcasted");
                    }
                }

            },
            ProposalStatus::NotValidIncorrectProposalHash => {

            },
            ProposalStatus::ProposalStatusError => {
                //TODO: throw error
            }
        }
    }
}

trait SyncCheck {
    fn sync_check(&mut self, proposal: Proposal, node_ip: String) -> ();
}

impl SyncCheck for Node {
    fn sync_check(&mut self, proposal: Proposal, node_ip: String) -> () {
        let local_block_id_option: Option<i64> = DB::get_latest_block_id();
        match local_block_id_option {
            Some(local_block_id) => {
                let current_block_by_id_option: Option<Block> = DB::get_block_by_block_id(local_block_id);
                match current_block_by_id_option {
                    Some(current_block_by_id) => {

                    },
                    None => {

                    }
                }
            },
            None => {

            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
