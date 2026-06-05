# Refactor: Converter Strategy Pattern

## Context
Replace the current `designer.py` (pure functions for buck specifically) with a Strategy pattern. Each converter implements a common interface with:
- `compute_components(DesignParams) → DesignResults`
- `analyze(DesignParams, DesignResults) → ConverterResults`

This makes adding Boost, VSI, or future converters trivial.

## Files to Create/Modify
- `tupan/domain/strategy.py` — New: ConverterStrategy ABC + BuckStrategy + registry
- `tupan/app/state.py` — Use strategy pattern instead of direct buck imports
- `tupan/domain/designer.py` — Delete: replaced by BuckStrategy
- `tupan/tests/test_designer.py` → `tupan/tests/test_strategy.py` — Update tests

## Steps
1. Create `tupan/domain/strategy.py`
2. Update `tupan/app/state.py` to use strategy
3. Delete `tupan/domain/designer.py`
4. Update tests
