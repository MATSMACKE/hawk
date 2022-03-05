# Hawk
## A programming language for physicists

### Introduction

Hawk is a language designed to help physicists automate tasks, such as performing calculations or processing data. As such, it has a lot of features (e.g. built in uncertainties) that will make these jobs easier.

Please note until 1.0.0 not all features in this spec are guaranteed to be implemented
### Basic Syntax

Hawk's basic syntax is akin to that of ES6 or sometimes Rust. The goal with the syntax is to make it as easy as possible to understand and reason about. It uses newlines for line endings (no semicolons).

#### Variables

##### Declare mutable

`let var = val`


##### Declare constant

`const var = val`

#### Conditionals:

##### For single line code
```
if condition // code
else if condition // code
else // code
```
##### For multi-line code
```
if condition {
    // code
} else if condition {
    // code
}
else {
    // code
}
```
As with other languages, `else if` and `else` are optional.

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
while condition {
    // code
}
```

##### C-style for loop
```
for (let i = 0; i < num_loops; i++) {
    // code
}
```

##### Loop through iterables or iterators
```
for val in iterable {
    // code
}

for (index, val) in iterable {
    // code
}
```

#### Classes

##### Create class
```
class ClassName {
    property
    other_property = default_val

    constructor()
}
```

#### Iterators

```
iter IteratorName {
    get start(seed) {
        // initialize initial_value
        return // initial_value
    }

    get next() {
        // calculate next_value
        return // next_value
    }
}
```

#### Finders

##### Finders with cases
```
finder suvat(s, u, v, a ,t) {
    find s given (u, v, t) = t * (u + v) / 2
}
```

##### Finders with equations
```
finder suvat(s, u, v, a, t) {
    equation v = u + a * t
    equation v ** 2 = u ** 2 + 2 * a * s
    equation s = u * t + (1/2) * a * t ** 2
    equation s = t * (u + v) / 2
}
```

Please note the two kinds of finders can be mixed and matched. If they are combined, cases will be preferred and general solutions will be a fallback. 

### The Physics Engine

#### Physics data types

##### Uncertain

Uncertain is a data type that contains two floating point numbers. A value and an uncertainty. 

To use uncertain with a ± key:

`let var = 1.2 ± 0.2`

To use uncertain without a ± key:
`let var = 1.2 +- 0.2`

When doing calculations with uncertains, the value will be calculated as expected and the uncertainty will also be updated to be correct as per the calculations that have been performed.

#### Finding unknowns with built in finders

##### suvat
```
let velocity = suvat(s: 5, u: 10, a: 1, v?)