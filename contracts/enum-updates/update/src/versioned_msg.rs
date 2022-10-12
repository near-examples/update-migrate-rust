use crate::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessageV2 {
    pub payment: u128,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedPostedMessage {
    V1(PostedMessageV1),
    V2(PostedMessageV2),
}

impl From<VersionedPostedMessage> for PostedMessageV2 {
    fn from(message: VersionedPostedMessage) -> Self {
        match message {
            VersionedPostedMessage::V2(posted) => posted,
            VersionedPostedMessage::V1(posted) => PostedMessageV2 {
                payment: 0,
                premium: posted.premium,
                sender: posted.sender,
                text: posted.text,
            },
        }
    }
}
