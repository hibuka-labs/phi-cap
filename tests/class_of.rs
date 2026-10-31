//! "Whoever declares, pins" regression (sdk-assembly §4.1).
//!
//! Locks the refinement table — every declaration variant maps to exactly
//! one decision class, scopes survive resolution, and cross-class methods
//! resolve to the highest-risk class (O7). If a variant is added without
//! touching `class_of`, compilation fails; if it maps wrong, this fails.

use phi_cap::{
    Capability, ResolvedCapability, ScopeKind, class_of, class_of_all, device_capture,
    device_input, fs_read, net_http, process_spawn, process_spawn_programs, resolve, scope_kind,
};

#[test]
fn every_variant_refines_to_its_class() {
    assert_eq!(class_of(&fs_read(&["data"])), Capability::ReadFs);
    assert_eq!(class_of(&net_http(&["api.example.com"])), Capability::Net);
    assert_eq!(class_of(&process_spawn()), Capability::Process);
    assert_eq!(
        class_of(&process_spawn_programs(&["git"])),
        Capability::Process
    );
    assert_eq!(
        class_of(&device_input(&["window:editor"])),
        Capability::Device
    );
    assert_eq!(
        class_of(&device_capture(&["window:editor"])),
        Capability::Device
    );
}

#[test]
fn every_variant_carries_its_scope_kind() {
    assert_eq!(scope_kind(&fs_read(&["data"])), ScopeKind::Paths);
    assert_eq!(
        scope_kind(&net_http(&["api.example.com"])),
        ScopeKind::Hosts
    );
    assert_eq!(scope_kind(&process_spawn()), ScopeKind::Programs);
    assert_eq!(
        scope_kind(&device_input(&["window:editor"])),
        ScopeKind::Targets
    );
    assert_eq!(
        scope_kind(&device_capture(&["window:editor"])),
        ScopeKind::Targets
    );
}

#[test]
fn resolution_pins_scope_verbatim() {
    // Resolution never rewrites semantics; the installer pins (absolutizes /
    // normalizes) and this type set carries the result.
    let read = resolve(&fs_read(&["data", "fixtures"]));
    assert_eq!(
        read,
        ResolvedCapability::FsRead {
            paths: vec!["data".into(), "fixtures".into()]
        }
    );
    assert_eq!(read.capability(), Capability::ReadFs);

    let spawn = resolve(&process_spawn_programs(&["git"]));
    assert_eq!(
        spawn,
        ResolvedCapability::ProcessSpawn {
            programs: vec!["git".into()]
        }
    );
    assert_eq!(spawn.capability(), Capability::Process);

    // Unrestricted spawn keeps an empty program list — the honest declaration
    // for a shell (`sh.run` runs whatever it is given).
    let shell = resolve(&process_spawn());
    assert_eq!(shell, ResolvedCapability::ProcessSpawn { programs: vec![] });

    let capture = resolve(&device_capture(&["window:editor"]));
    assert_eq!(capture.capability(), Capability::Device);
}

#[test]
fn cross_class_methods_take_the_highest_risk_o7() {
    // O7: device > process > net > write_fs > read_fs > pure.
    let mixed = [fs_read(&["data"]), process_spawn()];
    assert_eq!(class_of_all(&mixed), Capability::Process);

    let with_device = [process_spawn(), device_input(&["screen:main"])];
    assert_eq!(class_of_all(&with_device), Capability::Device);

    let read_only = [fs_read(&["data"]), fs_read(&["meta"])];
    assert_eq!(class_of_all(&read_only), Capability::ReadFs);

    // No capabilities at all = pure computation.
    assert_eq!(class_of_all(&[]), Capability::Pure);
}

#[test]
fn decision_order_matches_o7_risk_total_order() {
    use Capability::*;
    assert!(Pure < ReadFs);
    assert!(ReadFs < WriteFs);
    assert!(WriteFs < Net);
    assert!(Net < Process);
    assert!(Process < Device);
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use phi_cap::CapabilitySpec;

    #[test]
    fn wire_tags_match_the_plugin_manifest_vocabulary() {
        // The public standard is `face:scope` dotted tags; upstream
        // vocabularies never leak into the wire form.
        let spec: CapabilitySpec =
            serde_json::from_str(r#"{"type":"fs:read","paths":["data"]}"#).unwrap();
        assert_eq!(spec, fs_read(&["data"]));

        let spec: CapabilitySpec =
            serde_json::from_str(r#"{"type":"net:http","hosts":["api.example.com"]}"#).unwrap();
        assert_eq!(spec, net_http(&["api.example.com"]));

        let spec: CapabilitySpec = serde_json::from_str(r#"{"type":"process:spawn"}"#).unwrap();
        assert_eq!(spec, process_spawn());

        let spec: CapabilitySpec =
            serde_json::from_str(r#"{"type":"device:input","targets":["window:editor"]}"#).unwrap();
        assert_eq!(spec, device_input(&["window:editor"]));

        let spec: CapabilitySpec =
            serde_json::from_str(r#"{"type":"device:capture","targets":["window:editor"]}"#)
                .unwrap();
        assert_eq!(spec, device_capture(&["window:editor"]));
    }

    #[test]
    fn strict_oneof_rejects_cross_talk() {
        // Strict oneOf law: a net field on an fs entry is rejected, not ignored.
        let bad = r#"{"type":"fs:read","paths":["data"],"hosts":["x"]}"#;
        assert!(serde_json::from_str::<CapabilitySpec>(bad).is_err());

        let bad = r#"{"type":"device:capture","targets":["t"],"paths":["p"]}"#;
        assert!(serde_json::from_str::<CapabilitySpec>(bad).is_err());
    }
}
