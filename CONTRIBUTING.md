# Contributing

Contributions are welcome. This project is research software and the
bar for a contribution is: it must be correct, it must be documented,
and it must not reduce the evasion properties of the shipped binary.

## What is useful

- Support for additional Windows builds (offset updates, syscall index
  changes, new service numbers)
- New target process profiles in the config template
- Documentation and README improvements
- Test reports from real Windows builds (include OS build, HVCI on/off,
  and driver behavior)

## Pull request checklist

- Build with no warnings: `cargo build --release --target x86_64-pc-windows-gnu`
- No plaintext-sensitive strings added (device paths, API names, target
  names must stay behind the obfuscation layer)
- Document what was tested and on which Windows build

## New-driver killers

If you want to add a new vulnerable-driver killer, open an issue first
with: driver filename, SHA256, LOLDDrivers link, device path, IOCTL
codes, and what the primitive allows. Only signed, loadable drivers are
accepted.

## License

By contributing you agree that your contributions are licensed under the
same MIT license as the project.
