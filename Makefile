generate-bindings:
	cd vendor; bindgen sample.h -o bindings.rs
