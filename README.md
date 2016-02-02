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

	set -x TOX_INCLUDE_PATH /usr/local/include/tox/
	set -x CLANG_INCLUDE_PATH /usr/lib/clang/3.4.0/include/

bash:

	export TOX_INCLUDE_PATH=/usr/local/include/tox/
	export CLANG_INCLUDE_PATH=/usr/lib/clang/3.4.0/include/
