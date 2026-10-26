# Vendored LazyCat Protocol Sources

- Official SDK: `https://gitee.com/linakesi/lzc-sdk`
- Official SDK commit: `f5bd0470167da6df10f5ebb396525d9a12706d7c`
- BaseOS: `https://gitee.com/linakesi/lzc-baseos-protos`
- BaseOS commit: `a8715f13eac24edd3d307d50bde42abc6800b728`

Files are placed under protobuf package directories, imports are rewritten accordingly, and `buf format` is applied. Message, enum, field, and service definitions remain unchanged.

Upstream commits are pinned by the Git submodules under `upstream/`. Run `git submodule update --init` once, then `./scripts/sync-protos.sh` to refresh this tree offline. To advance the pins, run `git submodule update --remote upstream/lzc-sdk upstream/lzc-baseos-protos`, sync, and regenerate with `./scripts/generate.sh`. SDK builds do not run these maintenance commands.
