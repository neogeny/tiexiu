# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

"""Configuration options accepted by the tiexiu Python API.

Maps to crate::cfg::keys::CfgKey and constants in crate::cfg::constants.
"""
from enum import StrEnum


class CfgKey(StrEnum):
    """Valid keyword arguments accepted by tiexiu functions.

    Member names match the Python kwarg names (Rust CfgKey::map keys).
    String values match the constants in crate::cfg::constants.rs.

    Note: nameguard, left_recursion, parseinfo, and memoization map to
    Rust NoNameGuard/NoLeftRecursion/NoParseInfo/NoMemoization when
    their value is falsy.
    """

    Trace = "trace"
    Debug = "debug"
    Verbose = "verbose"
    Grammar = "grammar"
    Wsp = "whitespace"
    Cmt = "comments"
    Eol = "eol_comments"
    NameChars = "namechars"
    IgnoreCase = "ignorecase"
    NameGuard = "nameguard"
    LeftRecursion = "left_recursion"
    ParseInfo = "parseinfo"
    Memoization = "memoization"
