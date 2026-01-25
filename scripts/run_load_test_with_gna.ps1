<#
Find the built GNA library, set GNA_LIB_DIR for the command, and run the load_test example with passed args.
Usage:
  .\scripts\run_load_test_with_gna.ps1 --threads 8 --duration 60 --device-open-close true
#>
param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$Args
)

# If user already set GNA_LIB_DIR use it
if ($env:GNA_LIB_DIR -and (Test-Path $env:GNA_LIB_DIR)) {
    $buildRoot = $env:GNA_LIB_DIR
    Write-Host "Using GNA_LIB_DIR from environment: $buildRoot"
} else {
    $root = Split-Path -Parent $PSScriptRoot
    # Candidate locations to search for built GNA artifacts
    $candidates = @( Join-Path $root 'gna\build', Join-Path $root 'gna\bin', Join-Path $root 'build' )
    $buildRoot = $null
    foreach ($c in $candidates) {
        if (Test-Path $c) { $buildRoot = $c; break }
    }
}

if (-not $buildRoot) {
    Write-Error "GNA library not found. Build GNA first (run .\scripts\build_gna.ps1) or set GNA_LIB_DIR to the folder containing gna.lib / gna.dll / libgna.so"
    exit 1
}

# Locate gna.lib (Windows import lib) or libgna.so / libgnad.so (Linux/mac)
Write-Host "Searching for GNA import library under $buildRoot"
$lib = Get-ChildItem -Path $buildRoot -Recurse -Include 'gna.lib','libgna.so','libgnad.so','gna.dll' -ErrorAction SilentlyContinue | Select-Object -First 1
if (-not $lib) {
    Write-Error "GNA library not found under $buildRoot. Build GNA first (run .\scripts\build_gna.ps1) or set GNA_LIB_DIR to the folder containing gna.lib / gna.dll / libgna.so"
    exit 1
}

$libDir = $lib.DirectoryName
Write-Host "Found GNA library: $($lib.FullName)"
Write-Host "Using GNA_LIB_DIR=$libDir"

# Run cargo with env var set for this process
$env:GNA_LIB_DIR = $libDir

# Build and run the example with feature
$extra = $Args -join ' '
Write-Host "Running: cargo run --features link_gna --example load_test -- $extra"
cmd /c "cargo run --features link_gna --example load_test -- $extra"
