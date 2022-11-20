#![cfg_attr(not(test), no_std)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

pub use paste::paste;

#[macro_export]
macro_rules! fn2ty {
    () => {};
    ($vi:vis fn $name:ident<$($life:lifetime,)* $($t:ident $(: $bound:path)?),+>($($arg:ident: $argt:ty),*) -> $ret:ty $body:block $($rem:tt)*) => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
        $vi enum $name<$($life,)* $($t $(: $bound)?,)*> {
            __Generics(core::marker::PhantomData<($(&$life (),)* $($t,)*)>),
            $name,
        }

        $crate::paste! {
            mod [<__ $name>] {
                pub(super) use super::$name::$name;
            }

            #[allow(unused_imports)]
            $vi use [<__ $name>]::*;
        }

        impl<$($life,)* $($t $(: $bound)?,)*> Default for $name<$($life,)* $($t,)*> {
            fn default() -> Self {
                Self::$name
            }
        }

        impl<$($life,)* $($t $(: $bound)?,)*> core::ops::FnOnce<($($argt,)*)> for $name<$($life,)* $($t,)*> {
            type Output = $ret;

            extern "rust-call" fn call_once(self, ($($arg,)*): ($($argt,)*)) -> Self::Output {
                $body
            }
        }

        impl<$($life,)* $($t $(: $bound)?,)*> core::ops::FnMut<($($argt,)*)> for $name<$($life,)* $($t,)*> {
            extern "rust-call" fn call_mut(&mut self, ($($arg,)*): ($($argt,)*)) -> Self::Output {
                $body
            }
        }

        impl<$($life,)* $($t $(: $bound)?,)*> core::ops::Fn<($($argt,)*)> for $name<$($life,)* $($t,)*> {
            extern "rust-call" fn call(&self, ($($arg,)*): ($($argt,)*)) -> Self::Output {
                $body
            }
        }

        $crate::fn2ty! { $($rem)* }
    };
    ($vi:vis fn $name:ident($($arg:ident: $argt:ty),*) -> $ret:ty $body:block $($rem:tt)*) => {
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
    ($vi:vis fn $name:ident($($arg:ident: $argt:ty),*) $body:block $($rem:tt)*) => {
        $crate::fn2ty! { $vi fn $name($($arg: $argt),*) -> () $body $($rem)* }
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
    pub fn test4() -> test3 {
        test3
    }
    fn test5<'a, A>(a: A) -> A {
        a
    }
);

#[test]
#[cfg(test)]
fn test_functions() {
    assert_eq!((), test());
    assert_eq!(42, test2(0));
    assert_eq!(90, test3(30, 60));
    assert_eq!(132, test4()(42, 90));
    assert_eq!(132, test5::<u32>(132));
}
