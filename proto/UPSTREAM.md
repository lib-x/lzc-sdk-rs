# Vendored LazyCat Protocol Sources

- Official SDK: `https://gitee.com/linakesi/lzc-sdk`
- Official SDK commit: `81adfc8bfa8a46212ceed4cf9c5e4d675f9a0458`
- BaseOS module: `gitee.com/linakesi/lzc-baseos-protos@v0.0.0-20240409034726-d8d3d3375144`
- BaseOS commit: `d8d3d3375144`

Files are placed under protobuf package directories, imports are rewritten accordingly, and `buf format` is applied. Message, enum, field, and service definitions remain unchanged.

Run `./scripts/sync-protos.sh` to refresh this tree. This maintenance command requires Git, Buf, and network access unless source overrides are provided. SDK builds do not run it.
