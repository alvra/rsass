use css;
use functions::get_builtin_function;
use num_rational::Rational;
use num_traits::Zero;
use ordermap::OrderMap;
use sass::{CallArgs, SassString};
use value::{ListSeparator, Number, Operator, Quotes, Rgba, Unit};
use variablescope::Scope;

/// A sass value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    /// A special kind of escape.  Only really used for !important.
    Bang(String),
    /// A call has a name and an argument (which may be multi).
    Call(SassString, CallArgs),
    Literal(SassString),
    /// A comma- or space separated list of values, with or without brackets.
    List(Vec<Value>, ListSeparator, bool, bool),
    /// A Numeric value is a rational value with a Unit (which may be
    /// Unit::None) and flags.
    Numeric(Number, Unit),
    /// "(a/b) and a/b differs semantically.  Parens means the value
    /// should be evaluated numerically if possible, without parens /
    /// is not allways division.
    Paren(Box<Value>),
    Variable(String),
    /// Both a numerical and original string representation,
    /// since case and length should be preserved (#AbC vs #aabbcc).
    Color(Rgba, Option<String>),
    Null,
    True,
    False,
    /// A binary operation, two operands and an operator.
    /// The boolean represents possible whitespace.
    BinOp(Box<Value>, bool, Operator, bool, Box<Value>),
    UnaryOp(Operator, Box<Value>),
    Map(OrderMap<Value, Value>),
    /// The magic value "&", exanding to the current selectors.
    HereSelector,
    /// A unicode range for font selections. U+NN, U+N?, U+NN-MM.
    /// The string is the entire value, including the "U+" tag.
    UnicodeRange(String),
}

impl Value {
    pub fn scalar(v: isize) -> Self {
        Value::Numeric(Number::from_integer(v), Unit::None)
    }
    pub fn bool(v: bool) -> Self {
        if v {
            Value::True
        } else {
            Value::False
        }
    }
    pub fn black() -> Self {
        Value::Color(Rgba::from_rgb(0, 0, 0), Some("black".into()))
    }
    pub fn rgba(r: Rational, g: Rational, b: Rational, a: Rational) -> Self {
        Value::Color(Rgba::new(r, g, b, a), None)
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            Value::Color(..) => "color",
            Value::Literal(..) => "string",
            Value::Numeric(..) => "number",
            Value::List(..) => "list",
            Value::True | Value::False => "bool",
            Value::Null => "null",
            _ => "unknown",
        }
    }

    /// All values other than `False` and `Null` should be considered true.
    pub fn is_true(&self) -> bool {
        match *self {
            Value::False | Value::Null => false,
            _ => true,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Value::Null => true,
            Value::List(ref list, _, false, _) => {
                list.iter().all(|v| v.is_null())
            }
            _ => false,
        }
    }

    pub fn evaluate(&self, scope: &Scope) -> css::Value {
        self.do_evaluate(scope, false)
    }
    pub fn do_evaluate(&self, scope: &Scope, arithmetic: bool) -> css::Value {
        match *self {
            Value::Bang(ref s) => css::Value::Bang(s.clone()),
            Value::Literal(ref s) => {
                let (s, q) = s.evaluate(scope);
                if s == "" && q == Quotes::None {
                    css::Value::Null
                } else {
                    // TODO Is a value calculated if there was evaluations?
                    css::Value::Literal(s, q, false)
                }
            }
            Value::Paren(ref v) => v.do_evaluate(scope, true),
            Value::Color(ref rgba, ref name) => {
                css::Value::Color(rgba.clone(), name.clone())
            }
            Value::Variable(ref name) => scope.get(name).into_calculated(),
            Value::List(ref v, ref s, b, needs_requote) => css::Value::List(
                v.iter()
                    .map(|v| {
                        let v = v.do_evaluate(scope, false);
                        if needs_requote {
                            v.unrequote()
                        } else {
                            v
                        }
                    }).collect::<Vec<_>>(),
                s.clone(),
                b,
            ),
            Value::Call(ref name, ref args) => {
                let args = args.evaluate(scope, true);
                if let Some(name) = name.single_raw() {
                    match scope.call_function(name, &args) {
                        Some(value) => value,
                        None => get_builtin_function(name)
                            .and_then(|f| f.call(scope, &args).ok())
                            .unwrap_or_else(|| {
                                css::Value::Call(name.to_string(), args)
                            }),
                    }
                } else {
                    let (name, _) = name.evaluate(scope);
                    css::Value::Call(name, args)
                }
            }
            Value::Numeric(ref num, ref unit) => {
                let mut num = num.clone();
                if arithmetic {
                    num.lead_zero = true;
                }
                css::Value::Numeric(num, unit.clone(), arithmetic)
            }
            Value::Map(ref m) => css::Value::Map(
                m.iter()
                    .map(|&(ref k, ref v)| {
                        (
                            k.do_evaluate(scope, false),
                            v.do_evaluate(scope, false),
                        )
                    }).collect(),
            ),
            Value::Null => css::Value::Null,
            Value::True => css::Value::True,
            Value::False => css::Value::False,
            Value::BinOp(ref a, s1, ref op, s2, ref b) => {
                let (arithmetic, a, b) = {
                    let arithmetic = arithmetic
                        || (*op != Operator::Div
                            && *op != Operator::And
                            && *op != Operator::Or);

                    let aa = a.do_evaluate(scope, arithmetic);
                    let b = b
                        .do_evaluate(scope, arithmetic || aa.is_calculated());
                    if !arithmetic && b.is_calculated() && !aa.is_calculated()
                    {
                        (true, a.do_evaluate(scope, true), b)
                    } else {
                        (
                            arithmetic
                                || aa.is_calculated()
                                || b.is_calculated(),
                            aa,
                            b,
                        )
                    }
                };
                if arithmetic {
                    op.eval(a.clone(), b.clone()).unwrap_or_else(|| {
                        css::Value::BinOp(
                            Box::new(a),
                            s1,
                            op.clone(),
                            s2,
                            Box::new(b),
                        )
                    })
                } else {
                    css::Value::BinOp(
                        Box::new(a),
                        s1,
                        op.clone(),
                        s2,
                        Box::new(b),
                    )
                }
            }
            Value::UnaryOp(ref op, ref v) => {
                let value = v.do_evaluate(scope, true);
                match (op.clone(), value) {
                    (Operator::Not, css::Value::Numeric(v, ..)) => {
                        css::Value::bool(v.is_zero())
                    }
                    (Operator::Not, css::Value::True) => css::Value::False,
                    (Operator::Not, css::Value::False) => css::Value::True,
                    (Operator::Minus, css::Value::Numeric(v, u, _)) => {
                        css::Value::Numeric(-&v, u, true)
                    }
                    (Operator::Plus, css::Value::Numeric(v, u, _)) => {
                        css::Value::Numeric(
                            Number {
                                value: v.value,
                                plus_sign: true,
                                lead_zero: v.lead_zero,
                            },
                            u,
                            true,
                        )
                    }
                    (op, v) => css::Value::UnaryOp(op, Box::new(v)),
                }
            }
            Value::HereSelector => scope.get_selectors().to_value(),
            Value::UnicodeRange(ref s) => css::Value::UnicodeRange(s.clone()),
        }
    }
}
