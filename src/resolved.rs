//! The single refinement point: declaration → decision.
//!
//! One vocabulary, two layers (sdk-assembly §4.1):
//!
//! * [`CapabilitySpec`] — scoped, what a pack/plugin *declares*.
//! * [`Capability`] — coarse, what a `Policy` *decides* on.
//!
//! [`class_of`] is the only translation. Policy keys never see declaration
//! strings; declaration scopes never become decision keys. Cross-class
//! results resolve to the highest-risk class (O7), never a blend.

use crate::capability::Capability;
use crate::spec::CapabilitySpec;

/// What kind of scope a declaration carries. The registry pins exactly this
/// ("whoever declares, pins"): paths become absolute, hosts normalized,
/// targets/programs kept as declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    /// Filesystem paths (`fs:read`) — resolved absolute at install.
    Paths,
    /// Network hosts (`net:http`) — normalized (trim + lowercase) at install.
    Hosts,
    /// Process programs (`process:spawn`) — empty = unrestricted.
    Programs,
    /// Device targets (`device:input` / `device:capture`) — pinned as declared.
    Targets,
}

/// A declaration with its scope resolved/pinned — the registry's view.
///
/// Scope fields survive resolution; resolution never rewrites semantics
/// (absolute paths and normalized hosts only).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum ResolvedCapability {
    /// Pinned absolute paths.
    #[cfg_attr(feature = "serde", serde(rename = "fs:read"))]
    FsRead {
        /// Absolute readable paths.
        paths: Vec<String>,
    },
    /// Pinned normalized hosts.
    #[cfg_attr(feature = "serde", serde(rename = "net:http"))]
    NetHttp {
        /// Normalized hosts (trimmed, lowercased).
        hosts: Vec<String>,
    },
    /// Pinned program allow-list.
    #[cfg_attr(feature = "serde", serde(rename = "process:spawn"))]
    ProcessSpawn {
        /// Programs; empty = unrestricted.
        programs: Vec<String>,
    },
    /// Pinned input targets.
    #[cfg_attr(feature = "serde", serde(rename = "device:input"))]
    DeviceInput {
        /// Input targets as declared.
        targets: Vec<String>,
    },
    /// Pinned capture targets.
    #[cfg_attr(feature = "serde", serde(rename = "device:capture"))]
    DeviceCapture {
        /// Capture targets as declared.
        targets: Vec<String>,
    },
}

impl ResolvedCapability {
    /// The decision class this pinned declaration resolves to.
    pub const fn capability(&self) -> Capability {
        match self {
            Self::FsRead { .. } => Capability::ReadFs,
            Self::NetHttp { .. } => Capability::Net,
            Self::ProcessSpawn { .. } => Capability::Process,
            Self::DeviceInput { .. } | Self::DeviceCapture { .. } => Capability::Device,
        }
    }
}

/// The scope kind a declaration carries.
pub const fn scope_kind(spec: &CapabilitySpec) -> ScopeKind {
    match spec {
        CapabilitySpec::FsRead(_) => ScopeKind::Paths,
        CapabilitySpec::NetHttp(_) => ScopeKind::Hosts,
        CapabilitySpec::ProcessSpawn(_) => ScopeKind::Programs,
        CapabilitySpec::DeviceInput(_) | CapabilitySpec::DeviceCapture(_) => ScopeKind::Targets,
    }
}

/// **The** refinement: declaration → decision class (sdk-assembly §4.1).
///
/// Everything that needs a `Policy` key goes through here — nothing
/// re-implements the match, nothing invents a third vocabulary.
pub const fn class_of(spec: &CapabilitySpec) -> Capability {
    match spec {
        CapabilitySpec::FsRead(_) => Capability::ReadFs,
        CapabilitySpec::NetHttp(_) => Capability::Net,
        CapabilitySpec::ProcessSpawn(_) => Capability::Process,
        CapabilitySpec::DeviceInput(_) | CapabilitySpec::DeviceCapture(_) => Capability::Device,
    }
}

/// Resolve a declaration into its pinned form — scope is preserved verbatim
/// here; path/host pinning is the installer's job (see agent-works
/// `resolve_capabilities`, which this type set sinks from).
pub fn resolve(spec: &CapabilitySpec) -> ResolvedCapability {
    match spec.clone() {
        CapabilitySpec::FsRead(s) => ResolvedCapability::FsRead { paths: s.paths },
        CapabilitySpec::NetHttp(s) => ResolvedCapability::NetHttp { hosts: s.hosts },
        CapabilitySpec::ProcessSpawn(s) => ResolvedCapability::ProcessSpawn {
            programs: s.programs,
        },
        CapabilitySpec::DeviceInput(s) => ResolvedCapability::DeviceInput { targets: s.targets },
        CapabilitySpec::DeviceCapture(s) => {
            ResolvedCapability::DeviceCapture { targets: s.targets }
        }
    }
}

/// Collapse a multi-capability method to one policy key (O7): take the
/// **highest-risk** class — a method that both reads files and spawns
/// processes is decided as `process`.
pub fn class_of_all(specs: &[CapabilitySpec]) -> Capability {
    specs.iter().map(class_of).max().unwrap_or(Capability::Pure)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_of_is_total_on_every_variant() {
        let specs = [
            crate::spec::fs_read(&["data"]),
            crate::spec::net_http(&["api.example.com"]),
            crate::spec::process_spawn(),
            crate::spec::device_input(&["window:editor"]),
            crate::spec::device_capture(&["window:editor"]),
        ];
        let classes: Vec<Capability> = specs.iter().map(class_of).collect();
        assert_eq!(
            classes,
            [
                Capability::ReadFs,
                Capability::Net,
                Capability::Process,
                Capability::Device,
                Capability::Device
            ]
        );
    }
}
