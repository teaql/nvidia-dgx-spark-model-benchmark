# Round 4 Source Evidence

`teaql-vibe-kit-round4.tar.gz` is a Git archive of `app-playground` from
`teaql/teaql-agent-kit` commit `8069ca8`.

- Source branch: `autonomous`
- Source commit: `8069ca81ba2a50b5d1e466ed0c2f5fe1a520e302`
- Pre-removal HEAD: `87156bb` (no changes under `app-playground` since the
  source commit)
- Tracked files: 2,702
- Archive size: approximately 5.4 MiB
- SHA-256:
  `a531fd50d96916c506a1e2a188f4f4da3e71e5e942144f9290079c57defd564a`

The archive contains the 223-object moving-company model, model review,
generated TeaQL Rust core and console source, customer-owned cross-workbench
Q/E scenario, tests, and run report. Ignored Cargo `target/` build output and
the transient 53 MiB SQL log are intentionally excluded.

Verify the artifact with:

```bash
shasum -a 256 -c SHA256SUMS
```
