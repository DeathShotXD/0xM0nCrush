# Changelog

Notable changes to 0xM0nCrush, newest first.

## Unreleased

### Added

- Continuous integration that builds and runs the tests on Windows.
- Tests for the target list parser.
- Detection and compatibility notes under docs/.
- Driver lifecycle and unloading notes under docs/.

### Fixed

- The temporary driver service is now stopped and deleted before every
  exit path, including dry-run and driver-open failures. Previously,
  `process::exit` skipped `DriverService::drop`, leaving the service
  loaded.
- Cleanup now only stops and deletes a service that this run created. A
  pre-existing MonProcessEX service is reused and left in place.
- Clarified that `-n` accepts process names, not `.sys` paths, and that a
  driver already loaded before launch is reused rather than unloaded.

## 1.0.0

### Added

- Cross-version kernel-mode process terminator using the signed
  MonProcessEX.sys driver through a single IOCTL.
- Dry-run, JSON output, repeat loop with jitter, delay, self-destruct,
  and custom service names.
- Target lists from the command line, a config file, or the built-in set.
- Driver install, run, and cleanup through the Service Control Manager,
  leaving no persistent artifact.
