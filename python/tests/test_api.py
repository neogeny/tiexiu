# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

import tiexiu

def test_version():
    assert isinstance(tiexiu.__version__, str)
    assert tiexiu.__version__ >= "0.1.1"

def test_parse_grammar():
    tree = tiexiu.parse_grammar("start = /a/")
    assert tree is not None


def test_parse_grammar_to_json():
    result = tiexiu.parse_grammar_to_json("start = /a/")
    assert result is not None
    assert "start" in result["rules"][0]["name"]


def test_compile_to_json():
    result = tiexiu.compile_to_json("start = /a/")
    assert result is not None
    assert "start" in result["rules"][0]["name"]


def test_pretty():
    result = tiexiu.pretty("start = /a/")
    assert result is not None
    assert "start" in result


def test_load_boot_as_json():
    result = tiexiu.load_boot_as_json()
    assert result is not None
    assert "Rule" in str(result)


def test_boot_grammar_to_json():
    result = tiexiu.boot_grammar_to_json()
    assert result is not None
    assert "Rule" in str(result)


def test_parse():
    tree = tiexiu.parse("start = /a/", "a")
    assert tree is not None


def test_parse_to_json():
    result = tiexiu.parse_to_json("start = /a/", "a")
    assert result is not None


def test_kwargs_int():
    tree = tiexiu.parse_grammar("start = /a/", count=42)
    assert tree is not None


def test_kwargs_float():
    tree = tiexiu.parse_grammar("start = /a/", precision=3.14)
    assert tree is not None


def test_grammar_parse_input():
    g = tiexiu.boot_grammar()
    tree = g.parse_input("start = /hello/")
    assert tree is not None
    assert "Rule" in str(tree)