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
               ReadProposalFromDB,
               CreateProposalIndex,
               ProposalStatus,
               UpdateProposalInDB,
               JsonConverter,
               GetProposals,
               ProposalValidator,
               ProposalValidationResult};

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
    peers: Peers,
    server: Server,
    ip: String
}

pub trait Initiate {
    fn new(node_name: String, port: i32, ip: String) -> Node;
    fn init(&self);
    fn set_node_name(&mut self, name: String) -> ();
    fn set_port(&mut self, port: i32) -> ();
    fn set_initial_peers(&mut self, peer_list_string: String) -> ();
    fn set_node_ip(&mut self, ip: String) -> ();
}

impl Initiate for Node {
    fn new(node_name: String, port: i32, ip: String) -> Node {
        let server = Server{
           port: port
        };
        Proposal::create_proposal_index();
        Block::create_block_index();
        Transaction::create_transaction_index();
        State::create_state_db();
        Node {
            node_name: node_name,
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
    fn determine_transition_step(&mut self, status: ProposalStatus, proposal: Proposal) -> ();
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
        let delay: u32 = 5000;
        match proposals {
            Ok(p) => {
                for proposal_iterator in p.into_iter() {
                   let proposal_index: Option<JsonValue> = DB::get_proposal_index_as_json();
                    match proposal_index {
                        Some(parsed) => {
                            let proposal_id_string: &str = &format!("{}", proposal_iterator.proposal_id);
                            let proposal_index_version_of_proposal: JsonValue = parsed["proposals"][ proposal_id_string ].clone();
                            let proposal_from_json: Result<Proposal, String> = Proposal::from_json(proposal_index_version_of_proposal);
                            match proposal_from_json {
                                Ok(proposal) => {
                                    println!("Proposal ID: {}", proposal.proposal_id);
                                    let proposal_status: ProposalStatus = proposal.clone().proposal_status;
                                    let local_block_id_option: Option<i64> = DB::get_latest_block_id();
                                    match local_block_id_option {
                                        Some(local_block_id) => {
                                            let current_block_by_id_option: Option<Block> = DB::get_block_by_block_id(local_block_id);
                                            match current_block_by_id_option {
                                                Some(current_block_by_id) => {
                                                    let proposal_window: i64 = current_block_by_id.block_id - 5;
                                                    if proposal.proposal_block.block_id > proposal_window {
                                                        self.determine_transition_step(proposal_status, proposal.clone());
                                                        thread::sleep_ms(delay);
                                                    }
                                                    else {}
                                                },
                                                None => {
                                                    if local_block_id == -1 {
                                                        self.determine_transition_step(proposal_status, proposal.clone());
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
                }
            },
            Err(_) => {
                ()
            }
        }
        println!("[Done with state transition]")
    }

    fn determine_transition_step(&mut self, status: ProposalStatus, proposal: Proposal) -> (){
        println!("Performing Transition for proposal_id: {}", proposal.clone().proposal_id);
        let node_ip: String = self.ip.to_string();
        Self::sync_check(&mut self.clone(), proposal.clone(), node_ip.clone());
        match status {
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

            },
            ProposalStatus::Accepted => {
                for peer in self.peers.clone().peer_set {
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

            },
            ProposalStatus::NotValidIncorrectNextBlockIndex => {
                match Proposal::validate_proposal(proposal.clone()) {
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
            },
            ProposalStatus::NotValidIncorrectProposalHash => {
                //TODO:
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
