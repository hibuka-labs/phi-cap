# Changelog

All notable changes to the `phi-cap` crate are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-31

### Added
- **First release.** The capability vocabulary shared by plugin manifests and capability packs.
Two layers — a scoped declaration and a coarse decision class — with exactly
one translation point (`class_of`), so plugins and packs never grow a second
vocabulary.
