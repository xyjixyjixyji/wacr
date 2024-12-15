# Frontend Lab <span style="color:green">(EASY)</span>.

[toc]

In this tutorial, we are going to implement a compiler for language **SimpC**,
which is a very simple language for purely educational purpose.

First of all, we are **NOT** planning to release the solution for the labs, since most universities
have labs similar to this structure.

We also **STRONGLY DISCOURAGE** you to release your solution on GitHub, students
might look at your solution and copy it!

But we **ENCOURAGE** you to discuss your solution in a high level way online,
which aids learning in general.

## Introduction (Please read through)

In this lab, you will implement the **frontend** for your **SimpC** compiler,
we have written the **lexer and parser** for you!

Your work will contains two part, _AST elaboration_ and _type checking_.

We recommend you to do this lab in the following order:

1. Reads the front end part of the compiler (_scc/src/compiler/frontend_),
   especially take a look at the **Compiler trait**.
2. Implement Elaboration (_scc/src/compiler/frontend/parse/elab.rs_), the
   requirements will be described later.
3. Implement Typechecking (_scc/src/compiler/frontend/typecheck.rs_), the
   requirements will be described later.

However, please know that you are **NOT** required to follow the structure
we provided to you. You are solely on your own! But please pay attention that
the future labs **WILL** assume that it follows similar structures.

### Knowledge

In this section, we are going to go through some of the necessary knowledge
for the frontend lab.

### Grammar

#### BNF

todo

#### Precedence and Associates

todo

### Elaboration

todo

### Typechecking

todo

### Grammar Supported

#### Grammar

Below is a very typical BNF you would see in almost all programming
languages. But for this tutorial, I purposefully omitted some **important**
things for the grammar.

1.  No nested IfElse and every if must have an else branch -> easier lexer, you don't have to worry about lexer tricks.
2.  No function calls -> easier ABI handling, you don't need to care **too** much
    about the backend.
3.  No dynamic memory -> easier memory management
    1.  I plan to have a Garbage Collection Lab in the future.

Also, we purposefully omitted operators like **&, |, ^, <<, >>**, this makes
it not fully-functional, but reduce the work you are going to do when
implementing the backend. If you want to add them, you are free to modify
the frontend to support it, it should be **fairly easy to do**.

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

#### Precedence and Associates

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
