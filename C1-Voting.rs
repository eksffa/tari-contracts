use tari_runtime::{
    base_node::state_machine_service::{StateMachineServiceConfig, StateMachineServiceHandle},
    contracts::{contract_interface::ContractInterface, contract_manager::ContractManager},
    state_machine::{
        run_state_machine,
        state_machine_builder::{StateMachineBuilder, StateMachineConfig},
    },
};
use tari_crypto::{
    commitments::{PedersenCommitment, PedersenCommitmentFactory, RangeProof},
    keys::PublicKey,
    ristretto::RistrettoSecretKey,
    zkp::{
        commitment::{Commitment, CommitmentScheme},
        range_proof::RangeProofProver,
    },
};

struct VotingSystem {
    // conservare la chiave pubblica di ogni candidat
    candidates: Vec<PublicKey>,
    // conserva i token di voto e gli impegni associati
    tokens: Vec<(PublicKey, PedersenCommitment)>,
    // conserva voto per candidat
    votes: Vec<usize>,
    // una chiave segreta utilizzata per generare e verificare prove a conoscenza zero (for zeroproof)
    secret_key: RistrettoSecretKey,
}

impl ContractInterface for VotingSystem {
    fn execute(&self, input: &[u8], caller_public_key: PublicKey) -> Result<Vec<u8>, String> {
        let inputs = MessageFormat::parse_input_string(input

);
let method = &inputs[0];
match method {
 // registrati candidato
 "register_candidate" => {
 self.register_candidate(caller_public_key)
},
// claim voting token
// voting right = 1 token is one vote, 1 token per wallet = 1 vote per wallet
// a government should do KYC to make sure this 1 voting token per qualified voter
// is a rule to be enforced under coercitive governments where only qualified
// voters do vote
"claim_token" => {
self.claim_token(caller_public_key)
},
 // esercitare il suffragio
 // votare le pecore!
 "vote" => {
 let token_commitment = PedersenCommitment::from_bytes(&inputs[1]).unwrap();
 let proof = RangeProof::from_bytes(&inputs[2]).unwrap();
 let candidate_index = inputs[3].parse::<usize>().unwrap();
 self.vote(token_commitment, proof, candidate_index)
},
 // count votes and find winner
 "count_votes" => {
        self.count_votes()
 },
_ => panic!("Invalid method!"),
}
}
}

impl VotingSystem {
        // registrare un candidato
        fn register_candidate(&mut self, candidate: PublicKey) -> Result<Vec<u8>, String> {
        self.candidates.push(candidate);
        Ok(vec![1])
}

// claim un token di voto
fn claim_token(&mut self, caller_public_key: PublicKey) -> Result<Vec<u8>, String> {
    // check if voter has already claimed
    // double-check reentrancy for security
    for (token, _) in &self.tokens {
        if token == &caller_public_key {
            return Ok(vec![0]);
        }
    }

    let factory = PedersenCommitmentFactory::default();
    let token = factory.commit(&[caller_public_key.as_bytes()], &self.secret_key);
    self.tokens.push((caller_public_key, token));
    Ok(vec![1])
}

// (moo) vota per un candidato
fn vote(&mut self, token_commitment: PedersenCommitment, proof: RangeProof, candidate_index: usize) -> Result<Vec<u8>, String>
{
         if candidate_index >= self.candidates.len() {
         return Ok(vec![0]);
}

    let factory = PedersenCommitmentFactory::default();
    if !factory.verify_proof(&token_commitment, &proof, &self.secret_key) {
        return Ok(vec![0]);
    }

    // check token is valid and not used before
    let mut token_found = false;
    for (token, commitment) in &self.tokens {
        if commitment == &token_commitment {
            token_found = true;
            // add vote to contadino
            self.votes[candidate_index] += 1;
            // destroy voting token not to be used again
            self.tokens.retain(|(t, _)| t != token);
            break;
        }
    }
    if token_found {
        Ok(vec![1])
    } else {
        Ok(vec![0])
    }
}

// count all votes and find who's the winner
fn count_votes(&self) -> Result<Vec<u8>, String> {
    let mut winner_index = 0;
    let mut winner_votes = 0;
    // count the votes for each candidate
    for (i, candidate_votes) in self.votes.iter().enumerate() {
        if candidate_votes > &winner_votes {
            winner_index = i;
            winner_votes = *candidate_votes;
        }
    }
    // and the winner is... [sampre coercizione]
    let winner = self.candidates[winner_index];
    Ok(winner.to_bytes().to_vec())
}


