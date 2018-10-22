#![allow(unused_macros)]
macro_rules! gen_execute_macro {
    ($(($($idl:tt $it:tt $in:tt),*), ($($odl:tt $ot:tt $on:tt),*),)*) => {
        macro_rules! execute_2 {
            $(
            ($oname:ident, ($($idl$it:ty),*), ($($odl$ot:ty),*)) => {
                fn execute(&self, in_var: Self::InType) -> Self::OutType {
                    let mut res = Self::OutType::default();
                    let kmut = (*KERNEL).clone();
                    let mut k = kmut.lock();
                    unsafe {
                        $oname(&mut *k,
                               $(&mut res.$on),*,
                               $(in_var.$in),*);
                        futhark_context_sync(&mut *k);
                    }
                    res
                }
            };
            )*
        }
    };
    ($(($($idl:tt $it:tt $in:tt),*), $odl:tt $ot:tt,)*) => {
        macro_rules! execute_l {
            $(
            ($oname:ident, ($($idl$it:ty),*), $otype:ty) => {
                fn execute(&self, in_var: Self::InType) -> Self::OutType {
                    let mut res = Self::OutType::default();
                    let kmut = (*KERNEL).clone();
                    let mut k = kmut.lock();
                    unsafe {
                        $oname(&mut *k,
                               &mut res,
                               $(in_var.$in),*);
                        futhark_context_sync(&mut *k);
                    }
                    res
                }
            };
            )*
        }
    };
    ($($idl:tt $it:tt, ($($odl:tt $ot:tt $on:tt),*),)*) => {
        macro_rules! execute_r {
            $(
            ($oname:ident, $itype:ty, ($($odl$ot:ty),*)) => {
                fn execute(&self, in_var: Self::InType) -> Self::OutType {
                    let mut res = Self::OutType::default();
                    let kmut = (*KERNEL).clone();
                    let mut k = kmut.lock();
                    unsafe {
                        $oname(&mut *k,
                               $(&mut res.$on),*,
                               in_var);
                        futhark_context_sync(&mut *k);
                    }
                    res
                }
            };
            )*
        }
    };
}
macro_rules! execute_n {
    ($oname:ident, $itype:ty, $otype:ty) => {
        fn execute(&self, in_var: Self::InType) -> Self::OutType {
            let mut res = Self::OutType::default();
            let kmut = (*KERNEL).clone();
            let mut k = kmut.lock();
            unsafe {
                $oname(&mut *k,
                       &mut res,
                       in_var);
                futhark_context_sync(&mut *k);
            }
            res
        }
    };
}

gen_execute_macro! {
    ($a 0, $z 1), ($b 0, $c 1),
    ($a 0, $z 1), ($b 0, $c 1, $d 2),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2), ($b 0, $c 1),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3, $w 4), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $f 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $g 4),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    ($a 0, $z 1, $y 2, $x 3, $w 4, $v 5, $u 6, $t 7, $s 8, $s 9), ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
}

gen_execute_macro! {
    $a, ($b 0, $c 1),
    $a, ($b 0, $c 1, $d 2),
    $a, ($b 0, $c 1, $d 2, $e 3),
    $a, ($b 0, $c 1, $d 2, $e 3, $f 4),
    $a, ($b 0, $c 1, $d 2, $e 3, $g 4),
    $a, ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5),
    $a, ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6),
    $a, ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7),
    $a, ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8),
    $a, ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9),
}

gen_execute_macro! {
    ($b 0, $c 1), $a,
    ($b 0, $c 1, $d 2), $a,
    ($b 0, $c 1, $d 2, $e 3), $a,
    ($b 0, $c 1, $d 2, $e 3, $f 4), $a,
    ($b 0, $c 1, $d 2, $e 3, $g 4), $a,
    ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5), $a,
    ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6), $a,
    ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7), $a,
    ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8), $a,
    ($b 0, $c 1, $d 2, $e 3, $g 4, $h 5, $i 6, $j 7, $k 8, $l 9), $a,
}

macro_rules! execute {
    ($oname: ident, ($($itype:ty),*), ($($otype:ty),*)) => (
        execute_2!($oname, ($($itype),*), ($($otype),*));
    );
    ($oname: ident, ($($itype:ty),*), $otype:ty) => (
        execute_l!($oname, ($($itype),*), $otype);
    );
    ($oname: ident, $itype:ty, ($($otype:ty),*)) => (
        execute_r!($oname, $itype, ($($otype),*));
    );
    ($oname: ident, $itype:ty, $otype:ty) => (
        execute_n!($oname, $itype, $otype);
    );
}

macro_rules! futhark_entry {
    (@itype $itype:ty, $_:ty) => ($itype);
    (@otype $_:ty, $otype:ty) => ($otype);
    ($fname:ident, $oname:ident, $($types:tt)*) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = futhark_entry!(@itype $($types)*);
            type OutType = futhark_entry!(@otype $($types)*);

            execute!($oname, $($types)*);
        }
    };
}
