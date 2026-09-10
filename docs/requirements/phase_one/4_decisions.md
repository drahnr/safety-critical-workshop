# *RAD* Decision Improvements Phase 1

This section contains improvements for the *RAD* product during the first phase of the workshop.

## `rad.sw.operation.invariant`: Operation mode invariants

- **Parents:** ["risk.wrong-usage"]

While *RAD* is in `operation` mode, the checks from [req_link("rad.sw.operation.pre-condition")] must be continuously monitored
and if they are not fulfilled, the *RAD* must stop radiation therapy and go to `idle` mode.
This is a safety measure to prevent for example people from entering the safe environment during radiation therapy.
