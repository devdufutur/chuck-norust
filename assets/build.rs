extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_language(0x409);
        res.set_resource_file("assets/resource.rc");
        res.compile().unwrap();
    }
}