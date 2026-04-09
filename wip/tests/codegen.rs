// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Tests translated from TatSu's codegen_test.py
//!
//! These are skeletal tests - they will not compile until TieXiu
//! implements the full EBNF parsing bootstrap.

use crate::codegen::{CodeGenerator, ModelRenderer};

struct TestGenerator {
    rendered: String,
}

impl CodeGenerator for TestGenerator {
    fn render(&mut self, model: &dyn crate::model::Renderable) -> Result<String, String> {
        Ok(model.render())
    }
}

#[test]
fn test_basic_codegen() {
    struct Super;
    struct Sub;

    impl crate::model::Renderable for Super {
        fn render(&self) -> String {
            format!("OK {}", Sub.render())
        }
    }

    impl crate::model::Renderable for Sub {
        fn render(&self) -> String {
            "and OK too".to_string()
        }
    }

    let super_node = Super;
    let mut gen = TestGenerator {
        rendered: String::new(),
    };
    let result = gen.render(&super_node);
    assert_eq!(result.unwrap(), "OK and OK too");
}
