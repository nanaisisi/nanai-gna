# 実装方式（実装ドキュメント） 🔧

> このドキュメントは、RustによるGNAポーティング作業と、Rust/FFIバックエンドの選択機構に関する設計・実装方針をまとめたものです。

---

## 目的 🎯

- Rustで書き直したバックエンドを選択可能にする（ビルド時・実行時の両対応）。
- 元のC++ GNA実装を安全かつ段階的にRustへ移植し、単体テスト & 例で動作確認できる状態にする。

---

## 高レベル設計 💡

- ワークスペースに新しいクレート `gna-rs` を追加し、以下のレイヤーに分割して移植:
  - `common`：共通ユーティリティ（Address・Driver・例外など）
  - `gna_api`：外部API相当の薄いラッパ（Gna2\*関数群）
  - `gna_lib`：ライブラリ内部ロジック（Model/Request/BufferMapなど）
  - `kernels`：各種カーネルの実装／マッピング

- ビルド時フラグ:
  - Cargo features: `rust_backend`, `link_gna`（ffi利用）
  - `rust_backend` を使うとRust実装が有効化される。

- 実行時選択:
  - `examples/load_test.rs` に `--backend` オプションを追加（`auto|ffi|rust`）。
  - `auto` は「有効ならRustを使い、なければFFI」を選ぶ単純な優先ロジック。

---

## 実装上の重要ポイント 🔍

- bindgenの出力対策: `build.rs` で生成テキストに入る `unsafe unsafe` を補正する処理を追加。
- `BaseAddress` 等のポーティング時の安全性: テスト用の簡易実装で `Send`/`Sync` を明示している（将来的にはより厳密な安全化を行う）。
- リクエスト処理はまずシミュレーション（enqueue/wait）で動作検証を行い、段階的にカーネル実装を追加していく。

---

## テスト & 例 📋

- `gna-rs` にはユニットテストと1つの統合テスト（リクエストライフサイクル）を追加済み。現在、全テストが通っている。
- 主要コマンド:
  - ビルド: `cargo build --features rust_backend --examples`
  - テスト: `cargo test -p gna-rs`
  - 例の実行: `cargo run --example load_test --features rust_backend -- --backend=rust`

---

## 未実装 / 優先タスク（TODO） 📝

1. `gna_api` の残りの関数を実装する（モデル作成／破棄、詳細なメモリAPI等）。
2. `gna_lib` の Layer / Transform 実装を完了して、ソフトウェア実行経路を動かす。
3. カーネル実装を段階的に追加（transpose / affine / gmm など）。
4. `load_test` に `--instrument` を追加し、InstrumentationをRustバックエンドへ統合。
5. 未使用importや命名規則のワーニング修正、CIワークフロー追加。

---

## 安全性と注意点 ⚠️

- 生ポインタやFFIの扱いには注意が必要。現状はスケルトンのため一部簡略化しているが、本番品質にするには所有権とライフタイムの保証を強化する必要がある。
- bindgen 修正や `Send/Sync` のマークは暫定的な措置。レビュー時に重点確認を推奨。

---

## コントリビュート手順（短縮） 👩‍💻👨‍💻

1. 新しい実装を `gna-rs` 内の該当モジュールに追加。
2. ユニットテストを必ず追加して `cargo test -p gna-rs` を通す。
3. 例（`examples/load_test.rs`）でエンドツーエンド確認を行う。
4. 重大なAPI変更はこのドキュメントを更新し、PR説明に経緯を記載する。

---

## 関連ファイル（参照） 🔗

- `examples/load_test.rs` — 実行時バックエンド選択
- `build.rs` — bindgen後処理（`unsafe` 重複補正）
- `gna-rs/src/common` — BaseAddress, GnaDriver, 例外等
- `gna-rs/src/gna_api` — Gna2\* APIラッパ群
- `gna-rs/src/gna_lib` — Model / Request / BufferMap 等

---

## 追加：Instrumentation / 使用率 / 使用中判定の現状と今後

### 1. 進捗

- Rust側では `src/instrumentation.rs` に GNA Instrumentation API の結果から使用率を計算するヘルパーが実装済み。
- `examples/usage_report.rs` で実際に `compute_hw_usage(...)` を呼び出すサンプルがある。
- `gna-rs/tests/instrumentation_usage.rs` では、`HwTotalCycles` と `HwStallCycles` を収集して使用率を算出するテストが存在する。

### 2. 使用率算出の可能性

- 現状の実装は以下の式を使っている。
  - `usage = (TotalCycles - StallCycles) / TotalCycles`
- `TotalCycles` と `StallCycles` の計測ポイントを指定すれば、Rust側で使用率の算出が可能。
- `compute_hw_usage` は以下の条件を検証する。
  - 選択ポイントと結果の長さが一致すること
  - `TotalCycles` と `StallCycles` が両方存在すること
  - `TotalCycles` が 0 でないこと
- StallがTotalを超える場合は0.0にクランプされ、負の使用率にはならない。

### 3. 現在使用中か否かの判別

- Rust側は `gna-rs/src/gna_lib/request.rs` にリクエストのライフサイクル状態管理を追加し、`Pending` / `InFlight` / `Completed` を問い合わせできるようになった。
- `gna-rs` の現行実装では、`Gna2RequestEnqueue()` → `Gna2RequestWait()` → `Gna2RequestGetInstrumentationResults()` に加えて、`Gna2RequestGetState(request_id)` と `Gna2RequestIsInFlight(request_id)` で現在の処理状態を確認できる。
- これにより、実行中判定をアプリケーション側で行いやすくなった。
- 新しいサンプル `gna-rs/examples/request_state.rs` が追加され、リクエストの `Pending` / `InFlight` / `Completed` の遷移と Instrumentation 結果の取得を実際に確認できる。
- 既存のInstrumentationポイントには、`LibDeviceRequestReady` / `LibDeviceRequestSent` / `LibDeviceRequestCompleted` のようなライフサイクル情報が含まれており、これを追加の判定ロジックに組み込めばより正確な「使用中判定」も可能になる。

---

ご要望があれば、この部分も英語版に翻訳し、実装候補コード例を追加します。✨
