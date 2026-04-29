# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Protocol, Optional, Any


class TieXiu(Protocol):
    """Object-oriented API for TieXiu parser generator."""

    def get(self, grammar: str) -> Optional[Any]:
        """Get cached grammar if exists."""
        ...

    def get_or_compile(self, grammar: str) -> Any:
        """Get cached or compile new grammar."""
        ...

    def parse_grammar(self, grammar: str) -> Any:
        """Parse grammar string to AST."""
        ...

    def parse_grammar_to_json(self, grammar: str) -> str:
        """Parse grammar to JSON AST."""
        ...

    def compile(self, grammar: str) -> Any:
        """Compile grammar to parser."""
        ...

    def compile_to_json(self, grammar: str) -> str:
        """Compile grammar to JSON."""
        ...

    def load(self, json: str) -> Any:
        """Load grammar from JSON."""
        ...

    def load_tree(self, json: str) -> Any:
        """Load AST from JSON."""
        ...

    def boot_grammar(self) -> Any:
        """Get the bootstrap grammar."""
        ...

    def boot_grammar_to_json(self) -> str:
        """Get bootstrap grammar as JSON."""
        ...

    def boot_grammar_pretty(self) -> str:
        """Get bootstrap grammar as pretty-printed string."""
        ...


class Grammar(Protocol):
    """Compiled grammar object."""

    def to_json(self) -> str:
        """Convert grammar to JSON."""
        ...

    def pretty(self) -> str:
        """Pretty print the grammar."""
        ...