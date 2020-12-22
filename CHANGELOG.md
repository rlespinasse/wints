# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Add cli subcommand `scan` in replacement of option `--scan`
- Ignore globs also from `~/.wintsignore` during scan
- Add `--dry-run` option for search and scan features

### Removed

- Remove cli option `--scan`, replaced by subcommand `scan`

## [0.0.4] - 2014-08-09

### Added

- Ignore globs from `.wintsignore` during scan.

## [0.0.3] - 2014-08-09

### Added

- Add cli option `--scan` to find new URLs across current folder

## [0.0.2] - 2014-07-10

### Added

- Search URLs by terms contains in a context using fuzzy-matching

## [0.0.1] - 2014-07-10
### Added

- Search URLs by terms contains in a context

<!-- next-url -->
[unreleased]: https://github.com/rlespinasse/wints/compare/v0.0.4...HEAD
[0.4.0]: https://github.com/rlespinasse/wints/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/rlespinasse/wints/compare/e4cc720...v0.3.0
[0.2.0]: https://github.com/rlespinasse/wints/compare/220bb7d...e4cc720
[0.1.0]: https://github.com/rlespinasse/wints/compare/9a9f24b...220bb7d
