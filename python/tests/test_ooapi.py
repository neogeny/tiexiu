# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

import tiexiu


def test_tiexiu_function():
    tx = tiexiu.tiexiu()
    assert tx is not None


def test_parse_grammar():
    tx = tiexiu.tiexiu()
    tree = tx.parse_grammar("start = /a/")
    assert tree is not None


def test_parse_grammar_to_json():
    tx = tiexiu.tiexiu()
    json_str = tx.parse_grammar_to_json("start = /a/")
    assert json_str is not None
    assert "start" in json_str


def test_compile():
    tx = tiexiu.tiexiu()
    json_str = tx.compile("start = /a/")
    assert json_str is not None
    assert "start" in json_str


def test_compile_to_json():
    tx = tiexiu.tiexiu()
    json_str = tx.compile_to_json("start = /a/")
    assert json_str is not None
    assert "start" in json_str


def test_boot_grammar():
    tx = tiexiu.tiexiu()
    json_str = tx.boot_grammar()
    assert json_str is not None
    assert "Rule" in json_str


def test_boot_grammar_to_json():
    tx = tiexiu.tiexiu()
    json_str = tx.boot_grammar_to_json()
    assert json_str is not None
    assert "Rule" in json_str


def test_boot_grammar_pretty():
    tx = tiexiu.tiexiu()
    result = tx.boot_grammar_pretty()
    assert result is not None
    assert "start" in result


def test_load_tree():
    tx = tiexiu.tiexiu()
    json_str = tx.parse_grammar_to_json("start = /a/")
    tree = tx.load_tree(json_str)
    assert tree is not None

def test_kwargs():
    tx = tiexiu.tiexiu()
    tree = tx.parse_grammar("start = /a/")
    assert tree is not None