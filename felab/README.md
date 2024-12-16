# Frontend Lab <span style="color:green">(EASY)</span>.

1. [Frontend Lab (EASY).](#frontend-lab-easy)
    1. [Introduction (Please read through)](#introduction-please-read-through)
        1. [Knowledge](#knowledge)
        2. [Grammar](#grammar)
            1. [BNF](#bnf)
            2. [Precedence and Associates](#precedence-and-associates)
        3. [Elaboration](#elaboration)
        4. [Typechecking](#typechecking)
        5. [Grammar Supported](#grammar-supported)
            1. [Grammar](#grammar-1)
            2. [Precedence and Associates](#precedence-and-associates-1)
    2. [Let's get started!](#lets-get-started)
        1. [Elaboration](#elaboration-1)
        2. [Typechecking](#typechecking-1)

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

### Elaboration

Elaboration makes statements simpler. Language designer normally provides
many syntaxs for programmers to use, but many of the syntaxs are semantically
the same.

For example,

1. `a += 1` is the same as `a = a + 1`.
2. `int a = &p` is the "same" as `int *a = &p`, but compiler needs type information
   indicating that a is a syntatic sugar reference.

But for compiler engineers, we want the AST to be as simple as possible. Therefore,
we add an extra pass to minimize the AST and make it easier to process in the
later pipeline stages.

In this lab, elboration does three things.

1. For all assignments that uses _AsnOp_, make it a simple assignments.
    1. e.g. `a += 1` -> `a = a + 1`
2. Eliminate for-loop and make it a while-loop, since essentially there is no difference
3. Providing scoping information for typechecker

    1. This is very handy since for an invalid program like below

        ```
        {
          int x = 1;
        }

        x = 2;
        ```

        The typechecker needs to know that the variable _x_ is no longer _defined_
        in later context.

To be more specific, you need to transform the `Program` provided in the AST
into `ElabProgram`.

Note that not doing elaboration is **TOTALLY FINE**, but it will be a pain in
the ass for you to typecheck and generate IR. So please do this, and it is
**REQUIRED** for you to move forward.

### Typechecking

Typechecker is the guard for defined program behavior. It rejects code that
is syntatically correct but semantically wrong.

For example, the program below can easier pass the parser, but is semantically
wrong since `x` is not _defined_ when used in statement `x = 2`

```
{
    int x = 1;
}

x = 2;
```

In compiler theory, typechecker checks a series of [inference rules](https://en.wikipedia.org/wiki/Rule_of_inference),
which gives you the _premises_ and _conclusions_ for each rule.

Take a simple rule as an example

-   Any assignment should have the same type for the left hand side and right hand side.
    -   This means `int x = 1;` could pass, since both are typed `int`.
    -   `int x = true;` could not pass, since lhs is `int` while rhs is `bool`.
-   Any used variable should be _defined_ before _use_.

    -   This means
        ```
        int y;
        int x = y;
        ```
        cannot pass since `y` is not _defined_ but only _declared_ before its usage.
    -   But
        ```
        int y = 0;
        int x = y;
        ```
        could pass since `y` is _defined_ before assigned to `x`.

-   The expression inside the `if` clause should be typed `bool`
