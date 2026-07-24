# Round 3 Source Evidence

`teaql-vibe-kit-round3.tar.gz` is a Git archive of
`app-playground-round3` from `teaql/teaql-agent-kit` commit `8069ca8`.

- Source branch: `autonomous`
- Source commit: `8069ca81ba2a50b5d1e466ed0c2f5fe1a520e302`
- Pre-removal HEAD: `87156bb` (no changes under `app-playground-round3`
  since the source commit)
- Tracked files: 65
- Archive size: approximately 92 KiB
- SHA-256:
  `35e611ac3a4ff6190d393afdc577e6e4d351cbe27fc92b7d1efcee7b8093c3ca`

The archive contains the Round 3 school model, model review, generated TeaQL
Rust source, customer-owned Q/E scenario, integration test, and run report.
Build output under ignored `target/` directories is intentionally excluded.

Verify the artifact with:

```bash
shasum -a 256 -c SHA256SUMS
```
