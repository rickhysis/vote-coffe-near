use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, BorshStorageKey};

setup_alloc!();

pub type CoffeName = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Choice {
    Like,
    Dislike
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Coffe {
    account_id: Option<AccountId>,
    name: CoffeName,
    description: String,
    image: String,
    number_of_like: i8,
    number_of_dislike: i8,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote{
    coffe_name: CoffeName,
    choice: Choice
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    coffe: UnorderedMap<CoffeName, Coffe>,
    votes: UnorderedMap<AccountId, Vote>
}

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Coffe,
    Vote,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            coffe: UnorderedMap::new(StorageKey::Coffe),
            votes: UnorderedMap::new(StorageKey::Vote),
        }
    }
}

#[near_bindgen]
#[warn(unused_mut)]
impl Contract {
    pub fn add_coffe(&mut self, mut coffe: Coffe) -> Coffe {
        let account_id = env::predecessor_account_id();
        let name = coffe.name.clone();

        coffe.account_id = Some(account_id.clone());
        self.coffe.insert(&name, &coffe);

        return coffe;
    }

    pub fn vote(&mut self, name: CoffeName, choice: Choice) -> Vote {
        let account_id = env::signer_account_id();
        let coffe_name = name.clone();     
        
        let mut coffe = self.coffe.get(&name).unwrap_or_else(|| panic!("Coffe does not exist"));
        match choice {
            Choice::Like => coffe.number_of_like += 1,
            Choice::Dislike => coffe.number_of_dislike += 1,     
        }

        let vote = Vote { coffe_name: name, choice: choice};

        self.votes.insert(&account_id, &vote);

        self.coffe.insert(&coffe_name, &coffe);
        return vote;
    }

    pub fn get_coffes(self, skip: u64, limit: Option<u64>) -> Option<Vec<Coffe>> {
        let limit = limit.map(|v| v as usize).unwrap_or(usize::MAX);
        assert_ne!(limit, 0, "Cannot provide limit of 0.");

        assert!(
            self.coffe.len() > skip,
            "Out of bounds, please use a smaller skip."
        );

        return Some(
            self.coffe
                .iter()
                .skip(skip as usize)
                .take(limit as usize)
                .map(|(_, coffes)| coffes)
                .collect(),
        );
    }

    pub fn get_coffe(self, name: CoffeName) -> Coffe {
        self.coffe.get(&name).expect("Coffe does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_add_coffe() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
        let coffe = contract.add_coffe(Coffe {
            name: "Kopi Ijen Banyuwangi".to_string(),
            account_id: None,
            description: "Kopi ijen banyuwangi adalah kopi arabica yang ditanam oleh petani local".to_string(),
            image: "https://example.com".to_string(),
            number_of_like: 0,
            number_of_dislike: 0,
        });
        //println!("{:?}", coffe_ids);
        //let coffe = contract.get_coffe(coffe_ids.clone()).unwrap_or_else(|| panic!("Coffe does not exist"));

        //assert_eq!(coffe.unwrap(), coffe_ids);
        assert_eq!(coffe.name, "Kopi Ijen Banyuwangi".to_string());
        assert_eq!(coffe.image, "https://example.com".to_string());
    }

    #[test]
    fn test_vote_coffe() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Contract::default();
        let coffe_ids = contract.add_coffe(Coffe {
            name: "Kopi Ijen Banyuwangi".to_string(),
            account_id: None,
            description: "Kopi ijen banyuwangi adalah kopi arabica yang ditanam oleh petani local".to_string(),
            image: "https://example.com".to_string(),
            number_of_like: 0,
            number_of_dislike: 0,
        });

        contract.vote(coffe_ids.name, Choice::Like);
    }

}
