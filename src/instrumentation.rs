//! Helper utilities for working with GNA instrumentation results.
//!
//! 現状のヘルパーは、ハードウェア性能カウンタから「使用率」を計算するための
//! シンプルなラッパーを提供します。GNAのInstrumentation APIでは、計測したい
//! 計測ポイントをユーザーが配列で指定し、その順序のまま結果が格納されます。
//! ここでは TotalCycles と StallCycles の差分から使用率を算出します。

use crate::raw::{
    Gna2InstrumentationPoint,
    Gna2InstrumentationPoint_Gna2InstrumentationPointHwStallCycles as HW_STALL,
    Gna2InstrumentationPoint_Gna2InstrumentationPointHwTotalCycles as HW_TOTAL,
};

/// 使用率計算時のエラー。
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum UsageError {
    /// selected_points と results の長さが一致しない。
    #[error("selected_points と results の長さが一致しません")]
    LengthMismatch,
    /// 必要な計測ポイントが不足している（TotalCycles あるいは StallCycles が無い）。
    #[error("TotalCycles / StallCycles の計測ポイントが不足しています")]
    MissingRequiredPoints,
    /// 合計サイクルが 0 のため使用率を計算できない。
    #[error("合計サイクルが 0 のため使用率を計算できません")]
    TotalZero,
    /// ベンチマーク基準値が 0 のため比較型使用率を計算できない。
    #[error("benchmark total cycles must be positive")]
    BenchmarkZero,
}

/// ハードウェア使用率を計算する。
///
/// `selected_points` は Gna2InstrumentationConfigCreate に渡した計測ポイントの配列。
/// `results` は Gna2RequestWait() 後に埋められる結果バッファです。
///
/// 使用率 = (TotalCycles - StallCycles) / TotalCycles.
/// 戻り値は 0.0〜1.0 の範囲を想定しています。
pub fn compute_hw_usage(
    selected_points: &[Gna2InstrumentationPoint],
    results: &[u64],
) -> Result<f64, UsageError> {
    if selected_points.len() != results.len() {
        return Err(UsageError::LengthMismatch);
    }

    // 探索: TotalCycles と StallCycles の位置を見つける
    let mut total = None;
    let mut stall = None;

    for (&pt, &value) in selected_points.iter().zip(results.iter()) {
        match pt {
            x if x == HW_TOTAL => total = Some(value),
            x if x == HW_STALL => stall = Some(value),
            _ => {}
        }
    }

    let total = total.ok_or(UsageError::MissingRequiredPoints)?;
    let stall = stall.ok_or(UsageError::MissingRequiredPoints)?;

    if total == 0 {
        return Err(UsageError::TotalZero);
    }

    // Stall が total より大きい場合は 0 として扱い、負に傾かないよう clamp。
    let active = total.saturating_sub(stall);
    let usage = (active as f64) / (total as f64);
    Ok(usage)
}

/// ベンチマーク比較型の使用率を計算する。
///
/// `benchmark_total` は「フルロード時に想定される基準サイクル数」です。
/// 実測アクティブサイクルを基準サイクルで割り、0.0〜1.0 にクランプします。
pub fn compute_benchmark_hw_usage(
    selected_points: &[Gna2InstrumentationPoint],
    results: &[u64],
    benchmark_total: u64,
) -> Result<f64, UsageError> {
    if selected_points.len() != results.len() {
        return Err(UsageError::LengthMismatch);
    }

    let mut total = None;
    let mut stall = None;

    for (&pt, &value) in selected_points.iter().zip(results.iter()) {
        match pt {
            x if x == HW_TOTAL => total = Some(value),
            x if x == HW_STALL => stall = Some(value),
            _ => {}
        }
    }

    let total = total.ok_or(UsageError::MissingRequiredPoints)?;
    let stall = stall.ok_or(UsageError::MissingRequiredPoints)?;

    if benchmark_total == 0 {
        return Err(UsageError::BenchmarkZero);
    }

    let active = total.saturating_sub(stall) as f64;
    let benchmark = benchmark_total as f64;
    let usage = (active / benchmark).clamp(0.0, 1.0);
    Ok(usage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_usage_ok() {
        let pts = vec![HW_TOTAL, HW_STALL];
        let res = vec![1000u64, 250u64];
        let usage = compute_hw_usage(&pts, &res).unwrap();
        assert!((usage - 0.75).abs() < 1e-9);
    }

    #[test]
    fn compute_usage_clamped_when_stall_exceeds() {
        let pts = vec![HW_TOTAL, HW_STALL];
        let res = vec![1000u64, 2000u64];
        let usage = compute_hw_usage(&pts, &res).unwrap();
        assert_eq!(usage, 0.0);
    }

    #[test]
    fn compute_usage_errors_on_missing_points() {
        let pts = vec![HW_TOTAL];
        let res = vec![1000u64];
        let err = compute_hw_usage(&pts, &res).unwrap_err();
        assert_eq!(err, UsageError::MissingRequiredPoints);
    }

    #[test]
    fn compute_usage_errors_on_total_zero() {
        let pts = vec![HW_TOTAL, HW_STALL];
        let res = vec![0u64, 0u64];
        let err = compute_hw_usage(&pts, &res).unwrap_err();
        assert_eq!(err, UsageError::TotalZero);
    }

    #[test]
    fn compute_benchmark_usage_ok() {
        let pts = vec![HW_TOTAL, HW_STALL];
        let res = vec![1000u64, 250u64];
        let usage = compute_benchmark_hw_usage(&pts, &res, 2000).unwrap();
        assert!((usage - 0.375).abs() < 1e-9);
    }

    #[test]
    fn compute_benchmark_usage_clamps_at_one() {
        let pts = vec![HW_TOTAL, HW_STALL];
        let res = vec![1500u64, 100u64];
        let usage = compute_benchmark_hw_usage(&pts, &res, 1000).unwrap();
        assert_eq!(usage, 1.0);
    }

    #[test]
    fn compute_benchmark_usage_errors_on_zero_benchmark() {
        let pts = vec![HW_TOTAL, HW_STALL];
        let res = vec![1000u64, 250u64];
        let err = compute_benchmark_hw_usage(&pts, &res, 0).unwrap_err();
        assert_eq!(err, UsageError::BenchmarkZero);
    }
}
