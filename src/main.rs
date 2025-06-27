use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::{self, BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, de::DeserializeOwned};

#[derive(Deserialize)]
struct Page {
    #[serde(rename = "_id")]
    id: String,

    path: String,

    #[serde(rename = "isEmpty")]
    is_empty: bool,
}

#[derive(Deserialize)]
struct Revision {
    #[serde(rename = "pageId")]
    page_id: String,

    body: String,
}

fn print_err(message: impl Display) -> io::Result<()> {
    writeln!(io::stderr(), "{message}")
}

fn read_json_file<T>(path: &Path) -> anyhow::Result<T>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

/// `/Sandbox/Bootstrap4` -> `Sandbox/Bootstrap4`
///
/// 先頭に `/` が無ければ、先頭文字も含めて返す
fn trim_first_slash(path: &str) -> &str {
    let mut chars_iter = path.chars();
    match chars_iter.next() {
        Some(first) => {
            if first == '/' {
                // 残りの文字を返す
                chars_iter.as_str()
            } else {
                path
            }
        }
        None => path,
    }
}

fn page_output_path(output_dir: &Path, page: &Page) -> PathBuf {
    let mut page_path = trim_first_slash(&page.path);

    if page_path.is_empty() {
        page_path = "index";
    }

    output_dir.join(format!("{page_path}.md"))
}

fn main() -> anyhow::Result<()> {
    let input_dir = PathBuf::from("./input");
    let output_dir = PathBuf::from("./output");

    // json を読み込む
    let pages = read_json_file::<Vec<Page>>(&input_dir.join("pages.json"))?;
    // WARN : revisions.json を一気に全部読むのは重そう
    let revisions = read_json_file::<Vec<Revision>>(&input_dir.join("revisions.json"))?;

    // Revision を page_id で検索しやすくする
    let revisions_map = revisions
        .into_iter()
        .map(|r| (r.page_id.clone(), r))
        .collect::<HashMap<String, Revision>>();

    for page in &pages {
        if page.is_empty {
            continue;
        }

        // 該当する Revision を取得
        let Some(revision) = revisions_map.get(&page.id) else {
            print_err(format!("Revision not found - pageId = '{}' ", &page.id))?;
            continue;
        };

        // ファイルパスを作成
        let out_file_path = page_output_path(&output_dir, page);

        if out_file_path.exists() {
            print_err(format!(
                "File already exists: '{}'",
                out_file_path.display()
            ))?;
            continue;
        }

        if let Some(out_parent) = out_file_path.parent() {
            // ディレクトリが存在しなければ作成
            if !out_parent.exists() {
                fs::create_dir_all(out_parent)?;
            }
        }

        // ファイルを書き込む
        fs::write(&out_file_path, &revision.body)?;
    }

    Ok(())
}
