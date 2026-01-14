# darwin-ism

[![GitHub License](https://img.shields.io/github/license/cffnpwr/darwin-ism?style=flat)](./LICENSE)

macOSの入力ソースを管理するCLIツール

macOSが`defaults write`で保護している入力ソース（Input Source）を、Carbon FrameworkのTIS APIを使用して一覧表示・有効化・無効化できます。

[README.md for English is available here](./README.md).

## How to Install

### Nix (Flakes)

```bash
# 直接実行
nix run github:cffnpwr/darwin-ism

# インストール
nix profile install github:cffnpwr/darwin-ism
```

### Nix (non-Flakes)

```bash
nix-env -if https://github.com/cffnpwr/darwin-ism/archive/main.tar.gz
```

### GitHub Release

TBD

### Build from Source

#### Prerequisites

- macOS（aarch64-darwinまたはx86_64-darwin）

以下のいずれかの環境を準備してください。

- [Nix](https://nixos.org/) - Nix FlakesをサポートするNix環境
- [mise](https://mise.jdx.dev/) - miseがインストールされている環境
- Swift 6.0以上がインストールされている環境

Nixを使用しない場合は以下も必要です：

- Xcode 16以上（またはCommand Line Tools）- Carbon Frameworkを使用するためApple SDKが必要

#### How to build

1. リポジトリをクローン

```bash
git clone https://github.com/cffnpwr/darwin-ism.git
cd darwin-ism
```

2. 開発環境のセットアップ

<details>
<summary>Nixを使用する場合</summary>

```bash
nix develop
```

Nix環境では自動的にSwiftとApple SDKがセットアップされます。

</details>

<details>
<summary>miseを使用する場合</summary>

```bash
mise install
```

</details>

<details>
<summary>Swiftを直接使用する場合</summary>

この手順はスキップしてください。

</details>

3. ビルド

```bash
swiftc -O -o darwin-ism \
  -framework Carbon \
  -framework Foundation \
  Sources/darwin-ism/*.swift
```

または、Nixを使用する場合：

```bash
nix build
```

4. 実行

```bash
./darwin-ism --help

# Nixでビルドした場合
./result/bin/darwin-ism --help
```

## How to use

```
darwin-ism <COMMAND> [OPTIONS]
```

### List of command

| コマンド | 説明 |
|---------|------|
| `list` | すべての入力ソースを一覧表示 |
| `enable <id>` | 入力ソースを有効化 |
| `disable <id>` | 入力ソースを無効化 |
| `help` | ヘルプを表示 |
| `version` | バージョンを表示 |

### Options of `list` subcommand

| オプション | 説明 |
|-----------|------|
| `--enabled`, `-e` | 有効な入力ソースのみ表示 |
| `--bundle-id`, `-b <id>` | バンドルIDでフィルタリング |

### Examples

```bash
# すべての入力ソースを一覧表示
darwin-ism list

# 有効な入力ソースのみ表示
darwin-ism list --enabled

# 特定のバンドルIDでフィルタリング
darwin-ism list --bundle-id dev.ensan.inputmethod.azooKeyMac

# 入力ソースを有効化（IDはlistコマンドで確認）
darwin-ism enable <id>

# 入力ソースを無効化
darwin-ism disable <id>
```

## How to setup development environment

開発環境のセットアップは[「ソースからビルド」のPrerequisitesセクション](#Prerequisites)を参照してください。

## License

[MIT License](./LICENSE)
