(module
	(import "env" "dbg_print_string" (func $print_string (param i32 i32)))
	(func (result i32)
		i32.const 1
		i32.const 2
		i32.add
	)
)
