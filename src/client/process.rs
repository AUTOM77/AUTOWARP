use tokio::time::Duration;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const RETRY_DELAY: Duration = Duration::from_secs(360);
const MAX_CAPACITY: usize = 1000;
const MIN_CAPACITY: usize = 10;
const MAX_SEED: usize = 5;
const _EPOOL: &str = "SVQ4azYxMm4tSDhDNDJFajYtODAyN0pFVHAgbzY3OXZjMEYtM1ZMVzFrNzQtY0YwOWE0N1AgMHkyYjNVejUtUGNrSDk3NTQtM202NEExcnUgN1AwM0VXMk0tTWdsMzEyYjYtNXU2QzJ5NHMgMnZ0RDlJNzQtN3R3MTBhVDQtRVk1NzB1NlEgc0MyNTRPVjEtNkIwTTgxenMtOFltN0paMTMgOUw1R0VzODAtOHVTM3g5ZTEtOW12WTU2N2EgN2wyM1RXNWYtdjZUNTI5S3ctMWRvMlJBMzQgZTdNWEUyMDktaXg1MElyMTItOHEzY2IxN3ogM1BGSDg1OXktYzUxNEo5cHItMkU5NE0xUlggN0thYjEwTzQtOU9sdlc4NDMtNTBucWg3MmkgM0o0bmZ2OTIteDUzTTRBOFctWGQxdjkyN2IgYWhHMFM4MzEtME1tNTEzQkotSzduNlQxNVcgSzk0N0ZPRzUtNjRsMHZESDIteTc4MzFwVmkgcjE0dDlKeTUtOTZRM2gxVnYtN040YWkwcDggTTRGM2Q2N0EtYlo5azBxNzIteVVSZTg5MjQgSUJTTDgyNDYtSTM1OWlMNnEtNXdYMmZsNDcgUDZFMzE4Yk4tN2JJMlIwUTgtNzJKOWZ1NFYgVDVEWjgzRzAtUm1BMTQ5N3ctODFYeVJZMzcgOGw1MER4YzEtWms4ajdhOTAtMndOOTMxb0U=";

use super::warp::WARP;
use super::cipher;
use super::geo;

pub async fn batch_create(num: usize) -> Vec<WARP> {
    let mp = MultiProgress::new();
    let _style = ProgressStyle::with_template("[{elapsed_precise}] [{eta_precise}] {wide_bar:.cyan/green} {pos:>7}/{len:7}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let mut tasks = Vec::new();
    let mut warps = Vec::new();

    for _ in 0..num {
        let task = tokio::spawn(async {
            let ip = geo::generate_us_ip();
            WARP::build(ip).await
        });

        tasks.push(task);

        if tasks.len() == MAX_CAPACITY {
            let mut _tasks = vec![];
            _tasks.append(&mut tasks);

            let pb = mp.add(ProgressBar::new(_tasks.len() as u64));
            pb.set_style(_style.clone());

            for task in _tasks {
                if let Ok(Ok(warp)) = task.await {
                    warps.push(warp);
                }
                pb.inc(1);
            }
            tasks.clear();
            pb.finish();
            tokio::time::sleep(RETRY_DELAY).await;
        }
    }
    if tasks.len() > 0{
        if tasks.len()>MIN_CAPACITY{
            tokio::time::sleep(RETRY_DELAY).await;
        }
        let _pb = mp.add(ProgressBar::new(tasks.len() as u64));
        _pb.set_style(_style.clone());
        for task in tasks {
            if let Ok(Ok(warp)) = task.await {
                warps.push(warp);
            }
            _pb.inc(1);
        }
        _pb.finish();
    }
    warps
}

pub async fn batch_seed(warp: Vec<WARP>) -> Vec<WARP> {
    let mp = MultiProgress::new();
    let _style = ProgressStyle::with_template("[{elapsed_precise}] [{eta_precise}] {wide_bar:.cyan/yellow} {pos:>7}/{len:7}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    
    let epool = std::env::var("EPOOL").unwrap_or_else(|_| _EPOOL.to_string());
    let pool = cipher::decode(&epool);
    let seeds : Vec<_>  = pool.split(" ").into_iter().collect();

    println!("POOL: {:?}", seeds.len());
    let mut tasks = Vec::new();
    let mut warps = Vec::new();
    let mut _warps = warp.clone();

    for _ in 0..MAX_SEED {
        for seed in &seeds {
            if let Some(warp) = _warps.pop() {
                let seed = seed.to_string();
                let task = tokio::spawn(async move {
                    warp.update_license(seed).await
                });
                tasks.push(task);
            } else {
                break;
            }

            if tasks.len() == MAX_CAPACITY {
                tokio::time::sleep(RETRY_DELAY).await;
                let mut _tasks = vec![];
                _tasks.append(&mut tasks);

                let pb = mp.add(ProgressBar::new(_tasks.len() as u64));
                pb.set_style(_style.clone());

                for task in _tasks {
                    if let Ok(Ok(warp)) = task.await {
                        warps.push(warp);
                    }
                    pb.inc(1);
                }
                tasks.clear();
                pb.finish();
            }
        }
    }

    if tasks.len() > 0{
        if tasks.len()>MIN_CAPACITY{
            tokio::time::sleep(RETRY_DELAY).await;
        }
        let _pb = mp.add(ProgressBar::new(tasks.len() as u64));
        _pb.set_style(_style.clone());
        for task in tasks {
            if let Ok(Ok(warp)) = task.await {
                warps.push(warp);
            }
            _pb.inc(1);
        }
        _pb.finish();
    }
    warps
}

pub async fn batch_update(warp: Vec<WARP>) -> Vec<WARP> {
    let mp = MultiProgress::new();
    let _style = ProgressStyle::with_template("[{elapsed_precise}] [{eta_precise}] {wide_bar:.cyan/blue} {pos:>7}/{len:7}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let mut tasks = Vec::new();
    let mut warps = Vec::new();

    for sw in warp{
        let ss = sw.license();
        let task = tokio::spawn(async move {
            sw.update_license(ss).await
        });
        tasks.push(task);

        if tasks.len() == MAX_CAPACITY {
            tokio::time::sleep(RETRY_DELAY).await;
            let mut _tasks = vec![];
            _tasks.append(&mut tasks);

            let pb = mp.add(ProgressBar::new(_tasks.len() as u64));
            pb.set_style(_style.clone());

            for task in _tasks {
                if let Ok(Ok(warp)) = task.await {
                    warps.push(warp);
                }
                pb.inc(1);
            }
            tasks.clear();
            pb.finish();
        }
    }

    if tasks.len() > 0{
        if tasks.len()>MIN_CAPACITY{
            tokio::time::sleep(RETRY_DELAY).await;
        }
        let _pb = mp.add(ProgressBar::new(tasks.len() as u64));
        _pb.set_style(_style.clone());
        for task in tasks {
            if let Ok(Ok(warp)) = task.await {
                warps.push(warp);
            }
            _pb.inc(1);
        }
        _pb.finish();
    }

    warps
}

pub async fn batch_delete(warp: Vec<WARP>) -> Vec<WARP> {
    let mp = MultiProgress::new();
    let _style = ProgressStyle::with_template("[{elapsed_precise}] [{eta_precise}] {wide_bar:.cyan/red} {pos:>7}/{len:7}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let mut tasks = Vec::new();
    let mut warps = Vec::new();

    for dw in warp{
        let task = tokio::spawn(async move {
            dw.delete().await
        });
        tasks.push(task);

        if tasks.len() == MAX_CAPACITY {
            tokio::time::sleep(RETRY_DELAY).await;
            let mut _tasks = vec![];
            _tasks.append(&mut tasks);

            let pb = mp.add(ProgressBar::new(_tasks.len() as u64));
            pb.set_style(_style.clone());

            for task in _tasks {
                if let Ok(Ok(warp)) = task.await {
                    warps.push(warp);
                }
                pb.inc(1);
            }
            tasks.clear();
            pb.finish();
        }
    }

    if tasks.len() > 0{
        if tasks.len()> MIN_CAPACITY{
            tokio::time::sleep(RETRY_DELAY).await;
        }
        let _pb = mp.add(ProgressBar::new(tasks.len() as u64));
        _pb.set_style(_style.clone());
        for task in tasks {
            if let Ok(Ok(warp)) = task.await {
                warps.push(warp);
            }
            _pb.inc(1);
        }
        _pb.finish();
    }

    warps
}

pub async fn batch_info(warp: Vec<WARP>){
    let mp = MultiProgress::new();
    let _style = ProgressStyle::with_template("[{elapsed_precise}] [{eta_precise}] {wide_bar:.green/red} {pos:>7}/{len:7}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let mut tasks = Vec::new();

    for _warp in warp{
        let task = tokio::spawn(async move {
            _warp.get_quota().await
        });
        tasks.push(task);
    }

    let _pb = mp.add(ProgressBar::new(tasks.len() as u64));
    _pb.set_style(_style.clone());

    if tasks.len()>MIN_CAPACITY{
        tokio::time::sleep(RETRY_DELAY).await;
    }
    
    for task in tasks {
        if let Ok(Ok(info)) = task.await {
            let quota:usize = info.parse().expect("Failed to convert quota to usize");
            if quota == 24598562000000000{
                _pb.inc(1);
            }
            else {
                print!("-{}-", quota);
            }
        }
    }
    _pb.finish();
}