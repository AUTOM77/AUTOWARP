use rand::Rng as _;
use std::net::Ipv4Addr;

pub fn generate_geo_ip() -> String {
    let mut rng = rand::thread_rng();
    Ipv4Addr::new(
        rng.gen_range(1..=253),
        rng.gen(),
        rng.gen(),
        rng.gen_range(1..=254)
    ).to_string()
}

pub fn generate_us_ip() -> String {
    let mut rng = rand::thread_rng();
    let _blocks = [
        (3, 0, 0, 0, 4),       // 3.0.0.0/8
        (4, 0, 0, 0, 6),       // 4.0.0.0/6
        (8, 0, 0, 0, 7),       // 8.0.0.0/7
        (11, 0, 0, 0, 8),      // 11.0.0.0/8
        (12, 0, 0, 0, 6),      // 12.0.0.0/6
        (16, 0, 0, 0, 6),      // 16.0.0.0/6
        (20, 0, 0, 0, 7),      // 20.0.0.0/7
        (24, 0, 0, 0, 8),      // 24.0.0.0/8
        (26, 0, 0, 0, 7),      // 26.0.0.0/7
        (28, 0, 0, 0, 6),      // 28.0.0.0/6
        (32, 0, 0, 0, 3),      // 32.0.0.0/3
        (64, 0, 0, 0, 2),      // 64.0.0.0/2
        (128, 0, 0, 0, 3),     // 128.0.0.0/3
        (160, 0, 0, 0, 5),     // 160.0.0.0/5
        (168, 0, 0, 0, 6),     // 168.0.0.0/6
        (172, 0, 0, 0, 8),     // 172.0.0.0/8
        (173, 0, 0, 0, 8),     // 173.0.0.0/8
        (174, 0, 0, 0, 7),     // 174.0.0.0/7
        (184, 0, 0, 0, 6),     // 184.0.0.0/6
        (192, 0, 0, 0, 8),     // 192.0.0.0/8
        (198, 0, 0, 0, 7),     // 198.0.0.0/7
        (200, 0, 0, 0, 5),     // 200.0.0.0/5
        (208, 0, 0, 0, 4),     // 208.0.0.0/4
        ];
    let (base_a, _, _, _, mask) = _blocks[rng.gen_range(0.._blocks.len())];
    let ip = match mask {
        8 => Ipv4Addr::new(base_a, rng.gen(), rng.gen(), rng.gen()),
        7 => Ipv4Addr::new(base_a, rng.gen(), rng.gen(), rng.gen()),
        6 => Ipv4Addr::new(base_a + rng.gen_range(0..4), rng.gen(), rng.gen(), rng.gen()),
        5 => Ipv4Addr::new(base_a + rng.gen_range(0..8), rng.gen(), rng.gen(), rng.gen()),
        4 => Ipv4Addr::new(base_a + rng.gen_range(0..16), rng.gen(), rng.gen(), rng.gen()),
        3 => Ipv4Addr::new(base_a + rng.gen_range(0..32), rng.gen(), rng.gen(), rng.gen()),
        2 => Ipv4Addr::new(base_a + rng.gen_range(0..64), rng.gen(), rng.gen(), rng.gen()),
        _ => Ipv4Addr::new(base_a, rng.gen(), rng.gen(), rng.gen()),
    };
    ip.to_string()
}