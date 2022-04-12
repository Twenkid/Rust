
/// This example shows how to describe the adapter in use.
async fn run() {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    let adapter = wgpu::Instance::new(wgpu::Backends::all())
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
	println!("{:?}", adapter.get_info());	
    //OK SUCCESS!
	//	surface = unsafe { instance.create_surface(window) };
	let adapter = wgpu::Instance::new(wgpu::Backends::all())
        .request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, 
		   compatible_surface: None, force_fallback_adapter: false})
        .await
        .unwrap();
  //Some(&surface)
    #[cfg(not(target_arch = "wasm32"))]
    println!("{:?}", adapter.get_info());
	
	println!("ENUMERATE");
	for a in 0..=1 {
	let adapter = wgpu::Instance::new(wgpu::Backends::all())
    .enumerate_adapters(wgpu::Backends::all())
    //.filter(|adapter| 
        // Check if this adapter supports our surface
        //surface.get_preferred_format(&adapter).is_some()
		//wgpu::RequestAdapterOptions.is_some()
		//wgpu::PowerPreference == wgpu::PowerPreference::HighPerformance 	
    .next()
    .unwrap();
	println!("{}: {:?}", a, adapter.get_info());
	}
	
	//for a in adapters {
//		println!("{:?}", a.get_info());
	//}
	
	
/*
	 //let surface = unsafe { instance.create_surface(window) };
	//let options = wgpu::RequestAdapterOptions(wgpu::PowerPreference::HighPerformance);	
	    //let adapter_b = wgpu::Instance::new(wgpu::Backends::all())
	let adapter = wgpu::Instance::new(wgpu::Backends::all()).
		  request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                None, //compatible_surface: Some(&surface),
                force_fallback_adapter: false,
		  }).await.unwrap();
        
	#[cfg(not(target_arch = "wasm32"))]
    println!("{:?}", adapter_b.get_info())
	*/
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}
