# `risk`: Risk Analysis of *RAD*

This section covers the risk analysis of *RAD*.
Identifying potential risks and specifying related risk control measures is common practice for safety-critical products.

**Note:** To keep the demo simple, we ignore potential hardware faults and connection issues.

## `risk.wrong-usage`: Wrong usage

If the *RAD* is wrongfully configured or used outside a safe operation environment,
serious injury and even death is possible.

To prevent this, the *RAD* must be designed and built to prevent wrongful usage that could injure someone.

## `risk.wrong-cell`: Wrong cells targeted

- **Parents:** ["goal.kill-cancer"]
- **Manual Verification:** true

If the *RAD* targets healthy cells instead of malignant once, patient health is reduced while malignat cells may continue to grow.
This significantly increases the likelihood of death for a patient.

To prevent this risk, the *RAD* must reduce the number of affected healthy cells as much as possible while targeting malignant cells.

## `risk.patient-behavior`: Unpredictable patient behavior

- **Manual Verification:** true

A patient may behave in unpredictable ways that may increase the chance of injuries or damage the *RAD*. This may happen for example due to increased stress and unfamiliar environment for the patient.

To prevent this risk, personal must be instructed on how to handle such patients and how to shut down the *RAD* in a fast, safe, and secure manner.

**Note:** In practice, additional measures would be taken. For example, the *RAD* hardware would be designed for robustness and additional software control may be added for a fast and secure shutdown..
