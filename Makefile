
all:
	mkdir -p ./lib
	rustc --lib ./src/helloworld.rc --out-dir ./lib

test: all
	mkdir -p ./build
	rustc --test ./src/helloworld.rc --out-dir ./build
	rustc --test -L ./lib ./test/simple.rs --out-dir ./build
	find ./build -perm -u+x -type f -exec {} \;

clean:
	rm -rf ./lib
	rm -rf ./build
