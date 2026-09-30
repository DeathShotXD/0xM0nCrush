# Changelog

Notable changes to 0xM0nCrush, newest first.

## Unreleased

### Added

- Continuous integration that builds and runs the tests on Windows.
- Tests for the target list parser.
- Detection and compatibility notes under docs/.

## 1.0.0

### Added

- Cross-version kernel-mode process terminator using the signed
  MonProcessEX.sys driver through a single IOCTL.
- Dry-run, JSON output, repeat loop with jitter, delay, self-destruct,
  and custom service names.
- Target lists from the command line, a config file, or the built-in set.
- Driver install, run, and cleanup through the Service Control Manager,
  leaving no persistent artifact.
