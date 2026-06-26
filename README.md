# honyaku

OpenAI 互換の LLM API を使った、シンプルな日英翻訳コマンドラインアプリです。

## 必要なもの

- Rust ツールチェイン
- OpenAI 互換の LLM API エンドポイント（ローカルの LM Studio など）

## セットアップ

```bash
git clone <repository>
cd honyaku
cargo build --release
```

環境変数は `dot.honyaku-env` を参考に設定します。以下のいずれかの方法で読み込みます。

```bash
# シェル環境変数として読み込む
export $(grep -v '^#' dot.honyaku-env | xargs)

# または --env でファイルを明示的に指定する
honyaku --env ./dot.honyaku-env "こんにちは"
```

`honyaku` はカレントディレクトリの `.env` を自動では読みません。`$HOME/.env` と `--env <FILE>` のみを参照します。

## 使い方

### 自動判定

```bash
export $(grep -v '^#' dot.honyaku-env | xargs)
honyaku "こんにちは"
# => Hello
```

### 方向を指定

```bash
# 日本語 → 英語
honyaku --je "こんにちは"

# 英語 → 日本語
honyaku --ej "Hello, world!"
```

### 標準入力

```bash
echo "Hello, world!" | honyaku
# => こんにちは、世界!
```

標準入力の文字コードは既定で自動判定します。BOM 付き UTF-8/UTF-16、UTF-8 を優先し、Windows では CP932/Shift_JIS にもフォールバックします。

```bash
honyaku --stdin-encoding utf-8 < input.txt
honyaku --stdin-encoding cp932 < input.txt
```

### 特定の env ファイルを使う

```bash
honyaku --env ./production.env "Hello"
```

## 環境変数

| 変数名 | 説明 | 例 |
|---|---|---|
| `HONYAKU_API_KEY` | API キー | `sk-abracadabra` |
| `HONYAKU_ENDPOINT` | OpenAI 互換 API のベース URL | `http://127.0.0.1:1234/v1` |
| `HONYAKU_MODEL` | 使用するモデル名 | `qwen2.5-1.5b-instruct` |

## 自動判定のルール

入力テキストの日本語文字割合で翻訳方向を決めます。

- ひらがな・カタカナ・漢字が **70% 以上** → 日本語として **英語へ翻訳**
- 日本語文字が **含まれていない** → 英語として **日本語へ翻訳**
- それ以外（混在など）→ LLM に言語を尋ねてから翻訳方向を決定

## ライセンス

MIT
