# *RAD* Decision Improvements Phase 2

This section builds uppon the phase two system requirements to apply everything learned during the workshop.

## `rad.sw.limit-radiation`: Limit radiation output

- **Parents:** ["radiation.limit-output", "radiation.on-off", "rad.sw.operation"]

Independent of the current mode, radiation output must not be turned on if the upper threshold defined by [req_link("risk.intensity")]
is crossed.
For a simplistic control loop, radiation will be turned on again once radiation falls below a lower threshold if *RAD* is currently in mode `operation`.

**Note:** Both thresholds would in practice be set to a fixed value or range and not kept vague.
