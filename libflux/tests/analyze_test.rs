extern crate flux;

use flux::ast;
use flux::semantic::analyze_source;
use flux::semantic::nodes::*;
use flux::semantic::types::MonoType;

#[test]
fn analyze_end_to_end() {
    let got = analyze_source(
        r#"
n = 1
s = "string"
f = (a) => a + a
f(a: n)
f(a: s)
        "#,
    )
    .unwrap();
    let want = Package {
        loc: ast::BaseNode::default().location,
        package: "main".to_string(),
        files: vec![File {
            loc: ast::BaseNode::default().location,
            package: None,
            imports: Vec::new(),
            body: vec![
                Statement::Variable(VariableAssgn::new(
                    Identifier {
                        loc: ast::BaseNode::default().location,
                        name: "f".to_string(),
                    },
                    Expression::Function(Box::new(FunctionExpr {
                        loc: ast::BaseNode::default().location,
                        typ: MonoType::Int,
                        params: vec![
                            FunctionParameter {
                                loc: ast::BaseNode::default().location,
                                is_pipe: false,
                                key: Identifier {
                                    loc: ast::BaseNode::default().location,
                                    name: "a".to_string(),
                                },
                                default: None,
                            },
                            FunctionParameter {
                                loc: ast::BaseNode::default().location,
                                is_pipe: false,
                                key: Identifier {
                                    loc: ast::BaseNode::default().location,
                                    name: "b".to_string(),
                                },
                                default: None,
                            },
                        ],
                        body: Block::Return(Expression::Binary(Box::new(BinaryExpr {
                            loc: ast::BaseNode::default().location,
                            typ: MonoType::Int,
                            operator: ast::Operator::AdditionOperator,
                            left: Expression::Identifier(IdentifierExpr {
                                loc: ast::BaseNode::default().location,
                                typ: MonoType::Int,
                                name: "a".to_string(),
                            }),
                            right: Expression::Identifier(IdentifierExpr {
                                loc: ast::BaseNode::default().location,
                                typ: MonoType::Int,
                                name: "b".to_string(),
                            }),
                        }))),
                    })),
                    ast::BaseNode::default().location,
                )),
                Statement::Expr(ExprStmt {
                    loc: ast::BaseNode::default().location,
                    expression: Expression::Call(Box::new(CallExpr {
                        loc: ast::BaseNode::default().location,
                        typ: MonoType::Int,
                        pipe: None,
                        callee: Expression::Identifier(IdentifierExpr {
                            loc: ast::BaseNode::default().location,
                            typ: MonoType::Int,
                            name: "f".to_string(),
                        }),
                        arguments: vec![
                            Property {
                                loc: ast::BaseNode::default().location,
                                key: Identifier {
                                    loc: ast::BaseNode::default().location,
                                    name: "a".to_string(),
                                },
                                value: Expression::Integer(IntegerLit {
                                    loc: ast::BaseNode::default().location,
                                    typ: MonoType::Int,
                                    value: 2,
                                }),
                            },
                            Property {
                                loc: ast::BaseNode::default().location,
                                key: Identifier {
                                    loc: ast::BaseNode::default().location,
                                    name: "b".to_string(),
                                },
                                value: Expression::Integer(IntegerLit {
                                    loc: ast::BaseNode::default().location,
                                    typ: MonoType::Int,
                                    value: 3,
                                }),
                            },
                        ],
                    })),
                }),
            ],
        }],
    };
    assert_eq!(want, got);
}
