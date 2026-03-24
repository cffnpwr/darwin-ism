---
status: accepted
date: 2026-03-24
---

# ADR-0003: TIS APIバインディング手法の選定

## コンテキスト

darwin-ismはmacOS Carbon Framework傘下の`HIToolbox.framework`が提供するTIS（Text Input Sources）APIを使って入力ソースの列挙・有効化・無効化を行う。

TIS APIは純粋なC APIであり、返り値に`CFTypeRef`・`CFArrayRef`・`CFStringRef`等のCoreFoundation型を使う。
Rustへの移行（ADR-0002）にあたり、このC APIをどのようにRustから呼び出すかを決定する必要がある。

主に使用するTIS関数：
- `TISCreateInputSourceList`
- `TISSelectInputSource`
- `TISDeselectInputSource`
- `TISEnableInputSource`
- `TISDisableInputSource`
- `TISGetInputSourceProperty`

## 検討した選択肢

### 選択肢1: `bindgen`による自動バインディング生成

`build.rs`で`bindgen`を呼び出し、Carbon Frameworkのヘッダ（`TextInputSources.h`）からRust FFIコードを自動生成する。

#### 良い点

- ヘッダが変わっても再生成で追従できる
- 手書きによるシグネチャの誤りがない

#### 悪い点

- macOS Sequoia（SDK 15）でCFBase.hのavailabilityマクロ解析が失敗する既知の未解決バグがある（[rust-lang/rust-bindgen#2933](https://github.com/rust-lang/rust-bindgen/issues/2933)）
- `xcrun`とClangがビルド環境に必要であり、Nix sandboxビルドと相性が悪い
- TIS APIは10個程度の関数しかないため、自動生成のメリットが小さい

### 選択肢2: `objc2`エコシステム

`madsmtm/objc2`プロジェクトのクレート群を使い、Apple Frameworkへのアクセスを安全なRust APIとして利用する。

#### 良い点

- CoreFoundation型（`CFString`・`CFArray`等）の型安全なラッパーが得られる
- 他のObjective-Cフレームワークと組み合わせやすい

#### 悪い点

- TIS APIはC APIであり、objc2のObjective-Cランタイム層では直接扱えない
- `objc2-carbon`クレートはTIS APIを網羅していない
- 結局C FFIが別途必要になる

### 選択肢3: 手書き`unsafe extern "C"` FFI

使用するTIS関数・定数のみを`unsafe extern "C"`ブロックで手動宣言し、`core-foundation`クレートのCoreFoundation型ラッパーと組み合わせる。

```rust
use core_foundation::array::CFArrayRef;
use core_foundation::base::CFTypeRef;
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::string::CFStringRef;

pub enum TISInputSource {}
pub type TISInputSourceRef = *mut TISInputSource;

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    pub static kTISPropertyInputSourceID: CFStringRef;
    // ...
    pub fn TISCreateInputSourceList(
        properties: CFDictionaryRef,
        include_all_installed: bool,
    ) -> CFArrayRef;
    pub fn TISSelectInputSource(input_source: TISInputSourceRef) -> i32;
    // ...
}
```

`build.rs`は不要で、`#[link]`アトリビュートでCarbon Frameworkとのリンクが解決する。

#### 良い点

- 実装がシンプルで外部ツールへの依存が最小
- `xcrun`やClangがビルド環境に不要でNix sandboxと相性が良い
- Sequoiaのbindgenバグを回避できる
- TIS APIはmacOS 10.5以降ほぼ変更がなく、手書き管理のリスクが低い
- `core-foundation`クレートで返り値のCoreFoundation型を安全に扱える

#### 悪い点

- シグネチャを手書きするため型の間違いリスクがある
- APIが変更された場合に手動追従が必要

## 決定

手書き`unsafe extern "C"` FFI + `core-foundation`クレートの組み合わせを採用する。理由：

- TIS APIは関数・定数が十数個程度と小規模で安定しているため、手書きのリスクが低い
- bindgenのmacOS Sequoiaバグが未解決であり、プロダクション用途での採用は時期尚早
- Nix sandboxビルドでは`xcrun`に依存しない手書きFFIの方が再現性が高い
- `core-foundation` v0.10.1でCFTypeRefのダウンキャスト・メモリ管理が正しく扱えることを確認済み
- `#[link]`アトリビュートはNixで正常動作することを確認済み。本プロジェクトが使用するnixpkgs（2026年2月時点、25.11以降）では`darwin.apple_sdk.frameworks.*`が削除されており、SDKがstdenvに含まれるため`buildInputs`への追加は不要

## 結果

### 良い影響

- Nix sandboxビルドとの相性が良くなる
- ビルド依存が`core-foundation`クレートのみに絞られる

### 悪い影響

- TIS APIのシグネチャ手書きメンテナンスが発生する（関数数が少ないため影響は小さい）
