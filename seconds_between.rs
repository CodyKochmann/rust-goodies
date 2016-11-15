use std::time::Instant;

fn seconds_between(start: Instant, finish: Instant) -> f32 {
    let d = finish.duration_since(start);
    let secs = d.as_secs() as f32;
    let nanosecs = d.subsec_nanos() as f32;
    (secs*1000.0+nanosecs/1000000.0)/1000.0
}

fn main(){
  let ts: Instant = Instant::now();
  for x in 1..100000000 {
      let i = Instant::now();
  }
  let ending: Instant = Instant::now();
  println!("{:?}", seconds_between(ts,ending));
}
