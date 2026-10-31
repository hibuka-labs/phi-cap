//! Decision layer: the coarse capability classes a `Policy` matches on.
//!
//! Coarse classes describe the **action surface** — "what can this touch" —
//! never data semantics: a remote DB query is [`Capability::Net`], the same
//! query against a local file is [`Capability::ReadFs`] (sdk-assembly §4.3).
//!
//! This enum is the "right side" of the single translation point
//! [`crate::class_of`]. There is exactly one decision vocabulary; new
//! dimensions extend `CapabilitySpec` variants, never a third enum.

use core::fmt;

/// Coarse capability class — the decision key handed to `Policy::allow`.
///
/// Ordering follows the O7 risk total order
/// (`Device > Process > Net > WriteFs > ReadFs > Pure`); a cross-class
/// declaration resolves to the highest-ranked class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Capability {
    /// Pure computation, no side effects.
    Pure,
    /// Reads files.
    ReadFs,
    /// Writes files.
    WriteFs,
    /// Network access.
    Net,
    /// Spawns processes.
    Process,
    /// Device / GUI operations.
    Device,
}

impl Capability {
    /// Position on the O7 risk total order. Used to break cross-class ties.
    pub const fn risk(self) -> u8 {
        match self {
            Self::Pure => 0,
            Self::ReadFs => 1,
            Self::WriteFs => 2,
            Self::Net => 3,
            Self::Process => 4,
            Self::Device => 5,
        }
    }
}

impl Ord for Capability {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.risk().cmp(&other.risk())
    }
}

impl PartialOrd for Capability {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pure => "pure",
            Self::ReadFs => "read_fs",
            Self::WriteFs => "write_fs",
            Self::Net => "net",
            Self::Process => "process",
            Self::Device => "device",
        };
        f.write_str(s)
    }
}
