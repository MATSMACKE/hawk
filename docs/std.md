# Standard Library

The Hawk standard library is composed of many commonly used functions, as well as functions that provide IO capabilities to the Hawk language. 

#### Array operations

`len(arr)` returns length of array

`sort(arr)` sorts the array with selection sort

### Math

#### Constants

`pi()`, `e()`, `ln10`

#### General

`mod(x, y)`

`gcd(x, y)` returns greatest common denominator of x and y

`lcm(x, y)` return lowest common multiple of x and y

`ln(x)`, `log(base, x)`

#### Statistics



#### Trigonometry

The obvious are included:

`sin(x)`, `cos(x)`, `tan(x)`

As well as inverse trig:

`csc(x)`, `sec(x)`, `cot(x)`

And hyperbolic trig:

`sinh(x)`, `cosh(x)`, `tanh(x)`

#### Combinatorics

Once again, we have the obvious:

`factorial()`, `permutation(n, r)`, `combination(n, r)`

### Utilities

#### File IO

Read and write CSV data tables: 

`read("filename")`, `write("filename", datatable)`

Please not that the above is automated using `process`


Read and write plain text files:

`readfile("filename")`, `writefile("filename", string)`

#### Conversion

Convert to a different type, error if not possible.

`str(x)`

#### Type checking

These functions return a boolean representing whether or not the argument is the given type. Quite self-explanatory.

Please note different kinds of formatting are allowed (e.g. `isfloat(x)`, `is_float(x)` and `isFloat(x)` all do the same thing)

`is_float(x)`, `is_int(x)`, `is_bool(x)`, `is_uncertain(x)` or `has_uncertainty(x)` (not yet working), `is_string(x)` or `is_str(x)`, `is_array(x)` or `is_arr(x)`, `is_null(x)`