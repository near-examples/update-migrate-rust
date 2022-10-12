use crate::{*};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldState {
  messages: Vector<PostedMessage>,
}

#[near_bindgen]
impl GuestBook {

  #[private]
  #[init(ignore_state)]
  pub fn migrate() -> Self {
      let old_state: OldState = env::state_read().expect("failed");
      let mut payments:Vector<Balance> = Vector::new(b"p");

      // Iterate through the old state messages
      for message in old_state.messages.iter() {
        // Create info about the payments
        if message.premium {
          payments.push(&POINT_ONE)
        }else{
          payments.push(&0)
        }
      }

      // assert a minimum of consistency
      assert!(old_state.messages.len() == payments.len(), "Migration Error");

      // return the new state
      Self {
          messages: old_state.messages,
          payments: payments,
      }
  }
}