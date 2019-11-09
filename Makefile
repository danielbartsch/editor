bootstrap:
	cargo install cargo-tarpaulin

testCov:
	cargo tarpaulin \
	--verbose \
	--out Xml \
	--exclude-files \
		src/editor/sdl_experiment.rs \
		src/main.rs \
		src/raytrace.rs \
		*.test.rs
