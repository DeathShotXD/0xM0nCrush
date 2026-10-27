<p align="center">
  <img src="assets/hero.svg" alt="0xM0nCrush - kernel-mode process termination" width="100%">
</p>

<p align="center">
  <a href="docs/compatibility.md"><img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0b0f14?style=flat-square&amp;logo=windows&amp;logoColor=c8ff3d" alt="Windows 10 and 11"></a>
  <img src="https://img.shields.io/badge/Rust-2021-0b0f14?style=flat-square&amp;logo=rust&amp;logoColor=c8ff3d" alt="Rust 2021">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-0b0f14?style=flat-square" alt="MIT license"></a>
  <img src="https://img.shields.io/badge/BYOVD-research%20only-a78bfa?style=flat-square" alt="BYOVD research only">
  <a href="https://github.com/DeathShotXD/0xM0nCrush/stargazers"><img src="https://img.shields.io/github/stars/DeathShotXD/0xM0nCrush?style=flat-square&amp;color=c8ff3d" alt="GitHub stars"></a>
</p>

<p align="center">
  <strong>Cross-version Windows process termination from kernel context.</strong><br>
  One signed driver. One IOCTL. No offsets. No PDB.
</p>

A cross-version Windows process terminator. It loads a signed HONOR
kernel driver (`MonProcessEX.sys`), resolves the PID of every target
process, and terminates it from kernel context through a single IOCTL.
No kernel offsets, no PDB downloads, no build-specific shellcode - the
technique works identically on every Windows 10 and Windows 11 build.

The tool is a single self-contained executable. When it needs the driver,
it installs it through the Service Control Manager, performs the kill, and
stops and deletes the temporary service before exiting. If `MonProcessEX.sys`
is already loaded, it reuses the existing driver and leaves it in place.
Targets are configurable at runtime through a config file, command line, or
the built-in defaults.

> **Cross-version by design.** One driver, one IOCTL, one kill
> primitive. Works on all Windows 10 and Windows 11 builds.

## Quick start

```
1. Keep moncrush.exe and MonProcessEX.sys in the same folder.
2. Run from an elevated shell.

   moncrush.exe -n "notepad.exe,calc.exe"

3. Targets die. The temporary driver service is cleaned up. Done.
```

No toolchain, no offsets, no build step.

## Demo

<p align="center">
  <img src="assets/demo.gif" alt="0xM0nCrush demonstration" width="800">
</p>

## Features

| Feature | Details |
|---------|---------|
| Cross-version | Works on all Windows 10 and Windows 11 builds, no offsets |
| Kernel-mode kill | Driver terminates the PID from kernel context |
| PPL bypass | `MonProcessEX.sys` kill path bypasses protected-process checks |
| Signed driver | `MonProcessEX.sys` is a real signed HONOR driver |
| Not in MS block rules | Absent from Microsoft's vulnerable-driver block rules |
| Self-sufficient | Temporary driver installed, started, and cleaned up via SCM; a preloaded driver is reused as-is |
| Zero dependencies | Static Rust binary; drop exe + driver, run |
| Configurable | `targets.conf` or `-n`, no recompile needed |
| Obfuscated | Device path and target list encrypted at rest |
| Single executable | One binary; console output from a shell, silent when double-clicked |
| Dry-run mode | Enumerate targets and PIDs before committing |
| Jittered loop | `--repeat` re-checks with randomized interval |
| Exit codes + JSON | C2-friendly automation interface |

## How it works

```
+-------------------------------------------------------------------------------------------+
| USER MODE                                                                                 |
|                                                                                           |
|   moncrush.exe                                                                            |
|                                                                                           |
|   +-------------------+      +-------------------+      +---------------------+           |
|   |   enumerate all   |      |   resolve target  |      |   match against     |           |
|   |   running         |  ->  |   PID via process |  ->  |   target list,      |           |
|   |   processes       |      |   entry           |      |   collect PIDs      |           |
|   +-------------------+      +-------------------+      +----------+----------+           |
|                                                                         |                 |
|                                     CreateFileW("\.\MonProcessEX")      |                 |
|                                     DeviceIoControl(IOCTL 0x22400C)     |                 |
|                                     output = termination status         v                 |
+-------------------------------------------------------------------------------------------+
| KERNEL MODE                                                                               |
|                                                                                           |
|   MonProcessEX.sys                                        signed HONOR kernel driver      |
|   +---------------------------------------------------------------------------------+     |
|   |                                                                                 |     |
|   |   IOCTL 0x22400C  ->  PID termination dispatch                                  |     |
|   |        |                                                                        |     |
|   |        |  kernel-mode process lookup                                            |     |
|   |        v                                                                        |     |
|   |   EPROCESS located -> terminated from kernel context                            |     |
|   |        |                                                                        |     |
|   |        v                                                                        |     |
|   |   process exit path invoked                                                     |     |
|   |                                                                                 |     |
|   +---------------------------------------------------------------------------------+     |
|                                                                                           |
|   CLEANUP                                                                                 |
|   +---------------------------------------------------------------------------------+     |
|   |   temporary service stopped + deleted (when this run created it)                |     |
|   |   preloaded driver reused as-is; never force-unloaded                           |     |
|   +---------------------------------------------------------------------------------+     |
+-------------------------------------------------------------------------------------------+
```

<p align="center">
  <img src="assets/banner.jpeg" alt="0xM0nCrush kernel execution architecture" width="800">
</p>

The driver exposes a kill IOCTL that terminates a process given its PID.
The user-mode component enumerates running processes, resolves each
target's PID, and submits it through the device interface. No kernel
structures are touched from user mode, so the technique is immune to
Windows version changes.

## Build

```powershell
cargo build --release --target x86_64-pc-windows-gnu
```

The release profile enables LTO and a single codegen unit. The project is
self-contained with its own `[workspace]` declaration.

## Usage

```
moncrush.exe [options]

  -s, --silent           suppress all console output
  -r, --repeat           keep running, re-check targets
  -d, --dry-run          enumerate targets without killing
  -j, --json             machine-readable JSON output
  -l, --list             print target names and exit
  -v, --version          print version and exit
  -x, --self-destruct    delete self after successful run
      --no-check         skip VM and debugger checks
      --delay <ms>       sleep before executing
      --jitter <ms>      randomize repeat interval
      --max-attempts <n> stop after n kill passes (0=infinite)
      --svc <name>       custom service name
      --driver <path>    custom driver file path
  -n, --names <csv>      comma-separated target list override
  -c, --config <path>    load targets from config file
  -h, --help             show this help
```

Exit codes: `0` ok, `2` no targets, `3` driver failed, `5` environment
abort. Target resolution order: `--names` > `--config` > `targets.conf`
(disk) > built-in defaults.

## Driver lifecycle and unloading

0xM0nCrush terminates processes, not drivers. `-n` / `--names` expects
process names, so `moncrush.exe -n "example.sys"` does not unload
`example.sys`; it looks for a running process with that name.

```powershell
moncrush.exe -n "notepad.exe,calc.exe"      # correct: process names
```

When this run creates the temporary service for `MonProcessEX.sys`, the
loader stops and deletes that service before exiting. A driver that was
already loaded before launch is reused and left in place; it is not
force-unloaded. A service that already existed is never deleted.

To remove another driver, control its service from an elevated shell:

```powershell
sc.exe query type= driver state= all
sc.exe stop "<serviceName>"
sc.exe delete "<serviceName>"
```

`sc.exe stop` can fail while the driver is in use; `sc.exe delete` removes
the service registration and may require a reboot. Full details:
[docs/driver-unloading.md](docs/driver-unloading.md).

### Operational hardening

- **Environment checks.** Verifies the system is not a common
  virtualization environment before loading the driver. Bypass with
  `--no-check` when testing inside a VM.
- **Single instance.** A named mutex prevents two concurrent runs from
  racing IOCTLs into the driver.
- **Delayed execution.** `--delay <ms>` sleeps before doing anything,
  breaking time-correlation with initial execution.
- **Driver hygiene.** The driver is installed under a randomized service
  name, and only a service created by this run is stopped and deleted
  before exit. A pre-existing driver or service is reused and left
  untouched.
- **Self-destruct.** `-x` deletes the executable and purges its Prefetch
  entry after a successful run.

## Configuration

The target list is fully configurable without recompiling:

**Config file.** Drop a `targets.conf` next to the executable, one
process name per line. Lines starting with `#` are ignored:

```
MsMpEng.exe
csfalconservice.exe
SentinelAgent.exe
cortex_agent.exe
```

A template ships as `targets.example.conf`.

**Command line.** `moncrush.exe -n "MsMpEng.exe,csfalconservice.exe"`

**Built-in defaults.** With no config and no flags, the built-in set is:

- calc.exe
- notepad.exe
- MsMpEng.exe
- MpDefenderCoreService.exe
- SecurityHealthService.exe
- MsSense.exe
- SenseIR.exe
- SenseCncProxy.exe
- SenseSampleUploader.exe

## Detection

Signs of this tool on a host, and how to hunt them, are in
[docs/detection.md](docs/detection.md): the driver load through the Service
Control Manager, the device handle and the IOCTL, the randomized service
name, and the cleanup trail.

## Compatibility

Verified builds and blocklist status are recorded in
[docs/compatibility.md](docs/compatibility.md).

## Documentation

- [docs/detection.md](docs/detection.md)
- [docs/compatibility.md](docs/compatibility.md)
- [docs/driver-unloading.md](docs/driver-unloading.md)

## Credits

- HONOR for the signed driver
- The LOLDDrivers project for cataloging signed vulnerable drivers
- BlackSnufkin for the original Ksapi64-Killer reproduction this builds on

## License

MIT. See [LICENSE](LICENSE).

## Disclaimer

This project is published for research and authorized testing only.
Loading unsigned or vulnerable drivers into a system you do not own is
illegal in most jurisdictions. You are responsible for compliance with
all applicable laws and with the authorization scope of the systems you
test.