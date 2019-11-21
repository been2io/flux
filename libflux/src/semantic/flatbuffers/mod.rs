//! This module defines a set of types used for serializing and deserializing
//! the flatbuffer representation of flux datatypes. There is a one-to-one
//! mapping between the types defined in this module and the types defined in
//! the `semantic` module. As such, the `From` trait is implemented for each
//! of these types.
//!
//! # Example
//!
//! ```
//! use flux::semantic::types as semantic;
//!
//! pub struct Var {
//!     pub i: u64,
//! }
//!
//! impl From<semantic::Tvar> for Var {
//!     fn from(t: semantic::Tvar) -> Var {
//!         Var { i: t.0 }
//!     }
//! }
//!
//! impl From<Var> for semantic::Tvar {
//!     fn from(t: Var) -> semantic::Tvar {
//!         semantic::Tvar(t.i)
//!     }
//! }
//! ```
//!
//! The flux datatypes as defined in the `semantic` module should never be
//! directly encoded or decoded from flatbuffers. Instead serialization and
//! deserialization should always involve translating to the intermediate
//! representation defined here. Note that the types of the builtin
//! identifiers in the flux standard library are declared using these types.
//! Therefore any changes to the `semantic` module will not require any
//! updates to the standard library - just the methods defined here.
//!
mod semantic_generated;
use semantic_generated::fbsemantic as fb;

use crate::semantic::types as semantic;
use flatbuffers;
use std::collections::HashMap;

trait Compile<T>
where
    Self: std::marker::Sized,
{
    fn compile(r: T) -> Result<Self, String>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct PolyType {
    pub vars: Vec<Var>,
    pub cons: Vec<Constraint>,
    pub expr: MonoType,
}

impl From<semantic::PolyType> for PolyType {
    fn from(t: semantic::PolyType) -> PolyType {
        let mut vars = Vec::new();
        for tv in t.vars {
            vars.push(tv.into());
        }
        let mut cons = Vec::new();
        for (tv, kinds) in t.cons {
            for kind in kinds {
                cons.push(Constraint {
                    tvar: tv.into(),
                    kind: kind.into(),
                })
            }
        }
        PolyType {
            vars,
            cons,
            expr: t.expr.into(),
        }
    }
}

impl From<PolyType> for semantic::PolyType {
    fn from(t: PolyType) -> semantic::PolyType {
        let mut vars = Vec::new();
        for tv in t.vars {
            vars.push(tv.into());
        }
        let mut cons = HashMap::new();
        for constraint in t.cons {
            cons.entry(constraint.tvar.into())
                .or_insert(Vec::new())
                .push(constraint.kind.into());
        }
        semantic::PolyType {
            vars,
            cons,
            expr: t.expr.into(),
        }
    }
}

// Decode the flatbuffer representation of a polytype
impl Compile<fb::PolyType<'_>> for PolyType {
    fn compile(t: fb::PolyType) -> Result<PolyType, String> {
        let fb_vars = t.vars().unwrap();
        let l = fb_vars.len();
        let mut vars = Vec::with_capacity(l);
        for i in 0..l {
            vars.push(Var::from(fb_vars.get(i)));
        }
        let fb_cons = t.cons().unwrap();
        let l = fb_cons.len();
        let mut cons = Vec::with_capacity(l);
        for i in 0..l {
            cons.push(Constraint::from(fb_cons.get(i)));
        }
        Ok(PolyType {
            vars,
            cons,
            expr: MonoType::from_table(t.expr().unwrap(), t.expr_type())?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    pub tvar: Var,
    pub kind: Kind,
}

impl From<fb::Constraint<'_>> for Constraint {
    fn from(constraint: fb::Constraint) -> Constraint {
        Constraint {
            tvar: constraint.tvar().unwrap().into(),
            kind: constraint.kind().into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Addable,
    Subtractable,
    Divisible,
    Comparable,
    Equatable,
    Nullable,
}

impl From<semantic::Kind> for Kind {
    fn from(kind: semantic::Kind) -> Kind {
        match kind {
            semantic::Kind::Addable => Kind::Addable,
            semantic::Kind::Subtractable => Kind::Subtractable,
            semantic::Kind::Divisible => Kind::Divisible,
            semantic::Kind::Comparable => Kind::Comparable,
            semantic::Kind::Equatable => Kind::Equatable,
            semantic::Kind::Nullable => Kind::Nullable,
        }
    }
}

impl From<Kind> for semantic::Kind {
    fn from(kind: Kind) -> semantic::Kind {
        match kind {
            Kind::Addable => semantic::Kind::Addable,
            Kind::Subtractable => semantic::Kind::Subtractable,
            Kind::Divisible => semantic::Kind::Divisible,
            Kind::Comparable => semantic::Kind::Comparable,
            Kind::Equatable => semantic::Kind::Equatable,
            Kind::Nullable => semantic::Kind::Nullable,
        }
    }
}

impl From<fb::Kind> for Kind {
    fn from(kind: fb::Kind) -> Kind {
        match kind {
            fb::Kind::Addable => Kind::Addable,
            fb::Kind::Subtractable => Kind::Subtractable,
            fb::Kind::Divisible => Kind::Divisible,
            fb::Kind::Comparable => Kind::Comparable,
            fb::Kind::Equatable => Kind::Equatable,
            fb::Kind::Nullable => Kind::Nullable,
        }
    }
}

impl From<Kind> for fb::Kind {
    fn from(kind: Kind) -> fb::Kind {
        match kind {
            Kind::Addable => fb::Kind::Addable,
            Kind::Subtractable => fb::Kind::Subtractable,
            Kind::Divisible => fb::Kind::Divisible,
            Kind::Comparable => fb::Kind::Comparable,
            Kind::Equatable => fb::Kind::Equatable,
            Kind::Nullable => fb::Kind::Nullable,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MonoType {
    Bool,
    Int,
    Uint,
    Float,
    String,
    Duration,
    Time,
    Regexp,
    Var(Var),
    Arr(Box<Arr>),
    Row(Box<Row>),
    Fun(Box<Fun>),
}

impl From<semantic::MonoType> for MonoType {
    fn from(t: semantic::MonoType) -> MonoType {
        match t {
            semantic::MonoType::Bool => MonoType::Bool,
            semantic::MonoType::Int => MonoType::Int,
            semantic::MonoType::Uint => MonoType::Uint,
            semantic::MonoType::Float => MonoType::Float,
            semantic::MonoType::String => MonoType::String,
            semantic::MonoType::Duration => MonoType::Duration,
            semantic::MonoType::Time => MonoType::Time,
            semantic::MonoType::Regexp => MonoType::Regexp,
            semantic::MonoType::Var(tvr) => tvr.into(),
            semantic::MonoType::Arr(arr) => (*arr).into(),
            semantic::MonoType::Row(row) => (*row).into(),
            semantic::MonoType::Fun(fun) => (*fun).into(),
        }
    }
}

impl From<MonoType> for semantic::MonoType {
    fn from(t: MonoType) -> semantic::MonoType {
        match t {
            MonoType::Bool => semantic::MonoType::Bool,
            MonoType::Int => semantic::MonoType::Int,
            MonoType::Uint => semantic::MonoType::Uint,
            MonoType::Float => semantic::MonoType::Float,
            MonoType::String => semantic::MonoType::String,
            MonoType::Duration => semantic::MonoType::Duration,
            MonoType::Time => semantic::MonoType::Time,
            MonoType::Regexp => semantic::MonoType::Regexp,
            MonoType::Var(tvr) => tvr.into(),
            MonoType::Arr(arr) => (*arr).into(),
            MonoType::Row(row) => (*row).into(),
            MonoType::Fun(fun) => (*fun).into(),
        }
    }
}

impl From<fb::Basic<'_>> for MonoType {
    fn from(t: fb::Basic) -> MonoType {
        match t.t() {
            fb::Type::Bool => MonoType::Bool,
            fb::Type::Int => MonoType::Int,
            fb::Type::Uint => MonoType::Uint,
            fb::Type::Float => MonoType::Float,
            fb::Type::String => MonoType::String,
            fb::Type::Duration => MonoType::Duration,
            fb::Type::Time => MonoType::Time,
            fb::Type::Regexp => MonoType::Regexp,
        }
    }
}

impl MonoType {
    fn from_table(table: flatbuffers::Table, t: fb::MonoType) -> Result<MonoType, String> {
        match t {
            fb::MonoType::Var => Ok(MonoType::Var(Var::from(fb::Var::init_from_table(table)))),
            fb::MonoType::Arr => Ok(MonoType::Arr(Box::new(Arr::compile(
                fb::Arr::init_from_table(table),
            )?))),
            fb::MonoType::Row => Ok(MonoType::Row(Box::new(Row::compile(
                fb::Row::init_from_table(table),
            )?))),
            fb::MonoType::Fun => Ok(MonoType::Fun(Box::new(Fun::compile(
                fb::Fun::init_from_table(table),
            )?))),
            fb::MonoType::Basic => Ok(MonoType::from(fb::Basic::init_from_table(table))),
            fb::MonoType::NONE => Err(String::from("invalid type NONE")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub i: u64,
}

impl From<semantic::Tvar> for Var {
    fn from(t: semantic::Tvar) -> Var {
        Var { i: t.0 }
    }
}

impl From<semantic::Tvar> for MonoType {
    fn from(t: semantic::Tvar) -> MonoType {
        MonoType::Var(t.into())
    }
}

impl From<Var> for semantic::Tvar {
    fn from(t: Var) -> semantic::Tvar {
        semantic::Tvar(t.i)
    }
}

impl From<Var> for semantic::MonoType {
    fn from(t: Var) -> semantic::MonoType {
        semantic::MonoType::Var(t.into())
    }
}

impl From<fb::Var<'_>> for Var {
    fn from(t: fb::Var) -> Var {
        Var { i: t.i() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arr {
    pub t: MonoType,
}

impl From<semantic::Array> for Arr {
    fn from(t: semantic::Array) -> Arr {
        Arr { t: t.0.into() }
    }
}

impl From<semantic::Array> for MonoType {
    fn from(t: semantic::Array) -> MonoType {
        MonoType::Arr(Box::new(t.into()))
    }
}

impl From<Arr> for semantic::Array {
    fn from(t: Arr) -> semantic::Array {
        semantic::Array(t.t.into())
    }
}

impl From<Arr> for semantic::MonoType {
    fn from(t: Arr) -> semantic::MonoType {
        semantic::MonoType::Arr(Box::new(t.into()))
    }
}

impl Compile<fb::Arr<'_>> for Arr {
    fn compile(t: fb::Arr) -> Result<Arr, String> {
        Ok(Arr {
            t: MonoType::from_table(t.t().unwrap(), t.t_type())?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    pub props: Vec<Property>,
    pub extends: Option<Var>,
}

impl From<semantic::Row> for Row {
    fn from(mut t: semantic::Row) -> Row {
        let mut props = Vec::new();
        loop {
            match t {
                semantic::Row::Empty => {
                    return Row {
                        props,
                        extends: None,
                    };
                }
                semantic::Row::Extension {
                    head,
                    tail: semantic::MonoType::Row(o),
                } => {
                    props.push(head.into());
                    t = *o;
                }
                semantic::Row::Extension {
                    head,
                    tail: semantic::MonoType::Var(t),
                } => {
                    props.push(head.into());
                    return Row {
                        props,
                        extends: Some(t.into()),
                    };
                }
                semantic::Row::Extension { head, tail } => {
                    return Row {
                        props,
                        extends: None,
                    };
                }
            }
        }
    }
}

impl From<semantic::Row> for MonoType {
    fn from(t: semantic::Row) -> MonoType {
        MonoType::Row(Box::new(t.into()))
    }
}

impl From<Row> for semantic::MonoType {
    fn from(t: Row) -> semantic::MonoType {
        let extends = match t.extends {
            None => semantic::MonoType::Row(Box::new(semantic::Row::Empty)),
            Some(tv) => semantic::MonoType::from(tv),
        };
        t.props.into_iter().rev().fold(extends, |r, prop| {
            semantic::MonoType::Row(Box::new(semantic::Row::Extension {
                head: prop.into(),
                tail: r,
            }))
        })
    }
}

impl Compile<fb::Row<'_>> for Row {
    fn compile(t: fb::Row) -> Result<Row, String> {
        let fb_props = t.props().unwrap();
        let l = fb_props.len();
        let mut props = Vec::with_capacity(l);
        for i in 0..l {
            props.push(Property::compile(fb_props.get(i))?);
        }
        let extends = match t.extends() {
            None => None,
            Some(tv) => Some(Var::from(tv)),
        };
        Ok(Row { props, extends })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub k: String,
    pub v: MonoType,
}

impl From<semantic::Property> for Property {
    fn from(p: semantic::Property) -> Property {
        Property {
            k: p.k,
            v: p.v.into(),
        }
    }
}

impl From<Property> for semantic::Property {
    fn from(p: Property) -> semantic::Property {
        semantic::Property {
            k: p.k,
            v: p.v.into(),
        }
    }
}

impl Compile<fb::Prop<'_>> for Property {
    fn compile(t: fb::Prop) -> Result<Property, String> {
        Ok(Property {
            k: t.k().unwrap().to_owned(),
            v: MonoType::from_table(t.v().unwrap(), t.v_type())?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fun {
    pub args: Vec<Argument>,
    pub retn: MonoType,
}

impl From<semantic::Function> for Fun {
    fn from(t: semantic::Function) -> Fun {
        let mut args = Vec::new();
        if let Some(pipe) = t.pipe {
            args.push(Argument {
                name: pipe.k,
                t: pipe.v.into(),
                pipe: true,
                optional: false,
            })
        };
        for (k, v) in t.req {
            args.push(Argument {
                name: k,
                t: v.into(),
                pipe: false,
                optional: false,
            });
        }
        for (k, v) in t.opt {
            args.push(Argument {
                name: k,
                t: v.into(),
                pipe: false,
                optional: true,
            });
        }
        Fun {
            args,
            retn: t.retn.into(),
        }
    }
}

impl From<semantic::Function> for MonoType {
    fn from(t: semantic::Function) -> MonoType {
        MonoType::Fun(Box::new(t.into()))
    }
}

impl From<Fun> for semantic::Function {
    fn from(t: Fun) -> semantic::Function {
        let mut req = HashMap::new();
        let mut opt = HashMap::new();
        let mut pipe = None;
        for arg in t.args {
            match arg {
                Argument {
                    name,
                    t,
                    pipe: true,
                    ..
                } => {
                    pipe = Some(semantic::Property {
                        k: name,
                        v: t.into(),
                    });
                }
                Argument {
                    name,
                    t,
                    optional: true,
                    ..
                } => {
                    opt.insert(name, t.into());
                }
                Argument {
                    name,
                    t,
                    pipe: false,
                    optional: false,
                    ..
                } => {
                    req.insert(name, t.into());
                }
            };
        }
        semantic::Function {
            req,
            opt,
            pipe,
            retn: t.retn.into(),
        }
    }
}

impl From<Fun> for semantic::MonoType {
    fn from(t: Fun) -> semantic::MonoType {
        semantic::MonoType::Fun(Box::new(t.into()))
    }
}

impl Compile<fb::Fun<'_>> for Fun {
    fn compile(t: fb::Fun) -> Result<Fun, String> {
        let fb_args = t.args().unwrap();
        let l = fb_args.len();
        let mut args = Vec::with_capacity(l);
        for i in 0..l {
            args.push(Argument::compile(fb_args.get(i))?);
        }
        Ok(Fun {
            args,
            retn: MonoType::from_table(t.retn().unwrap(), t.retn_type())?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: String,
    pub t: MonoType,
    pub pipe: bool,
    pub optional: bool,
}

impl Compile<fb::Argument<'_>> for Argument {
    fn compile(t: fb::Argument) -> Result<Argument, String> {
        Ok(Argument {
            name: t.name().unwrap().to_owned(),
            t: MonoType::from_table(t.t().unwrap(), t.t_type())?,
            pipe: t.pipe(),
            optional: t.optional(),
        })
    }
}

// Encode a polytype using flatbuffers
fn build_polytype<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    t: PolyType,
) -> flatbuffers::WIPOffset<fb::PolyType<'a>> {
    let vars = build_vars(builder, t.vars);
    let vars = builder.create_vector(vars.as_slice());

    let cons = build_cons(builder, t.cons);
    let cons = builder.create_vector(cons.as_slice());

    let (offset_expr, expr) = build_type(builder, t.expr);
    fb::PolyType::create(
        builder,
        &fb::PolyTypeArgs {
            vars: Some(vars),
            cons: Some(cons),
            expr_type: expr,
            expr: Some(offset_expr),
        },
    )
}

fn build_cons<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    cons: Vec<Constraint>,
) -> Vec<flatbuffers::WIPOffset<fb::Constraint<'a>>> {
    let mut v = Vec::with_capacity(cons.len());
    for c in cons {
        v.push(build_constraint(builder, c));
    }
    v
}

fn build_constraint<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    constraint: Constraint,
) -> flatbuffers::WIPOffset<fb::Constraint<'a>> {
    let tvar = build_var(builder, constraint.tvar);
    fb::Constraint::create(
        builder,
        &fb::ConstraintArgs {
            tvar: Some(tvar),
            kind: constraint.kind.into(),
        },
    )
}

fn build_type<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    t: MonoType,
) -> (
    flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
    fb::MonoType,
) {
    match t {
        MonoType::Bool => (
            fb::Basic::create(builder, &fb::BasicArgs { t: fb::Type::Bool }).as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Int => (
            fb::Basic::create(builder, &fb::BasicArgs { t: fb::Type::Int }).as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Uint => (
            fb::Basic::create(builder, &fb::BasicArgs { t: fb::Type::Uint }).as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Float => (
            fb::Basic::create(builder, &fb::BasicArgs { t: fb::Type::Float }).as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::String => (
            fb::Basic::create(
                builder,
                &fb::BasicArgs {
                    t: fb::Type::String,
                },
            )
            .as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Duration => (
            fb::Basic::create(
                builder,
                &fb::BasicArgs {
                    t: fb::Type::Duration,
                },
            )
            .as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Time => (
            fb::Basic::create(builder, &fb::BasicArgs { t: fb::Type::Time }).as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Regexp => (
            fb::Basic::create(
                builder,
                &fb::BasicArgs {
                    t: fb::Type::Regexp,
                },
            )
            .as_union_value(),
            fb::MonoType::Basic,
        ),
        MonoType::Var(tvr) => (build_var(builder, tvr).as_union_value(), fb::MonoType::Var),
        MonoType::Arr(arr) => {
            let (off, typ) = build_type(builder, arr.t);
            (
                fb::Arr::create(
                    builder,
                    &fb::ArrArgs {
                        t_type: typ,
                        t: Some(off),
                    },
                )
                .as_union_value(),
                fb::MonoType::Arr,
            )
        }
        MonoType::Row(row) => {
            let props = build_props(builder, row.props);
            let props = builder.create_vector(props.as_slice());
            let extends = match row.extends {
                None => None,
                Some(tv) => Some(build_var(builder, tv)),
            };
            (
                fb::Row::create(
                    builder,
                    &fb::RowArgs {
                        props: Some(props),
                        extends,
                    },
                )
                .as_union_value(),
                fb::MonoType::Row,
            )
        }
        MonoType::Fun(fun) => {
            let (ret, typ) = build_type(builder, fun.retn);
            let args = build_args(builder, fun.args);
            let args = builder.create_vector(args.as_slice());
            (
                fb::Fun::create(
                    builder,
                    &fb::FunArgs {
                        args: Some(args),
                        retn_type: typ,
                        retn: Some(ret),
                    },
                )
                .as_union_value(),
                fb::MonoType::Fun,
            )
        }
    }
}

fn build_vars<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    vars: Vec<Var>,
) -> Vec<flatbuffers::WIPOffset<fb::Var<'a>>> {
    let mut v = Vec::with_capacity(vars.len());
    for var in vars {
        v.push(build_var(builder, var));
    }
    v
}

fn build_var<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    var: Var,
) -> flatbuffers::WIPOffset<fb::Var<'a>> {
    fb::Var::create(builder, &fb::VarArgs { i: var.i })
}

fn build_props<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    props: Vec<Property>,
) -> Vec<flatbuffers::WIPOffset<fb::Prop<'a>>> {
    let mut v = Vec::with_capacity(props.len());
    for prop in props {
        v.push(build_prop(builder, prop));
    }
    v
}

fn build_prop<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    prop: Property,
) -> flatbuffers::WIPOffset<fb::Prop<'a>> {
    let (off, typ) = build_type(builder, prop.v);
    let k = builder.create_string(&prop.k);
    fb::Prop::create(
        builder,
        &fb::PropArgs {
            k: Some(k),
            v_type: typ,
            v: Some(off),
        },
    )
}

fn build_args<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    args: Vec<Argument>,
) -> Vec<flatbuffers::WIPOffset<fb::Argument<'a>>> {
    let mut v = Vec::with_capacity(args.len());
    for arg in args {
        v.push(build_arg(builder, arg));
    }
    v
}

fn build_arg<'a>(
    builder: &mut flatbuffers::FlatBufferBuilder<'a>,
    arg: Argument,
) -> flatbuffers::WIPOffset<fb::Argument<'a>> {
    let (off, typ) = build_type(builder, arg.t);
    let name = builder.create_string(&arg.name);
    fb::Argument::create(
        builder,
        &fb::ArgumentArgs {
            name: Some(name),
            t_type: typ,
            t: Some(off),
            pipe: arg.pipe,
            optional: arg.optional,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic::types as semantic;

    #[rustfmt::skip]
    use crate::semantic::flatbuffers::semantic_generated::fbsemantic::{
        Expression,
        ExpressionStatement,
        ExpressionStatementArgs,
        File,
        FileArgs,
        FloatLiteral,
        FloatLiteralArgs,
        Operator,
        Package,
        PackageArgs,
        Statement,
        UnaryExpression,
        UnaryExpressionArgs,
        WrappedStatement,
        WrappedStatementArgs,
    };

    use maplit;

    fn fb_serde(want: PolyType) {
        // Initialize flatbuffer builder
        let mut fb = flatbuffers::FlatBufferBuilder::new();
        // Encode polytype using flatbuffer
        let offset = build_polytype(&mut fb, want.clone());
        // Serialize polytype
        fb.finish(offset, None);
        // Return raw buffer
        let buf = fb.finished_data();
        // Decode polytype
        let fb_poly = flatbuffers::get_root::<fb::PolyType>(buf);
        // Deserialize polytype
        let got = PolyType::compile(fb_poly).unwrap();
        // Assert we get the same type we started with
        assert_eq!(want, got)
    }

    fn compile(want: semantic::PolyType) {
        assert_eq!(want.clone(), semantic::PolyType::from(PolyType::from(want)))
    }

    #[test]
    fn serde_basic_types() {
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Bool,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Int,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Uint,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Float,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::String,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Duration,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Time,
        });
        fb_serde(PolyType {
            vars: Vec::new(),
            cons: Vec::new(),
            expr: MonoType::Regexp,
        });
    }
    #[test]
    fn compile_basic_types() {
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Bool,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Int,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Uint,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Float,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::String,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Duration,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Time,
        });
        compile(semantic::PolyType {
            vars: Vec::new(),
            cons: HashMap::new(),
            expr: semantic::MonoType::Regexp,
        });
    }
    #[test]
    fn serde_array_type() {
        fb_serde(PolyType {
            vars: vec![Var { i: 0 }],
            cons: Vec::new(),
            expr: MonoType::Arr(Box::new(Arr {
                t: MonoType::Var(Var { i: 0 }),
            })),
        });
    }
    #[test]
    fn compile_array_type() {
        compile(semantic::PolyType {
            vars: vec![semantic::Tvar(0)],
            cons: HashMap::new(),
            expr: semantic::MonoType::Arr(Box::new(semantic::Array(semantic::MonoType::Var(
                semantic::Tvar(0),
            )))),
        });
    }
    #[test]
    fn serde_function_types() {
        fb_serde(PolyType {
            vars: vec![Var { i: 0 }],
            cons: Vec::new(),
            expr: MonoType::Fun(Box::new(Fun {
                args: vec![
                    Argument {
                        name: String::from("tables"),
                        t: MonoType::Arr(Box::new(Arr {
                            t: MonoType::Var(Var { i: 0 }),
                        })),
                        pipe: true,
                        optional: false,
                    },
                    Argument {
                        name: String::from("fn"),
                        t: MonoType::Fun(Box::new(Fun {
                            args: vec![Argument {
                                name: String::from("r"),
                                t: MonoType::Var(Var { i: 0 }),
                                pipe: false,
                                optional: false,
                            }],
                            retn: MonoType::Bool,
                        })),
                        pipe: false,
                        optional: false,
                    },
                    Argument {
                        name: String::from("flag"),
                        t: MonoType::Bool,
                        pipe: false,
                        optional: true,
                    },
                ],
                retn: MonoType::Arr(Box::new(Arr {
                    t: MonoType::Var(Var { i: 0 }),
                })),
            })),
        });
        fb_serde(PolyType {
            vars: vec![Var { i: 0 }, Var { i: 1 }],
            cons: vec![
                Constraint {
                    tvar: Var { i: 0 },
                    kind: Kind::Addable,
                },
                Constraint {
                    tvar: Var { i: 1 },
                    kind: Kind::Divisible,
                },
            ],
            expr: MonoType::Fun(Box::new(Fun {
                args: vec![
                    Argument {
                        name: String::from("a"),
                        t: MonoType::Var(Var { i: 0 }),
                        pipe: false,
                        optional: false,
                    },
                    Argument {
                        name: String::from("b"),
                        t: MonoType::Var(Var { i: 1 }),
                        pipe: false,
                        optional: false,
                    },
                ],
                retn: MonoType::Bool,
            })),
        });
    }
    #[test]
    fn compile_function_types() {
        compile(semantic::PolyType {
            vars: vec![semantic::Tvar(0)],
            cons: HashMap::new(),
            expr: semantic::MonoType::Fun(Box::new(semantic::Function {
                req: maplit::hashmap! {
                    String::from("fn") => semantic::MonoType::Fun(Box::new(semantic::Function {
                        req: maplit::hashmap! {
                            String::from("r") => semantic::MonoType::Var(semantic::Tvar(0)),
                        },
                        opt: HashMap::new(),
                        pipe: None,
                        retn: semantic::MonoType::Bool,
                    }))
                },
                opt: maplit::hashmap! {
                    String::from("flag") => semantic::MonoType::Bool,
                },
                pipe: Some(semantic::Property {
                    k: String::from("tables"),
                    v: semantic::MonoType::Arr(Box::new(semantic::Array(semantic::MonoType::Var(
                        semantic::Tvar(0),
                    )))),
                }),
                retn: semantic::MonoType::Arr(Box::new(semantic::Array(semantic::MonoType::Var(
                    semantic::Tvar(0),
                )))),
            })),
        });
        compile(semantic::PolyType {
            vars: vec![semantic::Tvar(0), semantic::Tvar(1)],
            cons: maplit::hashmap! {
                semantic::Tvar(0) => vec![semantic::Kind::Addable],
                semantic::Tvar(1) => vec![semantic::Kind::Divisible],
            },
            expr: semantic::MonoType::Fun(Box::new(semantic::Function {
                req: maplit::hashmap! {
                    String::from("a") => semantic::MonoType::Var(semantic::Tvar(0)),
                    String::from("b") => semantic::MonoType::Var(semantic::Tvar(1)),
                },
                opt: HashMap::new(),
                pipe: None,
                retn: semantic::MonoType::Bool,
            })),
        });
    }
    #[test]
    fn serde_record_types() {
        fb_serde(PolyType {
            vars: vec![Var { i: 0 }],
            cons: Vec::new(),
            expr: MonoType::Row(Box::new(Row {
                props: vec![
                    Property {
                        k: String::from("a"),
                        v: MonoType::Int,
                    },
                    Property {
                        k: String::from("b"),
                        v: MonoType::Float,
                    },
                    Property {
                        k: String::from("c"),
                        v: MonoType::Row(Box::new(Row {
                            props: vec![
                                Property {
                                    k: String::from("d"),
                                    v: MonoType::String,
                                },
                                Property {
                                    k: String::from("d"),
                                    v: MonoType::String,
                                },
                                Property {
                                    k: String::from("d"),
                                    v: MonoType::Time,
                                },
                                Property {
                                    k: String::from("d"),
                                    v: MonoType::Row(Box::new(Row {
                                        props: Vec::new(),
                                        extends: None,
                                    })),
                                },
                            ],
                            extends: None,
                        })),
                    },
                ],
                extends: Some(Var { i: 0 }),
            })),
        });
    }
    #[test]
    fn compile_record_types() {
        compile(semantic::PolyType {
            vars: vec![semantic::Tvar(0)],
            cons: HashMap::new(),
            expr: semantic::MonoType::Row(Box::new(semantic::Row::Extension {
                head: semantic::Property {
                    k: String::from("a"),
                    v: semantic::MonoType::Int,
                },
                tail: semantic::MonoType::Row(Box::new(semantic::Row::Extension {
                    head: semantic::Property {
                        k: String::from("b"),
                        v: semantic::MonoType::Float,
                    },
                    tail: semantic::MonoType::Row(Box::new(semantic::Row::Extension {
                        head: semantic::Property {
                            k: String::from("c"),
                            v: semantic::MonoType::Row(Box::new(semantic::Row::Extension {
                                head: semantic::Property {
                                    k: String::from("d"),
                                    v: semantic::MonoType::String,
                                },
                                tail: semantic::MonoType::Row(Box::new(semantic::Row::Extension {
                                    head: semantic::Property {
                                        k: String::from("d"),
                                        v: semantic::MonoType::String,
                                    },
                                    tail: semantic::MonoType::Row(Box::new(
                                        semantic::Row::Extension {
                                            head: semantic::Property {
                                                k: String::from("d"),
                                                v: semantic::MonoType::Time,
                                            },
                                            tail: semantic::MonoType::Row(Box::new(
                                                semantic::Row::Extension {
                                                    head: semantic::Property {
                                                        k: String::from("d"),
                                                        v: semantic::MonoType::Row(Box::new(
                                                            semantic::Row::Empty,
                                                        )),
                                                    },
                                                    tail: semantic::MonoType::Row(Box::new(
                                                        semantic::Row::Empty,
                                                    )),
                                                },
                                            )),
                                        },
                                    )),
                                })),
                            })),
                        },
                        tail: semantic::MonoType::Var(semantic::Tvar(0)),
                    })),
                })),
            })),
        });
    }

    #[test]
    fn test_flatbuffers_semantic() {
        let mut builder = flatbuffers::FlatBufferBuilder::new_with_capacity(256);

        // Testing out a unary expression using a float
        let floatval = FloatLiteral::create(
            &mut builder,
            &FloatLiteralArgs {
                value: 3.5,
                ..FloatLiteralArgs::default()
            },
        );

        let increment = UnaryExpression::create(
            &mut builder,
            &UnaryExpressionArgs {
                operator: Operator::SubtractionOperator,
                argument: Some(floatval.as_union_value()),
                ..UnaryExpressionArgs::default()
            },
        );

        let statement = ExpressionStatement::create(
            &mut builder,
            &ExpressionStatementArgs {
                expression_type: Expression::UnaryExpression,
                expression: Some(increment.as_union_value()),
                ..ExpressionStatementArgs::default()
            },
        );

        let wrappedStatement = WrappedStatement::create(
            &mut builder,
            &WrappedStatementArgs {
                statement_type: Statement::ExpressionStatement,
                statement: Some(statement.as_union_value()),
            },
        );

        let statements = builder.create_vector(&[wrappedStatement]);

        let file = File::create(
            &mut builder,
            &FileArgs {
                body: Some(statements),
                ..FileArgs::default()
            },
        );

        let files = builder.create_vector(&[file]);

        let pkg = Package::create(
            &mut builder,
            &PackageArgs {
                files: Some(files),
                ..PackageArgs::default()
            },
        );

        builder.finish(pkg, None);
        let bytes = builder.finished_data();
        assert_ne!(bytes.len(), 0);
    }
}
