# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- Add module configuration inside `.wints/modules`
- Add cli subcommands
  - `init` for module initialization
  - `add` to one url to a context
  - `scan` to find new URL inside a directory tree
  - `url ignore` to ignore some URL during scan
  - `url ignore-glob` to ignore some file during scan
- Add options for all features
  - `--dry-run` to just log what are going to do
  - `--config` as folder of local configuration storage
  - `--global-config` as folder of global configuration storage
- Add options for all module-related features
  - `--module` to specify the module name to use
- Ignore globs from `.wints/ignore` or `HOME_DIR/.wints/ignore` during scan
- Add `path` selection to `scan` subcommand

### Fixed

- Output the correct number of new URLs during `scan`

### Removed

- Remove cli option `--scan`, replaced by subcommand `scan`
- Remove use of `.wints.yaml` file, replace by `.wints/module/main.yaml`
- Remove use of `.wintsignore` file, replace by `.wints/ignore`
- Remove use of `HOME_DIR/.wintsignore` file, replace by `HOME_DIR/.wints/ignore`
- Ignored Urls from `.wints.yaml` are move to  `.wints/options.yaml`

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
