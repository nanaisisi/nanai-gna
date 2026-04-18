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
- `gna-rs/src/gna_lib/component.rs` でコンポーネントの次元管理とカウント計算を実装。
- `gna-rs/src/gna_lib/external_buffer.rs` で外部バッファ操作のサポートを実装。
- これらの変更は `cargo test -p gna-rs` で確認済み。

---

## 3. 未実装部分（移植途中）

現時点で `gna-rs` ソース内に `TODO` や `未実装` の文字列はほぼ見当たりませんが、コード上には多くの「スケルトン／スタブ実装」が残っています。これらは「実装済みではあるが本番用の挙動を持たない」状態を意味します。

### 3.1 明確な未実装領域

- `gna-rs/src/gna_api/gna2_*_impl.rs` 系: ほとんどが C++ API のラッパ実装としてスタブ化されている。
- `gna-rs/src/gna_lib/*.rs` の多数モジュール: `active_list`, `affine_layers`, `bias`, `copy_layer`, `hw_module_interface`, `layer`, `recurrent_layer`, `software_only_model`, `transform`, `weight` などが「スケルトン実装」で残存。
- `gna-rs/src/gna_lib/kernels/*.rs`: `affine_*`, `convnet_*`, `gmm`, `rnn_*`, `transpose*`, `igemv*`, `igemm*` など、多数のカーネルが自動生成スタブとなっている。
- `gna-rs/src/gna_lib/hardware_*` / `device*`: `hardware_capabilities`, `hardware_model_*`, `device`, `device_manager`, `driver_interface` などの部分は簡易化されたダミー実装で、実際のデバイス検出・メモリ管理を再現していない。

### 3.2 ドキュメント化すべき実装差異

以下の点を、未実装状態としてドキュメントに明示すると信頼性が高まります。

1. `gna-rs` には「動作検証用の構造体・APIラッパは存在するが、内部処理はスタブ」であるモジュールが多い。
2. `gna_api` の `gna2-*` 実装ファイルは、C++ APIの呼び出しインタフェースを模しているが、実際の GNA 演算を行うものではない。
3. `gna_lib` の主要演算ロジック（Layer/Transform/Kernels）は、まだ本格的な計算実装が完了していない。
4. `software_model` / `hybrid_model` も、現時点では簡易な挙動を提供する骨組みが中心であり、C++ベースの完全実装には至っていない。

### 3.3 追加すべき注記

- 「未実装」は `TODO` ではなく、`gna-rs` の設計方針として段階的に置き換え中の『スタブ実装』という状態で存在している。
- そのため、`doc/intel_gna_porting_diff.md` では「現在のコードベースでの実装完了度合い」や「スタブ状態の領域」を記述する方が正確です。

### 3.4 参考: 実装済みだが限定的な領域

- `gna-rs/src/gna_lib/component.rs`: 部分的に実装済み（Shape/Count/Validatorの基本処理）。
- `gna-rs/src/gna_lib/tensor.rs`: 基本的な `Tensor` 構造とバッファ検証を実装。
- `gna-rs/src/gna_lib/data_mode.rs`: データ型／モード変換の基本実装。
- `gna-rs/src/gna_lib/request.rs` / `request_configuration.rs`: リクエストライフサイクルの骨組みと設定。

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
| 内部ロジック | `gna-rs/src/gna_lib/component.rs`                | コンポーネント次元管理と count 計算      |
| 内部ロジック | `gna-rs/src/gna_lib/external_buffer.rs`          | 外部バッファ操作のサポート               |
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
