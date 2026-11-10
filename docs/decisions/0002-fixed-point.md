# 0002: Fixed-point numbers

Status: settled.

## The gap in the spec

The spec says "integers and fixed-point for all state" without giving a format.

## Decision

- One type, `Fixed`: a signed 64-bit integer counting millionths. Six decimal places.
- Multiplication and division use a 128-bit intermediate and truncate toward zero.
- `mul_div(a, b, c)` computes `a * b / c` in one step, for when a separate divide would lose precision.
- Overflow panics. `overflow-checks` is on in release builds, so debug and release behave the same.
- Data files give quantities as whole numbers or decimal strings. TOML floats are rejected, so no value passes through floating point.

## Why

Six decimals is enough for hours, miles and tons, and leaves a range of about nine trillion, far beyond any quantity in the game. Truncation is the simplest rule to reason about and to match in a hand calculation.

## Consequence

Results that do not divide evenly lose up to one millionth per operation. Rules that chain many operations should use `mul_div` and should cap results where an invariant depends on them. Fuel burn does this: it is capped at the fuel aboard.
