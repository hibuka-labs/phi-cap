//! Declaration layer: scoped capability specs — the only vocabulary a pack
//! or plugin ever writes (sdk-assembly §4.1).
//!
//! Two layers, one refinement:
//!
//! ```text
//! CapabilitySpec (scoped, declaration)  --class_of-->  Capability (coarse, decision)
//!   fs:read{paths}                                       read_fs
//!   net:http{hosts}                                      net
//!   device:input{targets}                                device
//! ```
//!
//! Shape law (O13 decision): the structure is ours — scope-first variants that
//! match the existing `fs:read` / `net:http` wire format — while fine-grained
//! *tokens* inside a scope may adopt upstream vocabularies (e.g. cua's dotted
//! capability names) at the adapter boundary. Upstream strings never become
//! the public standard.

/// Readable directories/files (resolved to absolute paths and pinned by the
/// registry at install time — "whoever declares, pins").
///
/// Wire form `fs:read`; extra fields rejected (strict oneOf law).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct FsReadSpec {
    /// Readable directories/files, relative to the declaring root.
    pub paths: Vec<String>,
}

/// Outbound HTTP host allow-list (normalized and pinned at install time).
///
/// Wire form `net:http`; extra fields rejected (strict oneOf law).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct NetHttpSpec {
    /// Outbound HTTP hosts; off-list hosts are rejected at request time.
    pub hosts: Vec<String>,
}

/// Process spawn face (`process:spawn`).
///
/// `programs` narrows the declaration to named programs; empty means the
/// unrestricted process face (a shell runs whatever it is given — the
/// honest declaration for `sh.run`). Policy still decides per call.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct ProcessSpawnSpec {
    /// Program allow-list; empty = unrestricted.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Vec::is_empty")
    )]
    pub programs: Vec<String>,
}

/// Device input face (`device:input`): pointer / keyboard / touch injection.
///
/// `targets` is the scope that gets pinned — "can only touch these windows /
/// screens". Fine-grained upstream verb tokens (cua's `input.pointer.click`
/// and friends) land in `verb` at P3; the variant shape is already stable.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct DeviceInputSpec {
    /// Input targets, e.g. `["window:editor"]` / `["screen:main"]`.
    pub targets: Vec<String>,
}

/// Device capture face (`device:capture`): screenshots and friends.
///
/// Scope is mandatory at the policy layer (P3: an empty target list must be
/// rejected) — capturing outside the declared targets is an overreach.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct DeviceCaptureSpec {
    /// Capture targets, e.g. `["window:editor"]`.
    pub targets: Vec<String>,
}

/// Capability allow-list entry — the declaration form (fail-closed:
/// undeclared = not granted).
///
/// Every shape that declares capabilities uses this one enum: wasm plugin
/// manifests, capability packs' `MethodSpec.caps`, the plugin `sdk` form.
/// Wire tags keep the dotted `face:scope` names.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum CapabilitySpec {
    /// Readable directories/files (pinned as absolute paths at install).
    #[cfg_attr(feature = "serde", serde(rename = "fs:read"))]
    FsRead(FsReadSpec),
    /// Outbound HTTP host allow-list (pinned normalized at install).
    #[cfg_attr(feature = "serde", serde(rename = "net:http"))]
    NetHttp(NetHttpSpec),
    /// Process spawn face.
    #[cfg_attr(feature = "serde", serde(rename = "process:spawn"))]
    ProcessSpawn(ProcessSpawnSpec),
    /// Device input face (pointer / keyboard / touch).
    #[cfg_attr(feature = "serde", serde(rename = "device:input"))]
    DeviceInput(DeviceInputSpec),
    /// Device capture face (screenshots and friends).
    #[cfg_attr(feature = "serde", serde(rename = "device:capture"))]
    DeviceCapture(DeviceCaptureSpec),
}

/// `fs:read{paths}` — helper for declarations.
pub fn fs_read(paths: &[&str]) -> CapabilitySpec {
    CapabilitySpec::FsRead(FsReadSpec {
        paths: paths.iter().map(|s| (*s).to_string()).collect(),
    })
}

/// `net:http{hosts}` — helper for declarations.
pub fn net_http(hosts: &[&str]) -> CapabilitySpec {
    CapabilitySpec::NetHttp(NetHttpSpec {
        hosts: hosts.iter().map(|s| (*s).to_string()).collect(),
    })
}

/// `process:spawn{}` (unrestricted) — helper for declarations.
pub fn process_spawn() -> CapabilitySpec {
    CapabilitySpec::ProcessSpawn(ProcessSpawnSpec::default())
}

/// `process:spawn{programs}` — helper for declarations.
pub fn process_spawn_programs(programs: &[&str]) -> CapabilitySpec {
    CapabilitySpec::ProcessSpawn(ProcessSpawnSpec {
        programs: programs.iter().map(|s| (*s).to_string()).collect(),
    })
}

/// `device:input{targets}` — helper for declarations.
pub fn device_input(targets: &[&str]) -> CapabilitySpec {
    CapabilitySpec::DeviceInput(DeviceInputSpec {
        targets: targets.iter().map(|s| (*s).to_string()).collect(),
    })
}

/// `device:capture{targets}` — helper for declarations.
pub fn device_capture(targets: &[&str]) -> CapabilitySpec {
    CapabilitySpec::DeviceCapture(DeviceCaptureSpec {
        targets: targets.iter().map(|s| (*s).to_string()).collect(),
    })
}
