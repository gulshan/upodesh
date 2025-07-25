use peak_alloc::PeakAlloc;
use upodesh::suggest::Suggest;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let _suggest = Suggest::new();

    let current_mem = PEAK_ALLOC.current_usage_as_mb();
    println!("This program currently uses {} MB of RAM.", current_mem);
    let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
    println!("The max amount that was used {} MB.", peak_mem);
}
