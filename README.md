# WikivoyageConveniencer

Wikivoyageの記事編集・閲覧補助、および統計確認を目的としたデスクトップアプリです。  
Rust + Tauri + Nuxt.js（Vue 3 + TypeScript + SCSS）で構築されています。

---

## 技術スタック

- **Frontend**
  - Nuxt 3
  - Vue 3
  - TypeScript
  - SCSS

- **Backend**
  - Rust (Tauri command)

- **Desktop Runtime**
  - Tauri

---

## プロジェクト構成

```

WikivoyageConveniencer/
│
├─ app/              # Nuxt フロントエンド
│   ├─ app.vue
│   ├─ pages/
│   ├─ components/
│   ├─ assets/
│   └─ ...
│
├─ src-tauri/        # Rust（Tauriバックエンド）
│   ├─ src/main.rs
│   └─ tauri.conf.json
│
└─ package.json      # ルート管理用

```

---

## セットアップ

### 1. クローン

```bash
git clone https://github.com/yourname/WikivoyageConveniencer.git
cd WikivoyageConveniencer
````

### 2. 依存関係インストール

```bash
cd app
npm install
cd ..
npm install
```

---

## 開発起動

### フロントエンド（Nuxt）

```bash
cd app
npm run dev
```

### デスクトップアプリ（Tauri）

```bash
npm run tauri dev
```

---

## 機能（開発中）

* Rustコマンド呼び出し（Tauri IPC）
* MediaWiki API連携
* ページ編集補助UI（予定）
* 統計・変更履歴ビューア（予定）
* コンポーネントベースUI設計（AppButton / AppDropdownなど）

---

## UI設計

* SCSS変数ベースのデザインシステム
* Wikivoyageカラーをベースにしたテーマ

  * Primary: `#006699`
  * Danger: `#990000`
  * Success: `#339966`

---

## Tauri + Nuxt 構成について

* Nuxtのビルド成果物（`.output/public`）をTauriが読み込み
* Rust側は `invoke` 経由でフロントと通信
* 開発時は Nuxt dev server + Tauri dev を併用

---

## 注意

* 現在は開発初期段階のため仕様は頻繁に変更されます
* ディレクトリ構成やビルド方法も変更される可能性があります

---

## License
[MIT License](LICENSE)