//! # phi-cap
//!
//! Capability vocabulary for the phi family: **one declaration language, one
//! decision vocabulary.**
//!
//! Two layers, one refinement point:
//!
//! ```text
//! CapabilitySpec (scoped declaration)  --class_of-->  Capability (coarse decision)
//!   fs:read{paths}            read_fs
//!   net:http{hosts}           net
//!   process:spawn{programs}   process
//!   device:input{targets}     device
//!   device:capture{targets}   device
//! ```
//!
//! * Declaration layer — what a plugin manifest or capability pack *writes*.
//!   Scopes are mandatory; undeclared = not granted (fail-closed).
//! * Decision layer — what `Policy` *matches on*. Coarse action-surface
//!   classes; never data semantics (a remote DB query is `net`, the same
//!   query on a local file is `read_fs`).
//!
//! The only translation is [`class_of`]. New dimensions extend
//! [`CapabilitySpec`] variants — never a third enum (iron rule 2).
//!
//! Zero hard dependencies; enable `serde` for wire formats (plugin
//! manifests). Wire tags keep the dotted `face:scope` names
//! (`"fs:read"`, `"net:http"`, …).
//!
//! O7 cross-class rule: a method carrying several capabilities resolves to
//! the **highest-risk** class (`device > process > net > write_fs > read_fs
//! > pure`) — see [`class_of_all`].

mod capability;
mod resolved;
mod spec;

pub use capability::Capability;
pub use resolved::{ResolvedCapability, ScopeKind, class_of, class_of_all, resolve, scope_kind};
pub use spec::{
    CapabilitySpec, DeviceCaptureSpec, DeviceInputSpec, FsReadSpec, NetHttpSpec, ProcessSpawnSpec,
    device_capture, device_input, fs_read, net_http, process_spawn, process_spawn_programs,
};
