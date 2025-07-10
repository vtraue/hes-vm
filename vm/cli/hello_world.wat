(module
	(import "env" "dbg_print_string" (func $print (param i32 i32)))

	(func $main
		(i32.const 0)
		(i32.const 11)
		(i32.const 0)
		(memory.init 0)
		(i32.const 0)
		(i32.const 11)
		(call $print)
	)
	(memory 1)
	(data "hallo welt\n")

)
