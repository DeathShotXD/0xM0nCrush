# Compatibility

Builds are tested against the toolchain and the releases below. Nothing
here is a blanket claim; the table records what was observed.

## Toolchain

- Rust stable, target `x86_64-pc-windows-gnu`.
- Cross build from Linux with `mingw-w64`; native build on Windows works
  the same.

## Operating systems

| Version | Result | Notes |
|---------|--------|-------|
| Windows 11 (recent build) | verified | kill path and cleanup observed |
| Windows 10 (recent build) | expected | same IOCTL; not re-run per build |
| Server SKUs | not tested | driver signing and policy differ |

## Blocklist status

The signed driver has appeared on the Microsoft vulnerable driver
blocklist. Where the blocklist is enforced and current, the load fails
and the tool reports a driver failure exit code. Snapshot specific;
verify against the current list before you rely on it.

## Requirements

- Administrator rights to install and start the service.
- The driver file next to the executable, or provided with `--driver`.
- Secure Boot and memory integrity settings can block the load; record
  the state of the host when you document a result.
