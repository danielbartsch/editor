bootstrap:
	cargo install cargo-tarpaulin

testCov:
	cargo tarpaulin \
	--verbose \
	--out Xml \
	--exclude-files \
		src/editor/ui.rs \
		src/editor/text_rendering.rs \
		src/main.rs \
		*.test.rs
