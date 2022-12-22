use std::vec;

const EMPTY: f32 = -777.0;

fn _print_2d(text: &str, vec: &Vec<Vec<f32>>) {
  println!("{}", text);
  println!("[");
  for el in vec {
    println!("{:?}", el);
  }
  println!("]");
}

fn get_clean_num_4(f: f32) -> f32 {
  (f * 1000.0).round() / 1000.0
}

fn get_clean_num_3(f: f32) -> f32 {
  (f * 10.0).round() / 10.0
}

fn get_whole_frac(f: f32) -> (f32, f32) {
  if f > 0.0 {
    let frac = get_clean_num_4(f - f.abs().floor());
    (get_clean_num_4(f - frac), frac)
  } else {
    let new_f = f * -1.0;
    let frac = get_clean_num_4(new_f - new_f.abs().floor());
    (get_clean_num_4(new_f - frac) * -1.0, frac * -1.0)
  }
}

fn get_xn_based_on_current_indexes(
  x_n: &Vec<Vec<f32>>,
  current_x_indexes: &Vec<usize>,
) -> Vec<Vec<f32>> {
  let mut result = Vec::new();

  for el in current_x_indexes {
    let mut line = x_n[*el].clone();
    line.insert(0, *el as f32 + 1.0);
    result.push(line);
  }

  result
}

#[cfg(test)]
mod tests_get_whole_frac {
  use super::*;
  #[test]
  fn get_whole_frac_full() {
    assert_eq!(get_whole_frac(1.1), (1.0, 0.1));
    assert_eq!(get_whole_frac(0.200000048), (0.0, 0.2));
    assert_eq!(get_whole_frac(-1.200000048), (-1.0, -0.2));
    assert_eq!(get_clean_num_4(1.5 / 1.25), 1.2);
  }
}

fn get_min(data: &Vec<f32>, only_positive: bool, skip_index: Vec<usize>) -> (f32, usize) {
  let mut min_index: Option<usize> = None;
  for (index, el) in data.iter().enumerate() {
    if skip_index.contains(&index) {
      continue;
    }

    if only_positive && *el < 0.0 {
      continue;
    }
    if min_index == None {
      min_index = Some(index);
    }

    let min_value = data[min_index.unwrap()];

    if (*el < min_value || min_value == EMPTY) && *el != EMPTY {
      min_index = Some(index);
    }
  }

  let min_index = min_index.unwrap();

  (data[min_index], min_index)
}

#[cfg(test)]
mod tests_get_min {
  use super::*;
  #[test]
  fn get_min_main() {
    assert_eq!(
      get_min(&vec![1.1, 0.5, -1.0, 0.0], false, vec![]),
      (-1.0, 2)
    );
    assert_eq!(
      get_min(&vec![EMPTY, 0.5, -1.0, 0.0], false, vec![]),
      (-1.0, 2)
    );
    assert_eq!(
      get_min(&vec![EMPTY, 0.5, -1.0, 0.0], true, vec![]),
      (0.0, 3)
    );
    assert_eq!(
      get_min(&vec![EMPTY, 0.5, -1.0, -5.0], true, vec![3]),
      (0.0, 3)
    );
  }
}

fn to_minus(data: &Vec<f32>) -> Vec<f32> {
  data.iter().map(|el| el * -1.0).collect::<Vec<_>>()
}

#[derive(Debug, Clone)]
pub struct Base {
  x_n: Vec<Vec<f32>>,
  fx: Vec<f32>,
  basis: Vec<f32>,
  current_x_indexes: Vec<usize>,
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

fn is_whole(value: f32) -> bool {
  value.round() == get_clean_num_3(value)
}

fn is_whole_4(value: f32) -> bool {
  value.round() == get_clean_num_4(value)
}

fn basis_has_fractional(basis: &Vec<f32>, current_x_indexes: &Vec<usize>) -> bool {
  basis.iter().enumerate().any(|(index, el)| {
    if current_x_indexes.contains(&index) {
      !is_whole(*el)
    } else {
      false
    }
  })
}

mod Simplex {
  use super::*;

  fn get_marks_basis(
    x_n: &Vec<Vec<f32>>,
    fx: &Vec<f32>,
    basis: &Vec<f32>,
    current_x_indexes: &Vec<usize>,
  ) -> Vec<f32> {
    let (_, min_index) = get_min(&fx, false, vec![fx.len() - 1]);

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
    let (_, min_index) = get_min(&fx, false, vec![fx.len() - 1]);
    let (_, min_marks_index) = get_min(&marks, true, vec![]);
    let enabling_element = x_n[min_marks_index][min_index];

    return (enabling_element, min_marks_index, min_index);
  }

  fn is_xn_more_eq_zero(x_n: &Vec<Vec<f32>>) -> bool {
    x_n.iter().any(|el| {
      el.iter().enumerate().any(|(sub_index, sub_el)| {
        if sub_index == el.len() - 1 {
          return *sub_el < 0.0;
        }
        false
      })
    })
  }

  #[cfg(test)]
  mod tests_is_xn_more_eq_zero {
    use super::*;
    #[test]
    fn is_xn_more_eq_zero_main() {
      assert_eq!(
        is_xn_more_eq_zero(&vec![
          vec![],
          vec![],
          vec![2.0, 5.0, 1.0, 0.0, 0.0, 0.0],
          vec![5.0, 2.0, 0.0, 1.0, 0.0, 0.0],
          vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
          vec![0.0, 1.0, 0.0, 0.0, 0.0, -1.0],
        ]),
        true
      );

      // assert_eq!(
      //   is_xn_more_eq_zero(&vec![
      //     vec![],
      //     vec![],
      //     vec![2.0, 5.0, 1.0, 0.0, 0.0, 0.0],
      //     vec![5.0, 2.0, 0.0, 1.0, 0.0, 0.0],
      //     vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
      //     vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
      //   ]),
      //   false
      // );
    }
  }

  fn get_fx_big_m(fx: &Vec<f32>, x_n: &Vec<Vec<f32>>, basis: &Vec<f32>) -> Vec<f32> {
    const BIG_M: f32 = 100000.0;

    let mut val = Vec::new();

    for el in fx.clone() {
      val.push(el);
    }

    let len = val.len();
    val[len - 3] = BIG_M;

    for index in 0..val.len() {
      let value = if index == (val.len() - 1) {
        basis[basis.len() - 2]
      } else {
        x_n[x_n.len() - 2][index]
      };
      val[index] = -1.0 * value * BIG_M + val[index];
    }

    val
  }

  #[cfg(test)]
  mod tests_get_fx_big_m {
    use super::*;
    #[test]
    fn get_fx_big_m_main() {
      assert_eq!(
        get_fx_big_m(
          &vec![-3.0, -4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
          &vec![
            vec![],
            vec![],
            vec![2.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            vec![1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            vec![5.0, 4.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            vec![1.0, 2.0, 0.0, 0.0, 0.0, 1.0, -1.0],
            vec![],
          ],
          &vec![EMPTY, EMPTY, 600.0, 225.0, 1000.0, 150.0, EMPTY],
        ),
        [
          -100003.0,
          -200004.0,
          0.0,
          0.0,
          0.0,
          0.0,
          100000.0,
          -15000000.0
        ]
      );
    }
  }

  pub fn main(
    x_n: &Vec<Vec<f32>>,
    fx: &Vec<f32>,
    basis: &Vec<f32>,
    current_x_indexes: &Vec<usize>,
  ) -> Base {
    let mut x_n = x_n.clone();
    let mut fx = fx.clone();
    let mut basis = basis.clone();

    let mut current_x_indexes = if current_x_indexes.len() == 0 {
      get_initial_x_indexes(&x_n)
    } else {
      current_x_indexes.clone()
    };

    _print_2d("Xn", &x_n);
    let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
    _print_2d("XnF", &x_n_f);
    println!("Basis {:?}", basis);

    // Fx to -
    fx = to_minus(&fx);
    println!("Minus fx {:?}", fx);

    if is_xn_more_eq_zero(&x_n) {
      fx = get_fx_big_m(&fx, &x_n, &basis);
    }

    let mut is_fx_has_minus = fx_has_minus(&fx);
    let mut index = 0;

    while is_fx_has_minus {
      println!("\n---Cycle Simplex--- {index}");

      let mut check_fx = fx.clone();
      // Marks
      let marks = get_marks_basis(&x_n, &fx, &basis, &current_x_indexes);
      println!("Marks {:?}", marks);

      // Enabling element
      let (enabling_element, ee_row_index, ee_cell_index) = get_enabling_element(&x_n, &fx, &marks);
      println!(
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
      let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
      _print_2d("XnF", &x_n_f);
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
      let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
      _print_2d("XnF", &x_n_f);
      println!("Basis {:?} \nFx {:?}", basis, fx);

      is_fx_has_minus = fx_has_minus(&fx);

      index += 1;

      if check_fx == fx {
        println!("No results");
        break;
      }
    }

    Base {
      x_n,
      fx,
      basis,
      current_x_indexes,
    }
  }

  #[cfg(test)]
  mod tests_count_simplex {
    use super::*;
    #[test]
    fn count_simplex_main() {
      // assert_eq!(
      //   count_simplex(
      //     &vec![
      //       vec![],
      //       vec![],
      //       vec![1.0, 1.0, 1.0, 0.0],
      //       vec![10.0, 6.0, 0.0, 1.0],
      //     ],
      //     &vec![5.0, 4.0, 0.0, 0.0, 0.0],
      //     &vec![EMPTY, EMPTY, 5.0, 45.0],
      //     &vec![],
      //   )
      //   .fx,
      //   vec![0.0, 0.0, 2.5000002, 0.24999997, 23.75]
      // );

      // assert_eq!(
      //   count_simplex(
      //     &vec![
      //       vec![],
      //       vec![],
      //       vec![1.0, 1.0, 1.0, 0.0, 0.0],
      //       vec![10.0, 6.0, 0.0, 1.0, 0.0],
      //       vec![1.0, 0.0, 0.0, 0.0, 1.0],
      //     ],
      //     &vec![5.0, 4.0, 0.0, 0.0, 0.0, 0.0],
      //     &vec![EMPTY, EMPTY, 5.0, 45.0, 3.0],
      //     &vec![],
      //   )
      //   .fx,
      //   vec![0.0, 0.0, 4.0, 0.0, 1.0, 23.0]
      // );

      // assert_eq!(
      //   count_simplex(
      //     &vec![
      //       vec![],
      //       vec![],
      //       vec![1.0, 1.0, 1.0, 0.0, 0.0],
      //       vec![10.0, 6.0, 0.0, 1.0, 0.0],
      //       vec![1.0, 0.0, 0.0, 0.0, -1.0],
      //     ],
      //     &vec![5.0, 4.0, 0.0, 0.0, 0.0, 0.0],
      //     &vec![EMPTY, EMPTY, 5.0, 45.0, 4.0],
      //     &vec![],
      //   )
      //   .fx,
      //   vec![0.0, 0.0, 0.0, 0.6666666, 1.6666666, 23.333334]
      // );

      // assert_eq!(
      //   count_simplex(
      //     &vec![
      //       vec![],
      //       vec![],
      //       vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0],
      //       vec![10.0, 6.0, 0.0, 1.0, 0.0, 0.0],
      //       vec![1.0, 0.0, 0.0, 0.0, -1.0, 0.0],
      //       vec![0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
      //     ],
      //     &vec![5.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      //     &vec![EMPTY, EMPTY, 5.0, 45.0, 4.0, 0.0],
      //     &vec![],
      //   )
      //   .fx,
      //   vec![0.0, 0.0, 0.0, 0.5, 0.0, 1.0, 22.5]
      // );

      // assert_eq!(
      //   count_simplex(
      //     &vec![
      //       vec![],
      //       vec![],
      //       vec![2.0, 5.0, 1.0, 0.0, 0.0, 0.0],
      //       vec![5.0, 2.0, 0.0, 1.0, 0.0, 0.0],
      //       vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
      //       vec![0.0, 1.0, 0.0, 0.0, 1.0, -1.0],
      //       vec![],
      //     ],
      //     &vec![3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
      //     &vec![EMPTY, EMPTY, 13.0, 15.0, 2.0, 2.0, EMPTY],
      //     &vec![],
      //   )
      //   .fx,
      //   vec![0.0, 0.0, 1.5, 0.0, 99994.5, 5.5, 8.5]
      // );

      // f = 3x1 + 4x2 -> max
      // 2x1 + x2 <= 600,
      // x1 + x2 <= 225,
      // 5x1 + 4x2 <= 1000,
      // x1 + 2x2 >= 150,
      assert_eq!(
        main(
          &vec![
            vec![],
            vec![],
            vec![2.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            vec![1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0],
            vec![5.0, 4.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            vec![1.0, 2.0, 0.0, 0.0, 0.0, 1.0, -1.0],
            vec![],
          ],
          &vec![3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
          &vec![EMPTY, EMPTY, 600.0, 225.0, 1000.0, 150.0, EMPTY],
          &vec![],
        )
        .fx,
        vec![1.0, 0.0, 0.0, 4.0, 0.0, 100000.0, 0.0, 900.0]
      );

      // N = 4
      // f = 3x1 + 2x2 -> max
      // 2x1 + 5x2 <= 9 + N,
      // 5x1 + 2x2 <= 11 + N,
      // x1 >= 3,
      assert_eq!(
        main(
          &vec![
            vec![],
            vec![],
            vec![2.0, 5.0, 1.0, 0.0, 0.0, 0.0],
            vec![5.0, 2.0, 0.0, 1.0, 0.0, 0.0],
            vec![1.0, 0.0, 0.0, 0.0, 1.0, -1.0],
            vec![],
          ],
          &vec![3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
          &vec![EMPTY, EMPTY, 13.0, 15.0, 3.0, EMPTY],
          &vec![],
        )
        .fx,
        vec![0.0, 39999.2, 0.0, 20000.6, 0.0, 100000.0, 9.0]
      );

      // N = 2
      // f = 3x1 + 2x2 -> max
      // 2x1 + 5x2 <= 9 + N,
      // 5x1 + 2x2 <= 11 + N,
      // x1 >= 3,
      assert_eq!(
        main(
          &vec![
            vec![],
            vec![],
            vec![2.0, 5.0, 1.0, 0.0, 0.0, 0.0],
            vec![5.0, 2.0, 0.0, 1.0, 0.0, 0.0],
            vec![1.0, 0.0, 0.0, 0.0, 1.0, -1.0],
            vec![],
          ],
          &vec![3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0],
          &vec![EMPTY, EMPTY, 11.0, 13.0, 3.0, EMPTY],
          &vec![],
        )
        .fx,
        vec![0.0, 39999.2, 0.0, 20000.6, 0.0, 100000.0, 9.0]
      );
    }
  }
}

mod Gomori {
  use super::*;

  mod tests_basis_has_fractional {
    use super::*;
    #[test]
    fn get_whole_frac_full() {
      assert_eq!(
        basis_has_fractional(&vec![1.0, 2.0, 3.0], &vec![0, 1]),
        false
      );
      assert_eq!(
        basis_has_fractional(&vec![3.99990869, 2.0, 3.2], &vec![0, 1]),
        false
      );
      assert_eq!(
        basis_has_fractional(&vec![3.2, 2.0, 3.0], &vec![0, 1]),
        true
      );

      assert_eq!(
        basis_has_fractional(
          &vec![
            3.99990869,      // !
            2.00027871,      // !
            3.9987874,       // !
            -0.000101923943, // !
            -0.952000021,
            2.99839997, // !
            -0.936999976
          ],
          &vec![0, 1, 3, 2, 5]
        ),
        false
      );
    }
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
    // fx = to_minus(&fx);

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
          let mut val = el / x_n[last_index][index];
          if val < 0.0 {
            val *= -1.0;
          }
          val
        }
      })
      .collect::<Vec<_>>()
  }

  fn get_enabling_element_last(
    x_n: &Vec<Vec<f32>>,
    marks: &Vec<f32>,
    current_x_indexes: &Vec<usize>,
  ) -> (f32, usize, usize) {
    let min_index = *current_x_indexes.last().unwrap();
    let (_, min_marks_index) = get_min(&marks, false, vec![]);
    let enabling_element = x_n[min_index][min_marks_index];

    return (enabling_element, min_index, min_marks_index);
  }

  pub fn main(x_n: &Vec<Vec<f32>>, fx: &Vec<f32>, basis: &Vec<f32>, result_fx: Vec<f32>) {
    let mut x_n = x_n.clone();
    let mut fx = fx.clone();
    let mut basis = basis.clone();

    let mut current_x_indexes = get_initial_x_indexes(&x_n);

    println!("Data");
    _print_2d("Xn", &x_n);
    let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
    _print_2d("XnF", &x_n_f);
    println!("Fx {:?}", fx);
    println!("Basis {:?}", basis);
    println!("Current indexes {:?}", current_x_indexes);

    println!("Count Simplex");
    let base = Simplex::main(&x_n, &fx, &basis, &current_x_indexes);

    x_n = base.x_n;
    fx = base.fx;
    basis = base.basis;
    current_x_indexes = base.current_x_indexes;

    println!("Count Gomori");
    let mut is_basis_has_fractional = basis_has_fractional(&basis, &current_x_indexes);
    println!("Basis has fractional {:?}", is_basis_has_fractional);

    let mut index = 0;

    while is_basis_has_fractional {
      println!("\n---Cycle Fractional--- {index}");

      (x_n, fx, basis, current_x_indexes) = increase_x(&x_n, &fx, &basis, &current_x_indexes);
      _print_2d("Xn", &x_n);
      let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
      _print_2d("XnF", &x_n_f);
      println!("Fx {:?}", fx);
      println!("Basis {:?}", basis);

      let mut marks = get_marks_fx(&x_n, &fx, &current_x_indexes);
      // marks = to_minus(&marks);
      println!("Marks {:?}", marks);

      // Enabling element
      let (enabling_element, ee_row_index, ee_cell_index) =
        get_enabling_element_last(&x_n, &marks, &current_x_indexes);
      println!(
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
      let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
      _print_2d("XnF", &x_n_f);
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
      let x_n_f = get_xn_based_on_current_indexes(&x_n, &current_x_indexes);
      _print_2d("XnF", &x_n_f);
      println!("Basis {:?} \nFx {:?}", basis, fx);

      is_basis_has_fractional = basis_has_fractional(&basis, &current_x_indexes);

      index += 1;
    }

    assert_eq!(fx, result_fx);
  }
}

mod Branch_Bound {
  use super::*;
  fn get_whole_ranges(base: &Base) -> ([f32; 2], usize) {
    let indexes = [0, 1];
    let mut index = 0;
    let mut value = base.basis[indexes[index]];
    if is_whole_4(value) {
      index = 1;
      value = base.basis[indexes[index]];
    }
    ([value.floor(), value.ceil()], index)
  }

  pub fn main(base: Base, result_bassis: Vec<f32>) {
    let mut base = base.clone();
    let copied_base = base.clone();

    println!("Data");
    _print_2d("Xn", &base.x_n);
    let x_n_f = get_xn_based_on_current_indexes(&base.x_n, &base.current_x_indexes);
    _print_2d("XnF", &x_n_f);
    println!("Fx {:?}", base.fx);
    println!("Basis {:?}", base.basis);
    println!("Current indexes {:?}", base.current_x_indexes);

    base = Simplex::main(&base.x_n, &base.fx, &base.basis, &base.current_x_indexes);

    println!("{:?}", base);

    println!("Count Branch bound");
    let is_basis_has_fractional = basis_has_fractional(&base.basis, &base.current_x_indexes);
    println!("Basis has fractional {:?}", is_basis_has_fractional);

    #[derive(Debug)]
    struct Node {
      children: Vec<Node>,
      base: Base,
      result_base: Base,
    }

    impl Node {
      pub fn new(base: Base, result_base: Base) -> Node {
        Node {
          children: Vec::new(),
          base,
          result_base,
        }
      }
    }

    static mut MAX_BASE: Option<Base> = None;

    fn walk(node: &mut Node, index: usize) {
      println!("\n---Cycle Branches--- {index}");

      let (whole_ranges, whole_ranges_index) = get_whole_ranges(&node.result_base);
      println!("Whole ranges {:?}", whole_ranges);

      for i in 0..whole_ranges.len() {
        let mut base = node.base.clone();
        base = {
          base.basis.push(whole_ranges[i]);
          base.fx.push(0.0);

          for sub_el in base.x_n.iter_mut() {
            if sub_el.len() > 0 {
              sub_el.push(0.0);
              if i == 1 {
                sub_el.push(0.0);
              }
            }
          }
          let mut val = if whole_ranges_index == 0 {
            vec![1.0, 0.0]
          } else {
            vec![0.0, 1.0]
          };

          for _ in 3..(base.fx.len() - 1) {
            val.push(0.0);
          }
          val.push(1.0);
          base.x_n.push(val);

          if i == 1 {
            base.fx.push(0.0);
            let len_x_n = base.x_n.len();
            base.x_n[len_x_n - 1].push(-1.0);
            base.x_n.push(vec![]);
            base.basis.push(EMPTY);
          }
          base
        };
        _print_2d("Xn", &base.x_n);
        let x_n_f = get_xn_based_on_current_indexes(&base.x_n, &base.current_x_indexes);
        _print_2d("XnF", &x_n_f);
        println!("Fx {:?}", base.fx);
        println!("Basis {:?}", base.basis);
        let copied_base = base.clone();

        base = Simplex::main(&base.x_n, &base.fx, &base.basis, &vec![]);

        let is_basis_has_fractional = basis_has_fractional(&base.basis, &base.current_x_indexes);

        let last_value = base.fx.last().unwrap();

        let mut add_to_tree = true;

        unsafe {
          match &MAX_BASE {
            Some(origin) => {
              let max_last_value = origin.fx.last().unwrap();
              if max_last_value >= last_value {
                add_to_tree = false;
                continue;
              }
            }
            None => {}
          }

          if !is_basis_has_fractional && add_to_tree {
            MAX_BASE = Some(base.clone());
            println!("Max base {:?} {:?}", base.fx, base.basis);
            add_to_tree = false;
          }
        }

        if add_to_tree {
          node.children.push(Node::new(copied_base, base));
        }
      }

      for el in &mut node.children {
        walk(el, index + 1);
      }
    }

    let mut root = Node::new(copied_base, base);

    walk(&mut root, 0);

    unsafe {
      match &MAX_BASE {
        Some(origin) => {
          println!("Max base {:?} {:?}", origin.fx, origin.basis);
          assert_eq!(result_bassis, origin.fx);
        }
        None => {}
      }
    }
  }
}

fn task_0_gomori_yt() {
  // f = 2x1 + 3x2 -> max
  // x1 + 4x2 ≤ 14
  // 2x1 + 3x2 ≤ 12

  let x_n: Vec<Vec<f32>> = vec![
    vec![],
    vec![],
    vec![1.0, 4.0, 1.0, 0.0],
    vec![2.0, 3.0, 0.0, 1.0],
  ];
  let fx: Vec<f32> = vec![2.0, 3.0, 0.0, 0.0, 0.0];
  let basis: Vec<f32> = vec![EMPTY, EMPTY, 14.0, 12.0];

  Gomori::main(
    &x_n,
    &fx,
    &basis,
    vec![-0.0, -0.0, -0.0, 1.0, 0.0, 0.0, 12.0],
  );
}

fn task_1_gomori() {
  // f = 3x1 + 2x2 -> max
  // 2x1 + 5x2 ≤ 9 + N,
  // 5x1 + 2x2 ≤ 11 + N,
  // xi ≥ 0, xi – whole, i=1, 2,

  let n: f32 = 13.0;
  let x_n: Vec<Vec<f32>> = vec![
    vec![],
    vec![],
    vec![2.0, 5.0, 1.0, 0.0],
    vec![5.0, 2.0, 0.0, 1.0],
  ];
  let fx: Vec<f32> = vec![3.0, 2.0, 0.0, 0.0, 0.0];
  let basis: Vec<f32> = vec![EMPTY, EMPTY, 9.0 + n, 11.0 + n];

  Gomori::main(
    &x_n,
    &fx,
    &basis,
    vec![0.0, 0.0, 0.0, 0.0, 0.40000004, 0.0, 0.19999997, 16.000284],
  )
}

fn task_2_branch_bound_yt() {
  // f = 5x1 + 4x2 -> max
  // x1 + x2 ≤ 5,
  // 10x1 + 6x2 ≤ 45,
  // xi ≥ 0, xi – whole, i=1, 2,

  let base = Base {
    x_n: vec![
      vec![],
      vec![],
      vec![1.0, 1.0, 1.0, 0.0],
      vec![10.0, 6.0, 0.0, 1.0],
    ],
    fx: vec![5.0, 4.0, 0.0, 0.0, 0.0],
    basis: vec![EMPTY, EMPTY, 5.0, 45.0],
    current_x_indexes: vec![],
  };

  Branch_Bound::main(base, vec![0.0, 0.0, 4.0, 0.0, 1.0, 23.0]);
}

fn task_2_branch_bound() {
  // f = 3x1 + 2x2 -> max
  // 2x1 + 5x2 ≤ 9 + N,
  // 5x1 + 2x2 ≤ 11 + N,
  // xi ≥ 0, xi – whole, i=1, 2,

  let n: f32 = 13.0;

  Branch_Bound::main(
    Base {
      x_n: vec![
        vec![],
        vec![],
        vec![2.0, 5.0, 1.0, 0.0],
        vec![5.0, 2.0, 0.0, 1.0],
      ],
      fx: vec![3.0, 2.0, 0.0, 0.0, 0.0],
      basis: vec![EMPTY, EMPTY, 9.0 + n, 11.0 + n],
      current_x_indexes: vec![],
    },
    vec![0.0, 0.0, 0.0, 1.0, 99998.0, 1.9999999, 16.0],
  );
}

fn main() {
  // task_0_gomori_yt();
  // task_1_gomori();
  task_2_branch_bound_yt();
  // task_2_branch_bound();
}
