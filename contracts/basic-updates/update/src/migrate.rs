use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct OldPostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct OldState {
    messages: Vector<OldPostedMessage>,
    payments: Vector<NearToken>,
}

#[near_bindgen]
impl GuestBook {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        // retrieve the current state from the contract
        let old_state: OldState = env::state_read().expect("failed");

        // iterate through the state migrating it to the new version
        let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

        for (idx, posted) in old_state.messages.iter().enumerate() {
            let payment = old_state
                .payments
                .get(idx as u64)
                .unwrap_or(NearToken::from_near(0));

            new_messages.push(&PostedMessage {
                payment,
                premium: posted.premium,
                sender: posted.sender,
                text: posted.text,
            })
        }

        // return the new state
        Self {
            messages: new_messages,
        }
    }
}
