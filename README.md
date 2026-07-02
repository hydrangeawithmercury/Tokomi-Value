# Tokomi Value — ビルド手順

OKLCh の知覚的明度でイラストのバリューを可視化する、常に最前面の小窓ツール。
このフォルダは「設計図」です。あなたのPCで数コマンド打つと、本物の `.exe`（Windows）／アプリ（Mac・Linux）になります。

これは**第1弾ネイティブ版**です。目的は「確実にビルドが通り、Color Penguin のような使い勝手を手に入れること」。

---

## これで手に入るもの / まだ入らないもの

**入る（今回の主役）**
- 常に最前面に浮く独立ウィンドウ（クリスタをクリックしても背面に落ちない）
- どのアプリからでも `Ctrl + Shift + V` で 呼び出し / しまう
- 小さめの固定ウィンドウ、◉ ボタンで最前面固定の ON/OFF
- OKLCh の明度 L / 彩度 C / 色相 H 解析、明度ヒストグラム、ローキー・ハイキー判定
- 外部通信ゼロ（オフラインで完結、データは一切送信されない）

**まだ入らない（第2弾で追加予定）**
- 「共有ダイアログを消してクリスタを自動で掴む」ネイティブ画面キャプチャ。
  今回は実績のある `getDisplayMedia`（＝毎回ダイアログでウィンドウを選ぶ方式）のまま。

---

## 事前に入れるもの（初回だけ）

### 1. Rust（rustup）
https://rustup.rs/ を開き、案内どおりインストール。

### 2. OS ごとの必須ツール

**Windows**
- 「Microsoft C++ Build Tools」… https://visualstudio.microsoft.com/visual-cpp-build-tools/
  インストーラで「C++ によるデスクトップ開発」にチェック。
- WebView2 … Windows 10/11 なら基本プリインストール済み。無ければ Microsoft から入れる。

**macOS**
```
xcode-select --install
```

**Linux（Debian / Ubuntu 系）**
```
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### 3. Tauri CLI
```
cargo install tauri-cli --version "^2.0"
```

---

## 動かす

このフォルダ（`tokomi-value-app`）の中で：

```
cargo tauri dev
```

- 初回は依存クレートのダウンロード＆コンパイルで**数分〜十数分**かかります（コーヒー淹れてOK）。2回目以降は速い。
- ウィンドウが立ち上がったら「解析開始」→ 共有ダイアログでクリスタを選ぶ。
- `Ctrl + Shift + V` でしまう／呼び出す。◉ で最前面固定を切替。

## 配布用に固める（.exe / アプリを作る）

```
cargo tauri build
```

でき上がりは `src-tauri/target/release/` 以下（インストーラは `bundle/` 以下）。
この成果物を Ko-fi にアップすれば配布できます。

> macOS で `cargo tauri build` する場合のみ、`src-tauri/icons/icon.icns` が別途必要です（`icon.png` から生成してください）。`cargo tauri dev` では不要です。

---

## つまずいたら

- **`cargo tauri dev` で「command not found: tauri」** … 手順3の CLI インストールが未完了。`cargo install tauri-cli --version "^2.0"` を実行。
- **コンパイルエラーで C コンパイラ / link.exe が無い系** … Windows は「C++ Build Tools」未導入。手順2を確認。
- **解析開始を押しても共有ダイアログが出ない** … WebView2 が古い可能性。Microsoft から最新の WebView2 ランタイムを入れて再起動。
- **`Ctrl+Shift+V` が効かない** … 他アプリが同じショートカットを奪っている場合あり。`src-tauri/src/main.rs` の `Code::KeyV` を別キー（例 `Code::KeyB`）に変えて再ビルド。

---

## 中身の地図（どこを触れば何が変わるか）

```
value-penguin-app/
├─ ui/
│  └─ index.html        ← ツールの“脳みそ”。解析ロジック・見た目は全部ここ
└─ src-tauri/
   ├─ src/main.rs        ← ウィンドウ制御・グローバルショートカット
   ├─ tauri.conf.json    ← ウィンドウのサイズ / 最前面 / タイトル等
   ├─ capabilities/default.json ← 許可する操作の一覧
   ├─ Cargo.toml         ← Rust の依存関係
   └─ icons/             ← アプリアイコン
```

- ウィンドウの初期サイズを変えたい → `tauri.conf.json` の `width` / `height`
- 呼び出しショートカットを変えたい → `src/main.rs` の `Modifiers` と `Code`
- 解析の中身を変えたい → `ui/index.html`（ブラウザにドラッグすれば単体でも動くので、ここだけ先に試作できる）
