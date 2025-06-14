format:
	@clang-format -i -- src/*.c include/**.h

configure: format
	cmake -Bbuild/ -S.

build: configure
	cmake --build build/

test:
	@build/TestProgram

clean:
	rm -r build/*

valgrind:
	valgrind --leak-check=yes build/TestProgram

cppcheck:
	cppcheck include/ src/

checks: cppcheck valgrind
