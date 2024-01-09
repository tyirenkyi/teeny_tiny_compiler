## Teeny Tiny compiler

I built this compiler by following [Austin Henley's](https://github.com/AZHenley) Teeny Tiny compiler [blog series](https://austinhenley.com/blog/teenytinycompiler1.html) which is implemented in Python. The Teeny Tiny language is a dialect of BASIC. Teeny Tiny compiles to C code and this version of the compiler is implemented in Rust.

# Future Support
Here are a list of features I'll be adding to improve the Teeny Tiny language:
1. Parentheses for expressions
2. Logical operators (and, or, not)
3. ELSE IF and ELSE
4. FOR loop
5. Allow multiple code files
6. Functions with parameters and return values
7. Lexical scope
8. Standard library
8. Abstract syntax tree representation
9. More primitive types
10. Arrays
11. Type checking
12. More tests for the compiler


# Currently Supported
1. Numerical variables
2. Basic arithmetic
3. If statements
4. While loops
5. Print text and numbers
6. Input numbers
7. Labels and goto
8. Comments

# Instructions for use
1. Create a file with *teeny* as the file extension. eg: `hello.teeny`
2. Write your teeny program. eg program:
```
PRINT "How many fibonacci numbers do you want?"
INPUT nums

LET a = 0
LET b = 1
WHILE nums > 0 REPEAT
  PRINT a
  LET c = a + b
  LET a = b
  LET b = c
  LET nums = nums - 1
ENDWHILE	
```
3. Pass file to the compiler with `cargo run -- hello.teeny`
4. After successfully compiling to C code. You should find a C file called `out.c` in the root folder of the project.
5. You can compile that with [gcc](https://gcc.gnu.org) if you have it installed.
