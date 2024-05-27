use systemstat::{saturating_sub_bytes, Platform, System};

pub struct MemoryUsage {
    pub used_mem: u64,
    pub total_mem: u64,
}

pub fn memusage() -> Result<MemoryUsage, String> {
    let sys = System::new();

    match sys.memory() {
        Ok(mem) => {
            let total_mem = mem.total.as_u64() / (1024 * 1024);
            let used_mem = saturating_sub_bytes(mem.total, mem.free).as_u64() / (1024 * 1024);

            Ok(MemoryUsage {
                used_mem,
                total_mem,
            })
        }
        Err(x) => Err(format!("Error getting memory usage: {}", x)),
    }
}
