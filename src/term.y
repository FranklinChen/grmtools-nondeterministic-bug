%start Term

%%

Term -> Result<u32, TermError<'input>>:
     'NUM' {
         let v = $1?;
         let s = $lexer.span_str(v.span());
         s.parse::<u32>().map_err(|_| TermError::NumberTooBig(s))
     }
  | '(' Term ')' { $2 }
  | 'UNKNOWN_TOKEN' {
         let v = $1?;
         let s = $lexer.span_str(v.span());
         Err(TermError::UnknownToken(s))
     }
;

%%

use crate::error::TermError;
