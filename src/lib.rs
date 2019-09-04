//! This crate provides a macro similar to the old `try!` macro, or to the `?`, except it wraps
//! the error in something else.
//!
//! This is useful if you want to use a `?` for easily returning say, invalid input errors,
//! but you can't do so because you have an additional `Result` level for handling internal errors of a different nature.
//!
//! This macro allows you to do so:
//!
//! ```ignore
//! fn foo(input: Input) -> Result<Result<FinalOutput, InvalidInputError>, DatabaseError> {
//!     let validated_input: ValidatedInput = try_or_wrap!(Ok, validate_input_with_database(input)?);
//!     Ok(Ok(do_stuff_with_validated_input(validated_input)?))
//! }
//!
//! fn validate_input_with_database(input: Input) -> Result<Result<ValidatedInput, InvalidInputError>, DatabaseError>;
//! ```

#[macro_export]
/// Helper macro to wrap `?` into something else, for `Result`
///
/// # Example
/// ```ignore
/// fn foo(input: Input) -> Result<Result<FinalOutput, InvalidInputError>, DatabaseError> {
///     let validated_input: ValidatedInput = try_or_wrap!(Ok, validate_input_with_database(input)?);
///     Ok(Ok(do_stuff_with_validated_input(validated_input)?))
/// }
///
/// fn validate_input_with_database(input: Input) -> Result<Result<ValidatedInput, InvalidInputError>, DatabaseError>;
/// ````
macro_rules! try_or_wrap {
    ($wrapper:expr, $expr:expr) => {
        match $expr {
            std::result::Result::Ok(val) => val,
            std::result::Result::Err(err) => {
                return $wrapper(std::result::Result::Err(std::convert::Into::into(err)))
            }
        }
    };
}

/// Same as `try_or_wrap`, but for `Option`
#[macro_export]
macro_rules! try_or_wrap_opt {
    ($wrapper:expr, $expr:expr) => {
        match $expr {
            std::option::Option::Some(val) => val,
            std::option::Option::None => return $wrapper(std::option::Option::None),
        }
    };
}
