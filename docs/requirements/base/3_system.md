# *RAD* System Requirements

This section contains all system requirements for the *RAD* device that are derived from the [goals](./1_goals.md) and [risk](2_risk-analysis.md) sections. 

## `usage`: Using *RAD*

This requirement is the root for all requirements related to using *RAD*.

### `usage.start-stop`: Start/Stop radiation therapy

- **Parents:** ["goal.kill-cancer"]

Medical personal must be able to start and stop radiation therapy to treat patients.

### `usage.safe-environment`: Allow use only in safe environment

- **Parents:** ["risk.wrong-usage"]

The *RAD* product must only be used in an enclosable space that restricts operation to instructed personal.
To further reduce chance of injuries, the instructed personal must ensure that only the patient remains in the operational environement of *RAD*
during radiation therapy.

#### `usage.safe-environment.restrict-access`: Restrict access to safe environment

- **Manual Verification:** true

The *RAD* product must enforce that access to the safe environment is restricted and cannot be accessed during active radiation therapy.
Manual verification of the enclosure is needed to ensure that the *RAD* is able to enforce this.

#### `usage.safe-environment.confirmation`: Enforce safe environment confirmation

The *RAD* product must enforce explicit confirmation from instructed personal that the safe environment is guaranteed before radiation therapy may start. This increases the likelihood that instructed personal have successfully followed the mandatory instructions.

**Note:** It is also likely done for legal reasons to shift blame to the personal, but legal discussions are ignored in this demo.

**Note:** Similar to `usage.safe-environment.restrict-access`, manual verification would in practice be needed to ensure that the hardware
is properly built and connected, but this is ignored for the demo since we only focus on the software part.

### `usage.display-status`: Display status information

- **Optional:** true

*RAD* status information should be displayed in a way that is accessible by instructed personal.

**Note:** This requirement is set to optional and is therefore not necessary for the product to be verified.

## `radiation`: Output radiation

- **Parents:** ["goal.kill-cancer"]

The *RAD* product must be able to output radiation, which is needed to kill malignant cells.

### `radiation.on-off`: Turn radiation ON/OFF

- **Parents:** ["usage.start-stop"]

It must be possible to turn radiation ON and OFF to start and stop radiation therapy.

### `radiation.measure`: Measure output radiation

- **Optional:** true

It should be possible to measure the output radiation of *RAD* to validate that radiation is either turned ON or OFF.
