use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct FinPet {
    users: UnorderedMap<String, User>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct User {
    savings: u128,
    pet_level: u8,
    pet_status: String,
}

impl Default for FinPet {
    fn default() -> Self {
        Self {
            users: UnorderedMap::new(b"u".to_vec()),
        }
    }
}

#[near_bindgen]
impl FinPet {
    pub fn add_savings(&mut self, amount: u128) {
        let account_id = env::signer_account_id();
        let mut user = self.users.get(&account_id).unwrap_or(User {
            savings: 0,
            pet_level: 1,
            pet_status: "Happy".to_string(),
        });

        user.savings += amount;
        self.update_pet_status(&mut user);
        self.users.insert(&account_id, &user);
    }

    pub fn get_user_info(&self) -> User {
        let account_id = env::signer_account_id();
        self.users.get(&account_id).unwrap_or(User {
            savings: 0,
            pet_level: 1,
            pet_status: "Happy".to_string(),
        })
    }

    fn update_pet_status(&self, user: &mut User) {
        if user.savings >= 1000 {
            user.pet_level = 2;
            user.pet_status = "Growing".to_string();
        } else if user.savings >= 5000 {
            user.pet_level = 3;
            user.pet_status = "Mature".to_string();
        } else if user.savings >= 10000 {
            user.pet_level = 4;
            user.pet_status = "Evolving".to_string();
        }
    }
}