# text-input-source

[![GitHub License](https://img.shields.io/github/license/cffnpwr/darwin-ism?style=flat)](./LICENSE)

macOS Text Input Sources (TIS) APIのRustライブラリ

[README.md for English is available here](./README.md)

## 概要

**macOS専用**

`text-input-source`は、Carbon FrameworkのTIS APIを利用してmacOSのキーボードレイアウト・入力メソッドを管理するRustライブラリです。

## 機能

- インストール済みの入力ソースを一覧表示
- キーボード入力ソースを一覧表示
- 現在のキーボード入力ソースを取得
- 入力ソースを選択
- 入力ソースを有効化
- 入力ソースを無効化

## 動作要件

- macOS 10.5以降

## 使い方

`Cargo.toml`に以下を追加。

```toml
[dependencies]
text-input-source = "0.1"
```

### 使用例

```rust
use text_input_source::TisManager;

fn main() -> Result<(), text_input_source::TisError> {
    let manager = TisManager::new();

    // 有効なキーボード入力ソースを一覧表示
    let sources = manager.list_keyboard_input_sources(false)?;
    for source in &sources {
        println!(
            "{} ({}) enabled={}",
            source.localized_name()?.unwrap_or_else(|| "<unnamed>".into()),
            source.id()?.unwrap_or_else(|| "<unknown>".into()),
            source.is_enabled()?,
        );
    }

    // USキーボードレイアウトを選択
    if let Some(us) = sources
        .iter()
        .find(|s| s.id().ok().flatten().as_deref() == Some("com.apple.keylayout.US"))
    {
        us.select()?;
    }

    Ok(())
}
```

## API

### `TisManager`

| メソッド                                                   | 説明                             |
| ---------------------------------------------------------- | -------------------------------- |
| `TisManager::new()`                                        | マネージャーを作成               |
| `list_input_sources(include_all_installed: bool)`          | すべての入力ソースを一覧取得     |
| `list_keyboard_input_sources(include_all_installed: bool)` | キーボード入力ソースを一覧取得   |
| `current_keyboard_input_source()`                          | 現在のキーボード入力ソースを取得 |

`include_all_installed`が`false`の場合、有効な入力ソースのみを返します。
`true`の場合、無効なものを含むインストール済みのすべての入力ソースを返します。

### `InputSource`

| メソッド           | 説明                           |
| ------------------ | ------------------------------ |
| `id()`             | 入力ソースの識別子を取得       |
| `localized_name()` | ローカライズされた表示名を取得 |
| `is_enabled()`     | 入力ソースが有効かどうかを取得 |
| `select()`         | この入力ソースを選択           |
| `enable()`         | この入力ソースを有効化         |
| `disable()`        | この入力ソースを無効化         |

### `TisError`

| バリアント                             | 説明                                                |
| -------------------------------------- | --------------------------------------------------- |
| `NullResult(OperationKind)`            | TIS API呼び出しがNULLを返した                       |
| `Status(OperationKind, OSStatus)`      | TIS API呼び出しが非ゼロのOSStatusで失敗した         |
| `MissingProperty(PropertyKind)`        | 必須プロパティが見つからなかった                    |
| `UnexpectedPropertyType(PropertyKind)` | プロパティのCore Foundationの型が予期せぬものだった |

## スレッド安全性

TIS APIはスレッドセーフではないため、`TisManager`と`InputSource`は`!Send + !Sync`です。

## License

[MIT License](./LICENSE)
