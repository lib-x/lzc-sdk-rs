use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

#[test]
fn vendored_proto_manifest_matches_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let recorded = read_manifest(&root.join("proto/MANIFEST.sha256"));
    let actual = digest_files(collect_proto_files(
        &root.join("proto"),
        &root.join("proto"),
    ))
    .into_iter()
    .map(|(relative, digest)| (Path::new("proto").join(relative), digest))
    .collect::<BTreeMap<_, _>>();

    assert_eq!(
        actual.len(),
        42,
        "expected 41 official protos plus hserver.proto"
    );
    assert_eq!(recorded, actual, "manifest must match every vendored proto");
}

#[test]
fn source_manifest_covers_every_upstream_input() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let recorded = read_manifest(&root.join("proto/SOURCE_MANIFEST.sha256"));

    assert_eq!(recorded.len(), 42, "source manifest must cover all inputs");
    assert!(
        recorded.contains_key(Path::new("baseos/hserver.proto")),
        "source manifest must identify the BaseOS dependency"
    );
    assert_eq!(
        recorded
            .keys()
            .filter(|path| path.starts_with("common"))
            .count(),
        13
    );
    assert_eq!(
        recorded
            .keys()
            .filter(|path| path.starts_with("localdevice"))
            .count(),
        15
    );
    assert_eq!(
        recorded
            .keys()
            .filter(|path| path.starts_with("sys"))
            .count(),
        12
    );
    assert_eq!(
        recorded
            .keys()
            .filter(|path| path.starts_with("dlna"))
            .count(),
        1
    );
}

fn read_manifest(path: &Path) -> BTreeMap<PathBuf, String> {
    let manifest =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    manifest
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (digest, relative) = line
                .split_once("  ")
                .unwrap_or_else(|| panic!("invalid manifest line: {line}"));
            (PathBuf::from(relative), digest.to_owned())
        })
        .collect()
}

fn collect_proto_files(root: &Path, relative_to: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    collect_proto_files_into(root, relative_to, &mut files);
    files
}

fn collect_proto_files_into(
    path: &Path,
    relative_to: &Path,
    files: &mut BTreeMap<PathBuf, Vec<u8>>,
) {
    let mut entries = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|error| panic!("read {} entry: {error}", path.display()));
    entries.sort_by_key(fs::DirEntry::file_name);

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            collect_proto_files_into(&path, relative_to, files);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "proto")
        {
            let relative = path
                .strip_prefix(relative_to)
                .unwrap_or_else(|error| panic!("relative path for {}: {error}", path.display()));
            let content =
                fs::read(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            files.insert(relative.to_owned(), content);
        }
    }
}

fn digest_files(files: BTreeMap<PathBuf, Vec<u8>>) -> BTreeMap<PathBuf, String> {
    files
        .into_iter()
        .map(|(relative, content)| (relative, format!("{:x}", Sha256::digest(content))))
        .collect()
}
