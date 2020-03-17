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

use node::{Node, Initiate, StateTransition};
use std::env;
use std::thread;

fn main() {
    println!("Starting AOS...");

    /*
    TODO: implement responsive CLI to:
    - start()
    - add_peer() //shouldn't add peers over network
    - create_proposal()
    - accept_or_reject()
    */

    let node_name: String = String::from("default");
    let node_id: i32 = 1;
    let port_parameter: i32 = 8000;
    let node_ip: String = String::from("0.0.0.0");
    let mut node: Node = Node::new(node_name, node_id, port_parameter, node_ip);
    let args: Vec<String> = env::args().collect();
    let mut arg_iter = (&args).into_iter();

    //TODO: Abstract Parameters
    while let Some(param) = arg_iter.next(){
        println!("arg: {}", param);
        let split_param_by_assignment: Vec<&str> = param.split("=").collect();
        if split_param_by_assignment.len() == 2 { //equal is a binary operator
            let param_key: &str = split_param_by_assignment[0];
            let param_value: &str = split_param_by_assignment[1];
            println!("split_param_by_assignment: {} : {}", param_key, param_value);
            match param_key {
                "node-name" => node.set_node_name(param_value.to_string()),
                "node-id" => node.set_node_id( param_value.parse::<i32>().unwrap() ),
                "port" => node.set_port(param_value.parse::<i32>().unwrap()),
                "peers" => node.set_initial_peers(param_value.to_string()),
                "ip" => node.set_node_ip(param_value.to_string()),
                _ => ()
            }
        } else {
            println!("Param {} has no key and value combination", split_param_by_assignment[0]);
        }
    }

    let n = node.clone();
    thread::spawn(move || {
        n.init();
    });

    loop {
        node.transition();
        thread::sleep_ms(30000); //delay between every global state transition
    }

}




#[cfg(test)]
mod tests {
    use proposal::{Proposal, ProposalStatus, JsonConverter, ProposalValidator, ProposalValidationResult};
    use block::{Block, CreateNewBlock};
    use timestamp::{Timestamp, NewTimestamp};
    use network::{Server, PayloadParser, API};

    #[test]
    fn test_validate_proposal_isok() {
        let successful_msg: &str = "Successful Proposal Validation";
        let successful_result_stub: Result<String, std::io::Error> = Ok(String::from(successful_msg));
        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_block: Result<Block, String> = Block::new();

        println!("test_timestamp: {}", test_timestamp.clone().unwrap().timestamp);

        let test_proposal: Proposal = Proposal {
            proposal_id: 0,
            proposal_status: ProposalStatus::Created,
            proposal_hash: String::from("test proposal hash"),
            proposal_time: test_timestamp.unwrap(),
            proposal_sender: String::from("test proposal sender"),
            proposal_block: test_block.unwrap()
        };
        let proposal_validated: Result<ProposalValidationResult, std::io::Error> = Proposal::validate_proposal(test_proposal);
        assert!(proposal_validated.is_ok());
    }

    #[test]
    fn test_validate_proposal() {
        let successful_msg: &str = "Successful Proposal Validation";
        let successful_result_stub: Result<String, std::io::Error> = Ok(String::from(successful_msg));
        let test_timestamp: Option<Timestamp> = Timestamp::new();
        let test_block: Result<Block, String> = Block::new();
        println!("test_timestamp: {}", test_timestamp.clone().unwrap().timestamp);
        let test_proposal: Proposal = Proposal {
            proposal_id: 0,
            proposal_status: ProposalStatus::Created,
            proposal_hash: String::from("############TestHashValue############"),
            proposal_time: test_timestamp.unwrap(),
            proposal_sender: String::from("test proposal sender"),
            proposal_block: test_block.unwrap()
        };
        let proposal_validated: Result<ProposalValidationResult, std::io::Error> = Proposal::validate_proposal(test_proposal);
        assert_eq!(ProposalValidationResult::Valid, proposal_validated.unwrap());
    }

    #[test]
    fn test_invoke_action_proposal_created(){
        let invoked_action_result: Result<String, String> = Server::invoke_action("/proposal/created/", "eyJwcm9wb3NhbF9pZCI6MCwicHJvcG9zYWxfc3RhdHVzIjoiYWNjZXB0ZWQiLCJwcm9wb3NhbF9oYXNoIjoiIyMjIyMjIyMjIyMjVGVzdEhhc2hWYWx1ZSMjIyMjIyMjIyMjIyIsInByb3Bvc2FsX3RpbWUiOiIxNTc0OTA1ODAxIiwicHJvcG9zYWxfc2VuZGVyIjoiMTI3LjAuMC4xIiwicHJvcG9zYWxfYmxvY2siOnsiYmxvY2tfaWQiOjAsImJsb2NrX2hhc2giOiJURVNUIEJMT0NLIEhBU0giLCJibG9ja19wYXJlbnRfaGFzaCI6IlRFU1QgUEFSRU5UIEhBU0giLCJibG9ja190aW1lIjoiMTU3NDkwNTgwMSIsInByb3Bvc2FsX2hhc2giOiJURVNUIFBST1BPU0FMIEhBU0giLCJibG9ja19kYXRhIjoiVEVTVCBEQVRBIn19", String::from("127.0.0.1:8080"));
        assert!(invoked_action_result.is_ok());
    }

}
