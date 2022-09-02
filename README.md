# FFP

WIP...

Just **Full Functional Programming** supports for Rust.

## Features

These are just early features, more will be added to this library.

### Function operations
Allows you to operate some function in a convenient way.

#### `FnWrapper`
`FnWrapper` is the core type in this feature, it wraps a function, contains some utilities.

- `struct FnWrapper<I, O> { ... }`
I is *Input*, O is *Output*

- `fn map<B>(self: FnWrapper<I, A>, map_fn: Fn(A) -> B) -> FnWrapper<B>`
A combinator that return a new wrapper that map the output A into type B.

### Category
WIP

### Currying
WIP

