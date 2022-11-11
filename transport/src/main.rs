// M|i - place
// N|j - move

const EMPTY: i32 = -777;

fn _convert_nested_a_v<T: std::clone::Clone, const N: usize, const M: usize>(
  arr: [[T; M]; N],
) -> Vec<Vec<T>> {
  let mut ret = Vec::with_capacity(N);
  for v in arr.iter() {
    ret.push(v.to_vec());
  }
  ret
}

fn _get_empty_2d_array(data: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut result = Vec::new();
  for item in data {
    let v = vec![0; item.len()];
    result.push(v);
  }
  return result;
}

fn is_closed(production_place: &Vec<i32>, production_need: &Vec<i32>) -> (bool, i32) {
  let sum_p_place: i32 = production_place.iter().sum();
  let sum_p_need: i32 = production_need.iter().sum();

  if sum_p_need == sum_p_place {
    return (true, 0);
  } else {
    return (false, sum_p_place - sum_p_need);
  }
}

fn get_data_if_opened(
  data: &Vec<Vec<i32>>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
) -> (Vec<Vec<i32>>, Vec<i32>, Vec<i32>) {
  let mut new_data = data.clone();
  let mut new_production_place = production_place.clone();
  let mut new_production_need = production_need.clone();

  let (closed, diff_value) = is_closed(production_place, production_need);
  if closed {
    return (new_data, new_production_place, new_production_need);
  }

  if diff_value > 0 {
    for i in 0..new_data.len() {
      new_data[i].push(0);
    }

    new_production_need.push(diff_value);
  } else {
    new_data.push(vec![0; data[0].len()]);
    new_production_place.push(diff_value * -1);
  }

  return (new_data, new_production_place, new_production_need);
}

fn is_degenerated(send_list: &Vec<Vec<i32>>, data: &Vec<Vec<i32>>) -> bool {
  let mut count_filled = 0;

  for i in 0..send_list.len() {
    for j in 0..send_list[i].len() {
      if send_list[i][j] > 0 {
        count_filled += 1;
      }
    }
  }

  return data.len() + data[0].len() - 1 == count_filled;
}

// green squares
fn get_send_list_min_price(
  data: &Vec<Vec<i32>>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
) -> Vec<Vec<i32>> {
  let mut min_list = Vec::new();
  for i in 0..data.len() {
    for j in 0..data[i].len() {
      min_list.push([data[i][j], i as i32, j as i32]);
    }
  }

  min_list.sort();

  println!("Min {:?}", min_list);

  let mut send_list = _get_empty_2d_array(data);
  let mut new_production_place = production_place.clone();
  let mut new_production_need = production_need.clone();

  for i in &min_list {
    let mut val = new_production_need[i[2] as usize];
    if val > new_production_place[i[1] as usize] {
      val = new_production_place[i[1] as usize];
    }

    send_list[i[1] as usize][i[2] as usize] = val;
    new_production_place[i[1] as usize] -= val;
    new_production_need[i[2] as usize] -= val;
  }

  return send_list;
}

fn get_send_list_diagonal(
  data: &Vec<Vec<i32>>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
) -> Vec<Vec<i32>> {
  let mut send_list = _get_empty_2d_array(data);
  let mut new_production_place = production_place.clone();
  let mut new_production_need = production_need.clone();

  let mut i_index = 0;
  let mut j_index = 0;

  loop {
    let sum_production_place1: i32 = new_production_place.iter().sum();
    let sum_production_need1: i32 = new_production_place.iter().sum();

    if sum_production_place1 == 0 && sum_production_need1 == 0 {
      break;
    }

    if new_production_place[i_index] == new_production_need[j_index] {
      send_list[i_index][j_index] = new_production_place[i_index];
      new_production_place[i_index] = 0;
      new_production_need[j_index] = 0;
      i_index += 1;
      j_index += 1;
    } else if new_production_place[i_index] > new_production_need[j_index] {
      send_list[i_index][j_index] = new_production_need[j_index];
      new_production_place[i_index] -= new_production_need[j_index];
      new_production_need[j_index] = 0;
      j_index += 1;
    } else {
      send_list[i_index][j_index] = new_production_place[i_index];
      new_production_need[j_index] -= new_production_place[i_index];
      new_production_place[i_index] = 0;
      i_index += 1;
    }
  }

  return send_list;
}

const DEGENERATED_FILLED: i32 = -888;
fn get_send_list_if_not_degenerated(
  send_list: &Vec<Vec<i32>>,
  data: &Vec<Vec<i32>>,
) -> Vec<Vec<i32>> {
  let mut result = send_list.clone();
  let degenerated = is_degenerated(send_list, data);
  if degenerated {
    return result;
  };

  let mut min_value = data[0][0];
  let mut index = [0, 0];

  for i in 0..data.len() {
    for j in 0..data[i].len() {
      if min_value > data[i][j] && send_list[i][j] == 0 && data[i][j] > 0 {
        min_value = data[i][j];
        index = [i, j];
      }
    }
  }

  result[index[0]][index[1]] = DEGENERATED_FILLED;

  return result;
}

// potentials
fn get_uv_lines(send_list: &Vec<Vec<i32>>, data: &Vec<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {
  let mut u_line = vec![EMPTY; data.len()];
  u_line[0] = 0;

  let mut v_line = vec![EMPTY; data[0].len()];

  let mut count_filled = u_line.len() + v_line.len() - 1;

  while count_filled > 0 {
    for i in 0..send_list.len() {
      for j in 0..send_list[i].len() {
        if (send_list[i][j] > 0 || send_list[i][j] == DEGENERATED_FILLED)
          && (u_line[i] != EMPTY || v_line[j] != EMPTY)
          && (u_line[i] == EMPTY || v_line[j] == EMPTY)
        {
          count_filled -= 1;
          if u_line[i] == EMPTY {
            u_line[i] = data[i][j] - v_line[j];
          } else {
            v_line[j] = data[i][j] - u_line[i];
          }
        }
      }
    }
  }

  return (u_line, v_line);
}

fn get_uv_lines_price(
  u_line: &Vec<i32>,
  v_line: &Vec<i32>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
) -> i32 {
  let mut sum = 0;
  for i in 0..u_line.len() {
    sum += u_line[i] * production_place[i] as i32;
  }
  for i in 0..v_line.len() {
    sum += v_line[i] * production_need[i] as i32;
  }
  return sum;
}

fn get_corners(
  data: &Vec<Vec<i32>>,
  send_list: &Vec<Vec<i32>>,
  u_line: &Vec<i32>,
  v_line: &Vec<i32>,
) -> Vec<Vec<i32>> {
  let mut corners = data.clone();

  for i in 0..send_list.len() {
    for j in 0..send_list[i].len() {
      if send_list[i][j] == 0 {
        // Alternative way
        // corners[i][j] = (u_line[i] + v_line[j]) - data[i][j] as i32;
        corners[i][j] = data[i][j] - (u_line[i] + v_line[j]);
      } else {
        corners[i][j] = EMPTY;
      }
    }
  }

  return corners;
}

fn is_optimal(corners: &Vec<Vec<i32>>) -> bool {
  let mut result = true;
  for i in 0..corners.len() {
    for j in 0..corners[i].len() {
      // Alternative way
      // if corners[i][j] != EMPTY && corners[i][j] > 0 {
      if corners[i][j] != EMPTY && corners[i][j] < 0 {
        result = false;
        break;
      }
    }
  }
  return result;
}

// TODO: should be found programmatically
fn get_matrix_indexes(corners: &Vec<Vec<i32>>, send_list: &Vec<Vec<i32>>) {
  let mut max_delta = 0;
  let mut index_max_delta = [0, 0];

  for i in 0..corners.len() {
    for j in 0..corners[i].len() {
      // Alternative way
      // if max_delta < corners[i][j] && corners[i][j] != EMPTY {
      if max_delta > corners[i][j] && corners[i][j] != EMPTY {
        max_delta = corners[i][j];
        index_max_delta = [i, j];
      }
    }
  }

  println!("index_max_delta {:?}", max_delta);
  println!("index_max_delta {:?}", index_max_delta);
  println!("send_list {:?}", send_list);
}

fn get_new_send_list(matrix_indexes: Vec<[usize; 2]>, send_list: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut min_minus_value = EMPTY;

  for i in 0..matrix_indexes.len() {
    if i % 2 != 0 && i > 0 {
      let val = send_list[matrix_indexes[i][0]][matrix_indexes[i][1]] as i32;
      if (min_minus_value == EMPTY || min_minus_value > val) && val > 0 {
        min_minus_value = val;
      }
    }
  }

  println!("min_minus_value {}", min_minus_value);

  let mut result = send_list.clone();

  for i in 0..matrix_indexes.len() {
    let val = result[matrix_indexes[i][0]][matrix_indexes[i][1]];
    if val == DEGENERATED_FILLED {
      result[matrix_indexes[i][0]][matrix_indexes[i][1]] = 0;
    }

    if i % 2 == 0 {
      result[matrix_indexes[i][0]][matrix_indexes[i][1]] += min_minus_value;
    } else {
      result[matrix_indexes[i][0]][matrix_indexes[i][1]] -= min_minus_value;
    }
  }

  return result;
}

fn get_price(data: &Vec<Vec<i32>>, send_list: &Vec<Vec<i32>>) -> i32 {
  let mut sum = 0;
  for i in 0..send_list.len() {
    for j in 0..send_list[i].len() {
      if send_list[i][j] > 0 {
        sum += send_list[i][j] as i32 * data[i][j] as i32;
      }
    }
  }
  return sum;
}

fn has_alternative(corners: &Vec<Vec<i32>>) -> bool {
  let mut result = false;
  for i in 0..corners.len() {
    for j in 0..corners[i].len() {
      if corners[i][j] == 0 {
        result = true;
        break;
      }
    }
  }
  return result;
}

fn check_price(
  data: &Vec<Vec<i32>>,
  send_list: &Vec<Vec<i32>>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
  u_line: &Vec<i32>,
  v_line: &Vec<i32>,
) -> (bool, i32, i32) {
  let uv_lines_price = get_uv_lines_price(&u_line, &v_line, &production_place, &production_need);
  let price = get_price(&data, &send_list);

  return (uv_lines_price == price, uv_lines_price, price);
}

fn transport_task<const N0: usize, const M: usize, const N: usize>(
  data: [[i32; N]; M],
  production_place: [i32; M],
  production_need: [i32; N],
  matrix_indexes_list: [Vec<[usize; 2]>; N0],
  result_price: i32,
  is_diagonal: bool,
) {
  let mut data = _convert_nested_a_v(data);
  let mut production_place = production_place.to_vec();
  let mut production_need = production_need.to_vec();

  // Steps
  let closed = is_closed(&production_place, &production_need);
  println!("Closed: {:?}", closed);

  (data, production_place, production_need) =
    get_data_if_opened(&data, &production_place, &production_need);

  let mut send_list;
  if is_diagonal {
    send_list = get_send_list_diagonal(&data, &production_place, &production_need);
  } else {
    send_list = get_send_list_min_price(&data, &production_place, &production_need);
  }

  println!("Send list {:?}", send_list);

  let degenerated = is_degenerated(&send_list, &data);
  println!("Degenerated {:?}", degenerated);
  send_list = get_send_list_if_not_degenerated(&send_list, &data);

  let (u_line, v_line) = get_uv_lines(&send_list, &data);
  println!("U line V line {:?} {:?}", u_line, v_line);
  println!(
    "Check price {:?}",
    check_price(
      &data,
      &send_list,
      &production_place,
      &production_need,
      &u_line,
      &v_line,
    ),
  );

  let mut corners = get_corners(&data, &send_list, &u_line, &v_line);
  println!("Corners {:?}", corners);

  let mut optimal = is_optimal(&corners);
  let mut price = get_price(&data, &send_list);
  let mut index = 0;
  let mut alternative;

  println!("Optimal {:#?}", optimal);
  println!("Price {:#?}", price);

  while !optimal {
    println!("\n----- Cycle {} ------", index + 1);

    get_matrix_indexes(&corners, &send_list);
    let matrix_indexes = matrix_indexes_list[index].clone();

    send_list = get_new_send_list(matrix_indexes, &send_list);
    println!("Send list {:?}", send_list);

    let (u_line, v_line) = get_uv_lines(&send_list, &data);
    println!("U line V line {:?} {:?}", u_line, v_line);

    println!(
      "Check price {:?}",
      check_price(
        &data,
        &send_list,
        &production_place,
        &production_need,
        &u_line,
        &v_line,
      ),
    );

    corners = get_corners(&data, &send_list, &u_line, &v_line);
    println!("Corners {:?}", corners);

    optimal = is_optimal(&corners);
    println!("Optimal {:?}", optimal);
    price = get_price(&data, &send_list);
    println!("Price {:?}", price);
    alternative = has_alternative(&corners);
    println!("Alternative {:?}", alternative);

    index += 1;
  }

  assert_eq!(price, result_price, "Error result");
}

fn main() {
  fn task_00() {
    let data = [[1, 3, 2, 4], [2, 1, 4, 3], [3, 5, 6, 1]];
    let production_place = [35, 50, 15];
    let production_need = [30, 10, 20, 40];

    println!("\nTask youtube closed https://www.youtube.com/watch?v=1jBa_2IYDNY");

    transport_task(
      data,
      production_place,
      production_need,
      [[[1, 0], [0, 0], [0, 2], [1, 2]].to_vec()],
      185,
      false,
    );
  }

  fn task_0() {
    let data = [[2, 4, 1, 6], [2, 2, 4, 3], [1, 2, 3, 2]];
    let production_place = [70, 80, 50];
    let production_need = [40, 60, 25, 75];

    println!("\nTask 0");
    // [
    //   [0+, 0, 25, 45-],
    //   [0, 60, 0, 20],
    //   [40-, 0, 0, 10+]
    // ]

    // [
    //   [40, 0-, 25, 5+],
    //   [0, 60+, 0, 20-],
    //   [0, 0, 0, 50],
    // ]

    transport_task(
      data,
      production_place,
      production_need,
      [
        [[0, 0], [0, 3], [2, 3], [2, 0]].to_vec(),
        [[0, 1], [0, 3], [1, 3], [1, 1]].to_vec(),
      ],
      410,
      false,
    );
  }

  // degenerated
  fn task_1() {
    let data = [[2, 4, 1, 6], [2, 2, 4, 3], [1, 2, 3, 2]];
    let production_place = [50, 20, 30];
    let production_need = [30, 10, 25, 35];

    println!("\n=====> Task 1");
    // Diagonal
    // [
    //   [30, 10-, 10+, 0],
    //   [0, 0+, 15-, 5],
    //   [0, 0, 0, 30]
    // ]
    // [
    //   [30-, 0, 20+, 0],
    //   [0+, 10, 5-, 5],
    //   [0, 0, 0, 30]
    // ]

    // [[1, 1], [0, 1], [0, 2], [1, 2]].to_vec(),
    // [[1, 0], [0, 0], [0, 2], [1, 2]].to_vec(),

    // Min price
    // [
    //   [-888+, 0, 25, 25-],
    //   [0, 10, 0, 10],
    //   [30-, 0, 0, 0+]
    // ]
    transport_task(
      data,
      production_place,
      production_need,
      [[[2, 3], [2, 0], [0, 0], [0, 3]].to_vec()],
      180,
      false,
    );
  }

  fn task_2() {
    let data = [[4, 3, 5], [8, 7, 6], [4, 5, 9], [10, 9, 7]];
    let production_place = [60, 70, 30, 70];
    let production_need = [90, 90, 50];

    println!("\n=====> Task 2");
    // [
    //   [0, 60, 0],
    //   [0, 20+, 50-],
    //   [30, 0, 0],
    //   [60, 10-, 0+]
    // ]

    // [
    //   [0+, 60-, 0],
    //   [0, 30+, 40-],
    //   [30, 0, 0],
    //   [60-, 0, 10+]
    // ]

    transport_task(
      data,
      production_place,
      production_need,
      [
        [[3, 2], [3, 1], [1, 1], [1, 2]].to_vec(),
        [[0, 0], [0, 1], [1, 1], [1, 2], [3, 2], [3, 0]].to_vec(),
      ],
      1380,
      false,
    );
  }

  // opened/not degenerated
  fn task_000() {
    let data = [[4, 6, 3], [5, 5, 2]];
    let production_place = [2000, 2500];
    let production_need = [1000, 1300, 1200];

    println!(
      "\n=====> Task youtube opened not degenerated https://www.youtube.com/watch?v=qz3Vu1TR4Jk"
    );
    transport_task(data, production_place, production_need, [], 12900, false);
  }

  // opened
  fn task_3() {
    let data = [[700, 400], [200, 300], [100, 200]];
    let production_place = [4500, 3200, 5500];
    let production_need = [4700, 6500];

    println!("\n=====> Task 3");
    transport_task(data, production_place, production_need, [], 2590000, false);
  }

  // opened
  fn task_4() {
    let data = [[700, 400, 500], [200, 300, 400], [100, 200, 100]];
    let production_place = [4000, 3000, 5000];
    let production_need = [4700, 6500, 3500];

    println!("\n=====> Task 4");
    // [
    //   [0, 3500+, 500-],
    //   [0+, 3000-, 0],
    //   [2000-, 0, 3000+],
    //   [2700, 0, 0]
    // ]

    // [
    //   [0, 4000, 0],
    //   [500+, 2500-, 0],
    //   [1500, 0, 3500],
    //   [2700-, 0+, 0]
    // ]
    transport_task(
      data,
      production_place,
      production_need,
      [
        [[1, 0], [2, 0], [2, 2], [0, 2], [0, 1], [1, 1]].to_vec(),
        [[3, 1], [1, 1], [1, 0], [3, 0]].to_vec(),
      ],
      2700000,
      false,
    );
  }

  task_00();
  task_0();
  task_1();
  task_2();
  task_000();
  task_3();
  task_4();
}
