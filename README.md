Tox-rs
======

Rust bindings for [toxcore](https://github.com/irungentoo/toxcore).

Requirements
------------

* toxcore
* clang

Build
-----

You can use environment variables to specify include path.

fish:

	set -x INCLUDE_PATH /usr/local/include/
	set -x CLANG_INCLUDE_PATH /usr/lib/clang/3.4.0/include/

bash:

	export INCLUDE_PATH=/usr/local/include/
	export CLANG_INCLUDE_PATH=/usr/lib/clang/3.4.0/include/
