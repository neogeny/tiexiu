# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

import tiexiu.tiexiu as rust


def parse_grammar(grammar, **kwargs):
    return rust.parse_grammar(grammar, **kwargs)


def parse_grammar_to_json(grammar, **kwargs):
    return rust.parse_grammar_to_json(grammar, **kwargs)


def compile(grammar, **kwargs):
    return rust.compile_to_json(grammar, **kwargs)


def compile_to_json(grammar, **kwargs):
    return rust.compile_to_json(grammar, **kwargs)


def load(json_str, **kwargs):
    return rust.load(json_str, **kwargs)


def load_tree(json_str, **kwargs):
    return rust.load_tree(json_str, **kwargs)


def boot_grammar(**kwargs):
    return rust.boot_grammar(**kwargs)


def boot_grammar_to_json(**kwargs):
    return rust.boot_grammar_to_json(**kwargs)


def boot_grammar_as_json(**kwargs):
    return rust.boot_grammar_as_json(**kwargs)


def boot_grammar_pretty(**kwargs):
    return rust.boot_grammar(**kwargs)


def load_boot(**kwargs):
    return rust.load_boot(**kwargs)


def load_boot_as_json(**kwargs):
    return rust.load_boot_as_json(**kwargs)


def pretty(grammar, **kwargs):
    return rust.pretty(grammar, **kwargs)


def parse(grammar, text, **kwargs):
    return rust.parse(grammar, text, **kwargs)


def parse_to_json(grammar, text, **kwargs):
    return rust.parse_to_json(grammar, text, **kwargs)


def tiexiu(**kwargs):
    if kwargs:
        return rust.tiexiu(**kwargs)
    return rust.tiexiu()