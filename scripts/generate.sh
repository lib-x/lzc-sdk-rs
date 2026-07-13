#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mode="write"
if [[ "${1:-}" == "--check" ]]; then
  mode="check"
elif [[ $# -gt 0 ]]; then
  echo "usage: $0 [--check]" >&2
  exit 2
fi

tools_root="${root}/.tools"
tools_bin="${tools_root}/bin"
mkdir -p "${tools_bin}"

install_plugin() {
  local package="$1"
  local binary="$2"
  if [[ -x "${tools_bin}/${binary}" ]]; then
    return
  fi
  cargo install --locked --root "${tools_root}" --version 0.5.0 "${package}"
}

install_plugin protoc-gen-prost protoc-gen-prost
install_plugin protoc-gen-tonic protoc-gen-tonic
install_plugin protoc-gen-prost-crate protoc-gen-prost-crate

export PATH="${tools_bin}:${PATH}"
cd "${root}"
buf format --diff --exit-code
buf lint
buf build >/dev/null

if [[ "${mode}" == "write" ]]; then
  buf generate
  cargo fmt --all
  exit 0
fi

stage="$(mktemp -d "${root}/.codegen-check.XXXXXX")"
cleanup() {
  rm -rf "${stage}"
}
trap cleanup EXIT
relative_stage="${stage#${root}/}"
sed "s#out: src/gen#out: ${relative_stage}/gen#g" buf.gen.yaml > "${stage}/buf.gen.yaml"
buf generate --template "${stage}/buf.gen.yaml"
diff -ruN "${root}/src/gen" "${stage}/gen"
