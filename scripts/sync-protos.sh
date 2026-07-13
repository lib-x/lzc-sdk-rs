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

sdk_source="${LZC_SDK_SOURCE:-https://gitee.com/linakesi/lzc-sdk.git}"
sdk_revision="${LZC_SDK_REVISION:-81adfc8bfa8a46212ceed4cf9c5e4d675f9a0458}"
baseos_source="${LZC_BASEOS_SOURCE:-https://gitee.com/linakesi/lzc-baseos-protos.git}"
baseos_revision="${LZC_BASEOS_REVISION:-d8d3d3375144}"
baseos_module_version="${LZC_BASEOS_MODULE_VERSION:-v0.0.0-20240409034726-d8d3d3375144}"

stage="$(mktemp -d "${root}/.proto-sync.XXXXXX")"
cleanup() {
  chmod -R u+w "${stage}" 2>/dev/null || true
  rm -rf "${stage}"
}
trap cleanup EXIT

clone_revision() {
  local source="$1"
  local revision="$2"
  local destination="$3"

  if [[ -d "${source}" ]] && ! git -C "${source}" rev-parse --git-dir >/dev/null 2>&1; then
    mkdir -p "${destination}"
    cp -a "${source}/." "${destination}/"
    return
  fi

  git clone --quiet --filter=blob:none --no-checkout "${source}" "${destination}"
  git -C "${destination}" checkout --quiet --detach "${revision}"
}

sdk_checkout="${stage}/sdk-source"
baseos_checkout="${stage}/baseos-source"
clone_revision "${sdk_source}" "${sdk_revision}" "${sdk_checkout}"
clone_revision "${baseos_source}" "${baseos_revision}" "${baseos_checkout}"

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

  package="$(sed -nE 's/^package[[:space:]]+([^;]+);/\1/p' "${source}")"
  if [[ -z "${package}" ]]; then
    echo "missing protobuf package declaration: ${source_relative}" >&2
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
  printf -- '- BaseOS module: `gitee.com/linakesi/lzc-baseos-protos@%s`\n' "${baseos_module_version}"
  printf -- '- BaseOS commit: `%s`\n' "${baseos_revision}"
  printf '\nFiles are placed under protobuf package directories, imports are rewritten accordingly, and `buf format` is applied. Message, enum, field, and service definitions remain unchanged.\n'
  printf '\nRun `./scripts/sync-protos.sh` to refresh this tree. This maintenance command requires Git, Buf, and network access unless source overrides are provided. SDK builds do not run it.\n'
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
