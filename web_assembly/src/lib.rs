// Ancien :		cargo build --target wasm32-unknown-unknown --release
// Nouveau :	wasm-pack build

#[no_mangle]
pub extern "C" fn addition( x: i32, y: i32 ) -> i32
{
	x + y
}
