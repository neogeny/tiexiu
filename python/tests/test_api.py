# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

import tiexiu


def test_parse_grammar():
    tree = tiexiu.parse_grammar("start = /a/")
    assert tree is not None


def test_parse_grammar_to_json():
    json_str = tiexiu.parse_grammar_to_json("start = /a/")
    assert json_str is not None
    assert "start" in json_str


def test_compile_to_json():
    json_str = tiexiu.compile_to_json("start = /a/")
    assert json_str is not None
    assert "start" in json_str


def test_pretty():
    result = tiexiu.pretty("start = /a/")
    assert result is not None
    assert "start" in result


def test_load_boot_as_json():
    json_str = tiexiu.load_boot_as_json()
    assert json_str is not None
    assert "Rule" in json_str


def test_boot_grammar_as_json():
    json_str = tiexiu.boot_grammar_as_json()
    assert json_str is not None
    assert "Rule" in json_str


def test_parse():
    tree = tiexiu.parse("start = /a/", "a")
    assert tree is not None


def test_parse_to_json():
    json_str = tiexiu.parse_to_json("start = /a/", "a")
    assert json_str is not None
    assert json_str == '"a"'


def test_kwargs_int():
    tree = tiexiu.parse_grammar("start = /a/", count=42)
    assert tree is not None


def test_kwargs_float():
    tree = tiexiu.parse_grammar("start = /a/", precision=3.14)
    assert tree is not None


def test_kwargs_list():
    tree = tiexiu.parse_grammar("start = /a/", values=[1, 2, 3])
    assert tree is not None


def test_kwargs_dict():
    tree = tiexiu.parse_grammar("start = /a/", options={"key": "value"})
    assert tree is not None
