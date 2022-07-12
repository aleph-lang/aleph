%token <int> INT
%token PLUS MINUS TIMES DIV
%token LPAREN RPAREN DOT
%token EOL EOF
%left PLUS MINUS 
%left TIMES DIV
%nonassoc UMINUS
%start main
%type <token> main
%%
main:
    int_expr EOL                {  INT($1) }
  | int_expr EOF                {  INT($1) }
  | EOF                         {  EOF     }
;
;
int_expr:
    INT                             { $1 }
  | LPAREN int_expr RPAREN          { $2 }
  | int_expr PLUS int_expr          { $1 + $3 }
  | int_expr MINUS int_expr         { $1 - $3 }
  | int_expr TIMES int_expr         { $1 * $3 }
  | int_expr DIV int_expr           { $1 / $3 }
  | MINUS int_expr %prec UMINUS     { - $2 }
;