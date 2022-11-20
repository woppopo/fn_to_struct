#![cfg_attr(not(test), no_std)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

#[macro_export]
macro_rules! fn2ty {
    () => {};
    ($vi:vis fn $name:ident($($arg:ident: $argt:ty,)*) -> $ret:ty $body:block $($rem:tt)*) => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy, Hash, Default, Debug)]
        $vi struct $name;

        impl core::ops::FnOnce<($($argt,)*)> for $name {
            type Output = $ret;

            extern "rust-call" fn call_once(self, ($($arg,)*): ($($argt,)*)) -> Self::Output {
                $body
            }
        }

        impl core::ops::FnMut<($($argt,)*)> for $name {
            extern "rust-call" fn call_mut(&mut self, ($($arg,)*): ($($argt,)*)) -> Self::Output {
                $body
            }
        }

        impl core::ops::Fn<($($argt,)*)> for $name {
            extern "rust-call" fn call(&self, ($($arg,)*): ($($argt,)*)) -> Self::Output {
                $body
            }
        }

        $crate::fn2ty! { $($rem)* }
    };
    ($vi:vis fn $name:ident($($arg:ident: $argt:ty,)*) $body:block $($rem:tt)*) => {
        $crate::fn2ty! { $vi fn $name($($arg: $argt),*) -> () $body $($rem)* }
    };
    ($vi:vis fn $name:ident($($arg:ident: $argt:ty),*) -> $ret:ty $body:block $($rem:tt)*) => {
        $crate::fn2ty! { $vi fn $name($($arg: $argt,)*) -> $ret $body $($rem)* }
    };
    ($vi:vis fn $name:ident($($arg:ident: $argt:ty),*) $body:block $($rem:tt)*) => {
        $crate::fn2ty! { $vi fn $name($($arg: $argt,)*) -> () $body $($rem)* }
    };
}

#[cfg(test)]
fn2ty!(
    fn test() {}
    fn test2(a: u32) -> u32 {
        a + 42
    }
    pub fn test3(a: u32, b: u32) -> u32 {
        a + b
    }
);

#[test]
#[cfg(test)]
fn test_functions() {
    assert_eq!((), test());
    assert_eq!(42, test2(0));
    assert_eq!(90, test3(30, 60));
}
