# Frontend Lab <span style="color:green">(EASY)</span>.

1. [Frontend Lab (EASY).](#frontend-lab-easy)
    1. [Introduction (Please read through)](#introduction-please-read-through)
        1. [Knowledge](#knowledge)
            1. [Abstract Syntax Tree (AST)](#abstract-syntax-tree-ast)
            2. [Grammar](#grammar)
                1. [BNF](#bnf)
                2. [Precedence and Associates](#precedence-and-associates)
            3. [Elaboration](#elaboration)
            4. [Typechecking](#typechecking)
        2. [Grammar Supported](#grammar-supported)
            1. [Grammar](#grammar-1)
            2. [Precedence and Associates](#precedence-and-associates-1)
    2. [Let's get started!](#lets-get-started)
        1. [Elaboration](#elaboration-1)
        2. [Typechecking](#typechecking-1)

## Introduction (Please read through)

In this lab, you will implement the frontend for a simple educational language called SimpC.
The frontend includes both an elaboration phase and a typechecking phase. We have already provided the lexer and parser, so your primary tasks are:

1.  Elaboration: Transform the parsed Abstract Syntax Tree (AST) into a simpler, more uniform AST (which we call ElabProgram).
2.  Typechecking: Validate that the program is semantically correct (e.g., variables are defined before use, types match up, etc.).

**Important Notes:**

1. We will not provide reference solutions, as many universities have similar labs and we want to avoid plagiarism.
2. We strongly discourage posting your solution online (such as GitHub) to prevent cheating.
3. We encourage high-level discussion and conceptual explanations online, as it aids collective learning.

**Recommended workflow:**

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

> The knowledge is not introduced **formally** and **100% accurate**, so please
> refer to the link provided in each section if you want to learn more.

#### Abstract Syntax Tree (AST)

[AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) is a tree-like representation of a program’s structure after parsing. Each node represents a language construct, such as an expression or a statement.

For example, consider the expression `(a + b) + b` (without a trailing semicolon, so it's just an expression, not a statement):

Will be parsed into AST in the following form

```
Expr::BinOp(
    Expr::BinOp(Expr::Ident("a"), "+", Expr::Ident("b")),
    "+",
    Expr::Ident("b")
)
```

Notice the **recursive** nature of the AST: expressions contain other expressions.
Understanding this recursion is essential for elaboration and typechecking.

So spent some time on this if you do not understand it.

#### Grammar

##### BNF

[BNF](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form) is a classic way to describe a syntax of a formal language.

It is written as `<symbol> ::= __expression__`, on the left hand side it is normally a
**non-terminal** symbol and symbol `::=` means left hand side **must** be replaced by
the right hand side. On the right hand side it is normally **terminal** or **non-terminal**
symbols, using `"|"` indicating choices.

For example,

```
<stmts>    ::= ϵ
           |   <stmt> <stmts>
<stmt>     ::= <simp> ;
           |   <block> ;
           |   <control> ;
```

This means stmts can be empty (ϵ) or a <stmt> followed by more <stmts>. Similarly, stmt could be a simp, block, or control statement, each ending with a semicolon.

In this lab, we give you the BNF for SimpC to understand what the language constructs look like and how they form a valid program.

##### Precedence and Associates

Precedence and associates is crucial for compiler **parser**. They specify how
the parser is going to interpret the tokens and form AST.

Operator precedence and associativity rules determine how operators group within expressions (e.g., \* has higher precedence than +, and - is left-associative). We specify these rules to ensure the parser generates a consistent AST for expressions.

#### Elaboration

Elaboration simplifies complex language constructs into simpler ones. Although we haven't fully detailed it yet, elaboration is the next major step after parsing to “normalize” the AST for easier typechecking and further compilation.

For example, a elaboration pass could make a ternary expression `a | b` into
`a ? True : b` to make sure short-cut of logical or work.

#### Typechecking

Typechecking ensures that the program is semantically correct. Even if the program
s syntactically valid (it parsed correctly), it may still be semantically invalid
(for example, using a variable before defining it).

### Grammar Supported

#### Grammar

Below is a very typical BNF you would see in almost all programming
languages. But for this tutorial, I purposefully omitted some **important**
things for the grammar. And it is a **[context-free grammar](https://en.wikipedia.org/wiki/Context-free_grammar)**, i am not
elaborating on this now, but please take a look if you are curious.

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
    - e.g. `a += 1` -> `a = a + 1`
2. Eliminate for-loop and make it a while-loop, since essentially there is no difference
    - Think of how a `for (init; cond; incr) body` should look like in a `while-loop` form
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

Take some simple rules as an example

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

**You are required to implement the following rules for your typechecker**
Below are the inference rules in human language that you are required to implement.

1. The function entry point is named "main".
2. The final return value of "main" is typed `int`.
3. All variable are _defined_ before _used_ (not _declared_).
4. All variable can only be _declared_ **or** _defined_ once.
5. For assignments, left hand side and right hand side need to have the same type.
6. For if/while, the expression for loop condition needs to have type `bool`.
7. Numbers are typed `int`, `true` and `false` are typed `bool`.
8. Unary operators need to operate on the correct type, for example `!` needs
   to operate on type `bool`.
9. Binary operation `a op b` requires `a` and `b` to have the same type.
10. Binary operators need to operate on the correct type, fpr example `<` needs
    to operate on type `int`.
    - If you want to implement bit-wise operators such as `&`, they need to
      operate on type `bool`.

**Hint(s)**

1. For `IfElse`, the defined variable are considered as the **intersection** of
   both branches.
2. Remember AST has a recursive (tree-like) nature, and our provided function
   signatures (such as `tc_stmt()`) reflect that.
3. Keep track of variable definitions in a scoped symbol table so that out-of-scope variables can be easily detected.

---

In short:

**Parsing**: Converts source code into an AST.
**Elaboration**: Simplifies and normalizes the AST (flattening loops, normalizing assignments, establishing clear scoping).
**Typechecking**: Validates that the simplified AST follows type and definition rules.

We encourage you to revisit theoretical concepts if you find anything unclear, and to proceed step-by-step. Good luck!
