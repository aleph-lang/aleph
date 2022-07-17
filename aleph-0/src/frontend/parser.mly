%{
open Syntax
let addtyp x = (x, Type.gentyp ())
%}

%token <bool> BOOL
%token <int> INT
%token <float> FLOAT
%token NOT
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
%token <Id.t> IDENT
%token LET
%token IN
%token REC
%token COMMA
%token ARRAY_CREATE
%token DOT
%token LESS_MINUS
%token SEMICOLON
%token LPAREN
%token RPAREN
%token MODULO
%token EOF

/* associativity */
%nonassoc IN
%right prec_let
%right SEMICOLON
%right prec_if
%right LESS_MINUS
%nonassoc prec_tuple
%left COMMA
%left EQUAL LESS_GREATER LESS GREATER LESS_EQUAL GREATER_EQUAL
%left PLUS MINUS
%left AST SLASH
%right prec_unary_minus
%left prec_app
%left DOT

%type <Syntax.t> exp
%start exp
%%

simple_exp:
| LPAREN exp RPAREN
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
| simple_exp DOT LPAREN exp RPAREN
    { Get($1, $4) }

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
    { match $2 with
    | Float(f) -> Float(-.f)
    | e -> Neg(e) }
| exp PLUS exp
     { match $1,$3 with
        | Int(_),Int(_) -> Add($1, $3)
        | Int(i), Float(_) -> FAdd(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FAdd($1,Float(float_of_int i))
        | Float(_), Float(_) -> FAdd($1, $3)
        | _,_ -> Unit
    }
| exp MINUS exp
    { match $1,$3 with
        | Int(_),Int(_) -> Sub($1, $3)
        | Int(i), Float(_) -> FSub(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FSub($1,Float(float_of_int i))
        | Float(_), Float(_) -> FSub($1, $3)
        | _,_ -> Unit
    }
    | exp AST exp
    { match $1,$3 with
        | Int(_),Int(_) -> Mul($1, $3)
        | Int(i), Float(_) -> FMul(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FMul($1,Float(float_of_int i))
        | Float(_), Float(_) -> FMul($1, $3)
        | _,_ -> Unit
    }
| exp SLASH exp
    { match $1,$3 with
        | Int(_),Int(_) -> Div($1, $3)
        | Int(i), Float(_) -> FDiv(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FDiv($1,Float(float_of_int i))
        | Float(_), Float(_) -> FDiv($1, $3)
        | _,_ -> Unit
    }
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
| IF exp THEN exp ELSE exp
    %prec prec_if
    { If($2, $4, $6) }
| LET REC fundef IN exp
    %prec prec_let
    { LetRec($3, $5) }
| simple_exp actual_args
    %prec prec_app
    { App($1, $2) }
| elems
    %prec prec_tuple
    { Tuple($1) }
| LET LPAREN pat RPAREN EQUAL exp IN exp
    { LetTuple($3, $6, $8) }
| simple_exp DOT LPAREN exp RPAREN LESS_MINUS exp
    { Put($1, $4, $7) }
| IDENT EQUAL exp SEMICOLON exp
    %prec prec_let
    { Let(addtyp $1, $3, $5) }
| exp SEMICOLON exp
    { Let((Id.gentmp Type.Unit, Type.Unit), $1, $3) }
| ARRAY_CREATE simple_exp simple_exp
    %prec prec_app
    { Array($2, $3) }
| EOF
    { Unit }
| error
    { failwith
        (Printf.sprintf "parse error near characters %d-%d"
           (Parsing.symbol_start ())
           (Parsing.symbol_end ())) }

fundef:
| IDENT formal_args EQUAL exp
    { { name = addtyp $1; args = $2; body = $4 } }

formal_args:
| IDENT formal_args
    { addtyp $1 :: $2 }
| IDENT
    { [addtyp $1] }

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

pat:
| pat COMMA IDENT
    { $1 @ [addtyp $3] }
| IDENT COMMA IDENT
    { [addtyp $1; addtyp $3] }
