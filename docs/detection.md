# Detection

How the tool looks on a host, and where to look for it.

## Driver load

The driver is installed as a kernel service through the Service Control
Manager. Watch for a service creation followed immediately by a start,
where the binary path points at a `.sys` file with a randomized service
name.

- Event ID 7045 in the System log: a new service installed, with a
  random name and a `.sys` image path.
- Event ID 4697 in the Security log under the same condition when
  service creation auditing is on.

## Device access

The user-mode component opens the driver device and sends one IOCTL.

- Handle creation on `\\.\MonProcessEX` (the device name is obfuscated in
  the binary, so a string match on disk will not always hit).
- Process termination where the terminating process is not the parent and
  is not a known management agent.

## Cleanup trail

The service is stopped and deleted on exit, so the durable evidence is:

- The 7045/4697 service events and the SCM deletion.
- The Prefetch entry for the loader when self-destruct is not used,
  removed when `-x` runs.
- Process termination telemetry for protected security processes in a
  short window.

## Detection ideas

- Sigma: new kernel service with a random name and a `.sys` path,
  created and started within one second.
- YARA: the `MonProcessEX` device string, the IOCTL constant `0x22400C`,
  and the encoded target list.
- Blocklist: keep the Microsoft vulnerable driver blocklist current;
  the driver has appeared on that list, so the load fails where the
  blocklist is enforced.

## Notes

Detection is defensive work. Treat every signal as a lead and correlate
before you conclude.
