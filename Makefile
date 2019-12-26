generate-bindings:
	cd vendor; bindgen ../wrapper.h -o bindings.rs -- -I ./
