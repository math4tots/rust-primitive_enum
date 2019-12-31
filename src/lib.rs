//! This crate exports just the single macro `primitive_enum!`
//! that defines an enum backed by a user specified primitive
//! integer type.
//!
//! The intent is to emulate traditional C-style enums while
//! adding some handy associated functions useful in such
//! contexts (e.g. enumerating over each enum and converting
//! between the underlying types).
//!
//! # Example
//!
//! ```
//! #[macro_use] extern crate primitive_enum;
//!
//! primitive_enum! { MyEnum u16 ;
//!     A,
//!     B,
//!     C,
//!     D = 500,
//!     E,       // as you would expect, E maps to 501
//! }
//!
//! fn main() {
//!     use MyEnum::*;
//!
//!     // Get a slice of all enum elements:
//!     assert_eq!(
//!         MyEnum::list(),
//!         &[A, B, C, D, E],
//!     );
//!
//!     // Get the enum value given its integer value:
//!     assert_eq!(MyEnum::from(0), Some(A));
//!     assert_eq!(MyEnum::from(1000), None);
//!
//!     // User specified enum values behave as you would expect
//!     assert_eq!(D as u16, 500);
//!     assert_eq!(MyEnum::from(501), Some(E));
//!
//!     // You can also get an enum by its name
//!     assert_eq!(MyEnum::from_name("E"), Some(E));
//! }
//! ```
//!
//! # Expansion
//!
//! As of the current version, the macro
//!
//! ```
//! #[macro_use] extern crate primitive_enum;
//!
//! primitive_enum! { MyEnum u16 ;
//!     A,
//!     B,
//!     C,
//!     D = 500,
//!     E,
//! }
//!
//! ```
//!
//! is effectively equivalent to
//!
//! ```
//! #[repr(u16)]
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
//! pub enum MyEnum {
//!     A = 0,
//!     B = 1,
//!     C = 2,
//!     D = 500,
//!     E = 501,
//! }
//!
//! impl MyEnum {
//!     pub fn from(x: u16) -> Option<MyEnum> {
//!         // ...
//!         None
//!     }
//!
//!     pub fn from_name(name: &str) -> Option<MyEnum> {
//!         // ...
//!         None
//!     }
//!
//!     pub fn list() -> &'static [MyEnum] {
//!         &[
//!             MyEnum::A,
//!             MyEnum::B,
//!             MyEnum::C,
//!             MyEnum::D,
//!             MyEnum::E,
//!         ]
//!     }
//! }
//! ```
//!
//! This crate is a clean macro implementation that
//! expands to code shown above and doesn't rely on any
//! outside dependencies or magic.
//!

#[macro_export]
macro_rules! primitive_enum {
    (
        $name:ident
        $repr:ident ;
        $( $field:ident $( = $expr:expr )? , )*
    ) => {
        primitive_enum!(@imp make_enum $name $repr ; $( $( ( $expr ) )? $field )* );
        primitive_enum!(@imp make_from $name $repr ; $( $( ( $expr ) )? $field )* );
        primitive_enum!(@imp make_from_name $name ; $( $field )* );
        primitive_enum!(@imp make_list $name ; $( $field )* );
    };

    (@imp make_enum $name:ident $repr:ident ; $( $field:tt )* ) => {
        primitive_enum!(@imp make_enum_impl $name $repr ; 0 [ ] [ $( $field )* ]);
    };

    (@imp make_enum_impl $name:ident $repr:ident ; $count:tt [ $( $out:tt )* ] [ ] ) => {
        #[repr($repr)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $out )*
        }
    };

    (@imp make_enum_impl
            $name:ident
            $repr:ident ;
            $count:tt
            [ $( $out:tt )* ]
            [ $( ( $expr:expr ) )? $t:ident $( $in:tt )* ]) => {
        primitive_enum!(@imp make_enum_impl
            $name
            $repr ;
            (1 + ($count $( - $count + $expr )?))
            [ $( $out )* $t = ($count $( - $count + $expr )?) , ]
            [ $( $in )* ]
        );
    };

    (@imp make_from $name:ident $repr:ident ; $( $field:tt )* ) => {
        primitive_enum!(@imp make_from_impl $name arg $repr ; 0 [ ] [ $( $field )* ]);
    };

    (@imp make_from_impl
            $name:ident
            $arg:ident
            $repr:ident ;
            $count:tt
            [ $( $out:tt )* ]
            [ ]) => {
        impl $name {
            #[allow(dead_code)]
            pub fn from($arg: $repr) -> ::std::option::Option<$name> {
                $( $out )*
                None
            }
        }
    };

    (@imp make_from_impl
            $name:ident
            $arg:ident
            $repr:ident ;
            $count:tt
            [ $( $out:tt )* ]
            [ $( ( $expr:expr ) )? $t:ident $( $in:tt )* ]) => {
        primitive_enum!(@imp make_from_impl
            $name
            $arg
            $repr ;
            (1 + ($count $( - $count + $expr )?))
            [
                $( $out )*
                if $arg == ($count $( - $count + $expr )?) {
                    return ::std::option::Option::Some($name::$t);
                }
            ]
            [ $( $in )* ]
        );
    };

    (@imp make_from_name $name:ident ; $( $field:ident )* ) => {
        impl $name {
            #[allow(dead_code)]
            pub fn from_name(name: &str) -> Option<$name> {
                match name {
                    $(
                        stringify!($field) => Some($name::$field),
                    )*
                    _ => None,
                }
            }
        }
    };

    (@imp make_list $name:ident ; $( $field:ident )* ) => {
        impl $name {
            #[allow(dead_code)]
            pub fn list() -> &'static [$name] {
                &[ $( $name::$field ),* ]
            }
        }
    };
}

#[cfg(test)]
mod tests {
    primitive_enum! { MyEnum u16 ;
        A,
        B,
        C,
        D = 500,
        E,
    }

    #[test]
    fn test_enum_list() {
        use MyEnum::*;

        assert_eq!(MyEnum::list(), &[A, B, C, D, E]);

        for x in MyEnum::list() {
            assert_eq!(MyEnum::from((*x) as u16), Some(*x));
        }
    }

    #[test]
    fn test_enum_from() {
        use MyEnum::*;

        assert_eq!(MyEnum::from(0), Some(A));
        assert_eq!(MyEnum::from(1), Some(B));
        assert_eq!(MyEnum::from(2), Some(C));
        assert_eq!(MyEnum::from(3), None);
        assert_eq!(MyEnum::from(4), None);
        assert_eq!(MyEnum::from(500), Some(D));
        assert_eq!(MyEnum::from(501), Some(E));
        assert_eq!(MyEnum::from(502), None);

    }

    #[test]
    fn test_enum_from_name() {
        use MyEnum::*;

        assert_eq!(MyEnum::from_name("A"), Some(A));
        assert_eq!(MyEnum::from_name("B"), Some(B));
        assert_eq!(MyEnum::from_name("C"), Some(C));
        assert_eq!(MyEnum::from_name("D"), Some(D));
        assert_eq!(MyEnum::from_name("E"), Some(E));
        assert_eq!(MyEnum::from_name("X"), None);
        assert_eq!(MyEnum::from_name("asdf"), None);
    }
}
