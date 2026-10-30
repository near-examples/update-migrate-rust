use crate::*;

#[near(serializers=[borsh])]
#[derive(Clone)]
pub struct PostedMessageV1 {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(serializers=[borsh, json])]
#[derive(Clone)]
pub struct PostedMessageV2 {
    pub payment: NearToken,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near(serializers=[borsh])]
#[derive(Clone)]
pub enum VersionedPostedMessage {
    V1(PostedMessageV1),
    V2(PostedMessageV2),
}

impl From<VersionedPostedMessage> for PostedMessageV2 {
    fn from(message: VersionedPostedMessage) -> Self {
        match message {
            VersionedPostedMessage::V2(posted) => posted,
            VersionedPostedMessage::V1(posted) => PostedMessageV2 {
                payment: NearToken::from_near(0),
                premium: posted.premium,
                sender: posted.sender,
                text: posted.text,
            },
        }
    }
}
