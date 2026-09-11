# Safety-Critical Rust Development with Ferrocene

This repository contains the accompanying demo for the [Safety-Critical Rust Development with Ferrocene](https://oxidizeconf.com/sessions/safety_critical_rust_development_with_ferrocene) workshop.

## Prerequisites

**The following tools must be installed:**

- Install [criticalup](https://criticalup.ferrocene.dev/install.html)
- Docker installed and usable (see OS specific instructions below)
- **Optional:** Rust toolchain via [rustup](https://rust-lang.org/tools/install/).  
  See [Ferrocene Setup](#ferrocene-setup) on how to link Ferrocene with rustup.
- `cargo install mantra --locked` (This requires a [modern native C compiler](https://docs.rs/cc/latest/cc/#compile-time-requirements) via the cc binary (usually clang or gcc))
- `cargo install cargo-nextest --locked`
- `cargo install grcov --locked`
- `cargo install just --locked`
- `cargo install embsinth --locked`
- `probe-rs` following the official [installation section](https://probe.rs/docs/getting-started/installation/)

### macOS

To run docker container on macOS, you may install [lima](https://lima-vm.io).
Follow [their documentation](https://lima-vm.io/docs/examples/containers/docker/) to use it as alias for docker commands:

```sh
limactl start --mount-writable template:docker
export DOCKER_HOST=$(limactl list docker --format 'unix://{{.Dir}}/sock/docker.sock')
# To verify if it worked, run:
docker run -d --name nginx -p 127.0.0.1:8080:80 nginx:alpine
```

**Note:** `limactl start --mount-writable template:docker` must only be run the first time. Afterwards, run `limactl start docker`.

**WARN:** Setting `--mount-writable` makes the home directory writable from the container.
This is needed to get raw LLVM coverage data during unit testing, but may pose security risks if other docker container are run.

Once Lima and Docker are installed, try building the Dockerfile of this repository as described in section [QEMU Docker Container](#qemu-docker-container).

### Windows

For Windows users, Docker should be configured to use WSL 2 as documented in the official [docker documentation](https://docs.docker.com/desktop/features/wsl/).
Once Docker is installed, try building the Dockerfile of this repository as described in section [QEMU Docker Container](#qemu-docker-container).

To execute commands via [just](https://just.systems), replace the `justfile` with `windows.justfile`.
This file is configured to run commands via powershell.

**Note:** If you get errors executing for example `just rad-unit-tests`, try to remove the `#!powershell` line at the top of the task.
There seems to be an inconsistent handling of environmental variables with just on Windows.

## Facade Target Setup

Ferrocene's [Facade targets](https://public-docs.ferrocene.dev/main/user-manual/rustc/testing-facades.html)
allow to run regular Rust unit tests in an emulator of the CPU architecture of the actual target.

### QEMU Docker Container

For convenience, the Dockerfile in this repository provides the needed `qemu-arm-static` binary to run `thumbv7em` binaries.

**To build the image locally, run:**

```sh
docker buildx build --load -t ubuntu-qemu-arm .
```

The workspace level `.cargo/config.toml` file is set up to use the Docker container as runner for the Facade target.
The default configuration works for Linux and macOS, but must be changed for Windows hosts due to filepath incompatibilities.

**For Windows User:** Uncomment the runner configuration for Windows and comment the one for Linux and macOS.

### Ferrocene Setup

The Facade targets are special targets available with Ferrocene.
Similar to rustup, we provide [criticalup](https://criticalup.ferrocene.dev) to manage Ferrocene installations.

Assuming `criticalup` is installed, authenticate and install Ferrocene via:

```sh
# This will ask you for an authentication token
criticalup auth set
# This will install Ferrocene as configured in 'criticalup.toml'
criticalup install
```

This repository has set `ferrocene` as default rustup channel in `rust-toolchain.toml`
to ensure Ferrocene is used for all Cargo commands.

**To make Ferrocene available for rustup, run:**

```sh
criticalup link create
```

### Running Unit Tests

With Docker and the container set up and the prerequisites installed, you should be able to run:

```sh
just rad-unit-tests
```

This will run all unit tests and collect code coverage data.

## Hardware Setup

For manual and system testing, two NRF 52840 DK devices must be connected to your host machine.

Related Datasheets from Nordic:

- [nRF52840 DK Hardware](https://docs.nordicsemi.com/r/bundle/ug_nrf52840_dk/page/ug/dk/intro.html)
- [nRF52840 Product Specification](https://docs.nordicsemi.com/r/bundle/ps_nrf52840/page/keyfeatures_html5.html)

The main device is referred to as **RAD** and is the one the main `rad` application is being flashed on.
For testing purpose, the second device referred to as **SIM** is used to simulate the environment the application is being used in.

The two boards are connected through pins `P1.02` to `P1.08` with `P1.02` to `P1.04` being **RAD** outputs and **SIM** inputs.
Mapping is done in a way that makes it easy to use two 4-pin male-to-male cables as shown in the [HW-Setup.pdf](HW-Setup.pdf) schematic.

**Note:** Not all 8 pins are used, but using two 4-pin cables makes connecting the two boards easier.

### I/O Mapping

The following lists show how the I/O pins, LEDs and buttons of the two boards are connected.

**RAD:**

- Outputs
  - p1.01: Radiation Control
    - Low: Start Radiation
    - High: Stop Radiation
  - p1.02: Mode Indicator
    - Low: Operation
    - High: Idle
  - p1.03: Start-Stop Indicator
    - Low: Start requested
    - High: Stop requested
- Inputs
  - p1.05: Start-Stop Switch
    - Low: Start
    - High: Stop
  - p1.06: Door Sensor
    - Low: Closed
    - High: Open
  - p1.07: Confirmation Switch
    - Low: Confirmed
    - High: Unconfirmed
  - p1.08: Radiation Sensor
    - Low: Active
    - High: Deactive
- LEDS
  - 1: Mode Indicator (ON = Operation)
  - 2: Door Sensor (ON = Closed)
  - 3: Confirmation Switch (ON = Confirmed)
  - 4: Radiation Relay (ON = Active)

**SIM:**

- Outputs
  - p1.01: Start-Stop Switch
    - Low: Start
    - High: Stop
  - p1.02: Door Sensor
    - Low: Closed
    - High: Open
  - p1.03: Confirmation Switch
    - Low: Confirmed
    - High: Unconfirmed
  - p1.04: Radiation
    - Low: Active
    - High: Deactive
- Inputs
  - p1.05: Radiation Relay
    - Low: On
    - High: Off
  - p1.06: RAD Mode
    - Low: Operation
    - High: Idle
  - p1.07: Start Request Indicator
    - Low: Start requested
    - High: undefined
- LEDs
  - 1: Start-Stop Switch (ON = Start)
  - 2: Door Sensor (ON = Closed)
  - 3: Confirmation Switch (ON = Confirmed)
  - 4: Radiation State (ON = Active)
- Toggle Buttons
  - 1: Start-Stop Switch
  - 2: Door Sensor
  - 3: Confirmation Switch
  - 4: Radiation

### Adapt Debug Probe IDs

Once both devices are connected to the host and the 4-pin cables are connected between the devices,
the debug probe IDs must be updated to point to the actual attached devices.
To get the serial numbers of the attached probes, look at the white sticker on the DK devices.
The number at the bottom is the serial number. This can be confirmed by running:

```sh
probe-rs list
```

This should return a similar output to:

```stdout
The following debug probes were found:
[0]: J-Link -- 1366:1051:001050286871 (J-Link)
[1]: J-Link -- 1366:1051:001050272949 (J-Link)
```

The output ID per probe is: `<vendor ID>:<probe ID>:<serial number>`

Since vendor and probe ID should be the same for all DK devices,
only the serial numbers for the **RAD** and **SIM** devives must be replaced in:

- `rad/.cargo/config.toml` using the number of the **RAD**
- `sim/.cargo/config.toml` using the number of the **SIM**
- `system-tests/src/lib.rs` changing the two constants at the top

### Validate Connection

Follow the instructions documented in `docs/reviews/phase_one/rad_hw.json5` to ensure both devices are connected successfully and I/O mapping is wired as outlined in the hardware setup.

With this working, you can now run all system tests via:

```sh
just rad-system-tests
```
