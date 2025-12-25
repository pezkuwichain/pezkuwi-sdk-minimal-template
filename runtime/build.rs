//! Build script for pez-minimal-template-runtime.

fn main() {
	#[cfg(feature = "std")]
	{
		pezkuwi_sdk::bizinikiwi_wasm_builder::WasmBuilder::build_using_defaults();
	}
}
