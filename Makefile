generate-bindings:
	cd vendor; bindgen ../wrapper.h -o bindings.rs --no-layout-tests --rust-target nightly -- -I ./
