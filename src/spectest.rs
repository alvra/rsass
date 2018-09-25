extern crate deunicode;
use deunicode::deunicode;
use std::ffi::OsStr;
use std::fs::{create_dir, DirEntry, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    handle_suites().expect("handle suites");
}

fn handle_suites() -> Result<(), Error> {
    let base = PathBuf::from("sass-spec/spec");
    handle_suite(
        &base,
        "basic",
        &[
            "14_imports", // Need to handle separate files in tests
            "15_arithmetic_and_lists", // requirements changed
            "33_ambiguous_imports", // Need to handle separate files in tests
        ],
    )?;
    handle_suite(&base, "colors", &[])?;
    handle_suite(
        &base,
        "css",
        &[
            "bizarrely_formatted_comments", // Strange indent?
            "custom_properties",            // Most fails  :-(
            "moz_document",                 // Deprecated functionality
            "ms_long_filter_syntax",        // ?
            "multi_star_comments",          // Some problem with whitespace?
            "plain",                        // Not working yet
            "media",             // only subdir range not working yet
            "min_max", // Expected handling of raw css functions changed.
            "selector", // Only test requres @extend
            "unknown_directive", // Some problem with whitespace?
        ],
    )?;
    handle_suite(
        &base,
        "libsass",
        &[
            "Sa\u{301}ss-UT\u{327}F8", // resolves to duplicate name
            "at-error/feature-test",
            "at-root/ampersand",
            "at-root/extend",
            "at-root/138_test_at_root_in_media",
            "at-root/139_test_at_root_in_bubbled_media",
            "at-root/140_test_at_root_in_unknown_directive",
            "at-root/with_without",
            "at-stuff",
            "base-level-parent/imported",
            "base-level-parent/nested/at-root-alone-itpl",
            "base-level-parent/nested/at-root-postfix-itpl",
            "base-level-parent/nested/at-root-prefix-itpl",
            "base-level-parent/root/at-root-postfix-itpl",
            "base-level-parent/root/at-root-prefix-itpl",
            "bourbon",
            "calc",
            "charset",
            "color-functions/opacity/fade-out",
            "color-functions/opacity/transparentize",
            "color-functions/other/adjust-color/l",
            "color-functions/other/adjust-color/s",
            "color-functions/other/change-color/a",
            "color-functions/other/change-color/l",
            "color-functions/other/change-color/s",
            "color-functions/rgb/rgba/a",
            "color-functions/saturate",
            "conversions",
            "css-import",
            "css_nth_selectors",
            "css_unicode",
            "div",
            "env",
            "features/at-error",
            "features/extend-selector-pseudoclass",
            "http_import",
            "import",
            "inh",
            "length",
            "list-evaluation",
            "lists",
            "media",
            "mixin",
            "mixins-and-media-queries",
            "multi-blocks",
            "placeholder-mediaquery",
            "placeholder-nested",
            "precision/default",
            "precision/lower",
            "properties-in-media",
            "rel",
            "selector-functions/is_superselector",
            "selector-functions/selector-length",
            "selector-functions/simple-selector",
            "selectors/access",
            "selectors/interpolation",
            "selectors/mixin-argument",
            "selectors/simple",
            "selectors/variables/multiple/bare",
            "selectors/variables/multiple/interpolated",
            "selectors/variables/nested/bare",
            "selectors/variables/nested/interpolated",
            "test",
            "unary-ops",
            "unicode-bom/utf-16-big", // spectest only handles utf8
            "unicode-bom/utf-16-little", // spectest only handles utf8
            "unicode-bom/utf-8",
            "units/conversion/angle",
            "units/conversion/frequency",
            "units/conversion/resolution",
            "units/conversion/size",
            "units/conversion/time",
            "units/simple",
            "url",
            "variable-scoping/blead-global",
            "variable-scoping/defaults",
            "variable-scoping/lexical-scope",
            "variable-scoping/root-scope",
            "variables_in_media",
        ],
    )?;
    handle_suite(
        &base,
        "misc",
        &[
            "mixin_content", // ?
            "negative_numbers",
            "JMA-pseudo-test", // Requires @extend
            "trailing_comma_in_selector",
        ],
    )?;
    // Note: The round test is broken; it requires 1.49999999999 to be rounded to 1.
    handle_suite(&base, "number-functions", &["round"])?;
    handle_suite(
        &base,
        "values",
        &[
            "identifiers/escape/normalize",
            "identifiers/escape/script",
            "ids",
            "numbers/units/multiple",
        ],
    )?;
    Ok(())
}

fn handle_suite(
    base: &Path,
    suite: &str,
    ignored: &[&str],
) -> Result<(), Error> {
    eprintln!("Handle suite {:?}", suite);
    let suitedir = base.join(suite);
    let rssuitedir = PathBuf::from("tests").join(fn_name(suite));
    let _may_exist = create_dir(&rssuitedir);
    let mut rs = File::create(rssuitedir.join("main.rs"))?;
    writeln!(
        rs,
        "//! Tests auto-converted from {:?}\
         \n//! version {}\
         \n//! See <https://github.com/sass/sass-spec> for source material.\\n",
        suitedir,
        String::from_utf8(
            Command::new("git")
                .args(&["log", "-1", "--format=%h, %ai."])
                .current_dir(base)
                .output()?
                .stdout
        )?.trim(),
    )?;
    if !ignored.is_empty() {
        writeln!(
            rs,
            "//! The following tests are excluded from conversion:\
             \n//! {:?}",
            ignored,
        )?;
    }
    writeln!(
        rs,
        "extern crate rsass;\
         \nuse rsass::{{compile_scss, OutputStyle}};",
    )?;

    handle_entries(&mut rs, &suitedir, &rssuitedir, ignored)?;

    writeln!(
        rs,
        "\nfn rsass(input: &str) -> Result<String, String> {{\
         \n    compile_scss(input.as_bytes(), OutputStyle::Expanded)\
         \n        .map_err(|e| format!(\"rsass failed: {{}}\", e))\
         \n        .and_then(|s| String::from_utf8(s).map_err(|e| format!(\"{{:?}}\", e)))\
         \n}}")?;
    Ok(())
}

fn handle_entries(
    rs: &mut Write,
    suitedir: &Path,
    rssuitedir: &Path,
    ignored: &[&str],
) -> Result<(), Error> {
    let precision = load_options(suitedir); // TODO .or(precision);
    let mut entries: Vec<DirEntry> =
        suitedir.read_dir()?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        if entry.file_type()?.is_dir() {
            if entry.path().join("error").is_file() {
                ignore(
                    rs,
                    &entry.file_name(),
                    "tests with expected error not implemented yet",
                )?;
            } else if ignored.iter().any(|&i| &entry.file_name() == i) {
                ignore(rs, &entry.file_name(), "not expected to work yet")?;
            } else {
                let input = entry.path().join("input.scss");
                if input.exists() {
                    eprintln!("Should handle {:?}", entry.file_name());
                    spec_to_test(
                        rs,
                        &suitedir,
                        &entry.file_name(),
                        precision,
                    )?;
                } else {
                    let name = fn_name_os(&entry.file_name());
                    writeln!(rs, "\nmod {};", name);
                    let rssuitedir = rssuitedir.join(name);
                    let _may_exist = create_dir(&rssuitedir);
                    let mut rs = File::create(rssuitedir.join("mod.rs"))?;
                    writeln!(
                        rs,
                        "//! Tests auto-converted from {:?}\
                         \n#[allow(unused)]\
                         \nuse super::rsass;\
                         \n#[allow(unused)]\
                         \nuse rsass::set_precision;",
                        suitedir.join(entry.file_name()),
                    )?;
                    let tt =
                        format!("{}/", entry.file_name().to_string_lossy());
                    let ignored: Vec<&str> = ignored
                        .iter()
                        .filter_map(|p| {
                            if p.starts_with(&tt) {
                                Some(p.split_at(tt.len()).1)
                            } else {
                                None
                            }
                        }).collect();
                    handle_entries(
                        &mut rs,
                        &entry.path(),
                        &rssuitedir,
                        &ignored,
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn ignore(
    rs: &mut Write,
    name: &OsStr,
    reason: &str,
) -> Result<(), io::Error> {
    eprintln!("Ignoring {:?}, {}.", name, reason);
    writeln!(rs, "\n// Ignoring {:?}, {}.", name, reason)
}

fn spec_to_test(
    rs: &mut Write,
    suite: &Path,
    test: &OsStr,
    precision: Option<i64>,
) -> Result<(), Error> {
    let specdir = suite.join(test);
    let precision = load_options(&specdir).or(precision);
    let input = specdir.join("input.scss");
    let expected = specdir.join("expected_output.css");
    writeln!(
        rs,
        "\n/// From {:?}\
         \n#[test]\
         \nfn {}() {{",
        specdir,
        fn_name_os(test),
    )?;
    if let Some(precision) = precision {
        writeln!(rs, "    set_precision({});", precision)?;
    }
    let input = format!("{:?}", content(&input)?);
    let expected = format!("{:?}", content(&expected)?);
    if input.len() + expected.len() < 48 {
        writeln!(
            rs,
            "    assert_eq!(rsass({}).unwrap(), {});",
            input, expected
        )?;
    } else if input.len() < 55 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({}).unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )?;
    } else if input.len() < 62 {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass({})\
             \n            .unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )?;
    } else {
        writeln!(
            rs,
            "    assert_eq!(\
             \n        rsass(\
             \n            {}\
             \n        ).unwrap(),\
             \n        {}\
             \n    );",
            input, expected
        )?;
    }
    writeln!(rs, "}}")?;
    Ok(())
}

fn fn_name_os(name: &OsStr) -> String {
    fn_name(&name.to_string_lossy())
}
fn fn_name(name: &str) -> String {
    let t = deunicode(name)
        .to_lowercase()
        .replace('-', "_")
        .replace('.', "_");
    if t.chars().next().unwrap_or('0').is_numeric() {
        format!("t{}", t)
    } else if t == "else"
        || t == "for"
        || t == "if"
        || t == "static"
        || t == "while"
    {
        format!("test_{}", t)
    } else {
        t
    }
}

fn content(path: &Path) -> Result<String, io::Error> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

#[derive(Debug)]
struct Error(String);
use std::convert::From;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error(format!("io error: {}", e))
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error(format!("utf8 error: {}", e))
    }
}

extern crate yaml_rust;
use yaml_rust::YamlLoader;

/// Load options from options.yml.
///
/// Currently only `:precision` is suppoted, so the options struct is simply
/// an i64.
fn load_options(path: &Path) -> Option<i64> {
    let options = content(&path.join("options.yml")).ok()?;
    let options = YamlLoader::load_from_str(&options).ok()?;
    let data = &options[0];
    eprintln!("Found options: {:?}", data);
    data[":precision"].as_i64()
}
