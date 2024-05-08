/// Macro to add all of the derives for simple struct tuple data value.
///
/// - $vis:vis matches a visibility specifier (like pub),
/// - $name:ident matches an identifier (the struct name),
/// - $type:ty matches a type.
///
/// The macro generates the struct with the specified name and type,
/// and adds the specified attributes to it.
#[cfg(all(feature = "for-ts", feature = "for-sql"))]
#[macro_export]
macro_rules! derive_simple_data_type {
    ($(#[$meta:meta])* $vis:vis struct $name:ident($type:ty);) => {
        $(#[$meta])*
        #[cfg_attr(feature = "for-ts", derive(schemars::JsonSchema))]
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            PartialEq,
            PartialOrd,
            derive_more::From,
            derive_more::Into,
            derive_more::Display,
            derive_more::Deref,
            serde::Serialize,
            serde::Deserialize,
            modql::FromSqliteValue,
            modql::ToSqliteValue,
            modql::field::SeaFieldValue,
        )]
        $vis struct $name($type);
    };
}

// region:    --- derive_alias

/// derive_alias from the crate of this name.
/// See: https://github.com/ibraheemdev/derive-alias/blob/master/src/lib.rs
///
/// NOTE 1: Code was simple and very small, and the crate is understanbly not really maintained.
///         So, good oportunity to appropriate this code.
///
/// NOTE 2: Added the `#[macro_export]` in the macro generation for other crates, and changed
///         `$derive:ident` to `$derive:meta` to allow `serde::Deserialize`
///
/// NOTE 3: Sometime, when need cfg_attr, we have to be specific. This is why we might not
///         use this macro often. The more manual approach above seems more reliable
///
/// NOTE 4: Sometime some weird cannot access error when same crate access the generated macro_rules.
///         With manual macros above this does not happen
#[macro_export]
macro_rules! derive_alias {
    ($($name:ident => #[derive($($derive:meta),*)] $(,)?)*) => {
        $(
            #[macro_export]
            macro_rules! $name {
                ($i:item) => {
                    #[derive($($derive),*)]
                    $i
                }
            }
        )*
    }
}

// derive_alias! {
// 	derive_simple_tuple_data =>
// 	#[derive(
// 		Clone, Copy, Debug,
// 		Eq, PartialEq,
// 		derive_more::From, derive_more::Into, derive_more::Display, derive_more::Deref,
// 		serde::Serialize, serde::Deserialize,
// 		modql::FromSqliteValue, modql::field::Field
// 	)],
// }

// endregion: --- derive_alias
