use clap::Parser;
use nanai_gna::instrumentation::{compute_hw_usage, UsageError};
use nanai_gna::raw::{
    Gna2InstrumentationPoint,
    Gna2InstrumentationPoint_Gna2InstrumentationPointDrvCompletion,
    Gna2InstrumentationPoint_Gna2InstrumentationPointDrvDeviceRequestCompleted,
    Gna2InstrumentationPoint_Gna2InstrumentationPointDrvPreprocessing,
    Gna2InstrumentationPoint_Gna2InstrumentationPointDrvProcessing,
    Gna2InstrumentationPoint_Gna2InstrumentationPointHwStallCycles,
    Gna2InstrumentationPoint_Gna2InstrumentationPointHwTotalCycles,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibCompletion,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibDeviceRequestCompleted,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibDeviceRequestReady,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibDeviceRequestSent,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibExecution,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibPreprocessing,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibProcessing,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibReceived,
    Gna2InstrumentationPoint_Gna2InstrumentationPointLibSubmission,
};

#[derive(Parser, Debug)]
#[command(about = "GNA instrumentation usage breakdown sample", version)]
struct Args {
    /// Instrumentation points, by name or numeric value. Comma separated.
    /// Default: hw_total,hw_stall
    #[arg(long, value_delimiter = ',', value_parser = parse_point, default_values_t = default_points())]
    points: Vec<Gna2InstrumentationPoint>,

    /// Instrumentation results (u64), comma separated. Must match points length.
    /// Default: 1000,250 (total, stall)
    #[arg(long, value_delimiter = ',', default_values_t = default_results())]
    results: Vec<u64>,

    /// Treat results as timestamps and show per-segment timing share. If false, only usage is shown.
    #[arg(long, default_value_t = true)]
    timeline: bool,
}

fn main() {
    let args = Args::parse();

    if args.points.len() != args.results.len() {
        eprintln!("points と results の長さが一致しません");
        std::process::exit(1);
    }

    match compute_hw_usage(&args.points, &args.results) {
        Ok(usage) => println!("HW usage: {:.2}%", usage * 100.0),
        Err(UsageError::MissingRequiredPoints) => {
            println!("HW usage: 計算に必要な Total/Stall が不足しています");
        }
        Err(UsageError::TotalZero) => {
            println!("HW usage: Total が 0 のため計算不可");
        }
        Err(UsageError::LengthMismatch) => unreachable!("既に長さを検査済み"),
    }

    if args.timeline && args.results.len() >= 2 {
        println!("\nTimeline breakdown (差分/最後-最初):");
        let total_time = args.results.last().unwrap().saturating_sub(args.results[0]);
        for idx in 1..args.results.len() {
            let delta = args.results[idx].saturating_sub(args.results[idx - 1]);
            let pct = if total_time == 0 {
                0.0
            } else {
                (delta as f64) / (total_time as f64) * 100.0
            };
            println!(
                "  {:<35} Δ{:>6} ({:>5.1}%)",
                point_name(args.points[idx - 1]),
                delta,
                pct
            );
        }
    }

    println!("\nPoint mapping (入力に使える名前の例):");
    for (name, id) in KNOWN_POINTS {
        println!("  {:<40} -> {}", name, id);
    }
}

fn default_points() -> Vec<Gna2InstrumentationPoint> {
    vec![
        Gna2InstrumentationPoint_Gna2InstrumentationPointHwTotalCycles,
        Gna2InstrumentationPoint_Gna2InstrumentationPointHwStallCycles,
    ]
}

fn default_results() -> Vec<u64> {
    vec![1000, 250]
}

const KNOWN_POINTS: &[(&str, Gna2InstrumentationPoint)] = &[
    ("hw_total", Gna2InstrumentationPoint_Gna2InstrumentationPointHwTotalCycles),
    ("hw_stall", Gna2InstrumentationPoint_Gna2InstrumentationPointHwStallCycles),
    ("lib_preprocessing", Gna2InstrumentationPoint_Gna2InstrumentationPointLibPreprocessing),
    ("lib_submission", Gna2InstrumentationPoint_Gna2InstrumentationPointLibSubmission),
    ("lib_processing", Gna2InstrumentationPoint_Gna2InstrumentationPointLibProcessing),
    ("lib_execution", Gna2InstrumentationPoint_Gna2InstrumentationPointLibExecution),
    (
        "lib_device_request_ready",
        Gna2InstrumentationPoint_Gna2InstrumentationPointLibDeviceRequestReady,
    ),
    (
        "lib_device_request_sent",
        Gna2InstrumentationPoint_Gna2InstrumentationPointLibDeviceRequestSent,
    ),
    (
        "lib_device_request_completed",
        Gna2InstrumentationPoint_Gna2InstrumentationPointLibDeviceRequestCompleted,
    ),
    ("lib_completion", Gna2InstrumentationPoint_Gna2InstrumentationPointLibCompletion),
    ("lib_received", Gna2InstrumentationPoint_Gna2InstrumentationPointLibReceived),
    ("drv_preprocessing", Gna2InstrumentationPoint_Gna2InstrumentationPointDrvPreprocessing),
    ("drv_processing", Gna2InstrumentationPoint_Gna2InstrumentationPointDrvProcessing),
    (
        "drv_device_request_completed",
        Gna2InstrumentationPoint_Gna2InstrumentationPointDrvDeviceRequestCompleted,
    ),
    ("drv_completion", Gna2InstrumentationPoint_Gna2InstrumentationPointDrvCompletion),
];

fn point_name(id: Gna2InstrumentationPoint) -> &'static str {
    for (name, candidate) in KNOWN_POINTS {
        if *candidate == id {
            return name;
        }
    }
    "unknown"
}

fn parse_point(raw: &str) -> Result<Gna2InstrumentationPoint, String> {
    let lower = raw.to_ascii_lowercase();
    for (name, candidate) in KNOWN_POINTS {
        if lower == *name {
            return Ok(*candidate);
        }
    }
    match raw.parse::<i32>() {
        Ok(v) => Ok(v),
        Err(_) => Err(format!("未知の計測ポイント: {}", raw)),
    }
}
