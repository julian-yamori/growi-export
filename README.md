# growi_export

GROWI から export したページの .json を .md に変換するツール。

開発の参考にするページ : [GROWIでMarkdownファイルを一括エクスポートする #Python - Qiita](https://qiita.com/plumchang/items/ef078bdc9e6a18e3f2f4)
ただし添付ファイルのエクスポートには未対応 (attachmemnts.json の構造が v5.1.0 と違う)

## 利用方法

### 1. GROWI のデータをアーカイブ

[データのアーカイブ | GROWI Docs](https://docs.growi.org/ja/admin-guide/management-cookbook/export.html)

アーカイブの対象に以下を指定する (GROWI 7.2.5 時点)

- MongoDB Page Collections
  - [x] Pages
  - [x] Revisions

### 2. zip を展開

### 3. (準備中)

## DevContainer の設定手順

1. 空の `docker-compose.override.yml` を作成
2. `devcontainer.json.default` を `devcontainer.json` にリネームし、必要に応じて編集
3. VSCode の Dev Containers でコンテナを起動
