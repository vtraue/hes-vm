(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func))
  (type (;6;) (func))
  (import "env" "print" (func (;0;) (type 0)))
  (import "env" "printNum" (func (;1;) (type 1)))
  (func (;2;) (type 2) (param i32) (result i32)
    (local i32)
    i32.const 1)
  (func (;3;) (type 3) (param i32 i32) (result i32)
    (local i32 i32)
    i32.const 2)
  (func (;4;) (type 4) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 3)
  (func (;5;) (type 5)
    (local i32)
    global.get 0
    local.set 0
    global.get 0
    i32.const 4
    i32.add
    global.set 0
    local.get 0
    i32.const 0
    i32.store align=1
    i32.const 1
    call 1
    local.get 0
    i32.load align=1
    i32.const 3
    call 0)
  (func (;6;) (type 6)
    i32.const 2
    call 1)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 0))
  (export "should_work" (func 2))
  (export "should_work1" (func 3))
  (export "should_work2" (func 4))
  (export "test" (func 5))
  (export "main" (func 6))
  (export "memory" (memory 0))
  (start 6)
  (data (;0;) (i32.const 0) "\06\00\00\00blubbi\00"))
