# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2025-12-12

### Changed

- Yank `is_default_derive` v0.1.1 and rollback to v0.1.0.

## [0.1.3] - 2025-12-11

### Added

- Implementation for tuples.

### Changed

- Exclude `tests` for crates.io publication.

## [0.1.2] - 2025-12-07

### Changed

- Move ascii::Char implementation from std to core.

## [0.1.1] - 2025-11-10

### Added

- Implementations for: Ref, RefMut, rc::Weak.
- Optional generic implementation via Default+PartialEq by feature via_default_eq.
- `no_std` build.

### Changed

- Enable `derive` feature by default.
- Split implementations to features std, ascii_char, f16, f128.

## [0.1.0] - 2025-09-23

### Changed

- Major version to 0.


## [1.1.0] - 2025-09-06 [YANKED]

Yanked due switch major version to 0.

### Added

Implementations for:
- arrays
- CString
- Cow 
- OsString
- PathBuf
- &T reference
- &mut T reference

### Fixed

- Slice implementation.

## [1.0.0] - 2025-09-05 [YANKED]

Yanked due switch major version to 0.

### Added

- Initial release.
