######  vfs
build-vfs:
	cargo build --bin vfs

build-vfs-release:
	cargo build --bin vfs --release --quiet

vfs:
	./target/debug/vfs

###### vfsd
build-vfsd:
	cargo build --bin vfsd

build-vfsd-release:
	cargo build --bin vfsd --release --quiet

run-vfsd:
	./target/debug/vfsd