//! Tests auto-converted from "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers"
#[allow(unused)]
use super::rsass;
#[allow(unused)]
use rsass::set_precision;

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/01_inline"
#[test]
fn t01_inline() {
    assert_eq!(
        rsass(
            ".result {\n  output: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n  output: #{\\1\\2\\3\\4\\5\\6\\7\\8\\9};\n  output: \"[#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\n  output: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  output: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  output: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/02_variable"
#[test]
fn t02_variable() {
    assert_eq!(
        rsass(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n.result {\n  output: $input;\n  output: #{$input};\n  output: \"[#{$input}]\";\n  output: \"#{$input}\";\n  output: \'#{$input}\';\n  output: \"[\'#{$input}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\n  output: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  output: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  output: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/03_inline_double"
#[test]
fn t03_inline_double() {
    assert_eq!(
        rsass(
            ".result {\n  output: #{#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}};\n  output: #{\"[#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\"};\n  output: #{\"#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\"};\n  output: #{\'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\'};\n  output: #{\"[\'#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: [\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ];\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: [\'\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 \'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/04_variable_double"
#[test]
fn t04_variable_double() {
    assert_eq!(
        rsass(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n.result {\n  output: #{#{$input}};\n  output: #{\"[#{$input}]\"};\n  output: #{\"#{$input}\"};\n  output: #{\'#{$input}\'};\n  output: #{\"[\'#{$input}\']\"};\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: [\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ];\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: \\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 ;\n  output: [\'\\1 \\2 \\3 \\4 \\5 \\6 \\7 \\8 \\9 \'];\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/05_variable_quoted_double"
#[test]
fn t05_variable_quoted_double() {
    assert_eq!(
        rsass(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n.result {\n  dquoted: \"#{#{$input}}\";\n  dquoted: \"#{\"[#{$input}]\"}\";\n  dquoted: \"#{\"#{$input}\"}\";\n  dquoted: \"#{\'#{$input}\'}\";\n  dquoted: \"#{\"[\'#{$input}\']\"}\";\n  squoted: \'#{#{$input}}\';\n  squoted: \'#{\"[#{$input}]\"}\';\n  squoted: \'#{\"#{$input}\"}\';\n  squoted: \'#{\'#{$input}\'}\';\n  squoted: \'#{\"[\'#{$input}\']\"}\';\n}\n"
        )
        .unwrap(),
        ".result {\n  dquoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  dquoted: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\n  dquoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  dquoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  dquoted: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\n  squoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  squoted: \"[\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 ]\";\n  squoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  squoted: \"\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \";\n  squoted: \"[\'\\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7 \\\\8 \\\\9 \']\";\n}\n"
    );
}

/// From "sass-spec/spec/parser/interpolate/14_escapes_literal_numbers/06_escape_interpolation"
#[test]
fn t06_escape_interpolation() {
    assert_eq!(
        rsass(
            "$input: \\1\\2\\3\\4\\5\\6\\7\\8\\9;\n.result {\n  output: \"[\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
        )
        .unwrap(),
        ".result {\n  output: \"[\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}]\";\n  output: \"\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\";\n  output: \'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\';\n  output: \"[\'\\#{\\1\\2\\3\\4\\5\\6\\7\\8\\9}\']\";\n}\n"
    );
}