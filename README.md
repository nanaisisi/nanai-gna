AI生成です（Intelからの移植）。

# nanai-gna

Rust 版 GNA 移植プロジェクトです。

## 実行方法

### 1. ビルド

```bash
cargo build --workspace
```

### 2. `gna-rs` のサンプルを実行

- リクエスト状態確認サンプル:

```bash
cargo run --example request_state --manifest-path gna-rs/Cargo.toml
```

- 使用率計算サンプル:

```bash
cargo run --example usage_report --manifest-path gna-rs/Cargo.toml -- --points hw_total,hw_stall --results 1000,250
```

### 3. テスト実行

```bash
cargo test -p gna-rs
```

> 注: Intel® GNA ハードウェアを使う場合、対応するドライバーを事前にインストールしてください。

# PROJECT NOT UNDER ACTIVE MANAGEMENT

This project will no longer be maintained by Intel.  
Intel has ceased development and contributions including, but not limited to, maintenance, bug fixes, new releases, or updates, to this project.  
Intel no longer accepts patches to this project.  
 If you have an ongoing need to use this project, are interested in independently developing it, or would like to maintain patches for the open source software community, please create your own fork of this project.

# GNA - Gaussian & Neural Accelerator Library repository

[![LGPL-2.1-or-later](https://img.shields.io/badge/license-lgpl_2.1_or_later-green.svg)](LICENSE)

Intel® Gaussian & Neural Accelerator is a low-power neural coprocessor for continuous inference at the edge.

When power and performance are critical, the Intel® Gaussian & Neural Accelerator (Intel® GNA) provides power-efficient, always-on support. Intel® GNA is designed to deliver AI speech and audio applications such as neural noise cancellation, while simultaneously freeing up CPU resources for overall system performance and responsiveness.

GNA library provides an API to run inference on Intel® GNA hardware, as well as in the software execution mode on CPU.

GNA library is also a part of [OpenVINO™](https://github.com/openvinotoolkit/openvino).

Intel® GNA hardware requires a driver to be installed on the system. For Windows\* please see:
[Intel® Drivers \& Software](https://downloadcenter.intel.com/download/30139/Intel-GNA-Scoring-Accelerator-Driver-for-Intel-NUC11TN?wapkw=gna) or Windows\* Update.

## Repository components:

- GNA library
  - kernels (Software emulation kernels)
    - GMM (Gaussian Mixture Models kernels)
    - XNN (Neural Network kernels)
  - gna-api (core library and API)
- samples (minimalistic usage example)

## License

GNA library is licensed under [GNU Lesser General Public License v2.1 or later](LICENSE).
By contributing to the project, you agree to the license and copyright terms therein
and release your contribution under these terms.
