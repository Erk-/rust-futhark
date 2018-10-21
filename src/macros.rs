macro_rules! futhark_entry {
    ($fname:ident, $oname:ident, $itype:ident, $otype:ident) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), $otype:ty) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = $otype;

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut res: Self::OutType = Self::OutType::default();
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                res
            }
        }
    };
        ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2) = (Self::OutType.0::default(), Self::OutType.1::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9)
            }
        }
    };
    ($fname:ident, $oname:ident, $itype:ty, ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = $itype;
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let in1 = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, inn2, in3, in4) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3, in4);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3, in4, in5);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3, in4, in5, in6);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, i3, i4, i5, i6, i7);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3, in4, in5, in6, in7, in8);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
    ($fname:ident, $oname:ident, ($i1type:ty, $i2type:ty, $i3type:ty, $i4type:ty, $i5type:ty, $i6type:ty, $i7type:ty, $i8type:ty, $i9type:ty, $i10type:ty), ($o1type:ty, $o2type:ty, $o3type:ty, $o4type:ty, $o5type:ty, $o6type:ty, $o7type:ty, $o8type:ty, $o9type:ty, $o10type:ty)) => {
        pub struct $fname;

        impl FutharkFunction for $fname {
            type InType = ($i1type, $i2type, $i3type, $i4type, $i5type, $i6type, $i7type, $i8type, $i9type, $i10type);
            type OutType = ($o1type, $o2type, $o3type, $o4type, $o5type, $o6type, $o7type, $o8type, $o9type, $o10type);

            fn execute(&self, in_var: Self::InType) -> Self::OutType {
                let mut (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10) = (Self::OutType.0::default(), Self::OutType.1::default(), Self::OutType.2::default(), Self::OutType.3::default(), Self::OutType.4::default(), Self::OutType.5::default(), Self::OutType.6::default(), Self::OutType.7::default(), Self::OutType.8::default(), Self::OutType.9::default());
                let kmut = (*KERNEL).clone();
                let mut k = kmut.lock();
                let (in1, in2, in3, in4, in5, in6, in7, in8, in9, in10) = in_var;
                unsafe {
                    $oname(&mut *k,
                           &mut res1, &mut res2, &mut res3, &mut res4, &mut res5, &mut res6, &mut res7, &mut res8, &mut res9, &mut res10,
                           in1, in2, in3, in4, in5, in6, in7, in8, in9, in10);
                    futhark_context_sync(&mut *k);
                }
                (res1, res2, res3, res4, res5, res6, res7, res8, res9, res10)
            }
        }
    };
}
