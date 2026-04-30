# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Any

import tiexiu.tiexiu as rust


def pegapi(**kwargs: Any) -> Any:
    """Create TieXiu OO API instance."""
    if kwargs:
        return rust.pegapi(**kwargs)
    return rust.pegapi()

def parse_grammar(grammar: str, **kwargs: Any) -> Any:
    """Parse grammar string to AST tree."""
    return rust.parse_grammar(grammar, **kwargs)


def parse_grammar_to_json(grammar: str, **kwargs: Any) -> Any:
    """Parse grammar to JSON (dict, list, etc.)."""
    return rust.parse_grammar_to_json(grammar, **kwargs)


def parse_grammar_to_json_string(grammar: str, **kwargs: Any) -> str:
    """Parse grammar to JSON string."""
    return rust.parse_grammar_to_json_string(grammar, **kwargs)


# def parse_grammar_with(cursor: Any, **kwargs: Any) -> Any:


# def parse_grammar_to_json_with(cursor: Any, **kwargs: Any) -> Any:


def compile(grammar: str, **kwargs: Any) -> Any:
    """Compile grammar, return Grammar object."""
    return rust.compile(grammar, **kwargs)


def compile_to_json(grammar: str, **kwargs: Any) -> Any:
    """Compile grammar to JSON (dict, list, etc.)."""
    return rust.compile_to_json(grammar, **kwargs)


def compile_to_json_string(grammar: str, **kwargs: Any) -> str:
    """Compile grammar to JSON string."""
    return rust.compile_to_json_string(grammar, **kwargs)


# def compile_with(cursor: Any, **kwargs: Any) -> Any:


# def compile_to_json_with(cursor: Any, **kwargs: Any) -> Any:


def load_from_json(json: str, **kwargs: Any) -> Any:
    """Load Grammar from JSON."""
    return rust.load_from_json(json, **kwargs)


# def load_to_json(json: str, **kwargs: Any) -> Any:


def load_tree(json: str, **kwargs: Any) -> Any:
    """Load tree from JSON."""
    return rust.load_tree(json, **kwargs)


# def load_tree_to_json(json: str, **kwargs: Any) -> Any:


def boot_grammar(**kwargs: Any) -> Any:
    """Returns the bootstrap grammar."""
    return rust.boot_grammar(**kwargs)


def boot_grammar_to_json(**kwargs: Any) -> Any:
    """Get bootstrap grammar as JSON (dict, list, etc.)."""
    return rust.boot_grammar_to_json(**kwargs)


def boot_grammar_to_json_string(**kwargs: Any) -> str:
    """Get bootstrap grammar as JSON string."""
    return rust.boot_grammar_to_json_string(**kwargs)


def boot_grammar_pretty(**kwargs: Any) -> str:
    """Get bootstrap grammar as pretty-printed string."""
    return rust.boot_grammar_pretty(**kwargs)


def load_boot(**kwargs: Any) -> Any:
    """Load bootstrap grammar."""
    return rust.load_boot(**kwargs)


def load_boot_as_json(**kwargs: Any) -> Any:
    """Load bootstrap grammar as JSON (dict, list, etc.)."""
    return rust.load_boot_as_json(**kwargs)


def grammar_pretty(grammar: str, **kwargs: Any) -> str:
    """Pretty print grammar."""
    return rust.grammar_pretty(grammar, **kwargs)


# def pretty_tree(tree: Any, **kwargs: Any) -> str:


# def pretty_tree_json(tree: Any, **kwargs: Any) -> str:


def parse(grammar: str, text: str, **kwargs: Any) -> Any:
    """Parse text with grammar, return AST tree."""
    return rust.parse(grammar, text, **kwargs)


def parse_to_json(grammar: str, text: str, **kwargs: Any) -> Any:
    """Parse text with grammar to JSON (dict, list, etc.)."""
    return rust.parse_to_json(grammar, text, **kwargs)


def parse_to_json_string(grammar: str, text: str, **kwargs: Any) -> str:
    """Parse text with grammar to JSON string."""
    return rust.parse_to_json_string(grammar, text, **kwargs)


def parse_input(parser: Any, text: str, **kwargs: Any) -> Any:
    """Parse text with compiled grammar, return AST tree."""
    return rust.parse_input(parser, text, **kwargs)


def parse_input_to_json(parser: Any, text: str, **kwargs: Any) -> Any:
    """Parse text with compiled grammar to JSON (dict, list, etc.)."""
    return rust.parse_input_to_json(parser, text, **kwargs)


def parse_input_to_json_string(parser: Any, text: str, **kwargs: Any) -> str:
    """Parse text with compiled grammar to JSON string."""
    return rust.parse_input_to_json_string(parser, text, **kwargs)


def pretty(grammar: str, **kwargs: Any) -> str:
    """Pretty print grammar."""
    return rust.pretty(grammar, **kwargs)