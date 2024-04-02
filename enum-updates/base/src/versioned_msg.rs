use crate::*;

#[near(serializers=[borsh,json])]
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(serializers=[borsh])]
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
