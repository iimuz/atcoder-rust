---
title: AtCoder using Rust
date: 2023-05-01
lastmod: 2023-05-07
---

## 概要

AtCoder を Rust で書くためのフォルダです。

## ファイル構成

- フォルダ
  - `.vscode`: VSCode の基本設定を記述します。
  - `src`: AtCoder の各コンテストのコードを配置します。内部のコードは、 `cargo-compete` を利用して生成します。
- ファイル
  - `.gitignore`: [rust 用の gitignore](https://github.com/github/gitignore/blob/main/Rust.gitignore) です。
  - `LICENSE`: ライセンスを記載します。 MIT ライセンスを設定しています。
  - `README.md`: 本ドキュメントです。

## 実行方法

コンテストの問題を特に当たり下記の流れで実行します。
AtCoder の問題を取得したりテンプレートを作成する部分は、[cargo-compete](https://github.com/qryxip/cargo-compete) に依存しています。

```sh
cargo compete new abc111  # src/abc111にコンテストのデータを柵瀬

cd src/abc111
cargo compete open  # ブラウザでコンテストのページを開く

# 問題を解く

cargo compete test a  # a問題のテスト実行
cargo compete submit a  # a問題を提出
```

## code style

TBD

## 問題

- [typicl90](/src/til-20230501/src/typical90): [競プロ典型 90 問](https://atcoder.jp/contests/typical90)
