[windows]
set shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

mantra:
    if (Test-Path target/mantra-report) { Remove-Item -Recurse -Force target/mantra-report }
    mantra collect
    mantra report --output-dir=target/mantra-report

base:
    just test base

phase-one:
    just test phase-one

phase-two:
    just test phase-two

test phase='base':
    just rad-unit-tests
    just rad-system-tests {{ phase }}
    just mantra

[working-directory("sim")]
sim-manual:
    $env:DEFMT_LOG = "info"; cargo run --bin manual --no-default-features

[working-directory("sim")]
sim-start-stop:
    $env:DEFMT_LOG = "info"; cargo run --bin start_stop_flow --no-default-features

[working-directory("sim")]
sim-build-start-stop:
    $env:DEFMT_LOG = "info"; cargo build --bin start_stop_flow

[working-directory("sim")]
sim-build-invariant-check:
    $env:DEFMT_LOG = "info"; cargo build --bin invariant_check

[working-directory("sim")]
sim-invariant-check:
    $env:DEFMT_LOG = "info"; cargo run --bin invariant_check --no-default-features

[working-directory("sim")]
sim-build-limit-radiation:
    $env:DEFMT_LOG = "info"; cargo build --bin limit_radiation

[working-directory("sim")]
sim-limit-radiation:
    $env:DEFMT_LOG = "info"; cargo run --bin limit_radiation --no-default-features

[working-directory("sim")]
sim-reset:
    $env:DEFMT_LOG = "info"; cargo run --bin reset

[working-directory("sim")]
sim-build-reset:
    $env:DEFMT_LOG = "info"; cargo build --bin reset

[working-directory("rad")]
rad-run:
    $env:DEFMT_LOG = "info"; cargo run --bin rad --features=hw-testing

[working-directory("rad")]
rad-build features='hw':
    $env:DEFMT_LOG = "info"; cargo build --bin rad --features={{ features }}

profraw-file := justfile_directory() + "/target/nextest/default/raw-coverage/profdata-%p-%m.profraw"

rad-unit-tests:
    #!powershell
    if (Test-Path target/nextest/default) { Remove-Item -Recurse -Force target/nextest/default }
    New-Item -ItemType Directory -Force -Path target/nextest/default/coverage | Out-Null
    $env:RUSTFLAGS = "-Cinstrument-coverage"
    $env:LLVM_PROFILE_FILE = "{{ profraw-file }}"
    cargo nextest run -p rad --lib --target=thumbv7em-ferrocene.facade-eabi --no-default-features; $LASTEXITCODE = 0
    # $LASTEXITCODE = 0 set so code coverage is gathered even if tests failed
    grcov . -s . --binary-path ./target -t html -t cobertura-pretty --ignore-not-existing -o ./target/nextest/default/coverage/ --ignore='/**/' --ignore='target/'

export EMBSINTH_OUT_DIR := justfile_directory() + "/target"

[working-directory("system-tests")]
rad-system-tests phase='base':
    #!powershell
    if (Test-Path "$env:EMBSINTH_OUT_DIR/system-tests") { Remove-Item -Recurse -Force "$env:EMBSINTH_OUT_DIR/system-tests" }
    just rad-build {{ if phase == "phase-one" { "hw-auto-testing,phase-one" } else if phase == "phase-two" { "hw-auto-testing,phase-two" } else { "hw-auto-testing" } }}
    just sim-build-reset
    just sim-build-start-stop
    just sim-build-invariant-check
    just sim-build-limit-radiation
    # "-j=1" is important for cargo-nextest, because it otherwise uses multiple processes to run tests in parallel
    # $LASTEXITCODE = 0 set so 'just post-process' is called even if tests failed
    $env:RUST_LOG = "probe_rs=warn,tracing=warn,info"
    cargo nextest run -j=1 --target=host-tuple {{ if phase == "phase-one" { "--features=phase-one" } else if phase == "phase-two" { "--features=phase-two" } else { "" } }}; $LASTEXITCODE = 0
    just post-process

post-process tests='system-tests':
    embsinth post-process --out "$env:EMBSINTH_OUT_DIR/{{ tests }}/mantra_test_run.json" --test-run-name {{ tests }} "$env:EMBSINTH_OUT_DIR/{{ tests }}"
