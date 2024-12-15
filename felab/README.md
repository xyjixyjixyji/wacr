# *F*ront*End* Lab

The FrontEnd lab is not a very tough lab, it only shows you how to transform
a language from text into a interpretable Abstract Syntax Tree (**AST**).

Finishing this is just like filling out the blanks, should be easy and chill,
probably you will even learn nothing!

But this would be the base of our interesting optimizations!

## Grammar Supported

We implements a very simple language named **SimpC**, with **no nested IfElse**,
**no function calls**, **no dynamic memory**, and **limited number of operators**.

Since the core of modern compiler design is not at its front end in my opinion,
we uses this simple language to simplify the frontend for you to
read through.

### Grammar

Below is a very typical BNF you would see in almost all programming
languages. But for this tutorial, I purposefully omitted some **important**
things for the grammar.

1.  No nested IfElse and every if must have an else branch -> easier lexer, you don't have to worry about lexer tricks.
2.  No function calls -> easier ABI handling, you don't need to care **too** much
    about the backend.
3.  No dynamic memory -> easier memory management
    1.  I plan to have a Garbage Collection Lab in the future.
4.  Limited number of operators -> easier implementation on the backend, we are
    not using this language anyways!

```
<program>  ::= int ident () <block> (type checker should check the ident is "main")
<block>    ::= { <stmts> }
<stmts>    ::= ϵ
           |   <stmt> <stmts>
<stmt>     ::= <simp> ;
           |   <block> ;
           |   <control> ;
<type>     ::= int | bool ;
<decl>     ::= <type> ident
           |   <type> ident = <exp>
<control>  ::= if ( <exp> ) <stmt> <else>
           |   while ( <exp> ) <stmt>
           |   for ( <simpopt> ; <exp> ; <simpopt> ) <stmt>
           |   return <exp> ;
<else>  ::= else { <stmt> }
<simp>     ::= <lvalue> <asnop> <exp> | <decl> | <exp>
<simpopt>  ::= ϵ | <simp>
<lvalue>   ::= ident
           |   ( <lvalue> )
<exp>      ::= ( <exp> ) | num | true | false | ident
           |   <unop> <exp> | <exp> <binop> <exp>
<asnop>    ::= = | += | -= | *= | /=
<binop>    ::= + | - | * | / | == | != | > | < | && | ||
<unop>     ::= ! | ~ | -
```

### Precedence and Associates

```
----------------------------------------
Op                            Associates
----------------------------------------
()                            N/A
! ~ -                         R
* /                           L
+ -                           L
> <                           L
&&                            L
||                            L
= += -= *= /=                 R
----------------------------------------
```

## Let's get started!
