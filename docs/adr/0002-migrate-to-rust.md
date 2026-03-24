---
status: accepted
date: 2026-03-24
---

# ADR-0002: 実装言語をSwiftからRustへ移行

## コンテキスト

本プロジェクトはSwift 6.2とNix Flakesを組み合わせて実装されていた。
しかしNixpkgsのSwift 6対応が未完了であり、以下の問題が顕在化した。

- `swift-argument-parser` v1.6.0以降がswift-tools-version 6.0を要求するようになった
- Nixビルド環境で使用されるSwiftは5.10系であり、Swift 6の機能（`AccessLevelOnImport`等）をサポートしていない
- `swift-argument-parser` v1.7.0へのRenovateによる自動アップデートがCIのビルド失敗を引き起こした
- nixpkgsにおける周辺ツール（SwiftLint、SwiftFormat等）のNixとの統合も不安定

Swift 5系に固定し続けるか、別の言語に移行するかの選択が必要になった。

## 検討した選択肢

### 選択肢1: Swift 5系に固定し続ける

`swift-argument-parser`をv1.5.xに固定し、Swift 5.10環境を維持する。

#### 良い点

- 既存コードの変更が不要

#### 悪い点

- nixpkgsのSwift 6対応が完了するまで依存関係のアップデートを手動管理する必要がある
- SwiftとNixの統合が本質的に不安定であり、同様の問題が再発するリスクが高い
- SwiftLint等の周辺ツールもNixとの相性が悪く、CI維持コストが高い

### 選択肢2: Go

CGO経由でC APIを呼び出し、TIS APIにアクセスする。

#### 良い点

- シンプルな言語仕様で学習コストが低い
- `buildGoModule`によるNixとの統合が成熟している
- ビルドが高速

#### 悪い点

- TIS APIへのアクセスにCGOが必要であり、CGOを使うとNixでのクロスコンパイルが困難になる
- CGOはGoの「シンプルさ」という利点を損なう

### 選択肢3: Rust

`bindgen`や`cc`クレート経由でC APIを呼び出し、TIS APIにアクセスする。

#### 良い点

- `buildRustPackage`および`crane`によるNixとの統合が成熟・安定している
- `clap`クレートによるCLI構築体験がswift-argument-parserに近い
- CとのFFIがGoのCGOより予測可能
- `Cargo.lock`によって依存関係が完全に固定でき、Nixのハッシュ管理とも相性が良い

#### 悪い点

- Swiftと比較して記述量が増える場面がある
- macOS向けC APIバインディングの生成にビルド時の設定が必要

## 決定

Rustに移行する。

NixとSwiftの統合が根本的に不安定であり、nixpkgsのSwift 6対応完了時期も不明確なため、保守コストの観点から言語移行を選択した。
GoはCGOが必要な時点でNixとの相性問題が残るため除外した。
Rustは`crane`を使ったNixビルドが安定しており、`clap`によるCLI実装体験も良好なため採用する。

## 結果

### 良い影響

- NixとのビルドインテグレーションがRust向けの成熟した仕組みで安定する
- Renovateによる依存関係の自動アップデートがCIで安全に行えるようになる

### 悪い影響

- 既存のSwiftコードをRustで書き直す必要がある
- macOS TIS APIへのFFIバインディングを新たに整備する必要がある
