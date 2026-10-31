Capability vocabulary for the phi family: one declaration language, one decision vocabulary.

Two layers, one refinement point:

```text
CapabilitySpec (scoped declaration)  --class_of-->  Capability (coarse decision)
  fs:read{paths}            read_fs
  net:http{hosts}           net
  process:spawn{programs}   process
  device:input{targets}     device
  device:capture{targets}   device
```

* **Declaration layer** — what a plugin manifest or capability pack *writes*.
  Scopes are mandatory; undeclared means not granted (fail-closed).
* **Decision layer** — what a `Policy` *matches on*. Coarse action-surface
  classes; never data semantics (a remote DB query is `net`, the same query on
  a local file is `read_fs`).

The only translation is `class_of`. New dimensions extend `CapabilitySpec`
variants — never a third enum.

Zero hard dependencies. Enable the `serde` feature for wire formats (plugin
manifests).

```toml
[dependencies]
phi-cap = "0.1.0"
```

## License

MIT.
