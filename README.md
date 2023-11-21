# rust-primitive_enum
Little utility for dealing with C-style enums

This crate exports just the single macro `primitive_enum!`
that defines an enum backed by a user specified primitive
integer type.

The intent is to emulate traditional C-style enums while
adding some handy associated functions useful in such
contexts (e.g. enumerating over each enum and converting
between the underlying types).

# Example

```rust
#[macro_use] extern crate primitive_enum;

primitive_enum! { MyEnum u16 ;
    A,
    B,
    C,
    D = 500,
    E,       // as you would expect, E maps to 501
}

fn main() {
    use MyEnum::*;

    // Get a slice of all enum elements:
    assert_eq!(
        MyEnum::list(),
        &[A, B, C, D, E],
    );

    // Get the enum value given its integer value:
    assert_eq!(MyEnum::from(0), Some(A));
    assert_eq!(MyEnum::from(1000), None);

    // User specified enum values behave as you would expect
    assert_eq!(D as u16, 500);
    assert_eq!(MyEnum::from(501), Some(E));

    // You can also get an enum by its name
    assert_eq!(MyEnum::from_name("E"), Some(E));
}
```

# Expansion

As of the current version, the macro

```rust
#[macro_use] extern crate primitive_enum;

primitive_enum! { MyEnum u16 ;
    A,
    B,
    C,
    D = 500,
    E,
}

```

is effectively equivalent to

```rust
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MyEnum {
    A = 0,
    B = 1,
    C = 2,
    D = 500,
    E = 501,
}

impl MyEnum {
    pub fn from(x: u16) -> Option<MyEnum> {
        // ...
        None
    }

    pub fn from_name(name: &str) -> Option<MyEnum> {
        // ...
        None
    }

    pub fn list() -> &'static [MyEnum] {
        &[
            MyEnum::A,
            MyEnum::B,
            MyEnum::C,
            MyEnum::D,
            MyEnum::E,
        ]
    }
}
```

# Doc comments

Starting from version 1.1.0, doc comments are supported.

```rust
primitive_enum! {
/// Some comments about 'MyEnum'
MyEnum u16 ;
    A,
    B,

    /// Some special comments about variant C
    C,
    D = 500,
    E,
}
```

Starting from version 1.1.0 this crate is implemented as a procedural macro
to improve space efficiency of the generated code.
Prior to version 1.1.0, this crate was implemented as a simple declarative macro.

# Default trait

Originally, enums did not automatically get the `Default` trait. But starting from version `1.2.0`
the enum will automatically derive `Default` if you specify `#[default]`.

So for example, given

```rust
#[macro_use] extern crate primitive_enum;

primitive_enum! {
EnumWithDefault u16 ;
    A,
    B,
    #[default]
    C,
    D,
}

fn main() {
    assert_eq!(EnumWithDefault::default(), EnumWithDefault::C);
}
```

the resulting code is effectively eqivalent to

```rust
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnumWithDefault {
    A = 0,
    B = 1,
    #[default]
    C = 2,
    D = 3,
}

impl EnumWithDefault {
    // ... (same as with the other example above)
}

fn main() {
    assert_eq!(EnumWithDefault::default(), EnumWithDefault::C);
}
```

This crate is a clean macro implementation that
expands to code shown above and doesn't rely on any
outside dependencies or magic.
