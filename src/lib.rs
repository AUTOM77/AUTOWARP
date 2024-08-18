pub mod client;
use client::process;

pub fn interface(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    let new_warps = rt.block_on(process::batch_create(num));
    println!("{:#?}", new_warps.len());
    let seed_warps = rt.block_on(process::batch_seed(new_warps));
    println!("{:#?}", seed_warps.len());
    let update_warps = rt.block_on(process::batch_update(seed_warps));
    println!("{:#?}", update_warps.len());

    rt.block_on(process::batch_info(update_warps.clone()));

    let delete_warps = rt.block_on(process::batch_update(update_warps));
    println!("{:#?}", delete_warps.len());

    for warp in &delete_warps{
        println!("{}", warp.license());
    }

    Ok(())
}
