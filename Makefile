SQLITE3_LIB := $(shell rustc --crate-file-name src/sqlite3/lib.rs --crate-type=rlib)

default: target/$(SQLITE3_LIB)

target:
	mkdir -p target

clean:
	rm -rf target

target/$(SQLITE3_LIB): target src/sqlite3/lib.rs
	rustc src/sqlite3/lib.rs --out-dir target --crate-type=rlib

tests:
	rustc --test src/sqlite3/lib.rs --out-dir target -o target/tests
	./target/tests

doc: target
	mkdir -p target/doc
	rustdoc src/sqlite3/lib.rs -o target/doc

.PHONY: default clean tests
