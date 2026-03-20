# Phase 4: Complex Code Generation

## Overview

Now that the lexer, parser, and simple code generation are complete, we now move on to 
**complex code generation**. In this phase, programs will no longer execute strictly in a 
linear fashion. Instead, execution may branch and loop depending on conditions.

In Phase 4, you will generate IR for control flow constructs such as:
- While loops
- If and If/Else statements
- Break and Continue
- Nested loops

You will also continue performing **semantic error checking**.

## What is Complex Code Generation?

In high-level languages, constructs like `while` loops and `if` statements are used to control flow. 
However, these constructs do not exist in hardware. Instead, they must be translated into:
- Labels
- Conditional branches
- Unconditional jumps

Your goal is to convert structured programming constructs into low-level IR instructions 
that simulate the same behavior.

### Output 
In the previous Phase 3 of the project, you generated the corresponding intermediate code for the 
given program. The generated code will be executed using the provided interpreter. You will do the 
same for Phase 4, except generated code will now need to handle “while loops” and “if statements.”
One should be able to nest loops inside loops, and nest if statements within if statements. A user 
program should be able to define multiple loops within a program. In an if/else statement, if the if 
condition results in true, the if code block should be executed. And if the if condition results in 
false, the else code block should be executed.

You should perform one-pass code generation and directly output the generated code. There is no 
need to build or traverse a syntax tree. However, you will need to maintain a symbol table during 
code generation.

## Error handling
If a “break” or “continue” is placed outside of a loop, the compiler should treat that as an error. 
“break” and “continue” can only be used inside loops, and therefore any “break” or “continue” 
statement outside a loop should be treated as an error. If an error occurs, do not generate any code, 
instead, print out an error messsage such as “Error: break statement is outside a loop”.

**The code generator may halt compilation on the first error message.** If there is a user error in 
the high-level code, the code generator should not output any code. The compiler may optionally 
catch additional error messages.

## IR Syntax and Semantics

The following instructions are used for branching and looping:

| Instruction               | Description                                                                      |
|---------------------------|----------------------------------------------------------------------------------|
| :label                    | declares a label ':label'. Used in branching code                                |
| %jmp  :label              | jumps to ':label' unconditionally                                                |
| %branch_if var, :label    | jumps to ':label' if var is 1. Does nothing if var is 0                          |
| %branch_ifn var, :label   | jumps to ':label' if var is 0. Does nothing if var is 1                          |

Use `:label` to declare a point that someone can jump to. `%jmp` can be used to jump to the `:label`.

By generating the following code, you can create an infinite loop:

```
%func main() 
:label
%out 0
%jmp :label
%endfunc
```

When running the IR, it should continue to print 0 until infinity.

Use `%branch_if` to jump to the `:label` only if the `var` is 1. `%branch_if` will do nothing if
`var` is 0.

`%branch_ifn` is the opposite: it only jumps to the `:label` if the `var` is 0. `%branch_ifn` will do
nothing if the `var` is 1.

IR instructions take up exactly one line per instruction. You cannot output multiple IR instructions on a single line. 
Anything after the semicolon `;` will be treated as a comment.
The semicolon denotes a comment that goes until the end of the line.
```
%add c, a, b; adding 'a' and 'b' to get 'c'
```

## Translating While Loops

A `while` loop consists of:
1. A loop start label
2. A condition check
3. A branch to exit if false
4. Loop body
5. A jump back to the start

### Example

High-level:
```
while i < 10 {
  print(i);
  i = i + 1;
}
```
IR:
```
:loopbegin1
%int _temp1
%lt _temp1, i, 10
%branch_ifn _temp1, :endloop1
%out i
%int _temp2
%add _temp2, i, 1
%mov i, _temp2
%jmp :loopbegin1
:endloop1
```
## Translating If / Else Statements

An `if` statement evaluates a condition and branches accordingly.

### Example

High-level:
```
if a < b {
  c = 0;
} else {
  c = 1;
}
```
IR:
```
%int _temp1
%lt _temp1, a, b
%branch_if _temp1, :iftrue1
%jmp :else1
:iftrue1
%mov c, 0
%jmp :endif1
:else1
%mov c, 1
:endif1
```
## Nested Control Flow

Loops and if-statements can be nested inside one another. Each structure must use **unique labels**.

Example structure:
```
:loopbegin1
...
:loopbegin2
...
:endloop2
...
:endloop1
```
## Break and Continue

### Break

A `break` exits the nearest loop.

IR:
```
%jmp :endloopX
```
### Continue

A `continue` skips to the next iteration of the loop.

IR:
```
%jmp :loopbeginX
```
To implement this correctly, you must maintain a **loop stack** that tracks:
- Loop start labels
- Loop end labels

## Requirements

Your code generator must:

- Correctly translate:
  - While loops
  - If / Else statements
  - Nested control flow
  - Break and Continue
- Generate valid IR using labels and branches
- Maintain proper control flow
- Perform semantic error checking
- Ensure all labels are unique


## Generated Example IR Code

Here are some examples of possible generated IR outputs. One can generate any IR code for the given code, as
long as the generated IR functions in the same way. **Any IR generated is acceptable, as long as it outputs
the same numbers**.

#### loop

Given a simple loop `loop.min`:
```
func main() {
    int i;
    i = 0;
    while i < 10 {
        print(i);
        i = i + 1;
    }
}
```
You can generate the follow IR. Note that `%jmp :loopbegin1` is used to jump to the beginning of the loop,
marked `:loopbegin1`. We break out of the loop using `%branch_ifn _temp, :endloop1` to break out of the loop.
```
%func main()
%int i
%mov i, 0
:loopbegin1
%int _temp1
%lt _temp1, i, 10
%branch_ifn _temp1, :endloop1
%out i
%int _temp2
%add _temp2, i, 1
%mov i, _temp2
%jmp :loopbegin1
:endloop1
%endfunc
```
The output of the `loop.tt` IR should be:
```
0
1
2
3
4
5
6
7
8
9
```
---
Given the following `if.tt` program:
```
func main() {
    int a;
    int b;
    int c;

    
    a = 100;
    b = 50;
    if a < b {
        c = 0;
    } else {
        c = 1;
    }

    # Should print out '1'.
    print(c);



    a = 100;
    b = 50;
    if a >= b {
        c = 0;
    } else {
        c = 1;
    }

    # Should print out '0'
    print(c);
}
```

The following IR can be generated:
```
%func main()
%int a
%int b
%int c
%mov a, 100
%mov b, 50
%int _temp1
%lt _temp1, a, b
%branch_if _temp1, :iftrue1
%jmp :else1
:iftrue1
%mov c, 0
%jmp :endif1
:else1
%mov c, 1
:endif1
%out c
%mov a, 100
%mov b, 50
%int _temp2
%ge _temp2, a, b
%branch_if _temp2, :iftrue2
%jmp :else2
:iftrue2
%mov c, 0
%jmp :endif2
:else2
%mov c, 1
:endif2
%out c
%endfunc
```

The output of `if.tt` is the following:
```
1
0
```

---
Given the following `nested_loop.tt` program:
```
func main() {
    int i;
    int j;
    i = 0;
    while i < 2 {
        j = 0;
        while j < 3 {
            print(j);
            j = j + 1;
        }
        i = i + 1;
    }
}
```

For `nested_loop.tt`, the following IR is valid:
```
%func main()
%int i
%int j
%mov i, 0
:loopbegin1
%int _temp1
%lt _temp1, i, 2
%branch_ifn _temp1, :endloop1
%mov j, 0
:loopbegin2
%int _temp2
%lt _temp2, j, 3
%branch_ifn _temp2, :endloop2
%out j
%int _temp3
%add _temp3, j, 1
%mov j, _temp3
%jmp :loopbegin2
:endloop2
%int _temp4
%add _temp4, i, 1
%mov i, _temp4
%jmp :loopbegin1
:endloop1
%endfunc
```

The output of the `nested_loop.tt` IR should be:
```
0
1
2
0
1
2
```
---

Given the following `break.tt` high level language code:

```
func main() {
    int i;
    i = 0;
    while i < 10 {
        if i >= 4 {
            break;
        }
        print(i);
        i = i + 1;
    }
}
```

You can generate the following IR. The IR uses `%jmp :endloop1` to implement a break statement.

```
%func main()
%int i
%mov i, 0
:loopbegin1
%int _temp1
%lt _temp1, i, 10
%branch_ifn _temp1, :endloop1
%int _temp2
%ge _temp2, i, 4
%branch_if _temp2, :iftrue1
%jmp :endif1
:iftrue1
%jmp :endloop1
:endif1
%out i
%int _temp3
%add _temp3, i, 1
%mov i, _temp3
%jmp :loopbegin1
:endloop1
%endfunc
```

The IR code when executed should produce the following result:

```
0
1
2
3
```

---
Given the following `primes.tt` program (this is an extraneous test case that you do not need to handle):
```
func main() {

    int[100] primes;
    int i = 0;

    # initialization of the array.
    while i < 100 {
        primes[i] = 0;
        i = i + 1;
    }

    # compute primes using the Sieve of Eratosthenes
    # compute primes up to 100 by crossing out multiples of
    # successively larger primes in a boolean array.

    i = 2;
    while i < 10 {
        if primes[i] == 0 {
            int j = i + i;
            while j < 100 {
                primes[j] = 1;
                j = j + i;
            }
        }
        i = i + 1;
    }

    # print all primes from 1 to 100.
    i = 2;
    while i < 100 {
        if primes[i] == 0 {
            print(i);
        }
        i = i + 1;
    }
    
}
```

The generated bytecode should be:
```
%func main()
%int[] primes, 100
%int i
%mov i, 0
:loopbegin1
%int _temp1
%lt _temp1, i, 100
%branch_ifn _temp1, :endloop1
%mov [primes + i], 0
%int _temp2
%add _temp2, i, 1
%mov i, _temp2
%jmp :loopbegin1
:endloop1
%mov i, 2
:loopbegin2
%int _temp3
%lt _temp3, i, 10
%branch_ifn _temp3, :endloop2
%int _temp4
%mov _temp4, [primes + i]
%int _temp5
%eq _temp5, _temp4, 0
%branch_if _temp5, :iftrue1
%jmp :endif1
:iftrue1
%int _temp6
%add _temp6, i, i
%int j
%mov j, _temp6
:loopbegin3
%int _temp7
%lt _temp7, j, 100
%branch_ifn _temp7, :endloop3
%mov [primes + j], 1
%int _temp8
%add _temp8, j, i
%mov j, _temp8
%jmp :loopbegin3
:endloop3
:endif1
%int _temp9
%add _temp9, i, 1
%mov i, _temp9
%jmp :loopbegin2
:endloop2
%mov i, 2
:loopbegin4
%int _temp10
%lt _temp10, i, 100
%branch_ifn _temp10, :endloop4
%int _temp11
%mov _temp11, [primes + i]
%int _temp12
%eq _temp12, _temp11, 0
%branch_if _temp12, :iftrue2
%jmp :endif2
:iftrue2
%out i
:endif2
%int _temp13
%add _temp13, i, 1
%mov i, _temp13
%jmp :loopbegin4
:endloop4
%endfunc
```
The output of primes should be:
```
2
3
5
7
11
13
17
19
23
29
31
37
41
43
47
53
59
61
67
71
73
79
83
89
97
```

## Semantic Error Checking

In addition to doing code generation for if statements, while loops, and branching statements in general, you
must also do error checking for 'break' and 'continue' outside of a loop.

Invalid input code such as the following should result in a compile-time compiler error.
```
func main() {
    int i;
    i = 0;
    break;  # Error. Used 'break' outside of a loop.
    while i < 10 {
        print(i);
        i = i + 1;
    }
}
```
All projects can be turned in up to 1 week late. Each day the project is late, 3% will be deducted per
day for up to 21%. After a week, projects will not be accepted.
