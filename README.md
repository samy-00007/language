This is a simple language interpreter written in rust.

This is not intended for real usage, it is for educational purpose, do not use it.

# Profiler
`valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes target/debug/language`

`callgrind_annotate <callgrind.out.xxxxx>`