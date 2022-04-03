// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{PassportElementError, True, UserId};

impl_payload! {
    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns _True_ on success.
    ///
    /// Use this if the data submitted by the user doesn't satisfy the standards your service requires for any reason. For example, if a birthday date seems invalid, a submitted document is blurry, a scan shows evidence of tampering, etc. Supply some details in the error message to make sure the user knows how to correct the issues.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetPassportDataErrors (SetPassportDataErrorsSetters) => True {
        required {
            /// User identifier
            pub user_id: UserId,
            /// A JSON-serialized array describing the errors
            pub errors: Vec<PassportElementError> [collect],
        }
    }
}
