use crate::*;

#[near(serializers = [borsh])]
pub struct OldPostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(serializers = [borsh])]
pub struct OldState {
    messages: Vector<OldPostedMessage>,
    payments: Vector<NearToken>,
    manager: AccountId,
}

#[near]
impl GuestBook {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldState = env::state_read().expect("failed");
        let mut new_messages: Vector<PostedMessage> = Vector::new(b"p");

        // iterate through the messages of the previous state
        for (idx, posted) in old_state.messages.iter().enumerate() {
            // get the payment using the message index
            let payment = old_state
                .payments
                .get(idx as u64)
                .unwrap_or(NearToken::from_near(0));

            // Create a PostedMessage with the new format and push it
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
            manager: old_state.manager,
        }
    }
}
