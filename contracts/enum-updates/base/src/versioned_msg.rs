use crate::*;

#[derive(NearSchema, BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
#[abi(json, borsh)]
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
#[borsh(crate = "near_sdk::borsh")]
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
