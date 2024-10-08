use {
    crate::extension::{Extension, ExtensionType},
    bytemuck::{Pod, Zeroable},
};

#[cfg(feature = "serde-traits")]
use serde::{Deserialize, Serialize};

/// Default Account state extension instructions
pub mod instruction;

/// Default Account state extension processor
pub mod processor;

/// Default Account::state extension data for mints.
#[repr(C)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct DefaultAccountState {
    /// Default Account::state in which new Accounts should be initialized
    pub state: PodAccountState,
}
impl Extension for DefaultAccountState {
    const TYPE: ExtensionType = ExtensionType::DefaultAccountState;
}

type PodAccountState = u8;
