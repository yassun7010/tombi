mod format;
pub mod formatter;

use ast::AstNode;
use diagnostic::Diagnostic;
use format::Format;
pub use formatter::definitions::Definitions;
pub use formatter::options::Options;
pub use formatter::Formatter;
use syntax::TomlVersion;

pub fn format(source: &str) -> Result<String, Vec<Diagnostic>> {
    format_with(source, TomlVersion::default(), &Options::default())
}

pub fn format_with(
    source: &str,
    version: TomlVersion,
    options: &Options,
) -> Result<String, Vec<Diagnostic>> {
    let p = parser::parse(source);
    let errors = p.errors();

    let root = ast::Root::cast(p.into_syntax_node()).unwrap();
    tracing::trace!("ast: {:#?}", root);

    if errors.is_empty() {
        let mut formatted_text = String::new();
        let line_ending = {
            let mut f = Formatter::new_with_options(version, &mut formatted_text, options);
            root.fmt(&mut f).unwrap();
            f.line_ending()
        };

        Ok(formatted_text + line_ending)
    } else {
        Err(errors
            .into_iter()
            .map(|error| Diagnostic::new_error(error.message(), error.range()))
            .collect())
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_format {
    (#[test] fn $name:ident($source:expr) -> Ok(_);) => {
        crate::test_format!(#[test] fn $name($source) -> Ok($source););
    };

    (#[test] fn $name:ident($source:expr, $version:expr) -> Ok(_);) => {
        crate::test_format!(#[test] fn $name($source, $version) -> Ok($source););
    };

    (#[test] fn $name:ident($source:expr) -> Ok($expected:expr);) => {
        crate::test_format!(#[test] fn $name($source, Default::default()) -> Ok($expected););
    };

    (#[test] fn $name:ident($source:expr, $version:expr) -> Ok($expected:expr);) => {
        #[test]
        fn $name() {
            let p = parser::parse($source);
            let ast = ast::Root::cast(p.syntax_node()).unwrap();

            let mut formatted_text = String::new();
            ast.fmt(&mut crate::Formatter::new($version, &mut formatted_text))
                .unwrap();

            assert_eq!(formatted_text, textwrap::dedent($expected).trim());
            assert_eq!(p.errors(), vec![]);
        }
    };

    (#[test] fn $name:ident($source:expr) -> Err(_);) => {
        #[test]
        fn $name() {
            let p = parser::parse($source);

            assert_ne!(p.errors(), vec![]);
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    test_format! {
        #[test] fn test_key_values(r#"
            array5 = [
              1,
              {
                # inline begin dangling comment1
                # inline begin dangling comment2

                # key1 leading comment1
                # key1 leading comment2
                key1 = 1,  # key1 tailing comment
                # key2 leading comment1
                key2 = 2,  # key2 tailing comment

                # inline end dangling comment1
                # inline end dangling comment2
              },

              # comment
            ]
            "#,
            TomlVersion::V1_1_0_Preview
        ) -> Ok(r#"
            key = "value"
            bare_key = "value"
            bare-key = "value"
            1234 = "value"
        "#);
    }
}
