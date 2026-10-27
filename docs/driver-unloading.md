# Driver lifecycle and unloading

0xM0nCrush is a process terminator. It is not a generic kernel-driver
unloader.

## What `-n` accepts

`-n` / `--names` accepts **process names**, not driver files:

```powershell
moncrush.exe -n "notepad.exe,calc.exe"
moncrush.exe -n "MsMpEng.exe,csfalconservice.exe"
```

This is invalid because `example.sys` is a driver image, not a running
process:

```powershell
moncrush.exe -n "example.sys"
```

If you want to terminate the process that uses a driver, target the
process name. If you want to remove the driver service itself, use the
Service Control Manager as shown below.

## 0xM0nCrush's own driver

When this run installs `MonProcessEX.sys`, the loader creates a temporary
kernel service, starts it, opens the device, performs the kill, and then
stops and deletes the service before the process exits. The cleanup is
explicit, so it also runs on the dry-run and driver-open error paths.
`DriverService` records whether the service was created by this run; only
a service created here is stopped and deleted.

If `MonProcessEX` is already loaded before 0xM0nCrush starts, the tool
reuses the existing driver and does **not** force-unload it. If a service
with the selected name already exists but the device is not available,
the loader may start that service, but it will not delete it. A
pre-existing service is left in place because another component on the
host may own it.

## Unloading another driver

A kernel driver is not a process. It cannot be "terminated" with
`taskkill` or with `moncrush.exe -n`. You control its service:

```powershell
# List kernel-driver services
sc.exe query type= driver state= all

# Stop the service and ask the driver to unload
sc.exe stop "<serviceName>"

# Delete the service registration
sc.exe delete "<serviceName>"
```

Run these from an elevated shell. Depending on the driver's design and
whether its device handles are open:

- `sc.exe stop` may return an error if the driver does not implement
  unload or if the device is still in use.
- `sc.exe delete` removes the service configuration; a running driver
  may remain loaded until its handles close or the system reboots.
- A reboot is the reliable fallback when a driver cannot be stopped.

This is service control, not a generic kernel-driver kill primitive.
Do not unload a driver that the system or another security product is
actively using. Test on a disposable VM that you own or are authorized
to modify.
