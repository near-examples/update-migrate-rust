use crate::*;

#[near(serializers=[borsh])]
pub struct OldPostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(serializers=[borsh])]
pub struct OldState {
    messages: Vector<OldPostedMessage>,
    payments: Vector<NearToken>,
}

#[near]
impl GuestBook {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        // retrieve the current state from the contract
        let mut old_state: OldState = env::state_read().expect("failed");

        // new messages vector to hold the migrated messages
        let mut new_messages: Vector<PostedMessage> = Vector::new(MESSAGES_PREFIX);

        // iterate through the state migrating it to the new version
        for (idx, posted) in old_state.messages.iter().enumerate() {
            // get the payment and remove it from the old state payments vector so it won't be left in the new state
            let payment = old_state.payments.get(idx as u64)
                .expect("failed to get payment")
                .clone();

            // push the new message to the new messages vector
            new_messages.push(&PostedMessage {
                payment,
                premium: posted.premium,
                sender: posted.sender.clone(),
                text: posted.text.clone(),
            })
        }

        // remove the payments from the old state
        old_state.payments.clear();

        // return the new state
        Self {
            messages: new_messages,
        }
    }
}
