use std::time::Instant;

fn milliseconds_between(start: Instant, finish: Instant) -> f32 {
    let d = finish.duration_since(start);
    let secs = d.as_secs() as f32;
    let nanosecs = d.subsec_nanos() as f32;
    secs*1000.0+nanosecs/1000000.0
}

fn main(){
  let ts: Instant = Instant::now();
  for x in 1..100000000 {
      let i: Instant = Instant::now();
  }
  let ending: Instant = Instant::now();
  println!("{:?}", milliseconds_between(ts,ending));
}
