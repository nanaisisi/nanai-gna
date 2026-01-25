Quick helper scripts to build GNA native library and run the example with linking.

Windows (PowerShell):

1. Build GNA native library (Visual Studio generator):
   .\scripts\build_gna.ps1 -Configuration Release -Arch x64

2. Run the example with the built library (script finds library and sets GNA_LIB_DIR):
   PowerShell: .\scripts\run_load_test_with_gna.ps1 --threads 8 --duration 60 --device-open-close true
   POSIX: bash ./scripts/run_load_test_with_gna.sh --threads 8 --duration 60 --device-open-close true

3. cargo-make shortcuts (recommended):

   - Build native GNA (auto chooses script for your OS):
     cargo make build-gna

   - Run the load test and forward args:
     cargo make run-load-test -- --threads 8 --duration 60 --device-open-close true

Notes:

- The scripts assume you have CMake and build tools installed (Visual Studio on Windows / make/ninja on Linux/macOS).

Notes:

- The scripts assume you have CMake and Visual Studio build tools installed on Windows.
- If you prefer, you can set GNA_LIB_DIR manually to the directory containing gna.lib (or gna.dll / libgna.so) and then run:
  cargo run --features link_gna --example load_test -- --threads 8 --duration 60 --device-open-close true
