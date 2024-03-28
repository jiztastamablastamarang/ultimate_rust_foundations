fn main() {
    let core_ids = core_affinity::get_core_ids().unwrap();

    let handles = core_ids
        .into_iter()
        .map(|id| {
            std::thread::spawn(move || {
                let success = core_affinity::set_for_current(id);
                if success {
                    println!("hello from core {id:?}");
                } else {
                    println!("failed to set affinity for core {id:?}");
                }
            })
        })
        .collect::<Vec<_>>();
    
    handles.into_iter().for_each(|h| h.join().unwrap());
}
