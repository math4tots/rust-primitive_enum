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

```
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

```
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

```
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

This crate is a clean macro implementation that
expands to code shown above and doesn't rely on any
outside dependencies or magic.
