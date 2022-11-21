const EMPTY: f32 = -777.0;

fn _print_2d(text: &str, vec: &Vec<Vec<f32>>) {
  println!("{}", text);
  println!("[");
  for el in vec {
    println!("{:?}", el);
  }
  println!("]");
}

fn get_clean_num(f: f32) -> f32 {
  (f * 1000.0).round() / 1000.0
}

fn get_whole_frac(f: f32) -> (f32, f32) {
  if f > 0.0 {
    let frac = get_clean_num(f - f.abs().floor());
    (get_clean_num(f - frac), frac)
  } else {
    let new_f = f * -1.0;
    let frac = get_clean_num(new_f - new_f.abs().floor());
    (get_clean_num(new_f - frac) * -1.0, frac * -1.0)
  }
}

#[cfg(test)]
mod tests_get_whole_frac {
  use super::*;
  #[test]
  fn get_whole_frac_full() {
    assert_eq!(get_whole_frac(1.1), (1.0, 0.1));
    assert_eq!(get_whole_frac(0.200000048), (0.0, 0.2));
    assert_eq!(get_whole_frac(-1.200000048), (-1.0, -0.2));
    assert_eq!(get_clean_num(1.5 / 1.25), 1.2);
  }
}

fn get_min(data: &Vec<f32>) -> (f32, usize) {
  let mut min = data[0];
  let mut min_index = 0;
  for (index, el) in data.iter().enumerate() {
    if (*el < min || min == EMPTY) && *el != EMPTY {
      min = *el;
      min_index = index;
    }
  }

  (min, min_index)
}

fn to_minus(data: &Vec<f32>) -> Vec<f32> {
  data.iter().map(|el| el * -1.0).collect::<Vec<_>>()
}

fn get_marks_basis(
  x_n: &Vec<Vec<f32>>,
  fx: &Vec<f32>,
  basis: &Vec<f32>,
  current_x_indexes: &Vec<usize>,
) -> Vec<f32> {
  let (_, min_index) = get_min(&fx);

  basis
    .iter()
    .enumerate()
    .map(|(index, el)| {
      if !current_x_indexes.contains(&index) {
        EMPTY
      } else {
        el / x_n[index][min_index]
      }
    })
    .collect::<Vec<_>>()
}

fn get_enabling_element(
  x_n: &Vec<Vec<f32>>,
  fx: &Vec<f32>,
  marks: &Vec<f32>,
) -> (f32, usize, usize) {
  let (_, min_index) = get_min(&fx);
  let (_, min_marks_index) = get_min(&marks);
  let enabling_element = x_n[min_marks_index][min_index];

  return (enabling_element, min_marks_index, min_index);
}

fn get_enabling_element_last(
  x_n: &Vec<Vec<f32>>,
  marks: &Vec<f32>,
  current_x_indexes: &Vec<usize>,
) -> (f32, usize, usize) {
  let min_index = *current_x_indexes.last().unwrap();
  let (_, min_marks_index) = get_min(&marks);
  let enabling_element = x_n[min_index][min_marks_index];

  return (enabling_element, min_index, min_marks_index);
}

// ee_cell_index <=> ee_row_index
fn change_xn_to_xn1(
  x_n: &Vec<Vec<f32>>,
  basis: &Vec<f32>,
  current_x_indexes: &Vec<usize>,
  enabling_element: f32,
  current_x_index: usize,
  next_x_index: usize,
) -> (Vec<Vec<f32>>, Vec<f32>, Vec<usize>) {
  let mut x_n = x_n.clone();
  let mut basis = basis.clone();

  let mut x_n_data = Vec::new();
  for el in x_n[current_x_index].iter() {
    x_n_data.push(*el / enabling_element);
  }
  x_n[next_x_index] = x_n_data.clone();
  basis[next_x_index] = basis[current_x_index] / enabling_element;

  let current_x_indexes = get_current_x_indexes(current_x_indexes, current_x_index, next_x_index);

  (x_n, basis, current_x_indexes)
}

fn xn_to_zeros(
  x_n: &Vec<Vec<f32>>,
  fx: &Vec<f32>,
  basis: &Vec<f32>,
  current_x_indexes: &Vec<usize>,
  ee_row_index: usize,
  ee_cell_index: usize,
) -> (Vec<Vec<f32>>, Vec<f32>, Vec<f32>) {
  let mut fx = fx.clone();
  let mut basis = basis.clone();
  let mut x_n = x_n.clone();

  let main_value = fx[ee_cell_index] * -1.0;
  for i in 0..fx.len() {
    let val = if i == x_n[ee_row_index].len() {
      basis[ee_cell_index]
    } else {
      x_n[ee_cell_index][i]
    };

    fx[i] = main_value * val + fx[i];
  }

  for el in current_x_indexes {
    let i = *el;
    if i == ee_cell_index {
      continue;
    }

    let main_value = x_n[i][ee_cell_index] * -1.0;
    for j in 0..fx.len() {
      if j == x_n[i].len() {
        basis[i] = main_value * basis[ee_cell_index] + basis[i];
      } else {
        x_n[i][j] = main_value * x_n[ee_cell_index][j] + x_n[i][j];
      };
    }
  }

  (x_n, fx, basis)
}

fn get_initial_x_indexes(x_n: &Vec<Vec<f32>>) -> Vec<usize> {
  let mut result = Vec::new();
  for i in 0..x_n.len() {
    if x_n[i].len() > 0 {
      result.push(i);
    }
  }

  result
}

fn get_current_x_indexes(
  current_x_indexes: &Vec<usize>,
  current_x_index: usize,
  next_x_index: usize,
) -> Vec<usize> {
  let mut result = current_x_indexes.clone();
  let index = result.iter().position(|x| *x == current_x_index).unwrap();
  result.remove(index);

  result.push(next_x_index);

  result
}

fn fx_has_minus(fx: &Vec<f32>) -> bool {
  fx.iter().any(|el| *el < 0.0)
}

fn basis_has_fractional(basis: &Vec<f32>, current_x_indexes: &Vec<usize>) -> bool {
  basis.iter().enumerate().any(|(index, el)| {
    if current_x_indexes.contains(&index) {
      el.round() != *el
    } else {
      false
    }
  })
}

fn increase_x(
  x_n: &Vec<Vec<f32>>,
  fx: &Vec<f32>,
  basis: &Vec<f32>,
  current_x_indexes: &Vec<usize>,
) -> (Vec<Vec<f32>>, Vec<f32>, Vec<f32>, Vec<usize>) {
  let mut x_n = x_n.clone();
  let mut fx = fx.clone();
  let mut basis = basis.clone();
  let mut current_x_indexes = current_x_indexes.clone();

  let (_, mut max_frac) = get_whole_frac(basis[current_x_indexes[0]]);
  let mut max_frac_index = current_x_indexes[0];
  for index in &current_x_indexes {
    let (_, val) = get_whole_frac(basis[*index]);
    if max_frac < val {
      max_frac = val;
      max_frac_index = *index;
    }
  }

  let mut new_x = Vec::new();
  for el in &x_n[max_frac_index] {
    let mut val = {
      let (whole, _) = get_whole_frac(*el);
      if *el < 0.0 {
        (whole + -1.0) * -1.0 + el
      } else {
        el - whole
      }
    };

    new_x.push(val);
  }

  x_n.push(new_x);

  let (_, frac) = get_whole_frac(basis[max_frac_index]);
  basis.push(frac);

  let last_index = basis.len() - 1;
  basis[last_index] *= -1.0;
  x_n[last_index] = to_minus(&x_n[last_index]);

  current_x_indexes.push(last_index);

  for index in &current_x_indexes {
    let val = if last_index == *index {
      1.0
    } else {
      x_n[last_index][*index]
    };
    x_n[*index].push(val);
  }

  let last_fx = fx.pop().unwrap();
  fx.push(0.0);
  fx.push(last_fx);
  fx = to_minus(&fx);

  (x_n, fx, basis, current_x_indexes)
}

fn get_marks_fx(x_n: &Vec<Vec<f32>>, fx: &Vec<f32>, current_x_indexes: &Vec<usize>) -> Vec<f32> {
  let last_index = *current_x_indexes.last().unwrap();

  fx.iter()
    .enumerate()
    .map(|(index, el)| {
      if current_x_indexes.contains(&index)
        || index > (x_n[last_index].len() - 1)
        || x_n[last_index][index] == 0.0
      {
        EMPTY
      } else {
        el / x_n[last_index][index]
      }
    })
    .collect::<Vec<_>>()
}

fn gomori(x_n: &Vec<Vec<f32>>, fx: &Vec<f32>, basis: &Vec<f32>) {
  let mut x_n = x_n.clone();
  let mut fx = fx.clone();
  let mut basis = basis.clone();

  let mut current_x_indexes = get_initial_x_indexes(&x_n);

  println!("Data");
  _print_2d("Xn", &x_n);
  println!("Fx {:?}", fx);
  println!("Basis {:?}", basis);
  println!("Current indexes {:?}", current_x_indexes);

  // Fx to -
  fx = to_minus(&fx);
  println!("Minus fx {:?}", fx);

  let mut is_fx_has_minus = fx_has_minus(&fx);
  let mut index = 0;

  while is_fx_has_minus {
    println!("\n---Cycle Minus--- {index}");

    // Marks
    let marks = get_marks_basis(&x_n, &fx, &basis, &current_x_indexes);
    println!("Marks {:?}", marks);

    // Enabling element
    let (enabling_element, ee_row_index, ee_cell_index) = get_enabling_element(&x_n, &fx, &marks);
    print!(
      "Enabling element {} {} {}",
      enabling_element, ee_row_index, ee_cell_index
    );

    // Change x to new x
    (x_n, basis, current_x_indexes) = change_xn_to_xn1(
      &x_n,
      &basis,
      &current_x_indexes,
      enabling_element,
      ee_row_index,
      ee_cell_index,
    );

    println!("Change x {ee_row_index} to new x {ee_cell_index}");
    _print_2d("Xn", &x_n);
    println!("Basis {:?}", basis);

    // need to get all zeros in re (min_index) cell
    (x_n, fx, basis) = xn_to_zeros(
      &x_n,
      &fx,
      &basis,
      &current_x_indexes,
      ee_row_index,
      ee_cell_index,
    );
    _print_2d("Zeros Xn", &x_n);
    println!("Basis {:?} Fx {:?}", basis, fx);

    is_fx_has_minus = fx_has_minus(&fx);

    index += 1;
  }

  let mut is_basis_has_fractional = basis_has_fractional(&basis, &current_x_indexes);
  println!("Basis has fractional {:?}", is_basis_has_fractional);
  index = 0;

  while is_basis_has_fractional {
    println!("\n---Cycle Fractional--- {index}");

    (x_n, fx, basis, current_x_indexes) = increase_x(&x_n, &fx, &basis, &current_x_indexes);
    _print_2d("Xn", &x_n);
    println!("Fx {:?}", fx);
    println!("Basis {:?}", basis);

    let marks = get_marks_fx(&x_n, &fx, &current_x_indexes);
    println!("Marks {:?}", marks);

    // Enabling element
    let (enabling_element, ee_row_index, ee_cell_index) =
      get_enabling_element_last(&x_n, &marks, &current_x_indexes);
    print!(
      "Enabling element {} {} {}",
      enabling_element, ee_row_index, ee_cell_index
    );

    // Change x to new x
    (x_n, basis, current_x_indexes) = change_xn_to_xn1(
      &x_n,
      &basis,
      &current_x_indexes,
      enabling_element,
      ee_row_index,
      ee_cell_index,
    );
    println!("Change x {ee_row_index} to new x {ee_cell_index}");
    _print_2d("Xn", &x_n);
    println!("Basis {:?}", basis);

    // need to get all zeros in re (min_index) cell
    (x_n, fx, basis) = xn_to_zeros(
      &x_n,
      &fx,
      &basis,
      &current_x_indexes,
      ee_row_index,
      ee_cell_index,
    );
    _print_2d("Zeros Xn", &x_n);
    println!("Basis {:?} \nFx {:?}", basis, fx);

    is_basis_has_fractional = basis_has_fractional(&basis, &current_x_indexes);

    index += 1;
  }
}

fn task_0() {
  let x_n: Vec<Vec<f32>> = vec![
    vec![],
    vec![],
    vec![1.0, 4.0, 1.0, 0.0],
    vec![2.0, 3.0, 0.0, 1.0],
  ];
  let fx: Vec<f32> = vec![2.0, 3.0, 0.0, 0.0, 0.0];
  let basis: Vec<f32> = vec![EMPTY, EMPTY, 14.0, 12.0];

  gomori(&x_n, &fx, &basis);
}

fn main() {
  task_0();
}
