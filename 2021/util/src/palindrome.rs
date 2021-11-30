use std::borrow::Cow;

pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

macro_rules! impl_palindrome_str {
    ($($type:ty),*) => {
        $(
            impl Palindrome for $type {
                #[inline]
                fn is_palindrome(&self) -> bool {
                    let mut chars = self.chars();

                    loop {
                        match (chars.next(), chars.next_back()) {
                            (Some(a), Some(b)) if a != b => return false,
                            (_, None) => return true,
                            _ => {}
                        }
                    }
                }
            }
        )*
    }
}

macro_rules! impl_palindrome_uint {
    ($($type:ty),*) => {
        $(
            impl Palindrome for $type {
                #[inline]
                fn is_palindrome(&self) -> bool {
                    let mut m = *self;
                    let mut rev = 0;

                    while m > 0 {
                        rev = rev * 10 + m % 10;
                        m /= 10;
                    }

                    rev == *self
                }
            }
        )*
    };
}

impl_palindrome_str!(&str, String, &String, Cow<'_, str>);
impl_palindrome_uint!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_str() {
        assert!("".is_palindrome());
    }

    #[test]
    fn str_palindrome() {
        assert!("abcacba".is_palindrome());
        assert!("abcaacba".is_palindrome());
    }

    #[test]
    fn no_str_palindrome() {
        assert!(!"abcabca".is_palindrome());
    }

    #[test]
    fn one_digit() {
        assert!(0_u8.is_palindrome());
    }

    #[test]
    fn num_palindrome() {
        assert!(1234321_u32.is_palindrome());
        assert!(12344321_u32.is_palindrome());
    }

    #[test]
    fn no_num_palindrome() {
        assert!(!1234231_u32.is_palindrome());
    }
}
