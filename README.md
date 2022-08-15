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
p = arr[5]
s = |arr|  //size  pour les string |"mots"| = 4 
arr[3] = 5
arr[3+] = 5
a = []; a[3] = 5; ----> a = [0,0,0,5]
a = [0,0,0,5]; a[2+] = 7 -----> a = [0,0,7,0,5]
[1,2]  + [3,4] => [1,2,3,4] 

// V2
[1,2]/1 => [1] //position
[1,2] -2 => [1]  // valeur 


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

## Discution du 31/07/2022 a faire apres.

Ajout de filtres Backend :
    filtres par defaut genere du ocaml light dans le backend
    un filtre prend un syntax et renvoi un syntax ou unit (Parce print du language dest)

Ajout de filtre Frontend :
    Prend un syntax et renvoi un syntax ou prend la source et rend un source ou syntax.

## Discution du 15/08/2022
al.conf (yaml configuration des filtres de compilation)

Al0 : Al0 + Fonctionnel de Al1 + Pipe op + matching + import
Al1 : typage
Al2 : struct
Al3 : object

Pas de calcul pour le moment, seul les filtres feront les calcul de precisions. A voir pour en fournir un de base.
