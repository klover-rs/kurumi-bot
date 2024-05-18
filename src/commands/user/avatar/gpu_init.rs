use pollster::FutureExt;

lazy_static::lazy_static! {
    pub static ref INSTANCE: wgpu::Instance = {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        instance
    };

    pub static ref ADAPTER: wgpu::Adapter = {
        let adapter = INSTANCE
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .block_on()
            .unwrap();
        adapter
    };

    pub static ref DEVICE_QUEUE: (wgpu::Device, wgpu::Queue) = {
        let (device, queue) = ADAPTER
            .request_device(&Default::default(), None)
            .block_on()
            .expect("Couldn't create device and queue");
        (device, queue)
    };

}
