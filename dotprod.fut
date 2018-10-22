entry dotprod (xs: []i32) (ys: []i32): i32 =
  reduce (+) 0 (map2 (*) xs ys)

entry dotprod64 (xs: []i64) (ys: []i64): i64 =
  reduce (+) 0 (map2 (*) xs ys)