# Security Policy

## Reporting a vulnerability

This repository contains offensive security research software. If you have
identified a security issue in the code, a bypass, or a problem with the
disclosure of the bundled driver, report it privately before opening a
public issue.

Open a GitHub security advisory via the repository's Security tab, or
contact the maintainer directly through the profile on GitHub. Do not
share exploit details publicly until a fix or mitigation is published.

## Scope

- Vulnerabilities in the source code in this repository
- Incorrect handling of the bundled driver
- Anything that would cause unexpected behavior on a system where this
  tool is run legitimately during an authorized engagement

## Response

- Acknowledgment within 48 hours
- Status update within 5 business days
- Coordinated disclosure preferred

## Out of scope

- The bundled `MonProcessEX.sys` driver itself is a third-party signed driver
  and is documented as a known-vulnerable driver. Report driver issues
  through the LOLDDrivers project or the driver vendor.
