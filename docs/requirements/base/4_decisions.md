# `rad`: *RAD* Implementation Decisions

This requirement groups the implementation decisions derived from the [*RAD* system requirements](./3_system.md).

## `rad.mode`: Operation mode of *RAD*

- **Parents:** ["usage.start-stop"]

Since the *RAD* must only support starting and stopping radiation therapy, it is sufficient to use the two modes:

- `idle` ... *RAD* is waiting for input. No radiation is output
- `operation` ... *RAD* is performing radiation therapy

**Note:** Additional modes for e.g. `startup` or `shutdown` could be added, but are ignored for this demo.

## `rad.ux`: User experience for *RAD*

- **Parents:** ["usage", "risk.wrong-usage"]

This requirement groups all requirements that are related to the UI/UX of *RAD*.
If *RAD* is easy to use for medical personal, the chance for wrong usage is reduced.

### `rad.ux.mode-indicator`: Indicate mode of *RAD*

- **Parents:** ["rad.mode"]

*RAD* must have an indicator to show if it is in `operation` mode  or in `idle` mode,
so personal can better observe the state of *RAD*.

## `rad.hw`: Hardware requirements for *RAD*

- **Manual Verification:** true

This requirement groups all hardware requirements.
Manual verification enforces that a review is needed for this requirement to be verified.

### `rad.hw.mode-indicator`: Indicate mode of *RAD*

- **Parents:** ["rad.ux.mode-indicator"]

A LED will be used to indicate if *RAD* is performing radiation therapy (LED is ON) or is in `idle` mode (LED is OFF).

### `rad.hw.start-stop-switch`: Use switch to start/stop radiation therapy

- **Parents:** ["usage.start-stop", "risk.wrong-usage"]

A switch will be used to start and stop radiation therapy, because flipping the switch makes it easy to stop radiation therapy,
which should help correct usage if fast and safe shutdown is required.

**Note:** No explicit status LED for the switch, because the mode indicator LED showing if *RAD* started radiation therapy or is in `idle` mode is considered to be enough as indication.

### `rad.hw.door-sensor`: Sensor for the entrance door

- **Parents:** ["usage.safe-environment.restrict-access"]

A sensor on the entrance door of the enclosure will be used to verify that access is restricted during radiation therapy.
This assumes that the door is the only way to access the safe environment and the door can only be opened by instructed personal.

#### `rad.hw.door-sensor.status-LED`: Status LED for door sensor

- **Parents:** ["rad.ux"]
- **Optional:** true

A LED will be used to indicate if the entrance door is closed (LED is ON) or open (LED is OFF).

### `rad.hw.confirmation-switch`: Use key-switch for confirmation

- **Parents:** ["usage.safe-environment.confirmation"]

A key switch will be used for the safe-environment confirmation that must be given by the instructed personal before radiation therapy may start. The key switch ensures that only personal with access to the key can operate *RAD*.

#### `rad.hw.confirmation-switch.status-LED`: Status LED for confirmation switch

- **Parents:** ["rad.ux"]
- **Optional:** true

A LED will be used to indicate if the confirmation switch is locked/confirmed (LED is ON) or unlocked/unconfirmed (LED is OFF).

### `rad.hw.radiation-relay`: Control radiation via relay

- **Parents:** ["radiation.on-off"]

A relay will be used to control radiation output, because digital outputs of microcontrollers typically do not have enough power to control motor, valve, or other actuators needed to control radiation.

**Note:** This is a simplification for the demo and will be much more complex in practice.

#### `rad.hw.radiation-relay.status-LED`: Status LED for radiation output

- **Parents:** ["rad.ux"]

A LED will be used to indicate if radiation output is on (LED is ON) or off (LED is OFF).

### `rad.hw.radiation-sensor`: Sensor to measure radiation output

- **Parents:** ["radiation.measure"]

A digital input is used to detect if radiation is output.

### `rad.hw.mcu`: *RAD* microcontroller

- **Parents:** [
    "rad.hw.mode-indicator",
    "rad.hw.start-stop-switch",
    "rad.hw.door-sensor",
    "rad.hw.confirmation-switch",
    "rad.hw.radiation-sensor",
    "rad.hw.door-sensor.status-LED",
    "rad.hw.confirmation-switch.status-LED",
    "rad.hw.radiation-relay",
    "rad.hw.radiation-relay.status-LED"
    ]
- **Manual Verification:** true

The chosen microcontroller for *RAD* must support the number of inputs and outputs required by all requirements listed under [req_link("rad.hw")].

**Digital Inputs:**
- Start/Stop switch ... [req_link("rad.hw.start-stop-switch")]
- Door sensor ... [req_link("rad.hw.door-sensor")]
- Confirmation key switch ... [req_link("rad.hw.confirmation-switch")]
- Radiation sensor ... [req_link("rad.hw.radiation-sensor")]

**Digital Outputs:**
- *RAD* operation mode LED ... [req_link("rad.hw.mode-indicator")]
- Door sensor state LED ... [req_link("rad.hw.door-sensor.status-LED")]
- Confirmation switch state LED ... [req_link("rad.hw.confirmation-switch.status-LED")]
- Radiation relay ... [req_link("rad.hw.radiation-relay")]
- Radiation relay state LED ... [req_link("rad.hw.radiation-relay.status-LED")]

The demo is built for the [nRF52840 DK](https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dk), but other microcontrollers may also fit.

## `rad.sw`: Software requirements for *RAD*

This requirement groups all software requirements.

### `rad.sw.hal`: Hardware Abstraction Layer (HAL)

- **Parents:** ["rad.hw.mcu"]

The HAL must interact with the chosen microcontroller and I/O to control *RAD*.

### `rad.sw.idle`: Idle mode

- **Parents:** ["rad.mode"]

The `idle` mode is the default one *RAD* is in if no radiation therapy is active, meaning radiation is turned off.
This mode keeps the *RAD* in a safe mode in which instructed personal may position the patient to prepare for radiation therapy,
or to accompany a patient outside the safe environment after radiation therapy has been stopped.

### `rad.sw.operation`: Operation mode

- **Parents:** ["rad.mode", "usage.safe-environment.restrict-access", "usage.safe-environment.confirmation"]

The *RAD* is in `operation` mode during radiation therapy.
This mode must only be active if the restrictions defined by [req_link("usage.safe-environment.restrict-access")]
and [req_link("usage.safe-environment.confirmation")] are fulfilled.

#### `rad.sw.operation.pre-condition`: Operation pre-conditions

Before *RAD* is allowed to switch to `operation` mode, the following checks must be fulfilled:
- Door sensor is closed
- Safe-environment confirmation is confirmed

#### `rad.sw.operation.start`: Operation start

- **Parents:** ["rad.hw.start-stop-switch", "rad.hw.radiation-relay"]

If the checks from [req_link("rad.sw.operation.pre-condition")] are fulfilled,
radiation therapy may be started by setting the start/stop-switch to start.
This will change *RAD* to the `operation` mode and activate radiation output.

#### `rad.sw.operation.stop`: Operation stop

- **Parents:** ["rad.hw.start-stop-switch", "rad.hw.radiation-relay"]

Radiation therapy may be stopped by setting the start/stop-switch to stop.
This will change *RAD* to the `idle` mode and deactivate radiation output.

#### `rad.sw.operation.post-condition`: Operation post-conditions

- **Parents:** ["rad.hw.radiation-relay"]

The *RAD* must ensure that radiation output is turned off before switching to `idle` mode.

### `rad.sw.indicator`: Set LED indicators

- **Parents:** [
    "rad.hw.mode-indicator",
    "rad.hw.door-sensor.status-LED",
    "rad.hw.confirmation-switch.status-LED",
    "rad.hw.radiation-relay.status-LED"
]

The *RAD* must indicate the current mode it is in and the state of the enclosing door, the confirmation switch
and the radiation output so personal can get a better understanding of what is happening.

Since the status-LED can only be `ON` or `OFF`, the `operation` mode is indicated by the LED being `ON`,
while the led being `OFF` indicates that *RAD* is in `idle`.
