// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::types::Recipient;

impl_payload! {
    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns the edited invite link as a [`ChatInviteLink`] object.
    ///
    /// [`ChatInviteLink`]: crate::types::ChatInviteLink
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub EditChatInviteLink (EditChatInviteLinkSetters) => String {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// The invite link to edit
            pub invite_link: String [into],
        }
        optional {
            /// Invite link name; 0-32 characters
            pub name: String [into],
            /// Point in time (Unix timestamp) when the link will expire
            #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
            pub expire_date: DateTime<Utc> [into],
            /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
            pub member_limit: u32,
            /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
            pub creates_join_request: bool,
        }
    }
}
