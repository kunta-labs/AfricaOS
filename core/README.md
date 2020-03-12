# Development work

## Technological goals
Can we develop a system that:
- all heads of state have an uncompromised computer, they own
- each computer have the means of interacting with the other
- each computer serves as a decision making node to invoke state transitions in the System
- can we make a circular queue, of inputs to a state transition function
- each state that transitions the state transition function has to have certain aspects that are used to be computed


## Proposals
A proposal submits a motion to further "time" within the network. This is sent, and must be responded to.
- Pending: for proposals just made
- Created: for proposals made, and broadcasted to the network
- Accepted: proposals accepted by peers
- Rejected: proposals rejected by peers
- Committed: proposals agreed upon by peers
- NotValid: proposals that do not match any of the above enum values

### Proposal Index
Each replicate maintains a Proposal Index (a database of proposals)
```javascript
{proposals:
  {"proposal_id_as_key": Proposal (see below)}
}
```

### Proposal Structure
a Proposal contains the following information
```javascript
{
  proposal_id: Integer,
  proposal_status: String,
  proposal_hash: String,
  proposal_time: Timestamp,
  proposal_sender: String,
  proposal_block: Block (see below)
}
```

### Calculating Proposal Hash
To calculate the hash of the proposal, hash the JSON string representation of the *proposal_id*, and the *proposal_block*

## Block
A block will hold the contents (hash) of the proposal, combined with a timestamp. A new block can only be authored once a proposal has been sent, responded to, and resolved -- A three-way handshake.

A proposer proposes, receives an acceptance, and then notifies the network of it now committing the proposal, whereby each other replica can take the commitment notification, and verify it:

### Block structure
A block contains the following structure
```javascript
{
  block_id: i32,
  block_hash: String,
  block_parent_hash: String,
  block_time: Timestamp,
  proposal_hash: String
  block_data: String
}
```

## Accepting a proposal
Once a proposal is broadcasted from Alice to Bob, (A -> B), Bob performs the following (latest proposal_id = 0, and alice's proposal state is *created*):
- Bob checks if he already has a Proposal with that *proposal_id* (proposal_id = 1)
  - if bob has proposal_id = 1, he checks the status of it
    - if the *proposal_status* is *accepted*, or *rejected*, and the submitter is NOT bob, do nothing because bob already added it to the *proposal index*
- if bob doesn't have proposal_id = 1, bob verifies the proposal using the following criteria:
  - What is the current *block_id* bob has?
  - a valid proposal will ONLY be the current *block_id* + 1
  - calculate the hash of the proposal (see below)
  - validate the *proposal_hash* provided by alice against the *proposal_hash* bob just calculated
  - if the hashes are different, bob rejects the proposal, and sets the proposal to *NotValid* in the proposal index
  - What is the current *block_hash* of our highest block?
  - If the *block_hash* of bob's highest block is NOT equal to the *block_parent_hash* of the submitted proposal's *proposal_block*, bob rejects the block, and sets the proposal to *NotValid* in the proposal index
  - if all of the above does not reject the proposal, bob accepts alice's submitted proposal, responds to alice with "acceptance", and updates the proposal_index to *accepted* for the *proposal_id*
- If the proposal is rejected, update proposal index, and set proposal to *NotValid*
- If the proposal is valid, (bob doesn't add it yet), but bob responds to alice with "acceptance", and updates the proposal index to mark the proposal as *accepted*
- Every time alice receives a *accept* or a *reject*, alice checks how many responses the respective proposal requires
  - If alice receives enough *accept* responses, alice broadcasts the corresponding proposal to bob with a *committed* state
- When bob receives a committed proposal:
  - bob checks if he already has the same proposal with the submitter != bob, and with a *accepted* state
  - if bob already has the proposal with a *accepted* state:
    - bob verified the proposal (using the hash)
    - if the proposal is valid, bob commits the proposal's block to his block history


### Calculating Block Hash
Calculate the block hash by hashing the JSON string representation of the *block_id*, *block_parent_hash*, *block_time*, and the *block_time*

### State Transitioning
Upon each transition step, check the proposal index for:
- Proposals marked as *pending*, and the submitter is "me"
  - for each of these *pending* proposals
    - broadcast them to the network
    - mark the proposal as *created*
- Proposals marked as *accepted*
  - for each of these *accepted* proposals
    - verify the proposal's block
      - if valid
        - commit the proposal's block to "my" block history
        - mark the proposal as committed
      - if not valid, mark proposal as *NotValid*

# Chain Directory
