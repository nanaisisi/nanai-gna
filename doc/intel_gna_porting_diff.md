# intel/gna 移植差分サマリ（nanai-gna）

作成日: 2026-04-18

## 比較対象

- ベース実装（C++）: `gna/`（`intel/gna` サブモジュール）
  - 参照コミット: `38f21f5`（`main`）
- 移植実装（Rust）: `gna-rs/` とルートクレートの連携層

> 注記: この文書は「行単位の厳密な git diff」ではなく、移植観点での実装差分（追加実装・未実装）を整理したサマリ。

---

## 1. 差分の全体像

1. `intel/gna` の C++ 実装本体は `gna/` に保持（ベースライン）。
2. `nanai-gna` 側では Rust 移植層として `gna-rs/` を追加。
3. 差分の中心は以下:
   - Rust バックエンド化のための API/内部ロジック移植
   - 実行時バックエンド選択（FFI/Rust）
   - Instrumentation 利用・使用率算出・状態確認
   - テスト/サンプルの追加

---

## 2. 追加実装部分（intel/gna からの拡張）

### 2.1 Rust 移植クレートの新設

- `gna-rs/` を追加し、以下レイヤーへ分割:
  - `common`（共通型・ユーティリティ）
  - `gna_api`（Gna2\* API ラッパ）
  - `gna_lib`（内部ロジック）
  - `kernels`（演算カーネル）

### 2.2 バックエンド切替機構

- Cargo feature:
  - `rust_backend`
  - `link_gna`
- 実行時選択:
  - `examples/load_test.rs` に `--backend=auto|ffi|rust`

### 2.3 Instrumentation / 利用率算出

- `src/instrumentation.rs` で `TotalCycles` と `StallCycles` から使用率算出を実装。
- `examples/usage_report.rs` で利用例を提供。

### 2.4 リクエスト状態管理の追加

- `gna-rs/src/gna_lib/request.rs` で状態遷移管理（例: Pending/InFlight/Completed）。
- `gna-rs/examples/request_state.rs` を追加。
- `gna-rs/tests/request_state.rs` で状態遷移を検証。

### 2.5 テスト拡充

- `gna-rs/tests/` に以下テストを整備:
  - `api_request_lifecycle.rs`
  - `instrumentation_usage.rs`
  - `kernels_affine.rs`
  - `linux_driver_integration.rs`
  - `model_api.rs`
  - `request_state.rs`

### 2.6 GNA データモード / データ型の拡張

- `gna-rs/src/gna_api/gna2_types.rs` で `Gna2TensorMode::ConstantScalar` と追加のデータ型を追加。
- `gna-rs/src/gna_lib/data_mode.rs` で要素サイズマッピングを拡張し、`ConstantScalar` 時の `Int4` 型処理を実装。
- `gna-rs/src/gna_lib/gna_api/gna2_common_impl.rs` でデバイスバージョン変換、ステータス文字列マップ、汎用マップヘルパーを実装。
- `gna-rs/src/gna_lib/api_wrapper.rs` で例外安全な API 呼び出しラッパーを実装し、GNA ステータスのフォールバック処理を追加。
- `gna-rs/src/gna_lib/bias.rs` でバイアスバッファの基本処理と適用ヘルパーを実装。
- これらの変更は `cargo test -p gna-rs` で確認済み。

---

## 3. 未実装部分（移植途中）

`TODO` / `未実装` マーカーから確認できる主な未完了領域:

- `gna-rs/src/gna_lib/activation_function.rs`
- `gna-rs/src/gna_lib/active_list.rs`
- `gna-rs/src/gna_lib/affine_functions.rs`
- `gna-rs/src/gna_lib/component.rs`
- `gna-rs/src/gna_lib/convolutional_functions.rs`
- `gna-rs/src/gna_lib/convolutional_layer.rs`
- `gna-rs/src/gna_lib/convolutional_layer2d.rs`
- `gna-rs/src/gna_lib/copy_layer.rs`
- `gna-rs/src/gna_lib/driver_interface.rs`
- `gna-rs/src/gna_lib/export_device.rs`
- `gna-rs/src/gna_lib/external_buffer.rs`
- `gna-rs/src/gna_lib/gmm_layer.rs`
- `gna-rs/src/gna_lib/hardware_layer.rs`
- `gna-rs/src/gna_lib/layout.rs`
- `gna-rs/src/gna_lib/linux_driver_interface.rs`
- `gna-rs/src/gna_lib/sub_model.rs`

### 補足（設計ドキュメント上の未完了タスク）

`doc/implementation.md` では、以下が優先タスクとして整理されている。

1. `gna_api` の残関数（モデル作成/破棄、詳細メモリ API 等）の実装
2. `gna_lib` の Layer / Transform 実装完了
3. カーネル（transpose / affine / gmm など）の段階実装
4. `load_test` への instrumentation 統合
5. CI 整備と警告対応

---

## 4. 主要差分ファイル（移植作業の中心）

| 区分         | パス                                             | 内容                                     |
| ------------ | ------------------------------------------------ | ---------------------------------------- |
| 設計         | `doc/implementation.md`                          | 移植方針・未実装タスク定義               |
| クレート     | `gna-rs/Cargo.toml`                              | Rust 移植クレート構成                    |
| API          | `gna-rs/src/gna_api/gna2_inference_api.rs`       | 推論系 API 実装追加                      |
| 内部ロジック | `gna-rs/src/gna_lib/request.rs`                  | リクエスト状態管理                       |
| 内部ロジック | `gna-rs/src/gna_lib/request_configuration.rs`    | リクエスト設定処理                       |
| 内部ロジック | `gna-rs/src/gna_lib/gna_api/gna2_common_impl.rs` | 共通 API ヘルパー実装                    |
| 内部ロジック | `gna-rs/src/gna_lib/api_wrapper.rs`              | API 呼び出しの例外安全ラッパー           |
| 内部ロジック | `gna-rs/src/gna_lib/bias.rs`                     | バイアスバッファの基本処理と適用ヘルパー |
| 内部ロジック | `gna-rs/src/gna_lib/data_mode.rs`                | DataMode 型・サイズ処理の拡張            |
| サンプル     | `gna-rs/examples/request_state.rs`               | 状態遷移確認サンプル                     |
| テスト       | `gna-rs/tests/request_state.rs`                  | 状態遷移の検証                           |
| 利用率       | `src/instrumentation.rs`                         | HW 使用率算出ヘルパ                      |
| サンプル     | `examples/usage_report.rs`                       | Instrumentation レポート例               |

---

## 5. まとめ

- `intel/gna` の C++ ベースは維持しつつ、`nanai-gna` は Rust バックエンドを追加する形で拡張している。
- 現時点の差分は「実行経路・観測機能・テスト」の追加が先行しており、演算層や一部 API の移植は未完。
- 移植完了に向けては、`gna_lib` 未実装モジュールと `gna_api` 残関数の実装が主要ボトルネック。
