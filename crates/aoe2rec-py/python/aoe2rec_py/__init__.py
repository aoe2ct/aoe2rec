from .summary import RecSummary

try:
    from typing import override
except ImportError:
    def override(f):
        return f

__all__ = ["RecSummary", "override"]
