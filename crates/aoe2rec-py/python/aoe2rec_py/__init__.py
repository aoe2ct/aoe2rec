try:
    from typing import override
except ImportError:
    def override(f):
        return f