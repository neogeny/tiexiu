// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests for parameters

use tiexiu::api::compile;
use tiexiu::util::indent::dedent_all;
use tiexiu::*;

#[test]
fn test_numbers_and_unicode() -> Result<()> {
    let grammar = dedent_all(
        r#"
        rúle[1, -23, 4.56, 7.89e-11, Añez]: "a"

        rúlé[Añez]: "ñ"

        "#,
    );

    let _tree = parse_grammar(grammar.as_ref(), &[])?;
    // eprintln!("{:#?}", tree);
    // eprintln!("{}", tree.to_json_string_pretty());

    let model = compile(grammar.as_ref(), &[])?;
    // eprintln!("{:#?}", model);

    let pretty = model.pretty_print();
    assert_eq!(grammar.trim(), pretty.trim());
    Ok(())
}
