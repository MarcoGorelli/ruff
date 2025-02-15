use rustpython_ast::{Expr, ExprKind};

use crate::ast::helpers::find_useless_f_strings;
use crate::autofix::Fix;
use crate::checkers::ast::Checker;
use crate::registry::{Diagnostic, RuleCode};
use crate::violations;

/// F541
pub fn f_string_missing_placeholders(expr: &Expr, values: &[Expr], checker: &mut Checker) {
    if !values
        .iter()
        .any(|value| matches!(value.node, ExprKind::FormattedValue { .. }))
    {
        for (prefix_range, tok_range) in find_useless_f_strings(expr, checker.locator) {
            let mut diagnostic = Diagnostic::new(violations::FStringMissingPlaceholders, tok_range);
            if checker.patch(&RuleCode::F541) {
                diagnostic.amend(Fix::deletion(
                    prefix_range.location,
                    prefix_range.end_location,
                ));
            }
            checker.diagnostics.push(diagnostic);
        }
    }
}
