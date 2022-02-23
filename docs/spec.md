# Hawk
## A language for scripts, calculations and physics simulations

### Basic Syntax

Hawk's basic syntax is akin to that of Rust, Swift and TypeScript. It uses newlines for line endings.

#### Variables

##### Declare mutable

`let var = val`;

##### With type annotation

`let var: type = val`

##### Declare constant

```
const var = val
const var: type = val
```

#### Conditionals:

##### For single line code
```
if condition code
else if condition code
else code
```
##### For multi-line code
```
if condition {
    code
} else if condition {
    code
}
else {
    code
}
```
As with other languages, `else if` and `else` are optional.

#### Loops

Loops are similar to those in Rust.

##### Loop until `break`:
```
loop {
    code

    if condition break
}
```
Hawk will accept an infinite loop (without `break`), but this is not recommended. 

##### Loop while
```
while condition {
    code
}
```

##### C-style for loop
```
for (let i = 0; i < num_loops; i++) {
    code
}
```

##### Loop through array
```
for val in arr {
    code
}

for (index, val) in arr {
    code
}
```
