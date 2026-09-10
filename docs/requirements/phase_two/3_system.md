# *RAD* System Requirement Improvements Phase 2

This section builds uppon the phase two risk analysis to apply everything learned during the workshop.

## `radiation.measure-v2`: Measure output radiation

- **Parents:** ["risk.intensity"]
- **Replaces:** ["radiation.measure"]

It must be possible to measure the output radiation of *RAD* to validate that radiation is either turned ON or OFF,
and to prevent radiation output from exceeding the threshold defined by [req_link("risk.intensity")](./2_risk-analysis.md).

**Note:** Since this is a significant change to the original [req_link("radiation.measure")] requirement,
a new requirement with the postfix `-v2` is created and the [req_link("radiation.measure")]
is replaced, which marks it as *deprecated*. 

**Note:** Replacing [req_link("radiation.measure")] forces to update all locations that trace to it,
which is useful for a significant change of the requirement. For smaller changes to a requirement,
updating the related information and checking that the implementation still holds may be sufficient.

## `radiation.limit-output`: Limit radiation output

- **Parents:** ["risk.intensity"]

The *RAD* product must ensure that the safe threshold defined by [req_link("risk.intensity")](./2_risk-analysis.md) is not exceeded
to prevent serious injury or death of patients.
