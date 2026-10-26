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

sdk_checkout="${LZC_SDK_SOURCE:-${root}/upstream/lzc-sdk}"
baseos_checkout="${LZC_BASEOS_SOURCE:-${root}/upstream/lzc-baseos-protos}"

for checkout in "${sdk_checkout}" "${baseos_checkout}"; do
  if [[ ! -e "${checkout}/.git" ]]; then
    echo "missing upstream Git checkout: ${checkout}" >&2
    echo "run: git submodule update --init" >&2
    exit 1
  fi
done

sdk_revision="$(git -C "${sdk_checkout}" rev-parse HEAD)"
baseos_revision="$(git -C "${baseos_checkout}" rev-parse HEAD)"

check_proto_sources() {
  local checkout="$1"
  shift
  if [[ -n "$(git -C "${checkout}" status --porcelain --untracked-files=all --ignored -- "$@")" ]]; then
    echo "uncommitted protobuf changes in upstream checkout: ${checkout}" >&2
    echo "commit the protobuf changes or use a clean checkout before synchronizing" >&2
    exit 1
  fi
}

check_proto_sources "${sdk_checkout}" \
  'protos/common/*.proto' 'protos/dlna/*.proto' \
  'protos/localdevice/*.proto' 'protos/sys/*.proto'
check_proto_sources "${baseos_checkout}" baseos/hserver.proto

stage="$(mktemp -d "${root}/.proto-sync.XXXXXX")"
cleanup() {
  chmod -R u+w "${stage}" 2>/dev/null || true
  rm -rf "${stage}"
}
trap cleanup EXIT

if [[ ! -d "${sdk_checkout}/protos" ]]; then
  echo "official SDK revision has no protos directory" >&2
  exit 1
fi
if [[ ! -f "${baseos_checkout}/baseos/hserver.proto" ]]; then
  echo "BaseOS revision has no baseos/hserver.proto" >&2
  exit 1
fi

mkdir -p "${stage}/proto"
mapping="${stage}/import-map.tsv"
: > "${mapping}"
source_manifest="${stage}/source-manifest.unsorted"
: > "${source_manifest}"

sources=()
while IFS= read -r -d '' source; do
  sources+=("${source}")
done < <(
  find \
    "${sdk_checkout}/protos/common" \
    "${sdk_checkout}/protos/dlna" \
    "${sdk_checkout}/protos/localdevice" \
    "${sdk_checkout}/protos/sys" \
    -type f -name '*.proto' -print0 | sort -z
)
sources+=("${baseos_checkout}/baseos/hserver.proto")

for source in "${sources[@]}"; do
  if [[ "${source}" == "${sdk_checkout}/protos/"* ]]; then
    source_relative="${source#"${sdk_checkout}/protos/"}"
  else
    source_relative="baseos/hserver.proto"
  fi

  if [[ ! "${source_relative}" =~ ^[A-Za-z0-9_./-]+\.proto$ ]]; then
    printf 'unsupported protobuf source path: %q\n' "${source_relative}" >&2
    exit 1
  fi

  package="$(sed -nE 's/^package[[:space:]]+([^;]+);/\1/p' "${source}")"
  if [[ -z "${package}" ]]; then
    echo "missing protobuf package declaration: ${source_relative}" >&2
    exit 1
  fi
  if [[ ! "${package}" =~ ^[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]*)*$ ]]; then
    echo "invalid protobuf package declaration: ${source_relative}" >&2
    exit 1
  fi
  package_directory="${package//./\/}"
  destination_relative="${package_directory}/$(basename "${source}")"
  destination="${stage}/proto/${destination_relative}"
  if [[ -e "${destination}" ]]; then
    echo "duplicate standard protobuf destination: ${destination_relative}" >&2
    exit 1
  fi

  mkdir -p "$(dirname "${destination}")"
  cp "${source}" "${destination}"
  chmod u+w "${destination}"
  printf '%s\t%s\n' "${source_relative}" "${destination_relative}" >> "${mapping}"
  printf '%s  %s\n' "$(sha256sum "${source}" | cut -d ' ' -f 1)" "${source_relative}" \
    >> "${source_manifest}"
done

while IFS=$'\t' read -r source_relative destination_relative; do
  escaped_source="${source_relative//./\\.}"
  while IFS= read -r -d '' proto; do
    sed -i \
      "s#import \"${escaped_source}\";#import \"${destination_relative}\";#" \
      "${proto}"
  done < <(find "${stage}/proto" -type f -name '*.proto' -print0)
done < "${mapping}"

buf format "${stage}/proto" -w
buf format "${stage}/proto" -w
buf format "${stage}/proto" --diff --exit-code
sort -k2 "${source_manifest}" > "${stage}/proto/SOURCE_MANIFEST.sha256"

{
  printf '# Vendored LazyCat Protocol Sources\n\n'
  printf -- '- Official SDK: `https://gitee.com/linakesi/lzc-sdk`\n'
  printf -- '- Official SDK commit: `%s`\n' "${sdk_revision}"
  printf -- '- BaseOS: `https://gitee.com/linakesi/lzc-baseos-protos`\n'
  printf -- '- BaseOS commit: `%s`\n' "${baseos_revision}"
  printf '\nFiles are placed under protobuf package directories, imports are rewritten accordingly, and `buf format` is applied. Message, enum, field, and service definitions remain unchanged.\n'
  printf '\nUpstream commits are pinned by the Git submodules under `upstream/`. Run `git submodule update --init` once, then `./scripts/sync-protos.sh` to refresh this tree offline. To advance the pins, run `git submodule update --remote upstream/lzc-sdk upstream/lzc-baseos-protos`, sync, and regenerate with `./scripts/generate.sh`. SDK builds do not run these maintenance commands.\n'
} > "${stage}/proto/UPSTREAM.md"

(
  cd "${stage}"
  find proto -type f -name '*.proto' -print0 \
    | sort -z \
    | xargs -0 sha256sum
) > "${stage}/proto/MANIFEST.sha256"

if [[ "${mode}" == "check" ]]; then
  diff -ruN "${root}/proto" "${stage}/proto"
  exit 0
fi

mkdir -p "${root}/proto"
for directory in cloud io lzc common dlna localdevice sys baseos; do
  rm -rf "${root}/proto/${directory}"
done
for directory in cloud io lzc; do
  cp -a "${stage}/proto/${directory}" "${root}/proto/${directory}"
done
cp "${stage}/proto/UPSTREAM.md" "${root}/proto/UPSTREAM.md"
cp "${stage}/proto/MANIFEST.sha256" "${root}/proto/MANIFEST.sha256"
cp "${stage}/proto/SOURCE_MANIFEST.sha256" "${root}/proto/SOURCE_MANIFEST.sha256"
