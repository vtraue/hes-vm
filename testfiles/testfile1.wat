(module
  (import "console" "log" (func $log (param i32)))
  (func $main

    (local $var i32) ;; create a local variable named $var
    (local.set $var (i32.const 10)) ;; set $var to 10
    local.get $var ;; load $var onto the stack
    call $log ;; log the result

  )
  (func $try (param i32)
    (local $var1 i32))
    (func $try2 (param i32)
    (local $var1 i32))
  (start $main)
)
