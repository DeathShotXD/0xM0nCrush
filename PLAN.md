# Maintenance plan

0xM0nCrush is maintained in small, focused commits. One change a day, each
commit doing a single thing and leaving the tree working.

This is a working plan, not a contract.

## Week 1

- Mon - record the supported driver and blocklist status
- Tue - detection notes for the driver load and IOCTL
- Wed - compatibility matrix for Windows 10 and 11 builds
- Thu - configuration parser tests
- Fri - continuous integration on Windows
- Sat - exit code table in the usage section
- Sun - README pass

## Week 2

- Mon - document the service lifecycle and cleanup trail
- Tue - operator notes for driver hygiene
- Wed - JSON output schema
- Thu - dry-run walkthrough
- Fri - troubleshooting guide
- Sat - credits and provenance pass
- Sun - docs pass

## Week 3

- Mon - prefetch cleanup notes
- Tue - custom service name guidance
- Wed - jitter and delay guidance
- Thu - self-destruct notes
- Fri - issue templates
- Sat - pull request template
- Sun - changelog pass

## Week 4

- Mon - release checklist
- Tue - verify the release workflow
- Wed - tag the release
- Thu - README pass
- Fri - detection rule check
- Sat - final review
- Sun - announce

## Commit conventions

- one change per commit, present tense
- subject under 72 characters, no trailing period
- do not mix refactors with behaviour changes

## Daily checklist

1. Pick the next item.
2. Make the smallest change that completes it.
3. Build and test.
4. Commit with a plain message.
5. Push to main.
