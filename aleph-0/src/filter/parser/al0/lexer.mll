{
open Parser
open Syntax
}

let space = [' ' '\t' '\n' '\r']
let digit = ['0'-'9']
let lower = ['a'-'z']
let upper = ['A'-'Z']

rule token = parse
| space+
    { token lexbuf }
| "//"
    { commentLine lexbuf;
      token lexbuf }
| "/*"
    { comment lexbuf;
      token lexbuf }
| '('
    { LPAREN }
| ')'
    { RPAREN }
| '['
    { LSQUAREBRACKET }
| ']'
    { RSQUAREBRACKET }
| "true"
    { BOOL(true) }
| "false"
    { BOOL(false) }
| '!'
    { NOT }
| '&'
    { AND }
| '|'
    { PIPE }
| digit+
    { INT(int_of_string (Lexing.lexeme lexbuf)) }
| digit+ ('.' digit*)? (['e' 'E'] ['+' '-']? digit+)?
    { FLOAT(float_of_string (Lexing.lexeme lexbuf)) }
| '-'
    { MINUS }
| '+'
    { PLUS }
| '*'
    { AST }
| '/'
    { SLASH }
| '?'
    { QUESTION }
| '='
    { EQUAL }
| "<>"
    { LESS_GREATER }
| "<="
    { LESS_EQUAL }
| ">="
    { GREATER_EQUAL }
| '<'
    { LESS }
| '>'
    { GREATER }
| ':'
    { COLON }
| ','
    { COMMA }
| '_'
    { IDENT(Syntax.gentmp Syntax.Unit) }
| '.'
    { DOT }
| "<-"
    { LESS_MINUS }
| ';'
    { SEMICOLON }
| '%'
    { MODULO }
| '"'
    { DOUBLEQUOTE }
| '{'
    { LBRACE }
| '}'
    { RBRACE }
| "fun"
    { FUN }
| "import"
    { IMPORT }
| eof
    { EOF }
| lower (digit|lower|upper|'_')*
    { IDENT(Lexing.lexeme lexbuf) }
| "\"" [^'"']* "\""
    { STRING(Lexing.lexeme lexbuf) }
| _
    { failwith
        (Printf.sprintf "unknown token %s near characters %d-%d"
           (Lexing.lexeme lexbuf)
           (Lexing.lexeme_start lexbuf)
           (Lexing.lexeme_end lexbuf)) }
and comment = parse
| "*/"
    { () }
| "/*"
    { comment lexbuf;
      comment lexbuf }
| eof
    { Format.eprintf "warning: unterminated comment@." }
| _
    { comment lexbuf }
and commentLine = parse
| '\n'
    { () }
| "//"
    { commentLine lexbuf;
      commentLine lexbuf }
| _
    { commentLine lexbuf }
