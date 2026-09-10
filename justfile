mantra:
    rm -rf target/mantra-report
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
    DEFMT_LOG=info cargo run --bin manual --no-default-features

[working-directory("sim")]
sim-start-stop:
    DEFMT_LOG=info cargo run --bin start_stop_flow --no-default-features

[working-directory("sim")]
sim-build-start-stop:
    DEFMT_LOG=info cargo build --bin start_stop_flow

[working-directory("sim")]
sim-build-invariant-check:
    DEFMT_LOG=info cargo build --bin invariant_check

[working-directory("sim")]
sim-invariant-check:
    DEFMT_LOG=info cargo run --bin invariant_check --no-default-features

[working-directory("sim")]
sim-build-limit-radiation:
    DEFMT_LOG=info cargo build --bin limit_radiation

[working-directory("sim")]
sim-limit-radiation:
    DEFMT_LOG=info cargo run --bin limit_radiation --no-default-features

[working-directory("sim")]
sim-reset:
    DEFMT_LOG=info cargo run --bin reset

[working-directory("sim")]
sim-build-reset:
    DEFMT_LOG=info cargo build --bin reset

[working-directory("rad")]
rad-run:
    DEFMT_LOG=info cargo run --bin rad --features=hw-testing

[working-directory("rad")]
rad-build features='hw':
    DEFMT_LOG=info cargo build --bin rad --features={{ features }}

profraw-file := justfile_directory() + "/target/nextest/default/raw-coverage/profdata-%p-%m.profraw"

rad-unit-tests:
    rm -rf target/nextest/default
    mkdir -p target/nextest/default/coverage/
    - RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="{{ profraw-file }}" cargo nextest run -p rad --lib --target=thumbv7em-ferrocene.facade-eabi --no-default-features
    grcov . -s . --binary-path ./target -t html -t cobertura-pretty --ignore-not-existing -o ./target/nextest/default/coverage/ --ignore='/**/*' --ignore='target/*'

export EMBSINTH_OUT_DIR := justfile_directory() + "/target"

[working-directory("system-tests")]
rad-system-tests phase='base':
    rm -rf $EMBSINTH_OUT_DIR/system-tests
    just rad-build {{ if phase == "phase-one" { "hw-auto-testing,phase-one" } else if phase == "phase-two" { "hw-auto-testing,phase-two" } else { "hw-auto-testing" } }}
    just sim-build-reset
    just sim-build-start-stop
    just sim-build-invariant-check
    just sim-build-limit-radiation
    # "-j=1" is important for cargo-nextest, because it otherwise uses multiply processes to run tests in parallel
    - RUST_LOG=probe_rs=warn,tracing=warn,info cargo nextest run -j=1 --target=host-tuple {{ if phase == "phase-one" { "--features=phase-one" } else if phase == "phase-two" { "--features=phase-two" } else { "" } }}
    just post-process

post-process tests='system-tests':
    embsinth post-process --out $EMBSINTH_OUT_DIR/{{ tests }}/mantra_test_run.json --test-run-name {{ tests }} $EMBSINTH_OUT_DIR/
