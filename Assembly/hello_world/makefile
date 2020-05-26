hello: hello_world.o
	gcc -o hello hello_world.o -no-pie
	chmod +x hello
hello_world.o: hello_world.asm
	nasm -f elf64 -g -F dwarf hello_world.asm -l hello.lst
