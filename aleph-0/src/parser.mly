%token <int> INT
%token <string> IDENT
%token <string> STRING
%token PLUS MINUS TIMES DIV
%token LPAREN RPAREN DOT MODULO
%token QUOTE DOUBLE_QUOTE
%token EOL EOF
%left PLUS MINUS 
%left TIMES DIV
%nonassoc UMINUS
%start main
%type <token> main
%%
main:
    MODULO LPAREN int_expr RPAREN        { INT($3) }
  | MODULO LPAREN string_expr RPAREN     { STRING($3) }
  | EOL                                  { EOL}
  | EOF                                  { EOF}
;

int_expr:
    INT                                  { $1 }
  | LPAREN int_expr RPAREN               { $2 }
  | int_expr PLUS int_expr               { $1 + $3 }
  | int_expr MINUS int_expr              { $1 - $3 }
  | int_expr TIMES int_expr              { $1 * $3 }
  | int_expr DIV int_expr                { $1 / $3 }
  | MINUS int_expr %prec UMINUS          { - $2 }
;
string_expr:
    IDENT                                 { $1 }
  | STRING                                { $1 }
  | DOUBLE_QUOTE string_expr DOUBLE_QUOTE { $2 }
  | LPAREN string_expr RPAREN             { $2 }
  | string_expr PLUS string_expr          { $1 ^ $3 }
  | int_expr PLUS string_expr             { string_of_int($1) ^ $3 }
  | string_expr PLUS int_expr             { $1 ^ string_of_int($3) }
  