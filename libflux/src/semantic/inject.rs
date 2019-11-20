use crate::semantic::nodes::*;
use crate::semantic::sub::Substitution;
use crate::semantic::walk::{walk_mut, NodeMut, VisitorMut};

pub fn inject_types(mut pkg: &mut Package, sub: Substitution) {
    let mut v = TypeInjector { sub };
    walk_mut(&mut v, &mut NodeMut::Package(&mut pkg));
}

struct TypeInjector {
    sub: Substitution,
}

impl VisitorMut for TypeInjector {
    fn visit(&mut self, node: &mut NodeMut) -> bool {
        node.replace_type(&self.sub);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::semantic::types::{MonoType, Tvar};
    use crate::semantic::walk::{walk, Node};
    use maplit::hashmap;
    use std::rc::Rc;

    #[test]
    fn test_inject_types() {
        let b = ast::BaseNode::default();
        let mut pkg = Package {
            loc: b.location.clone(),
            package: "main".to_string(),
            files: vec![File {
                loc: b.location.clone(),
                package: None,
                imports: Vec::new(),
                body: vec![
                    Statement::Variable(VariableAssgn::new(
                        Identifier {
                            loc: b.location.clone(),
                            name: "f".to_string(),
                        },
                        Expression::Function(Box::new(FunctionExpr {
                            loc: b.location.clone(),
                            typ: MonoType::Var(Tvar(0)),
                            params: vec![
                                FunctionParameter {
                                    loc: b.location.clone(),
                                    is_pipe: true,
                                    key: Identifier {
                                        loc: b.location.clone(),
                                        name: "piped".to_string(),
                                    },
                                    default: None,
                                },
                                FunctionParameter {
                                    loc: b.location.clone(),
                                    is_pipe: false,
                                    key: Identifier {
                                        loc: b.location.clone(),
                                        name: "a".to_string(),
                                    },
                                    default: None,
                                },
                            ],
                            body: Block::Return(Expression::Binary(Box::new(BinaryExpr {
                                loc: b.location.clone(),
                                typ: MonoType::Var(Tvar(1)),
                                operator: ast::Operator::AdditionOperator,
                                left: Expression::Identifier(IdentifierExpr {
                                    loc: b.location.clone(),
                                    typ: MonoType::Var(Tvar(2)),
                                    name: "a".to_string(),
                                }),
                                right: Expression::Identifier(IdentifierExpr {
                                    loc: b.location.clone(),
                                    typ: MonoType::Var(Tvar(3)),
                                    name: "piped".to_string(),
                                }),
                            }))),
                        })),
                        b.location.clone(),
                    )),
                    Statement::Expr(ExprStmt {
                        loc: b.location.clone(),
                        expression: Expression::Call(Box::new(CallExpr {
                            loc: b.location.clone(),
                            typ: MonoType::Var(Tvar(4)),
                            pipe: Some(Expression::Integer(IntegerLit {
                                loc: b.location.clone(),
                                typ: MonoType::Var(Tvar(5)),
                                value: 3,
                            })),
                            callee: Expression::Identifier(IdentifierExpr {
                                loc: b.location.clone(),
                                typ: MonoType::Var(Tvar(6)),
                                name: "f".to_string(),
                            }),
                            arguments: vec![Property {
                                loc: b.location.clone(),
                                key: Identifier {
                                    loc: b.location.clone(),
                                    name: "a".to_string(),
                                },
                                value: Expression::Integer(IntegerLit {
                                    loc: b.location.clone(),
                                    typ: MonoType::Var(Tvar(7)),
                                    value: 2,
                                }),
                            }],
                        })),
                    }),
                ],
            }],
        };
        let sub: Substitution = hashmap! {
            Tvar(0) => MonoType::Int,
            Tvar(1) => MonoType::Int,
            Tvar(2) => MonoType::Int,
            Tvar(3) => MonoType::Int,
            Tvar(4) => MonoType::Int,
            Tvar(5) => MonoType::Int,
            Tvar(6) => MonoType::Int,
            Tvar(7) => MonoType::Int,
        }
        .into();
        inject_types(&mut pkg, sub);
        let mut no_types_checked = 0;
        walk(
            &mut |node: Rc<Node>| {
                let typ = node.type_of();
                if let Some(typ) = typ {
                    assert_eq!(typ, &MonoType::Int);
                    no_types_checked += 1;
                }
            },
            Rc::new(Node::Package(&pkg)),
        );
        assert_eq!(no_types_checked, 8);
    }
}
