use rand::Rng;

fn main() {
  let mut weights = [0.0; 3];
  
  fn activation(input: f64) -> f64 {
    if input <= 0.0 {
        0.0
    } else {
        1.0
    }
  }

  fn right_prop(input: [f64; 3], w: [f64; 3]) -> f64 {
    let mut res = 0.0;
    for i in 0..input.len() {
      res += w[i] * input[i];
    }
    res
  }

  fn train(data: [[f64; 3]; 3], exp: [f64; 3], w: &mut [f64; 3]) {
    const LR: f64 = 0.1;
    const EPOCH: i64 = 1;
    
    for i in 0..EPOCH {
      for j in 0..data.len() {
        let error = exp[j] - activation(right_prop(data[j], *w));
        println!("Error: {:?} {:?} {:?} {:?} {:?} {}", data, exp, w, right_prop(data[j], *w), activation(right_prop(data[j], *w)), error);
        for n in 0..w.len() {
          w[n] += LR * error * data[j][n];
        }
        println!("New weights: {:?}", w);
      }
    }
  }

  let training_data = [
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
  ];

  let expected_results = [0.0, 0.0, 1.0];

  let mut rng = rand::thread_rng();

  for i in 0..weights.len() {
    weights[i] = rng.gen();
  }
  println!("Weights: {:?}", weights);
  train(training_data, expected_results, &mut weights);

  for i in 0..weights.len() {
    weights[i] = if weights[i] < 0.0 { 0.0 } else { weights[i] };
  }
  println!("Weights trained: {:?}", weights);
  
  let new = [0.0, 1.0, 0.0];
  println!("New {:?} {:?} {:?}", new, right_prop(new, weights), activation(right_prop(new, weights)));
}
