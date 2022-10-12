use crate::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedPostedMessage {
    V1(PostedMessageV1),
}

impl From<VersionedPostedMessage> for PostedMessageV1 {
    fn from(message: VersionedPostedMessage) -> Self {
        match message {
            VersionedPostedMessage::V1(posted) => posted,
        }
    }
}
