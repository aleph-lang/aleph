%{
open Syntax
let addtyp x = (x, Type.gentyp ())
%}

%token <bool> BOOL
%token <int> INT
%token <float> FLOAT
%token NOT
%token AND
%token OR
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
%token COMMA
%token ARRAY_CREATE
%token DOT
%token LESS_MINUS
%token SEMICOLON
%token LPAREN
%token RPAREN
%token LBRACE
%token RBRACE
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
| DOUBLEQUOTE STRING DOUBLEQUOTE
    { String($2) }

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
| exp OR exp
    { Or($1,$3) }
| exp PLUS exp
    { match $1,$3 with
        | Int(_),Int(_) -> Add($1, $3)
        | Int(i), Float(_) -> FAdd(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FAdd($1,Float(float_of_int i))
        | Float(_), Float(_) -> FAdd($1, $3)
        | _,Int(_) -> Add($1, $3)
        | _,Float(_) -> FAdd($1, $3)
        | Int(_),_ -> Add($1, $3)
        | Float(_),_ -> FAdd($1, $3)
        | String(s1),String(s2) -> String(s1 ^ s2)
        | _,_ -> Unit
    }
| exp MINUS exp
    { match $1,$3 with
        | Int(_),Int(_) -> Sub($1, $3)
        | Int(i), Float(_) -> FSub(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FSub($1,Float(float_of_int i))
        | Float(_), Float(_) -> FSub($1, $3)
        | _,Int(_) -> Sub($1, $3)
        | _,Float(_) -> FSub($1, $3)
        | Int(_),_ -> Sub($1, $3)
        | Float(_),_ -> FSub($1, $3)
        | _,_ -> Unit
    }
| exp AST exp
    { match $1,$3 with
        | Int(_),Int(_) -> Mul($1, $3)
        | Int(i), Float(_) -> FMul(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FMul($1,Float(float_of_int i))
        | Float(_), Float(_) -> FMul($1, $3)
        | _,Int(_) -> Mul($1, $3)
        | _,Float(_) -> FMul($1, $3)
        | Int(_),_ -> Mul($1, $3)
        | Float(_),_ -> FMul($1, $3)
        | _,_ -> Unit
    }
| exp SLASH exp
    { match $1,$3 with
        | Int(_),Int(_) -> Div($1, $3)
        | Int(i), Float(_) -> FDiv(Float(float_of_int i), $3)
        | Float(_), Int(i) -> FDiv($1,Float(float_of_int i))
        | Float(_), Float(_) -> FDiv($1, $3)
        | _,Int(_) -> Div($1, $3)
        | _,Float(_) -> FDiv($1, $3)
        | Int(_),_ -> Div($1, $3)
        | Float(_),_ -> FDiv($1, $3)
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
| exp QUESTION exp COLON exp
    %prec prec_if
    { If($1, $3, $5) }
| exp QUESTION AST exp
    %prec prec_if
    { While(Unit, $1, $4, Unit) }
| LPAREN exp SEMICOLON exp SEMICOLON exp RPAREN QUESTION AST exp
    %prec prec_if
    { While($2, $4, $10, $6) }
| fundef EQUAL exp
    %prec prec_let
    { LetRec($1, $3) }
| simple_exp actual_args
    %prec prec_app
    { App($1, $2) }
| elems
    %prec prec_tuple
    { Tuple($1) }
| LPAREN pat RPAREN EQUAL exp SEMICOLON exp
    { LetTuple($2, $5, $7) }
| IDENT EQUAL exp
    %prec prec_let
    { Let(addtyp $1, $3, Unit) }
| IDENT EQUAL exp SEMICOLON exp
    %prec prec_let
    { Let(addtyp $1, $3, $5) }
| exp SEMICOLON exp
    { Let((Id.gentmp Type.Unit, Type.Unit), $1, $3) }
| MINUS GREATER exp
    { Return($3) }
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
