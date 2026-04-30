# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Any


def parse_grammar(grammar: str, **kwargs: Any) -> Any:
    """Parse grammar string to AST tree."""
    import tiexiu.tiexiu as rust
    return rust.parse_grammar(grammar, **kwargs)


def parse_grammar_to_json(grammar: str, **kwargs: Any) -> Any:
    """Parse grammar to JSON (dict, list, etc.)."""
    import tiexiu.tiexiu as rust
    return rust.parse_grammar_to_json(grammar, **kwargs)


def parse_grammar_to_json_string(grammar: str, **kwargs: Any) -> str:
    """Parse grammar to JSON string."""
    import tiexiu.tiexiu as rust
    return rust.parse_grammar_to_json_string(grammar, **kwargs)


# def parse_grammar_with(cursor: Any, **kwargs: Any) -> Any:
#     """NOT IMPLEMENTED - requires generic cursor support."""
#     raise NotImplementedError("parse_grammar_with requires generic cursor support")


# def parse_grammar_to_json_with(cursor: Any, **kwargs: Any) -> Any:
#     """NOT IMPLEMENTED - requires generic cursor support."""
#     raise NotImplementedError("parse_grammar_to_json_with requires generic cursor support")


def compile(grammar: str, **kwargs: Any) -> Any:
    """Compile grammar, return Grammar object."""
    import tiexiu.tiexiu as rust
    return rust.compile(grammar, **kwargs)


def compile_to_json(grammar: str, **kwargs: Any) -> Any:
    """Compile grammar to JSON (dict, list, etc.)."""
    import tiexiu.tiexiu as rust
    return rust.compile_to_json(grammar, **kwargs)


def compile_to_json_string(grammar: str, **kwargs: Any) -> str:
    """Compile grammar to JSON string."""
    import tiexiu.tiexiu as rust
    return rust.compile_to_json_string(grammar, **kwargs)


# def compile_with(cursor: Any, **kwargs: Any) -> Any:
#     """NOT IMPLEMENTED - requires generic cursor support."""
#     raise NotImplementedError("compile_with requires generic cursor support")


# def compile_to_json_with(cursor: Any, **kwargs: Any) -> Any:
#     """NOT IMPLEMENTED - requires generic cursor support."""
#     raise NotImplementedError("compile_to_json_with requires generic cursor support")


def load(json: str, **kwargs: Any) -> Any:
    """NOT IMPLEMENTED - returns Grammar."""
    raise NotImplementedError("load returns Grammar, not yet implemented")


# def load_to_json(json: str, **kwargs: Any) -> Any:
#     """NOT IMPLEMENTED - returns Value."""
#     raise NotImplementedError("load_to_json returns Value, not yet implemented")


def load_tree(json: str, **kwargs: Any) -> Any:
    """NOT IMPLEMENTED - returns Tree."""
    raise NotImplementedError("load_tree returns Tree, not yet implemented")


# def load_tree_to_json(json: str, **kwargs: Any) -> Any:
#     """NOT IMPLEMENTED - returns Value."""
#     raise NotImplementedError("load_tree_to_json returns Value, not yet implemented")


def boot_grammar(**kwargs: Any) -> Any:
    """Returns the bootstrap grammar."""
    import tiexiu.tiexiu as rust
    return rust.boot_grammar(**kwargs)


def boot_grammar_to_json(**kwargs: Any) -> Any:
    """Get bootstrap grammar as JSON (dict, list, etc.)."""
    import tiexiu.tiexiu as rust
    return rust.boot_grammar_to_json(**kwargs)


def boot_grammar_to_json_string(**kwargs: Any) -> str:
    """Get bootstrap grammar as JSON string."""
    import tiexiu.tiexiu as rust
    return rust.boot_grammar_to_json_string(**kwargs)


def boot_grammar_pretty(**kwargs: Any) -> str:
    """Get bootstrap grammar as pretty-printed string."""
    import tiexiu.tiexiu as rust
    return rust.boot_grammar_pretty(**kwargs)


def load_boot(**kwargs: Any) -> Any:
    """NOT IMPLEMENTED - returns Grammar."""
    raise NotImplementedError("load_boot returns Grammar, not yet implemented")


def load_boot_as_json(**kwargs: Any) -> Any:
    """Load bootstrap grammar as JSON (dict, list, etc.)."""
    import tiexiu.tiexiu as rust
    return rust.load_boot_as_json(**kwargs)


def grammar_pretty(grammar: str, **kwargs: Any) -> str:
    """Pretty print grammar."""
    import tiexiu.tiexiu as rust
    return rust.grammar_pretty(grammar, **kwargs)


# def pretty_tree(tree: Any, **kwargs: Any) -> str:
#     """NOT IMPLEMENTED - requires Tree object."""
#     raise NotImplementedError("pretty_tree requires Tree object, not yet implemented")


# def pretty_tree_json(tree: Any, **kwargs: Any) -> str:
#     """NOT IMPLEMENTED - requires Tree object."""
#     raise NotImplementedError("pretty_tree_json requires Tree object, not yet implemented")


def parse(grammar: str, text: str, **kwargs: Any) -> Any:
    """Parse text with grammar, return AST tree."""
    import tiexiu.tiexiu as rust
    return rust.parse(grammar, text, **kwargs)


def parse_to_json(grammar: str, text: str, **kwargs: Any) -> Any:
    """Parse text with grammar to JSON (dict, list, etc.)."""
    import tiexiu.tiexiu as rust
    return rust.parse_to_json(grammar, text, **kwargs)


def parse_to_json_string(grammar: str, text: str, **kwargs: Any) -> str:
    """Parse text with grammar to JSON string."""
    import tiexiu.tiexiu as rust
    return rust.parse_to_json_string(grammar, text, **kwargs)


def parse_input(parser: Any, text: str, **kwargs: Any) -> Any:
    """Parse text with compiled grammar, return AST tree."""
    import tiexiu.tiexiu as rust
    return rust.parse_input(parser, text, **kwargs)


def parse_input_to_json(parser: Any, text: str, **kwargs: Any) -> Any:
    """Parse text with compiled grammar to JSON (dict, list, etc.)."""
    import tiexiu.tiexiu as rust
    return rust.parse_input_to_json(parser, text, **kwargs)


def parse_input_to_json_string(parser: Any, text: str, **kwargs: Any) -> str:
    """Parse text with compiled grammar to JSON string."""
    import tiexiu.tiexiu as rust
    return rust.parse_input_to_json_string(parser, text, **kwargs)


def pretty(grammar: str, **kwargs: Any) -> str:
    """Pretty print grammar."""
    import tiexiu.tiexiu as rust
    return rust.pretty(grammar, **kwargs)


def pegapi(**kwargs: Any) -> Any:
    """Create TieXiu OO API instance."""
    import tiexiu.tiexiu as rust
    if kwargs:
        return rust.pegapi(**kwargs)
    return rust.pegapi()