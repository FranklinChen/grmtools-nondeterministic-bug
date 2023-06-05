pub mod error;

use lrlex::lrlex_mod;
lrlex_mod!("term.l");

use lrpar::lrpar_mod;
lrpar_mod!("term.y");

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn err_no_number() {
        let input = "(())";

        let lexerdef = term_l::lexerdef();
        let lexer = lexerdef.lexer(input);

        assert_debug_snapshot!(term_y::parse(&lexer));
    }
}
