# Truce CLAP compatibility patch

`truce-clap` 6.3.0 is vendored unchanged except for a host parameter rescan at the end of `state_load` in `src/lib.rs`. Restoring parameters must notify the host with `CLAP_PARAM_RESCAN_VALUES`; without it clap-validator fails its three state reproducibility checks. This is a main-thread notification only. Remove the override when an upstream release includes it.

Upstream: https://github.com/truce-audio/truce . Original package metadata and license are retained.
