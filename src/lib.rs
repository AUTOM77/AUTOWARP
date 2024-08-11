use tokio_stream::{StreamExt, iter};
pub mod client;

const CAPACITY: usize = 5;
const SEED: &str ="0U7h98No-L6987BhV-uDd80i36";

pub async fn run_tokio(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(CAPACITY));

    let tasks: Vec<_> = iter(0..num)
        .map(|_| {
            let semaphore = semaphore.clone();
            async move {
                let permit = semaphore.acquire_owned().await.unwrap();
                let license = match client::WARP::build().await {
                    Ok(mut a) => a.get_license(SEED.into()).await.unwrap(),
                    Err(_) => format!("error"),
                };

                println!("{:#?}", license);
                drop(permit);
            }
        })
        .collect().await;

    for task in tasks {
        task.await;
    }

    Ok(())
}

pub fn interface(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    println!("Processing num: {:?}", num);
    let _ = rt.block_on(run_tokio(num));
    Ok(())
}
