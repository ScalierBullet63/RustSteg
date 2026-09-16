# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/ScalierBullet63/RustSteg/compare/v0.1.0...v0.1.1) - 2026-09-16

### Other

- setup cargo-dist workflows for cross-platform distribution ([#25](https://github.com/ScalierBullet63/RustSteg/pull/25))
- update readme ([#24](https://github.com/ScalierBullet63/RustSteg/pull/24))
- release v0.1.0 ([#21](https://github.com/ScalierBullet63/RustSteg/pull/21))

## [0.1.0](https://github.com/ScalierBullet63/RustSteg/releases/tag/v0.1.0) - 2026-09-16

### Added

- add decryption ([#12](https://github.com/ScalierBullet63/RustSteg/pull/12))
- add decode functionality ([#9](https://github.com/ScalierBullet63/RustSteg/pull/9))

### Other

- update license and description in Cargo.toml ([#22](https://github.com/ScalierBullet63/RustSteg/pull/22))
- create release-plz workflow for automated releases ([#20](https://github.com/ScalierBullet63/RustSteg/pull/20))
- update GitHub Actions workflow with permissions ([#19](https://github.com/ScalierBullet63/RustSteg/pull/19))
- add security policy ([#18](https://github.com/ScalierBullet63/RustSteg/pull/18))
- add pull-request_template.md ([#17](https://github.com/ScalierBullet63/RustSteg/pull/17))
- add CONTRIBUTING.md ([#16](https://github.com/ScalierBullet63/RustSteg/pull/16))
- add feature request template ([#15](https://github.com/ScalierBullet63/RustSteg/pull/15))
- add bug report template ([#14](https://github.com/ScalierBullet63/RustSteg/pull/14))
- remove obsolete debug functions ([#13](https://github.com/ScalierBullet63/RustSteg/pull/13))
- separate responsibilities ([#11](https://github.com/ScalierBullet63/RustSteg/pull/11))
- improve GitHub Actions
- remove unnecessary log
- bump deps
- *(deps)* bump argon2 from 0.5.3 to 0.6.0
- improve error handling and add subcommands
- add missing `auth_tag` to `bytes`, serialize payload bits MSB-first
- add payload encryption with `ChaCha20Poly1305`, derive keys with
- implement image steganography properly and improve error handling
- add image saving functionality
- add `NotEnoughBits` error type and check if the image is big
- improve code quality
- improve payload binary convertion and embed payload bits into
- simplify imports and improve type usage
- improve error handling
- add `Image` struct, improve debugging and code quality
- add `Payload` struct
- improve debugging and make `msg` and `target-file` arguments
- bump deps
- split image and payload logic into modules
- add payload binary conversion
- Bump clap from 4.6.1 to 4.6.5
- Bump clap from 4.6.0 to 4.6.1
- Bump rand from 0.9.2 to 0.9.4
- Bump clap from 4.5.60 to 4.6.0
- Bump image from 0.25.9 to 0.25.10
- Add dependabot.yml
- improve image matrix printing
- update dependencies
- add `bacon.toml` for faster development
- Refactor type annotations for clarity in main and image_reader functions
- Add ImageMatrixType and improve matrix printing
- Improve image reading
- Add debug folder and implement image reading
- Include Clap dependencies and argument parsing
- Update rust.yml
- Create rust.yml
- Add Cargo blank project
- Initial commit
