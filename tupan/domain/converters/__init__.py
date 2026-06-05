"""Converter modules — analytical models and strategy classes.

Each converter module (buck.py, boost.py, etc.) exports:
  - Pure analytical functions (output_voltage, inductor_current_ripple, etc.)
  - A Strategy class (e.g. BuckStrategy) inheriting from ConverterStrategy

The strategy registry lives here.
"""

from abc import ABC, abstractmethod

from tupan.domain.design_params import DesignParams, DesignResults
from tupan.domain import ConverterResults


def clamp_duty(d: float, lo: float = 0.01, hi: float = 0.99) -> float:
    """Clamp duty cycle to valid range."""
    return max(lo, min(hi, d))


class ConverterStrategy(ABC):
    """Abstract base for all converter design strategies.

    Each strategy implements two methods:
      compute_components(design_params) -> DesignResults
      analyze(design_params, design_results) -> ConverterResults
    """

    @abstractmethod
    def name(self) -> str:
        """Human-readable converter name."""
        ...

    @abstractmethod
    def label(self) -> str:
        """Short UI label (e.g. 'Buck')."""
        ...

    @abstractmethod
    def compute_components(self, params: DesignParams) -> DesignResults:
        """Design params -> required L, C, R."""
        ...

    @abstractmethod
    def analyze(self, params: DesignParams,
                components: DesignResults) -> ConverterResults:
        """Design params + components -> operating point results."""
        ...

    def duty_from_vout(self, vin: float, vout: float) -> float:
        if vin <= 0:
            return 0.01
        return clamp_duty(vout / vin)

    def vout_from_duty(self, vin: float, duty: float) -> float:
        return vin * clamp_duty(duty)


# ── Strategy Registry ──

_strategies: dict[str, ConverterStrategy] = {}

def register_strategy(strategy: ConverterStrategy):
    """Register a converter strategy."""
    _strategies[type(strategy).__name__] = strategy


def get_strategy(name: str) -> ConverterStrategy | None:
    """Get a strategy by class name."""
    return _strategies.get(name)


def get_all_strategies() -> list[ConverterStrategy]:
    """Get all registered strategies."""
    return list(_strategies.values())
