use std::time::Instant;

fn main() {
    let start = Instant::now();

    // do stuff

    let elapsed = start.elapsed();
    // debug format:
    println!("{:?}", elapsed);
    // or format as milliseconds:
    println!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

}
