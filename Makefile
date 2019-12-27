generate-bindings:
	bindgen wrapper.h -o src/bindings.rs --no-layout-tests -- -I ./vendor
