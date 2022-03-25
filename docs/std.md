# Standard Library

The Hawk standard library is composed of many commonly used functions, as well as functions that provide IO capabilities to the Hawk language. 

### Math


### Utilities

#### Conversion

Convert to a different type, error if not possible.

`str()`

#### Type checking

These functions return a boolean representing whether or not the argument is the given type. Quite self-explanatory.

Please note different kinds of formatting are allowed (e.g. `isfloat()`, `is_float()` and `isFloat()` all do the same thing)

`is_float()`
`is_int()`
`is_bool()`
`is_uncertain()` or `has_uncertainty()`
`is_string()` or `is_str()`