# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: Apache-2.0

"""Ported from tatsu/tests/grammar/syntax_test.py"""

import pytest

import tiexiu


def test_group_ast():
    grammar = """
        start = '1' ('2' '3') '4' $ ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('1 2 3 4', nameguard=False)
    assert ast == ['1', '2', '3', '4']


def test_update_ast():
    grammar = """
        foo = name:"1" [ name: bar ] ;
        bar = { "2" } * ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('1 2')
    assert ast.name == ['1', ['2']]

    grammar = """
        start = items: { item } * $ ;
        item = @:{ subitem } * "0" ;
        subitem = /1+/ ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('1101110100')
    assert ast.items == [['11'], ['111'], ['1'], []]


def test_optional_closure():
    grammar = 'start = foo+:"x" foo:{"y"}* {foo:"z"}* ;'
    model = tiexiu.compile(grammar)
    ast = model.parse_input('xyyzz')
    assert ast.foo == ['x', ['y', 'y'], 'z', 'z']

    grammar = 'start = foo+:"x" [foo+:{"y"}*] {foo:"z"}* ;'
    model = tiexiu.compile(grammar)
    ast = model.parse_input('xyyzz')
    assert ast.foo == ['x', ['y', 'y'], 'z', 'z']

    grammar = 'start = foo+:"x" foo:[{"y"}*] {foo:"z"}* ;'
    model = tiexiu.compile(grammar)
    ast = model.parse_input('xyyzz')
    assert ast.foo == ['x', ['y', 'y'], 'z', 'z']

    grammar = 'start = foo+:"x" [foo:{"y"}*] {foo:"z"}* ;'
    model = tiexiu.compile(grammar)
    ast = model.parse_input('xyyzz')
    assert ast.foo == ['x', ['y', 'y'], 'z', 'z']


def test_optional_sequence():
    grammar = """
        start = '1' ['2' '3'] '4' $ ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('1234')
    assert ast == ['1', '2', '3', '4']

    grammar = """
        start = '1' foo:['2' '3'] '4' $ ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('1234')
    assert ast.foo == ['2', '3']


def test_partial_options():
    grammar = """
        start
            =
            [a]
            [
                'A' 'A'
            |
                'A' 'B'
            ]
            $
            ;
        a
            =
            'A' !('A'|'B')
            ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('AB')
    assert ast == ['A', 'B']


def test_partial_choice():
    grammar = """
        start
            =
            o:[o]
            x:'A'
            $
            ;
        o
            =
            'A' a:'A'
            |
            'A' b:'B'
            ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('A')
    assert ast == {'x': 'A', 'o': None}


def test_new_override():
    grammar = """
        start
            =
            @:'a' {@:'b'}
            $
            ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('abb')
    assert ast == ['a', 'b', 'b']


def test_list_override():
    grammar = """
        start
            =
            @+:'a' {@:'b'}
            $
            ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('a')
    assert ast == ['a']

    grammar = """
        start
            =
            @:'a' {@:'b'}
            $
            ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('a')
    assert ast == 'a'


def test_based_rule():
    grammar = """\
        start: b $

        a: ='a'

        b < a: {='b'}
        """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('abb')
    assert ast == ['a', 'b', 'b']


def test_rule_include():
    grammar = """
        start = b $;

        a = @:'a' ;
        b = >a {@:'b'} ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('abb')
    assert ast == ['a', 'b', 'b']


def test_48_rule_override():
    grammar = """
        start = ab $;

        ab = 'xyz' ;

        @override
        ab = @:'a' {@:'b'} ;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('abb')
    assert ast == ['a', 'b', 'b']


def test_empty_closure():
    grammar = """
        start = {'x'}+ {} 'y'$;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('xxxy')
    assert ast == [['x', 'x', 'x'], [], 'y']


def test_any():
    grammar = """
        start = /./ 'xx' /./ /./ 'yy' $;
    """
    model = tiexiu.compile(grammar)
    ast = model.parse_input('1xx 2 yy')
    assert ast == ['1', 'xx', ' ', '2', 'yy']


def test_constant():
    grammar = """
        start = ()
            _0:`0` _1:`+1` _n123:`-123`
            _xF:`0xF`
            _string:`string`
            _string_space:`'string space'`
            _true:`True` _false:`False`
            $;
    """

    model = tiexiu.compile(grammar)
    ast = model.parse_input('')

    assert ast._0 == 0
    assert ast._1 == 1
    assert ast._n123 == -123
    assert ast._xF == 0xF
    assert ast._string == 'string'
    assert ast._string_space == 'string space'
    assert ast._true is True
    assert ast._false is False


def test_non_capturing_group_exclusion():
    grammar = """
        start: header (?: delimiter ) body

        header: /[A-Z]+/

        delimiter: /[:,-]+/

        body: /[a-z]+/
    """
    parser = tiexiu.compile(grammar)

    input_str = "INFO---data"
    ast = parser.parse_input(input_str)

    assert ast[0] == "INFO"
    assert ast[1] == "data"
    assert len(ast) == 2
    assert "---" not in ast


def test_non_capturing_group_failure():
    grammar = r"start = (?: 'FIX' ) value ; value = /\d+/"
    parser = tiexiu.compile(grammar)

    with pytest.raises(ValueError):
        parser.parse_input("BUG123")


def test_nested_non_capturing_groups():
    grammar = """
    start: (?: '(' (?: inner ) ')' )

    inner: 'content'
    """
    parser = tiexiu.compile(grammar)

    ast = parser.parse_input("(content)")

    assert ast is None


def test_ast_assignment():
    grammar = """
        n  = @: {"a"}* $ ;
        f  = @+: {"a"}* $ ;
        nn = @: {"a"}*  @: {"b"}* $ ;
        nf = @: {"a"}*  @+: {"b"}* $ ;
        fn = @+: {"a"}* @: {"b"}* $ ;
        ff = @+: {"a"}* @+: {"b"}* $ ;
    """

    model = tiexiu.compile(grammar)

    def parse(input, rule):
        return model.parse_input(input, start=rule)

    assert parse('', 'n') == []
    assert parse('a', 'n') == ['a']
    assert parse('aa', 'n') == ['a', 'a']

    assert parse('', 'f') == [[]]
    assert parse('a', 'f') == [['a']]
    assert parse('aa', 'f') == [['a', 'a']]

    for r in ('nn', 'nf', 'fn', 'ff'):
        assert parse('', r) == [[], []]
        assert parse('a', r) == [['a'], []]
        assert parse('b', r) == [[], ['b']]
        assert parse('aa', r) == [['a', 'a'], []]
        assert parse('bb', r) == [[], ['b', 'b']]
        assert parse('aab', r) == [['a', 'a'], ['b']]


def test_parse_hash():
    grammar = r"""
        @@comments :: /@@@@@@@/
        @@eol_comments :: /@@@@@@@/

        start = '#' ;
    """

    parser = tiexiu.compile(grammar, eol_comments='')
    parser.parse_input('#')


def test_parse_void():
    grammar = r"""
        start = () $ ;
    """

    ast = tiexiu.parse(grammar, '')
    assert ast is None


def test_no_default_comments():
    grammar = """
        start = 'a' $;
    """

    text = """
        # no comments are valid
        a
    """
    with pytest.raises(ValueError):
        tiexiu.parse(grammar, text)


def test_deprecated_comments_override_failures():
    grammar = """
        @@comments :: /@@@@@@/
        @@eol_comments :: /@@@@@@/

        start = 'a' $;
    """

    text = """
        # This comment should be stripped
        a
    """
    result = tiexiu.parse(grammar, text, eol_comments=r"(?m)#[^\n]*$")
    assert result is not None
