# aleph

aleph lang spec and implementation

## Vision

Aleph regroupement de language

Aleph-0 : le plus bas niveau
  imperatif
  c like minimal, procedural
  struct, int, float, char ...
  (voir v lang)

Aleph-1 :
  Fonctionnel
  Allégé

Aleph-2:
    Permet d'inclure Aleph-0 et 1 a l'interieur en cascade
    struct spécif, objet

Aleph-N:
  Prolog...


## Quelque specs


ASML de chaque compilo ca soit Aleph-0
Ce qui permet de faire des backend différent en c, java, python, .net, autres...

Pipe operator : différentié par le context
   b | b : boolean op (Al0 -> Al1)
   expr | expr : pipe op (Al1)

Backend Al0 -> c ou binaire directement



```
3 + 4 => 7
3 + 4.5 => 7.5

arr= [1,2,3,4]
arr<<5, arr<<5|3
p = arr>>
p = arr[5]
s = |arr|  //size

x = 3
(x = 3)?*{ } //while
c?{expr}:{expr} //if
{ } block
?*(b) { } //while
-> //(return)

(c)?*{expr} //while
(a;c;s)?*{expr} //for
(c)?{expr}:{expr} //if

name (params) = expr | { }

"bonjour"
expr crlf expr ou expr ; expr


import : en v2, struct
```

