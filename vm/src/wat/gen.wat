(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (type (;6;) (func))
  (type (;7;) (func))
  (import "env" "print" (func (;0;) (type 0)))
  (import "env" "printNum" (func (;1;) (type 1)))
  (import "env" "trap" (func (;2;) (type 2)))
  (func (;3;) (type 3) (param i32) (result i32)
    (local i32)
    i32.const 1)
  (func (;4;) (type 4) (param i32 i32) (result i32)
    (local i32 i32)
    i32.const 2)
  (func (;5;) (type 5) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    i32.const 3)
  (func (;6;) (type 6)
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
  (func (;7;) (type 7)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    i32.const 2
    call 1
    global.get 0
    local.set 0
    global.get 0
    i32.const 4
    i32.add
    global.set 0
    local.get 0
    i32.const 5
    i32.store align=1
    block  ;; label = @1
      global.get 0
      local.set 1
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 1
      i32.const 5
      i32.store align=1
      global.get 0
      local.set 2
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 2
      i32.const 10
      i32.store align=1
    end
    global.get 0
    local.set 5
    global.get 0
    i32.const 4
    i32.add
    global.set 0
    local.get 5
    block (result i32)  ;; label = @1
      global.get 0
      local.set 3
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 3
      i32.const 5
      i32.store align=1
      global.get 0
      local.set 4
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 4
      i32.const 10
      i32.store align=1
      i32.const 9
      br 0 (;@1;)
    end
    i32.store align=1
    local.get 0
    i32.load align=1
    i32.const 5
    i32.eq
    if  ;; label = @1
      global.get 0
      local.set 6
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 6
      i32.const 99
      i32.store align=1
      global.get 0
      local.set 7
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 7
      i32.const 98
      i32.store align=1
    else
      global.get 0
      local.set 8
      global.get 0
      i32.const 4
      i32.add
      global.set 0
      local.get 8
      i32.const 100
      i32.store align=1
    end)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 0))
  (export "should_work" (func 3))
  (export "should_work1" (func 4))
  (export "should_work2" (func 5))
  (export "test" (func 6))
  (export "main" (func 7))
  (export "memory" (memory 0))
  (start 7)
  (data (;0;) (i32.const 0) "\06\00\00\00blubbi\00"))
