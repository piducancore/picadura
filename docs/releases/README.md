# Building and publishing

CI runs the same reusable build workflow on Windows x64, macOS Apple Silicon,
and Linux x64. Every platform runs tests and allocation checks, validates the
exact VST3/CLAP bundles, checks the dependency notice inventory, and uploads a
ZIP as a workflow artifact.

To release:

1. Update the Cargo version, lockfile, `docs/releases/VERSION.md`, and notice
   inventory when dependencies change.
2. Commit and push main, then create and push an annotated `vVERSION` tag.
3. The Release workflow checks the version, runs all three build jobs and
   creates a draft prerelease with ZIPs and `SHA256SUMS`.
4. Inspect the draft artifacts and checksums, then publish the prerelease.
5. The Website workflow imports published releases and refreshes GitHub Pages.
   Dispatch `pages.yml` manually after publication for an immediate refresh;
   the daily sync also handles GitHub token event suppression.

Published releases are never overwritten by the workflow. A failed draft build
can be retried with `gh workflow run release.yml -f tag=vVERSION`.

The website lives at https://piducan.dev/picadura/ and reads GitHub Releases
at build time. It never publishes fake download links for missing platforms.

## Notices

`THIRDPARTY.json` starts from cargo-bundle-licenses 4.2.0. Missing texts for
identical dependency versions were recovered from Buffer Uppercut's existing
notice inventory; remaining missing notices were obtained from the exact
upstream source commits recorded in each crate. Slint uses the desktop royalty-
free option, with the required download-page attribution. JetBrains Mono's OFL
is included separately as `FONT-LICENSE.txt`. Original Picadura code is MIT;
dependency licenses remain their own.

Regenerate the inventory with:

```sh
cargo bundle-licenses --format json --output target/THIRDPARTY-discovered.json
node tools/check-notices.mjs docs/releases/THIRDPARTY.json target/THIRDPARTY-discovered.json
```

The notice checker validates completeness and package versions; it is not a
substitute for inspecting newly introduced dependency license terms.
