use clap::Parser;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// GNA-focused load test using the same tiny model as the C++ sample.
#[derive(Parser, Debug)]
#[command(about = "GNA-only load tester (creates inference requests)", version)]
struct Args {
    /// Number of worker threads (each creates its own model/config/memory)
    #[arg(long, default_value_t = 4)]
    threads: usize,

    /// Test duration in seconds
    #[arg(long, default_value_t = 10)]
    duration: u64,

    /// Request timeout in milliseconds for Gna2RequestWait
    #[arg(long, default_value_t = 1000u32)]
    timeout_ms: u32,

    /// If true, open/close device around the workload (emulates churn)
    #[arg(long, default_value_t = false)]
    device_open_close: bool,

    /// Backend to use [auto|ffi|rust]
    #[arg(long, default_value = "auto")]
    backend: String,
}

enum SelectedBackend { Ffi, Rust }

fn choose_backend(s: &str) -> Result<SelectedBackend, String> {
    match s {
        "auto" => {
            if cfg!(feature = "rust_backend") { Ok(SelectedBackend::Rust) }
            else if cfg!(feature = "link_gna") { Ok(SelectedBackend::Ffi) }
            else { Err("no backend compiled into this binary".into()) }
        }
        "ffi" => { if cfg!(feature = "link_gna") { Ok(SelectedBackend::Ffi) } else { Err("ffi backend not available: build with --features link_gna".into()) } }
        "rust" => { if cfg!(feature = "rust_backend") { Ok(SelectedBackend::Rust) } else { Err("rust backend not available: build with --features rust_backend".into()) } }
        other => Err(format!("unknown backend '{}', expected auto|ffi|rust", other)),
    }
}

fn main() {
    let args = Args::parse();
    let backend = match choose_backend(&args.backend) {
        Ok(b) => b,
        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
    };

    println!("Starting GNA load test: threads={}, duration={}s backend={}", args.threads, args.duration, args.backend);

    let stop = Arc::new(AtomicBool::new(false));
    let ops = Arc::new(AtomicU64::new(0));
    let latency_sum = Arc::new(AtomicU64::new(0)); // micros
    let latency_min = Arc::new(AtomicU64::new(u64::MAX));
    let latency_max = Arc::new(AtomicU64::new(0));

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(args.threads);

    match backend {
        SelectedBackend::Ffi => {
            #[cfg(feature = "link_gna")]
            {
                for i in 0..args.threads {
                    let stop = stop.clone();
                    let ops = ops.clone();
                    let latency_sum = latency_sum.clone();
                    let latency_min = latency_min.clone();
                    let latency_max = latency_max.clone();
                    let device_open_close = args.device_open_close;
                    let timeout_ms = args.timeout_ms;

                    let handle = thread::spawn(move || {
                        gna_worker_ffi(i as u32, stop, ops, latency_sum, latency_min, latency_max, device_open_close, timeout_ms);
                    });
                    handles.push(handle);
                }
            }
            #[cfg(not(feature = "link_gna"))]
            {
                eprintln!("ffi backend not available at runtime.");
                return;
            }
        }
        SelectedBackend::Rust => {
            #[cfg(feature = "rust_backend")]
            {
                for i in 0..args.threads {
                    let stop = stop.clone();
                    let ops = ops.clone();
                    let latency_sum = latency_sum.clone();
                    let latency_min = latency_min.clone();
                    let latency_max = latency_max.clone();
                    let device_open_close = args.device_open_close;
                    let timeout_ms = args.timeout_ms;

                    let handle = thread::spawn(move || {
                        nanai_gna::backend::gna_worker_rust(i as u32, stop, ops, latency_sum, latency_min, latency_max, device_open_close, timeout_ms);
                    });
                    handles.push(handle);
                }
            }
            #[cfg(not(feature = "rust_backend"))]
            {
                eprintln!("rust backend not available at runtime.");
                return;
            }
        }
    }

    // metrics printer
    let stop_report = stop.clone();
    let ops_report = ops.clone();
    let latency_sum_report = latency_sum.clone();
    let latency_min_report = latency_min.clone();
    let latency_max_report = latency_max.clone();

    let metrics_handle = thread::spawn(move || {
        let mut last_ops = 0u64;
        let mut last_instant = Instant::now();
        while !stop_report.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            let now = Instant::now();
            let cur_ops = ops_report.load(Ordering::Relaxed);
            let delta_ops = cur_ops.saturating_sub(last_ops);
            let elapsed = now.duration_since(last_instant).as_secs_f64();
            last_ops = cur_ops;
            last_instant = now;
            let sum = latency_sum_report.load(Ordering::Relaxed);
            let min = latency_min_report.load(Ordering::Relaxed);
            let max = latency_max_report.load(Ordering::Relaxed);
            let avg = if cur_ops == 0 { 0.0 } else { (sum as f64) / (cur_ops as f64) };
            println!(
                "ops_total={}  ops/s={:.2}  avg_latency_us={:.2}  min_us={}  max_us={}",
                cur_ops,
                (delta_ops as f64) / elapsed,
                avg,
                if min == u64::MAX { 0 } else { min },
                max
            );
        }
    });

    let start = Instant::now();
    while Instant::now().duration_since(start) < Duration::from_secs(args.duration) {
        thread::sleep(Duration::from_millis(100));
    }

    stop.store(true, Ordering::Relaxed);

    for h in handles {
        let _ = h.join();
    }
    let _ = metrics_handle.join();

    let total = ops.load(Ordering::Relaxed);
    let sum = latency_sum.load(Ordering::Relaxed);
    let min = latency_min.load(Ordering::Relaxed);
    let max = latency_max.load(Ordering::Relaxed);
    let secs = Instant::now().duration_since(start).as_secs_f64();
    let avg = if total == 0 { 0.0 } else { (sum as f64) / (total as f64) };
    println!("Done. Total ops={} (avg {:.2} ops/s) avg_latency_us={:.2} min_us={} max_us={}", total, (total as f64) / secs, avg, if min == u64::MAX { 0 } else { min }, max);
}

#[cfg(feature = "link_gna")]
/// Helpers
fn round_up_to64(x: usize) -> usize { ((x + 63) / 64) * 64 }
#[cfg(feature = "link_gna")]
fn round_up(x: usize, align: usize) -> usize { ((x + align - 1) / align) * align }

#[cfg(feature = "link_gna")]
fn gna_worker_ffi(
    thread_id: u32,
    stop: Arc<AtomicBool>,
    ops: Arc<AtomicU64>,
    latency_sum: Arc<AtomicU64>,
    latency_min: Arc<AtomicU64>,
    latency_max: Arc<AtomicU64>,
    device_open_close: bool,
    timeout_ms: u32,
) {
    use nanai_gna::raw as raw;

    unsafe {
        // 1) Query devices
        let mut device_count: u32 = 0;
        let mut status = raw::Gna2DeviceGetCount(&mut device_count as *mut u32);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2DeviceGetCount failed: {:?}", thread_id, status);
            return;
        }
        if device_count == 0 {
            eprintln!("Thread {}: No GNA devices found", thread_id);
            return;
        }

        // choose last device like sample does
        let device_index = device_count - 1;

        if device_open_close {
            status = raw::Gna2DeviceOpen(device_index);
                if raw::Gna2Status_Gna2StatusSuccess != status {
                eprintln!("Thread {}: Gna2DeviceOpen failed: {:?}", thread_id, status);
                return;
            }
        }

        // Prepare small sample model data (from sample01)
        const W: usize = 16;
        const H: usize = 8;
        const B: usize = 4;
        let weights: [i16; H * W] = [
            -6, -2, -1, -1, -2, 9, 6, 5, 2, 4, -1, 5, -2, -4, 0, 9,
            -8, 8, -4, 6, 5, 3, -7, -9, 7, 0, -4, -1, 1, 7, 6, -6,
            2, -8, 6, 5, -1, -2, 7, 5, -1, 4, 8, 7, -9, -1, 7, 1,
            0, -2, 1, 0, 6, -6, 7, 4, -6, 0, 3, -2, 1, 8, -6, -2,
            -6, -3, 4, -2, -8, -6, 6, 5, 6, -9, -5, -2, -5, -8, -6, -2,
            -7, 0, 6, -3, -1, -6, 4, 1, -4, -5, -3, 7, 9, -9, 9, 9,
            0, -2, 6, -3, 5, -2, -1, -3, -5, 7, 6, 6, -8, 0, -4, 9,
            2, 7, -8, -7, 8, -6, -6, 1, 7, -4, -4, 9, -6, -6, 5, -7,
        ];
        let inputs: [i16; W * B] = [
            -5, 9, -7, 4, 5, -4, -7, 4, 0, 7, 1, -7, 1, 6, 7, 9, 2, -4, 9, 8, -5, -1, 2, 9, -8, -8, 8, 1, -7, 2, -1, -1,
            -9, -5, -8, 5, 0, -1, 3, 9, 0, 8, 1, -2, -9, 8, 0, -7, -9, -8, -1, -4, -3, -7, -2, 3, -8, 0, 1, 3, -4, -6, -8, -2,
        ];
        let biases: [i32; H] = [5, 4, -2, 5, -7, -5, 4, -1];

        // Compute buffer sizes and allocate GNA memory
        let buf_size_weights = round_up_to64(weights.len() * std::mem::size_of::<i16>());
        let buf_size_inputs = round_up_to64(inputs.len() * std::mem::size_of::<i16>());
        let buf_size_biases = round_up_to64(biases.len() * std::mem::size_of::<i32>());
        let buf_size_outputs = round_up_to64(H * B * std::mem::size_of::<i32>());
        let rw_buffer_size = round_up(buf_size_inputs + buf_size_outputs, 0x1000);
        let bytes_requested = (rw_buffer_size + buf_size_weights + buf_size_biases) as u32;

        let mut bytes_granted: u32 = 0;
        let mut memory: *mut std::ffi::c_void = std::ptr::null_mut();
        status = raw::Gna2MemoryAlloc(bytes_requested, &mut bytes_granted as *mut u32, &mut memory as *mut *mut std::ffi::c_void);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2MemoryAlloc failed: {:?}", thread_id, status);
            if device_open_close { let _ = raw::Gna2DeviceClose(device_index); }
            return;
        }

        // Layout memory and copy data
        let mut model_memory = memory as *mut u8;
        let rw_buffers = model_memory;
        let pinned_inputs = rw_buffers as *mut i16;
        std::ptr::copy_nonoverlapping(inputs.as_ptr(), pinned_inputs, inputs.len());
        let pinned_outputs = (rw_buffers.add(buf_size_inputs)) as *mut i32;

        model_memory = model_memory.add(rw_buffer_size);
        let weights_buffer = model_memory as *mut i16;
        std::ptr::copy_nonoverlapping(weights.as_ptr(), weights_buffer, weights.len());
        model_memory = model_memory.add(buf_size_weights);
        let biases_buffer = model_memory as *mut i32;
        std::ptr::copy_nonoverlapping(biases.as_ptr(), biases_buffer, biases.len());

        // Prepare tensors and operation
        let mut input_tensor = raw::Gna2TensorInit2D(W as u32, B as u32, raw::Gna2DataType_Gna2DataTypeInt16, pinned_inputs as *mut std::ffi::c_void);
        let mut output_tensor = raw::Gna2TensorInit2D(H as u32, B as u32, raw::Gna2DataType_Gna2DataTypeInt32, pinned_outputs as *mut std::ffi::c_void);
        let mut weight_tensor = raw::Gna2TensorInit2D(H as u32, W as u32, raw::Gna2DataType_Gna2DataTypeInt16, weights_buffer as *mut std::ffi::c_void);
        let mut bias_tensor = raw::Gna2TensorInit1D(H as u32, raw::Gna2DataType_Gna2DataTypeInt32, biases_buffer as *mut std::ffi::c_void);
        let mut activation_tensor = raw::Gna2TensorInitDisabled();

        let mut operation: raw::Gna2Operation = std::mem::zeroed();
        status = raw::Gna2OperationInitFullyConnectedAffine(&mut operation as *mut raw::Gna2Operation,
                                                            Some(custom_alloc),
                                                            &mut input_tensor as *mut _,
                                                            &mut output_tensor as *mut _,
                                                            &mut weight_tensor as *mut _,
                                                            &mut bias_tensor as *mut _,
                                                            &mut activation_tensor as *mut _);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2OperationInitFullyConnectedAffine failed: {:?}", thread_id, status);
            let _ = raw::Gna2MemoryFree(memory);
            if device_open_close { let _ = raw::Gna2DeviceClose(device_index); }
            return;
        }

        // Create model
        let model = raw::Gna2Model { NumberOfOperations: 1, Operations: &mut operation };
        let mut model_id: u32 = raw::GNA2_DISABLED as u32;
        status = raw::Gna2ModelCreate(device_index, &model as *const raw::Gna2Model, &mut model_id as *mut u32);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2ModelCreate failed: {:?}", thread_id, status);
            let _ = raw::Gna2MemoryFree(memory);
            if device_open_close { let _ = raw::Gna2DeviceClose(device_index); }
            return;
        }

        // Create request config
        let mut config_id: u32 = raw::GNA2_DISABLED as u32;
        status = raw::Gna2RequestConfigCreate(model_id, &mut config_id as *mut u32);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2RequestConfigCreate failed: {:?}", thread_id, status);
            let _ = raw::Gna2ModelRelease(model_id);
            let _ = raw::Gna2MemoryFree(memory);
            if device_open_close { let _ = raw::Gna2DeviceClose(device_index); }
            return;
        }

        status = raw::Gna2RequestConfigSetOperandBuffer(config_id, 0, 0, pinned_inputs as *mut std::ffi::c_void);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2RequestConfigSetOperandBuffer input failed: {:?}", thread_id, status);
        }
        status = raw::Gna2RequestConfigSetOperandBuffer(config_id, 0, 1, pinned_outputs as *mut std::ffi::c_void);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2RequestConfigSetOperandBuffer output failed: {:?}", thread_id, status);
        }

        status = raw::Gna2RequestConfigSetAccelerationMode(config_id, raw::Gna2AccelerationMode_Gna2AccelerationModeAuto);
        if raw::Gna2Status_Gna2StatusSuccess != status {
            eprintln!("Thread {}: Gna2RequestConfigSetAccelerationMode failed: {:?}", thread_id, status);
        }

        // Main loop: enqueue + wait
        while !stop.load(Ordering::Relaxed) {
            // Optionally open/close device to emulate churn
            if device_open_close {
                let _ = raw::Gna2DeviceOpen(device_index);
            }

            let mut request_id: u32 = raw::GNA2_DISABLED as u32;
            status = raw::Gna2RequestEnqueue(config_id, &mut request_id as *mut u32);
            if raw::Gna2Status_Gna2StatusSuccess != status {
                eprintln!("Thread {}: Gna2RequestEnqueue failed: {:?}", thread_id, status);
                break;
            }

            let t0 = Instant::now();
            status = raw::Gna2RequestWait(request_id, timeout_ms);
            if raw::Gna2Status_Gna2StatusSuccess != status {
                eprintln!("Thread {}: Gna2RequestWait failed: {:?}", thread_id, status);
                break;
            }
            let dur = Instant::now().duration_since(t0);
            let us = dur.as_micros() as u64;

            ops.fetch_add(1, Ordering::Relaxed);
            latency_sum.fetch_add(us, Ordering::Relaxed);
            // update min
            let mut cur_min = latency_min.load(Ordering::Relaxed);
            while us < cur_min {
                match latency_min.compare_exchange(cur_min, us, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(old) => cur_min = old,
                }
            }
            // update max
            let mut cur_max = latency_max.load(Ordering::Relaxed);
            while us > cur_max {
                match latency_max.compare_exchange(cur_max, us, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(old) => cur_max = old,
                }
            }

            if device_open_close {
                let _ = raw::Gna2DeviceClose(device_index);
            }
        }

        // cleanup
        let _ = raw::Gna2RequestConfigRelease(config_id);
        let _ = raw::Gna2ModelRelease(model_id);
        let _ = raw::Gna2MemoryFree(memory);
        if device_open_close { let _ = raw::Gna2DeviceClose(device_index); }
    }
}

#[cfg(feature = "link_gna")]
unsafe extern "C" fn custom_alloc(dumped_model_size: u32) -> *mut std::ffi::c_void {
    if dumped_model_size == 0 {
        eprintln!("custom_alloc invalid size");
        return std::ptr::null_mut();
    }
    // allocate aligned memory (cross-platform) using std::alloc
    let layout = match std::alloc::Layout::from_size_align(dumped_model_size as usize, 4096) {
        Ok(l) => l,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe {
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() { std::ptr::null_mut() } else { ptr as *mut std::ffi::c_void }
    }
}


