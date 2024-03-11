#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn hello(a: String) -> String { 
    a.repeat(2) 
}
