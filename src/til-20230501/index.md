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
AtCoder の問題を取得したりテンプレートを作成する部分は、cargo-compete に依存しています。

```sh
cargo compete new abc111  # src/abc111にコンテストのデータを柵瀬

cd src/abc111
cargo compete open  # ブラウザでコンテストのページを開く

# 問題を解く

cargo compete test a  # a問題のテスト実行
cargo compete submit a  # a問題を提出
```

### cargo compete の導入

[cargo-compete](https://github.com/qryxip/cargo-compete) は下記の方法で導入できる。

```sh
cargo install cargo-compete
```

## code style

TBD

## 問題

- [typicl90](/src/til-20230501/src/typical90): [競プロ典型 90 問](https://atcoder.jp/contests/typical90)

## 参考資料

- [AtCoder での実力アップを目指そう！ ～競プロ典型 90 問～](https://qiita.com/e869120/items/1b2a5f0f07fd927e44e9): 基本的な問題を解くために良い問題集を探していた時に見つけた。
- [[2023 年 1 月版]競技プログラミングを始めたばかりの人にオススメの問題集](https://qiita.com/ktateish/items/afe1494a5cccb1ef13c7): AtCoder 以外の問題も含めて取り組める問題や書籍が紹介されている。
