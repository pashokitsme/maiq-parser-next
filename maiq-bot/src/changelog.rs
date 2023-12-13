use include_dir::include_dir;
use include_dir::*;

const CHANGELOG_NOTES: Dir<'_> = include_dir!("$OUT_DIR/../.changes");

pub fn changelog_len() -> usize {
  CHANGELOG_NOTES.files().count()
}

pub fn changelog_names() -> Vec<(usize, String)> {
  CHANGELOG_NOTES
    .files()
    .enumerate()
    .filter_map(|(idx, entry)| {
      entry
        .path()
        .file_stem()
        .and_then(|os| os.to_str())
        .map(|name| (idx, name.to_string()))
    })
    .collect()
}

pub fn changelog_by_index(index: usize) -> Option<String> {
  CHANGELOG_NOTES
    .files()
    .nth(index)
    .and_then(|file| file.contents_utf8().map(Into::into))
}
