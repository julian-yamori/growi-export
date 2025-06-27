# growi_export

GROWI から export したページの JSON を Markdown に変換するツールの Rust 実装。

開発の参考にしたページ : [GROWIでMarkdownファイルを一括エクスポートする #Python - Qiita](https://qiita.com/plumchang/items/ef078bdc9e6a18e3f2f4)
ただし添付ファイルのエクスポートには未対応 (attachmemnts.json の構造が v5.1.0 と違う)

## 利用方法

実行ファイルは未作成です。`cargo run` で実行してください。

### 1. GROWI のデータをアーカイブ

[データのアーカイブ | GROWI Docs](https://docs.growi.org/ja/admin-guide/management-cookbook/export.html)

アーカイブの対象に以下を指定する (GROWI 7.2.5 時点)

- MongoDB Page Collections
  - [x] Pages
  - [x] Revisions

### 2. zip を展開

### 3. ファイルの配置

展開した zip 内の `pages.json` と `revisions.json` を `./input` ディレクトリ内に配置してください。

Markdorn ファイルは `./output` ディレクトリに生成されます (空にしておいてください)。

### 4. プログラムを実行

以下のコマンドで、JSON ファイルを元に Markdown ファイルが生成されます。

```bash
cargo run
```

## DevContainer の設定手順

1. 空の `docker-compose.override.yml` を作成
2. `devcontainer.json.default` を `devcontainer.json` にリネームし、必要に応じて編集
3. VSCode の Dev Containers でコンテナを起動
