pub mod env;
pub mod path;
pub mod sudo;
pub mod system_usage;

#[cfg(all(unix))]
pub fn is_actice(process: &str) -> bool {
    use procfs::process::{Process, ProcessesIter};

    let p_iter = procfs::process::all_processes().unwrap();

    for p in p_iter {
        let proc = match p {
            Ok(p) => p,
            Err(_) => continue,
        };
        println!("pid: {:?}", proc.exe());

        if proc.exe().is_ok() && proc.exe().unwrap().to_str().unwrap().contains(process) {
            return true;
        }
    }

    false
}

// #[cfg(target_os = "macos")]
// pub fn is_actice(process: &str) -> bool {
//     false
// }

#[cfg(target_os = "windows")]
pub fn is_actice(process: &str) -> bool {
    unsafe {
        let tl = tasklist::Tasklist::new();
        for i in tl {
            if i.get_pname.contains(process) {
                return true;
            }
        }
    }
    false
}

//#[cfg(all(unix))]
pub fn is_root() -> bool {
    match crate::utils::sudo::check() {
        crate::utils::sudo::RunningAs::Root => true,
        _ => false,
    }
}
