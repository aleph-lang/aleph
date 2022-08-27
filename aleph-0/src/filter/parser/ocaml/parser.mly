%{
open Syntax
%}

%token <bool> BOOL
%token <int> INT
%token <float> FLOAT
%token NOT
%token AND
%token PIPE
%token MINUS
%token PLUS
%token AST
%token QUESTION
%token AST
%token SLASH
%token COLON
%token EQUAL
%token LESS_GREATER
%token LESS_EQUAL
%token GREATER_EQUAL
%token LESS
%token GREATER
%token IF
%token THEN
%token ELSE
%token FUN
%token <string> IDENT
%token COMMA
%token DOT
%token LESS_MINUS
%token SEMICOLON
%token LPAREN
%token RPAREN
%token LBRACE
%token RBRACE
%token LSQUAREBRACKET
%token RSQUAREBRACKET
%token GET
%token MODULO
%token DOUBLEQUOTE
%token <string> STRING
%token EOF

/* associativity */
%right prec_let
%right SEMICOLON
%right prec_if
%right LESS_MINUS
%nonassoc prec_tuple
%nonassoc STRING
%left COMMA
%left EQUAL LESS_GREATER LESS GREATER LESS_EQUAL GREATER_EQUAL
%left PLUS MINUS
%left AST SLASH
%left AND OR
%right prec_unary_minus
%left prec_app
%left DOT

%type <Syntax.t> exp
%start exp
%%

simple_exp:
| LPAREN exp RPAREN
    { $2 }
| LBRACE exp RBRACE
    { $2 }
| LPAREN RPAREN
    { Unit }
| BOOL
    { Bool($1) }
| INT
    { Int($1) }
| FLOAT
    { Float($1) }
| IDENT
    { Var($1) }
| STRING
    { String($1) }

exp:
| SEMICOLON
    { Unit }
| simple_exp
    { $1 }
| NOT exp
    %prec prec_app
    { Not($2) }
| MINUS exp
    %prec prec_unary_minus
    {   match $2 with
        | Float(f) -> Float(-.f)
        | e -> Neg(e) 
    }
| exp AND exp
    { And($1,$3) }
| exp PIPE exp
    { Or($1,$3) }
| exp PLUS exp
    { Add($1,$3) }
| exp MINUS exp
    { Sub($1,$3) }
| exp AST exp
    { Mul($1,$3) }
| exp SLASH exp
    { Div($1, $3) }
| exp EQUAL exp
    { Eq($1, $3) }
| exp LESS_GREATER exp
    { Not(Eq($1, $3)) }
| exp LESS exp
    { Not(LE($3, $1)) }
| exp GREATER exp
    { Not(LE($1, $3)) }
| exp LESS_EQUAL exp
    { LE($1, $3) }
| exp GREATER_EQUAL exp
    { LE($3, $1) }
| exp QUESTION exp COLON exp
    %prec prec_if
    { If($1, $3, $5) }
| exp QUESTION AST exp
    %prec prec_if
    { While(Unit, $1, $4, Unit) }
| LPAREN exp SEMICOLON exp SEMICOLON exp RPAREN QUESTION AST exp
    %prec prec_if
    { While($2, $4, $10, $6) }
| FUN IDENT actual_args EQUAL exp
    %prec prec_let
    { LetRec($2, $3, $5) }
| simple_exp actual_args
    %prec prec_app
    { App($1, $2) }
| elems
    %prec prec_tuple
    { Tuple($1) }
| IDENT EQUAL exp
    %prec prec_let
    { Let($1, $3, Unit) }
| IDENT EQUAL exp SEMICOLON exp
    %prec prec_let
    { Let($1, $3, $5) }
| exp SEMICOLON exp
    { Stmts($1, $3) }
| LSQUAREBRACKET elems RSQUAREBRACKET
    %prec prec_tuple
    { Array($2) }
| IDENT LSQUAREBRACKET exp RSQUAREBRACKET EQUAL exp
    { Put($1,$3,$6,false) }
| IDENT LSQUAREBRACKET exp PLUS RSQUAREBRACKET EQUAL exp
    { Put($1,$3,$7,true) }
| IDENT LSQUAREBRACKET exp RSQUAREBRACKET
    { Get($1,$3) }
| PIPE IDENT PIPE
    { Length($2) }
| EOF
    { Unit }
| error
    { failwith
        (Printf.sprintf "parse error near characters %d-%d"
           (Parsing.symbol_start ())
           (Parsing.symbol_end ())) }

actual_args:
| actual_args simple_exp
    %prec prec_app
    { $1 @ [$2] }
| simple_exp
    %prec prec_app
    { [$1] }

elems:
| elems COMMA exp
    { $1 @ [$3] }
| exp COMMA exp
    { [$1; $3] }
