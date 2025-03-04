(module
  (type $t0 (func))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i32)))
  (func $f0 (type $t0)
    (local $l0 i32)
    (local.set $l0
      (local.get $l0)))
  (func $f1 (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32)
    (local.set $l1
      (local.get $p0)))
  (func $f2 (type $t1) (param $p0 i32)
    (local $l1 i32) (local $l2 i32)))
