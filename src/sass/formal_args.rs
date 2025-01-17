use crate::css;
use crate::error::Error;
use crate::sass::Value;
use crate::value::ListSeparator;
use crate::variablescope::{Scope, ScopeImpl};
use std::default::Default;

/// The declared arguments of a mixin or function declaration.
///
/// The arguments are ordered (so they have a position).
/// Each argument also has a name and may have a default value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct FormalArgs(Vec<(String, Value)>, bool);

impl FormalArgs {
    pub fn new(a: Vec<(String, Value)>, is_varargs: bool) -> Self {
        FormalArgs(a, is_varargs)
    }

    pub fn eval<'a>(
        &self,
        scope: &'a dyn Scope,
        args: &css::CallArgs,
    ) -> Result<ScopeImpl<'a>, Error> {
        let mut argscope = ScopeImpl::sub(scope);
        let n = self.0.len();
        for (i, &(ref name, ref default)) in self.0.iter().enumerate() {
            if let Some(value) = args
                .iter()
                .find(|&&(ref k, ref _v)| k.as_ref() == Some(name))
                .map(|&(ref _k, ref v)| v)
            {
                argscope.define(name, value);
            } else if self.1 && i + 1 == n && args.len() > n {
                let args = args
                    .iter()
                    .skip(i)
                    .map(|&(_, ref v)| v.clone())
                    .collect();
                argscope.define(
                    name,
                    &css::Value::List(args, ListSeparator::Comma, false),
                );
            } else {
                match args.get(i) {
                    Some(&(None, ref v)) => argscope.define(name, v),
                    _ => {
                        let v = default.do_evaluate(&argscope, true)?;
                        argscope.define(name, &v)
                    }
                };
            }
        }
        Ok(argscope)
    }
}

impl Default for FormalArgs {
    fn default() -> Self {
        FormalArgs::new(vec![], false)
    }
}
