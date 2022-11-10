// M|i - place
// N|j - move

const EMPTY: i32 = -777;

fn _convert_v_a<T, const N: usize>(v: Vec<T>) -> [T; N] {
  v.try_into()
    .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn _get_empty_2d_array<const M: usize, const N: usize>(data: [[i32; M]; N]) -> [[i32; M]; N] {
  let mut v_result = Vec::new();
  for i in 0..data.len() {
    let mut vec = Vec::new();
    for j in 0..data[i].len() {
      vec.push(0);
    }
    let arr = _convert_v_a(vec);
    v_result.push(arr);
  }
  let result = _convert_v_a(v_result);
  return result;
}

fn is_closed<const M: usize>(
  production_place: [i32; M],
  production_move: &Vec<i32>,
) -> (bool, i32) {
  let sum_p_place: i32 = production_place.iter().sum();
  let sum_p_move: i32 = production_move.iter().sum();

  if sum_p_move == sum_p_place {
    return (true, 0);
  } else {
    return (false, sum_p_place - sum_p_move);
  }
}

fn get_data_if_opened(production_move: Vec<i32>) -> Vec<i32> {
  // let a:[i32; N0] = production_move as [i32; N0];
  // return a;
  let mut vec = production_move;
  vec.push(1);
  return vec;
}

fn is_degenerated<const M: usize, const N: usize>(
  send_list: [[i32; N]; M],
  data: [[i32; N]; M],
) -> bool {
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
fn get_send_list_min_price<const M: usize, const N: usize>(
  data: [[i32; N]; M],
  production_place: [i32; M],
  production_move: &Vec<i32>,
) -> [[i32; N]; M] {
  let mut min_list = Vec::new();
  for i in 0..data.len() {
    for j in 0..data[i].len() {
      min_list.push([data[i][j], i as i32, j as i32]);
    }
  }

  min_list.sort();

  println!("Min {:?}", min_list);

  let mut send_list = _get_empty_2d_array(data);
  let mut production_place1 = production_place;
  let mut production_move1 = production_move.clone();

  for i in &min_list {
    let mut val = production_move1[i[2] as usize];
    if val > production_place1[i[1] as usize] {
      val = production_place1[i[1] as usize];
    }

    send_list[i[1] as usize][i[2] as usize] = val;
    production_place1[i[1] as usize] -= val;
    production_move1[i[2] as usize] -= val;
  }

  return send_list;
}

fn get_send_list_diagonal<const M: usize, const N: usize>(
  data: [[i32; N]; M],
  production_place: [i32; M],
  production_move: [i32; N],
) -> [[i32; N]; M] {
  let mut send_list = _get_empty_2d_array(data);
  let mut production_place1 = production_place;
  let mut production_move1 = production_move;

  let mut i_index = 0;
  let mut j_index = 0;

  loop {
    let sum_production_place1: i32 = production_place1.iter().sum();
    let sum_production_move1: i32 = production_place1.iter().sum();

    if sum_production_place1 == 0 && sum_production_move1 == 0 {
      break;
    }

    if production_place1[i_index] == production_move1[j_index] {
      send_list[i_index][j_index] = production_place1[i_index];
      production_place1[i_index] = 0;
      production_move1[j_index] = 0;
      i_index += 1;
      j_index += 1;
    } else if production_place1[i_index] > production_move1[j_index] {
      send_list[i_index][j_index] = production_move1[j_index];
      production_place1[i_index] -= production_move1[j_index];
      production_move1[j_index] = 0;
      j_index += 1;
    } else {
      send_list[i_index][j_index] = production_place1[i_index];
      production_move1[j_index] -= production_place1[i_index];
      production_place1[i_index] = 0;
      i_index += 1;
    }
  }

  return send_list;
}

const DEGENERATED_FILLED: i32 = -888;
fn get_send_list_if_degenerated<const M: usize, const N: usize>(
  send_list: [[i32; N]; M],
  data: [[i32; N]; M],
) -> [[i32; N]; M] {
  let degenerated = is_degenerated(send_list, data);
  if !degenerated {
    return send_list;
  };

  let mut min_value = data[0][0];
  let mut result = send_list;

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
fn get_uv_lines<const M: usize, const N: usize>(
  send_list: [[i32; N]; M],
  data: [[i32; N]; M],
) -> ([i32; M], [i32; N]) {
  let mut v_result = Vec::new();
  for i in 0..data.len() {
    v_result.push(EMPTY);
  }
  let mut u_line = _convert_v_a(v_result);
  u_line[0] = 0;

  let mut v_result = Vec::new();
  for j in 0..data[0].len() {
    v_result.push(EMPTY);
  }
  let mut v_line = _convert_v_a(v_result);

  let mut count_filled = u_line.len() + v_line.len() - 1;

  while count_filled > 0 {
    for i in 0..send_list.len() {
      for j in 0..send_list[i].len() {
        if send_list[i][j] > 0
          || send_list[i][j] == DEGENERATED_FILLED
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

fn get_uv_lines_price<const M: usize, const N: usize>(
  u_line: [i32; M],
  v_line: [i32; N],
  production_place: [i32; M],
  production_move: Vec<i32>,
) -> i32 {
  let mut sum = 0;
  for i in 0..u_line.len() {
    sum += u_line[i] * production_place[i] as i32;
  }
  for i in 0..v_line.len() {
    sum += v_line[i] * production_move[i] as i32;
  }
  return sum;
}

fn get_corners<const M: usize, const N: usize>(
  data: [[i32; N]; M],
  send_list: [[i32; N]; M],
  u_line: [i32; M],
  v_line: [i32; N],
) -> [[i32; N]; M] {
  let mut corners = _get_empty_2d_array(data);

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

fn is_optimal<const M: usize, const N: usize>(corners: [[i32; N]; M]) -> bool {
  let mut optimal = true;
  for i in 0..corners.len() {
    for j in 0..corners[i].len() {
      // Alternative way
      // if corners[i][j] != EMPTY && corners[i][j] > 0 {
      if corners[i][j] != EMPTY && corners[i][j] < 0 {
        optimal = false;
        break;
      }
    }
  }
  return optimal;
}

// TODO: should be found programmatically
fn get_matrix_indexes<const M: usize, const N: usize>(
  corners: [[i32; N]; M],
  send_list: [[i32; N]; M],
) {
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

fn get_new_send_list<const N0: usize, const M: usize, const N: usize>(
  matrix_indexes: [[usize; 2]; N0],
  send_list: [[i32; N]; M],
) -> [[i32; N]; M] {
  let mut min_minus_value = EMPTY;

  for i in 0..matrix_indexes.len() {
    if i % 2 != 0 && i > 0 {
      let val = send_list[matrix_indexes[i][0]][matrix_indexes[i][1]] as i32;
      if min_minus_value == EMPTY || min_minus_value > val {
        min_minus_value = val;
      }
    }
  }

  println!("min_minus_value {}", min_minus_value);

  let mut send_list1 = send_list;

  for i in 0..matrix_indexes.len() {
    if i % 2 == 0 {
      send_list1[matrix_indexes[i][0]][matrix_indexes[i][1]] += min_minus_value;
    } else {
      send_list1[matrix_indexes[i][0]][matrix_indexes[i][1]] -= min_minus_value;
    }
  }

  return send_list1;
}

fn get_price<const M: usize, const N: usize>(data: [[i32; N]; M], send_list: [[i32; N]; M]) -> i32 {
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

fn main() {
  fn task_00() {
    let data = [[1, 3, 2, 4], [2, 1, 4, 3], [3, 5, 6, 1]];
    let production_place = [35, 50, 15];
    let production_move = [30, 10, 20, 40];
    let production_move = production_move.to_vec();

    // Steps
    let closed = is_closed(production_place, &production_move);
    println!("Closed: {:?}", closed);

    let a = [1, 2, 3];
    let b = [5];

    let send_list = get_send_list_min_price(data, production_place, &production_move);
    println!("Send list {:?}", send_list);

    let (u_line, v_line) = get_uv_lines(send_list, data);
    // println!("U line {:?}", u_line);
    // println!("V line {:?}", v_line);
    // println!("get_uv_lines_price {}", get_uv_lines_price(u_line, v_line, production_place, production_move));
    // println!("get_price {}", get_price(data, send_list));

    let corners = get_corners(data, send_list, u_line, v_line);
    // println!("Corners {:?}", corners);
    // let optimal = is_optimal(corners);
    // println!("Optimal {:#?}", optimal);
    // println!("Price {:#?}", get_price(data, send_list));

    // Cycle 2
    println!("Cycle 2");
    get_matrix_indexes(corners, send_list);
    let matrix_indexes = [[1, 0], [0, 0], [0, 2], [1, 2]];

    let send_list = get_new_send_list(matrix_indexes, send_list);
    // println!("Send list {:?}", send_list);
    // let (u_line, v_line) = get_uv_lines(send_list, data);
    // println!("U line {:?}", u_line);
    // println!("V line {:?}", v_line);

    println!(
      "get_uv_lines_price {}",
      get_uv_lines_price(u_line, v_line, production_place, production_move)
    );
    // println!("get_price {}", get_price(data, send_list));
    let corners = get_corners(data, send_list, u_line, v_line);
    println!("Corners {:?}", corners);
    let optimal = is_optimal(corners);
    println!("Optimal {:#?}", optimal);
    let price = get_price(data, send_list);
    println!("Price {:#?}", price);

    assert_eq!(price, 185, "Error result");
  }

  task_00();
}
