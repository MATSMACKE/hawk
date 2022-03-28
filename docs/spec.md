# Hawk
## A programming language for physicists

### Introduction

Hawk is a language designed to help physicists automate tasks, such as performing calculations or processing data. As such, it has features (e.g. built in uncertainties) that will make these jobs easier.

Please note: Until 1.0.0, not all features in this spec are guaranteed to be implemented
### Basic Syntax

Hawk has a C-style syntax, and is oftentimes similar to ES6. The syntax is very permissive. There are no semicolons and not even newlines are required, although they certainly help with code style and readability.

#### Statements

Statements in Hawk can either be a single statement using a keyword such as `let`, an expression statement, or a block (multiple statements encased in `{}`).

#### Variables

##### Declare mutable

`let var = val`

#### Conditionals:

##### With blocks
```
if condition {
    // code block
}
else {
    // code block
}
```
##### For single statements
```
if condition statement
else statement
```
As with other languages, `else` is optional. `else if` is not hardcoded into the language itself but will work as a byproduct of the way `if` and `else` work.

##### Unless (not implemented)

```
unless condition statement
unless condition {
    // block
}
```
Using `if` is generally preferred in keeping with programming traditions, however, `unless` might be more readable in certain circumstances. 

#### Loops

Loops are similar to those in Rust.

##### Loop until `break`:
```
loop {
    // code

    if /* condition */ break
}
```
Hawk will accept an infinite loop (without `break`), but this is not recommended. 

##### Loop while
```
while condition statement
```

#### Finders

##### Finders
To define a finder:

```
finder force {
    equation f = m * a
}
```

Then to use it:
```
>> print find force (f: 3, m?, a: 1.5)
2
```

#### Testing and Peace of Mind

##### Expect

The `expect` keyword (synonomous with `assert`) exists to give Hawk programmers some peace of mind. Using expect tests to make sure operations worked as expected.

If the expression after `expect` evaluates to true, nothing happens and the program happily goes about its day.

```
expect 1 == 1
```

If the expression evaluates to false, a warning will be thrown
```
>> expect 1 == 2
Warning: Expect failed (expected 1 == 2)
```



#### Building projects

##### Importing files

```
import "filename.hawk"
```

### The Physics Engine

#### Physics data types

##### Uncertain

Uncertain is a data type that contains two floating point numbers. A value and an uncertainty. 

To use uncertain with a ± key:

`let var = 1.2 ± 0.2`

To use uncertain without a ± key:

`let var = 1.2 +- 0.2`

When doing calculations with uncertains, the value will be calculated as expected and the uncertainty will also be updated to be correct as per the calculations that have been performed.

#### Finding unknowns with built in finders (not implemented)

##### suvat
```
let velocity = suvat(s: 5, u: 10, a: 1, v?)