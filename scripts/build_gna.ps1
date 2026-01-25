<#
PowerShell helper: Configure and build the GNA native library (Windows, Visual Studio generator).
Usage:
  .\scripts\build_gna.ps1 -Configuration Release -Arch x64
#>
param(
    [ValidateSet('Debug','Release')]
    [string]$Configuration = 'Release',
    [ValidateSet('x64','x86')]
    [string]$Arch = 'x64'
)

$root = Resolve-Path -Path "$(Split-Path -Parent $PSScriptRoot)"
$gnaSrc = Join-Path $root 'gna'
$buildDir = Join-Path $gnaSrc 'build'

Write-Host "Configuring GNA (src=$gnaSrc build=$buildDir)"
cmake -S $gnaSrc -B $buildDir -A $Arch -DCMAKE_BUILD_TYPE=$Configuration

Write-Host "Building GNA (config=$Configuration)"
cmake --build $buildDir --config $Configuration --target gna -- -m

Write-Host "Build finished. You can run scripts/run_load_test_with_gna.ps1 to set GNA_LIB_DIR and run the example."
