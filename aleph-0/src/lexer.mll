{
open Parser
exception Eof
}

let space = [' ' '\t' '\r']
let digit = ['0'-'9']
let lower = ['a'-'z']
let upper = ['A'-'Z']
let other = [':']

rule token = parse
    space          { token lexbuf }     (* skip blanks *)
  | ['\n']         { EOL }
  | digit+ as lxm  { INT(int_of_string lxm) }
  | '+'            { PLUS }
  | '-'            { MINUS }
  | '*'            { TIMES }
  | '/'            { DIV }
  | '('            { LPAREN }
  | ')'            { RPAREN }
  | '.'            { DOT }
  | '%'            { MODULO }
  | '"'            { DOUBLE_QUOTE }
  | '\''           { QUOTE }
  | eof            { EOF }
  | lower (digit|lower|upper|'_')*
    { IDENT(Lexing.lexeme lexbuf) }    (* var *)
  | (lower|upper)+ (digit|lower|upper|space|other|'_')*
    { STRING(Lexing.lexeme lexbuf) }