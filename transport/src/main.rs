// M|i - place
// N|j - move
// production_need -> bottom
// production_place -> right
// V - bottom
// U - right

// use serde::{Deserialize, Serialize};
// use serde_json::Result;

use std::time::Instant;

use serde_json::to_vec;

const EMPTY: i32 = -777;
const M: i32 = -999;

fn _print_2d(text: &str, vec: &Vec<Vec<i32>>) {
  println!("{}", text);
  println!("[");
  for el in vec {
    println!("{:?}", el);
  }
  println!("]");
}

fn _convert_nested_a_v<T: std::clone::Clone, const N: usize, const M: usize>(
  arr: [[T; M]; N],
) -> Vec<Vec<T>> {
  let mut result = Vec::with_capacity(N);
  for el in arr.iter() {
    result.push(el.to_vec());
  }
  result
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

#[cfg(test)]
mod tests_is_closed {
  use super::*;
  #[test]
  fn is_closed_true() {
    let production_place = vec![1, 2, 3];
    let production_need = vec![2, 1, 3];
    assert_eq!(is_closed(&production_place, &production_need), (true, 0));
  }
}

fn get_data_if_opened(
  data: &Vec<Vec<i32>>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
  part_index: Option<[usize; 2]>,
) -> (Vec<Vec<i32>>, Vec<i32>, Vec<i32>) {
  let mut new_data = data.clone();
  let mut new_production_place = production_place.clone();
  let mut new_production_need = production_need.clone();

  let (closed, diff_value) = is_closed(production_place, production_need);
  if closed {
    return (new_data, new_production_place, new_production_need);
  }

  if diff_value > 0 {
    if let Some(inx) = part_index {
      for i in 0..new_data.len() {
        new_data[i].push(if inx[0] > i { M } else { 0 });
      }
    } else {
      for i in 0..new_data.len() {
        new_data[i].push(0);
      }
    }

    new_production_need.push(diff_value);
  } else {
    if let Some(inx) = part_index {
      new_data.insert(inx[0], vec![0; data[0].len()]);
      let len = new_data[0].len();
      for i in 0..inx[1] {
        new_data[inx[0]][len - i - 1] = M;
      }
      new_production_place.insert(inx[0], diff_value * -1);
    } else {
      new_data.push(vec![0; data[0].len()]);
      new_production_place.push(diff_value * -1);
    }
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
  let mut zero_list = Vec::new();
  let mut m_list = Vec::new();
  for i in 0..data.len() {
    for j in 0..data[i].len() {
      let val = data[i][j];
      if val == M {
        m_list.push([val, i as i32, j as i32]);
      } else if val == 0 {
        zero_list.push([val, i as i32, j as i32]);
      } else {
        min_list.push([val, i as i32, j as i32]);
      }
    }
  }

  min_list.sort();
  min_list = [min_list, zero_list, m_list].concat();

  // println!("Min {:?}", min_list);

  let mut send_list = _get_empty_2d_array(data);
  let mut new_production_place = production_place.clone();
  let mut new_production_need = production_need.clone();

  for el in &min_list {
    let mut val = new_production_need[el[2] as usize];
    if val > new_production_place[el[1] as usize] {
      val = new_production_place[el[1] as usize];
    }

    send_list[el[1] as usize][el[2] as usize] = val;
    new_production_place[el[1] as usize] -= val;
    new_production_need[el[2] as usize] -= val;
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

fn get_gammas(
  data: &Vec<Vec<i32>>,
  send_list: &Vec<Vec<i32>>,
  u_line: &Vec<i32>,
  v_line: &Vec<i32>,
) -> Vec<Vec<i32>> {
  let mut gammas = data.clone();

  for i in 0..send_list.len() {
    for j in 0..send_list[i].len() {
      if data[i][j] == M {
        gammas[i][j] = M;
      } else if send_list[i][j] == 0 {
        // Alternative way
        // gammas[i][j] = (u_line[i] + v_line[j]) - data[i][j] as i32;
        gammas[i][j] = data[i][j] - (u_line[i] + v_line[j]);
      } else {
        gammas[i][j] = EMPTY;
      }
    }
  }

  return gammas;
}

fn is_optimal(gammas: &Vec<Vec<i32>>) -> bool {
  let mut result = true;
  for i in 0..gammas.len() {
    for j in 0..gammas[i].len() {
      // Alternative way
      // if gammas[i][j] != EMPTY && gammas[i][j] != M && gammas[i][j] > 0 {
      if gammas[i][j] != EMPTY && gammas[i][j] != M && gammas[i][j] < 0 {
        result = false;
        break;
      }
    }
  }
  return result;
}

fn get_matrix_indexes(
  gammas: &Vec<Vec<i32>>,
  send_list: &Vec<Vec<i32>>,
  skip_indexes: &Vec<[usize; 2]>,
) -> (Vec<Vec<[usize; 2]>>, Vec<[usize; 2]>) {
  let mut max_delta = 0;
  let mut index_max_delta = [0, 0];

  for i in 0..gammas.len() {
    for j in 0..gammas[i].len() {
      // Alternative way
      // if max_delta < gammas[i][j] && gammas[i][j] != EMPTY && gammas[i][j] != M {
      if max_delta > gammas[i][j] && gammas[i][j] != EMPTY && gammas[i][j] != M {
        max_delta = gammas[i][j];
        index_max_delta = [i, j];
      }
    }
  }

  if max_delta == 0 {
    for i in 0..gammas.len() {
      for j in 0..gammas[i].len() {
        if gammas[i][j] == 0 && !skip_indexes.contains(&[i, j]) {
          index_max_delta = [i, j];
          break;
        }
      }
    }
  }

  println!("DATA for get_matrix_indexes");
  _print_2d("Gammas", &gammas);
  _print_2d("Send list", &send_list);
  println!("Max delta {:?}", max_delta);
  println!("Index max delta {:?}", index_max_delta);

  // #[derive(Debug, Serialize)]
  #[derive(Debug)]
  struct Node {
    children: Vec<Node>,
    index: [usize; 2],
    parent_indexes: Vec<[usize; 2]>,
    found: bool,
  }

  impl Node {
    pub fn new(index: [usize; 2], parent_indexes: Vec<[usize; 2]>) -> Node {
      Node {
        children: Vec::new(),
        index,
        parent_indexes,
        found: false,
      }
    }
  }

  // [
  //    0   1  2  3
  //   [0+, 0, 0, 45-], 0
  //   [0, 60, 0, 20], 1
  //   [40-, 0, 0, 10+] 2
  // ]
  // 0,0 -> 0,3 -> 1,3 -> 2,3 -> 2,0 -> found !
  //                   -> 1,1 -> false !
  //            -> 2,3 -> 2,0 -> found
  //                   -> 1,3 -> 1,1 -> false
  //     -> 2,0 -> 2,3 -> 1,3 -> 1,1 -> false
  //                          -> 0,3 -> found
  //                   -> 0,3 -> found

  fn is_root(node: &Node) -> bool {
    return node.parent_indexes.len() == 0;
  }

  fn next(node: &mut Node, send_list: &Vec<Vec<i32>>, index_max_delta: [usize; 2]) {
    let [i_index, j_index] = node.index;

    for i in 0..4 {
      let mut new_i_index = i_index;
      let mut new_j_index = j_index;
      loop {
        if i == 0 {
          new_j_index += 1;
        } else if i == 1 {
          new_i_index += 1;
        } else if i == 2 {
          if new_j_index == 0 {
            break;
          }
          new_j_index -= 1;
        } else {
          if new_i_index == 0 {
            break;
          }
          new_i_index -= 1;
        }

        if new_j_index == send_list[0].len() || new_i_index == send_list.len() {
          break;
        }

        let index = [new_i_index, new_j_index];

        if send_list[new_i_index][new_j_index] != 0 && !node.parent_indexes.contains(&index) {
          let mut parent_indexes = node.parent_indexes.clone();
          parent_indexes.push(node.index);
          node.children.push(Node::new(index, parent_indexes));
        }

        fn is_line_changed(index_max_delta: [usize; 2], path: &Vec<[usize; 2]>) -> bool {
          let mut lines_changed = (false, false);
          for el in path.iter() {
            if el[0] != index_max_delta[0] {
              lines_changed.0 = true;
            }
            if el[1] != index_max_delta[1] {
              lines_changed.1 = true;
            }
          }

          return lines_changed.0 && lines_changed.1;
        }

        let mut path = node.parent_indexes.clone();
        path.push(node.index);
        if index == index_max_delta
          && node.parent_indexes.len() > 1
          && is_line_changed(index_max_delta, &path)
        {
          node.children = Vec::new();
          node.found = true;
          return;
        }
      }
    }
  }

  let mut root = Node::new(index_max_delta, Vec::new());

  fn walk(node: &mut Node, send_list: &Vec<Vec<i32>>, index_max_delta: [usize; 2]) {
    if is_root(node) {
      next(node, &send_list, index_max_delta);
    }
    for el in &mut node.children {
      next(el, &send_list, index_max_delta);
      walk(el, &send_list, index_max_delta);
    }
  }

  walk(&mut root, &send_list, index_max_delta);
  // println!("{:?}", serde_json::to_string(&root));

  fn found_valid_path(node: &Node) -> Vec<Vec<[usize; 2]>> {
    let mut result = Vec::new();
    let mut valid_paths = Vec::new();

    fn walk(
      node: &Node,
      result: &mut Vec<Vec<[usize; 2]>>,
      level: usize,
      current_index: i32,
      valid_paths: &mut Vec<usize>,
    ) {
      if node.found {
        valid_paths.push(current_index as usize);
        return;
      }

      for (index, el) in node.children.iter().enumerate() {
        let mut next_index = current_index;
        if current_index == -1 {
          result.push(Vec::new());
          next_index = result.len() as i32 - 1;
          result[next_index as usize].push(el.index);
        } else {
          if index == 0 {
            result[current_index as usize].push(el.index);
          } else {
            let mut s_vec = result[current_index as usize].as_slice();
            s_vec = &s_vec[0..level];
            let mut vec = s_vec.to_vec();
            vec.push(el.index);
            next_index = result.len() as i32;
            result.push(vec);
          }
        }

        walk(&el, result, level + 1, next_index, valid_paths);
      }
    }

    walk(node, &mut result, 0, -1, &mut valid_paths);

    result = result
      .into_iter()
      .enumerate()
      .filter(|(index, _)| valid_paths.contains(index))
      .map(|(_, el)| el)
      .collect::<Vec<_>>();

    return result;
  }

  let mut path_list = found_valid_path(&root);
  path_list.sort_by(|a, b| a.len().cmp(&b.len()));

  let mut path = vec![index_max_delta.clone()];
  path = [path, path_list[0].clone()].concat();

  return (path_list, path);
}

#[cfg(test)]
mod tests_get_matrix_indexes {
  use super::*;
  #[test]
  fn get_matrix_indexes_compare() {
    // [
    //   [0+, 0, 0, 45-],
    //   [0, 60, 0, 20],
    //   [40-, 0, 0, 10+]
    // ]

    let gammas = _convert_nested_a_v([
      [-3, -1, EMPTY, EMPTY],
      [0, EMPTY, 6, EMPTY],
      [EMPTY, 1, 6, EMPTY],
    ]);
    let send_list = _convert_nested_a_v([[0, 0, 0, 45], [0, 60, 0, 20], [40, 0, 0, 10]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(result, [[0, 0], [0, 3], [2, 3], [2, 0]].to_vec());
  }

  #[test]
  fn get_matrix_indexes_case1() {
    // task_0
    // [
    //   [0+, 0, 25, 45-],
    //   [0, 60, 0, 20],
    //   [40-, 0, 0, 10+]
    // ]

    let gammas = _convert_nested_a_v([
      [-3, -1, EMPTY, EMPTY],
      [0, EMPTY, 6, EMPTY],
      [EMPTY, 1, 6, EMPTY],
    ]);
    let send_list = _convert_nested_a_v([[0, 0, 25, 45], [0, 60, 0, 20], [40, 0, 0, 10]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(result, [[0, 0], [0, 3], [2, 3], [2, 0]].to_vec());
  }

  #[test]
  fn get_matrix_indexes_case2() {
    // task_0
    // [
    //   [40, 0-, 25, 5+],
    //   [0, 60+, 0, 20-],
    //   [0, 0, 0, 50],
    // ]

    let gammas = _convert_nested_a_v([
      [EMPTY, -1, EMPTY, EMPTY],
      [3, EMPTY, 6, EMPTY],
      [3, 1, 6, EMPTY],
    ]);
    let send_list = _convert_nested_a_v([[40, 0, 25, 5], [0, 60, 0, 20], [0, 0, 0, 50]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(result, [[0, 1], [0, 3], [1, 3], [1, 1]].to_vec());
  }

  #[test]
  fn get_matrix_indexes_case3() {
    // task_1
    // [
    //   [-888+, 0, 25, 25-],
    //   [0, 10, 0, 10],
    //   [30-, 0, 0, 0+]
    // ]

    let gammas = _convert_nested_a_v([
      [EMPTY, -1, EMPTY, -EMPTY],
      [3, EMPTY, 6, -EMPTY],
      [EMPTY, -2, 3, -3],
    ]);
    let send_list = _convert_nested_a_v([
      [DEGENERATED_FILLED, 0, 25, 25],
      [0, 10, 0, 10],
      [30, 0, 0, 0],
    ]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(result, [[2, 3], [2, 0], [0, 0], [0, 3]].to_vec());
  }

  #[test]
  fn get_matrix_indexes_case4() {
    // task_2
    // [
    //   [0, 60, 0],
    //   [0, 20+, 50-],
    //   [30, 0, 0],
    //   [60, 10-, 0+]
    // ]

    let gammas = _convert_nested_a_v([
      [0, EMPTY, 3],
      [0, EMPTY, EMPTY],
      [EMPTY, 2, 7],
      [EMPTY, EMPTY, -1],
    ]);
    let send_list = _convert_nested_a_v([[0, 60, 0], [0, 20, 50], [30, 0, 0], [60, 10, 0]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(result, [[3, 2], [3, 1], [1, 1], [1, 2]].to_vec());
  }

  #[test]
  fn get_matrix_indexes_case5() {
    // task_2
    // [
    //   [0+, 60-, 0],
    //   [0, 30+, 40-],
    //   [30, 0, 0],
    //   [60-, 0, 10+]
    // ]

    let gammas = _convert_nested_a_v([
      [-1, EMPTY, 3],
      [-1, EMPTY, EMPTY],
      [EMPTY, 3, 8],
      [EMPTY, 1, EMPTY],
    ]);
    let send_list = _convert_nested_a_v([[0, 60, 0], [0, 30, 40], [30, 0, 0], [60, 0, 10]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(
      result,
      [[0, 0], [0, 1], [1, 1], [1, 2], [3, 2], [3, 0]].to_vec()
    );
  }

  #[test]
  fn get_matrix_indexes_case6() {
    // task_4
    // [
    //   [0, 3500+, 500-],
    //   [0+, 3000-, 0],
    //   [2000-, 0, 3000+],
    //   [2700, 0, 0]
    // ]

    let gammas = _convert_nested_a_v([
      [200, EMPTY, EMPTY],
      [-200, EMPTY, 0],
      [EMPTY, 200, EMPTY],
      [EMPTY, 100, 0],
    ]);
    let send_list =
      _convert_nested_a_v([[0, 3500, 500], [0, 3000, 0], [2000, 0, 3000], [2700, 0, 0]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(
      result,
      [[1, 0], [1, 1], [0, 1], [0, 2], [2, 2], [2, 0]].to_vec()
    );
  }

  #[test]
  fn get_matrix_indexes_case7() {
    // task_4
    // [
    //   [0, 4000, 0],
    //   [500+, 2500-, 0],
    //   [1500, 0, 3500],
    //   [2700-, 0+, 0]
    // ]

    let gammas = _convert_nested_a_v([
      [400, EMPTY, 200],
      [EMPTY, EMPTY, 200],
      [EMPTY, 0, EMPTY],
      [EMPTY, -100, 0],
    ]);
    let send_list =
      _convert_nested_a_v([[0, 4000, 0], [500, 2500, 0], [1500, 0, 3500], [2700, 0, 0]]);
    let (_, result) = get_matrix_indexes(&gammas, &send_list, &Vec::new());
    assert_eq!(result, [[3, 1], [3, 0], [1, 0], [1, 1]].to_vec());
  }
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

  println!("Min minus value {}", min_minus_value);

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

fn get_alternative_indexes(gammas: &Vec<Vec<i32>>) -> Vec<[usize; 2]> {
  let mut result = Vec::new();
  for i in 0..gammas.len() {
    for j in 0..gammas[i].len() {
      if gammas[i][j] == 0 {
        result.push([i, j]);
        break;
      }
    }
  }
  return result;
}

fn has_alternative(gammas: &Vec<Vec<i32>>) -> bool {
  return get_alternative_indexes(gammas).len() > 0;
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

fn get_path(
  send_list: &Vec<Vec<i32>>,
  production_place: &Vec<i32>,
  production_need: &Vec<i32>,
  part_index: Option<[usize; 2]>,
) -> Vec<Vec<i32>> {
  let mut result = send_list.clone();

  let (closed, diff_val) = is_closed(production_place, production_need);
  if closed {
    return result;
  }

  if diff_val > 0 {
    if let Some(inx) = part_index {
      // for el in &mut result {
      //   el.pop();
      // }
    } else {
      for el in &mut result {
        el.pop();
      }
    }

    return result;
  }

  if let Some(inx) = part_index {
    // result.remove(inx[0]);
  } else {
    result.pop();
  }

  return result;
}

fn transport_task<const N: usize>(
  data: Vec<Vec<i32>>,
  production_place: Vec<i32>,
  production_need: Vec<i32>,
  matrix_indexes_list: [Vec<[usize; 2]>; N],
  result_price: i32,
  part_index: Option<[usize; 2]>,
) {
  let start = Instant::now();

  let production_place_initial = production_place.clone();
  let production_need_initial = production_need.clone();

  let mut production_place = production_place.clone();
  let mut production_need = production_need.clone();
  let mut data = data.clone();

  _print_2d("Data", &data);
  println!("Production place {:?}", &production_place);
  println!("Production need {:?}", &production_need);

  let closed = is_closed(&production_place, &production_need);
  println!("Closed: {:?}", closed);

  (data, production_place, production_need) =
    get_data_if_opened(&data, &production_place, &production_need, part_index);
  _print_2d("Data", &data);
  println!("Production place {:?}", &production_place);
  println!("Production need {:?}", &production_need);

  fn go_cycle(
    data: &Vec<Vec<i32>>,
    production_place: &Vec<i32>,
    production_need: &Vec<i32>,
    production_place_initial: &Vec<i32>,
    production_need_initial: &Vec<i32>,
    initial_send_list: &Vec<Vec<i32>>,
    initial_gammas: &Vec<Vec<i32>>,
    index: i32,
    skip_matrix_indexes: &Vec<[usize; 2]>,
    part_index: Option<[usize; 2]>,
  ) -> (Vec<Vec<i32>>, Vec<Vec<i32>>, bool, i32, bool) {
    println!("---Cycle--- {}", index);
    let mut send_list = initial_send_list.clone();
    let mut gammas = initial_gammas.clone();
    if index == 0 {
      send_list = get_send_list_min_price(&data, &production_place, &production_need);
    } else {
      let (_, matrix_indexes) = get_matrix_indexes(&gammas, &send_list, skip_matrix_indexes);
      println!("Matrix indexes {:?}", matrix_indexes);
      // let matrix_indexes = matrix_indexes_list[index - 1].clone();

      send_list = get_new_send_list(matrix_indexes, &send_list);
    }

    _print_2d("Send list", &send_list);

    let degenerated = is_degenerated(&send_list, &data);
    println!("Degenerated {:?}", degenerated);
    send_list = get_send_list_if_not_degenerated(&send_list, &data);

    let (u_line, v_line) = get_uv_lines(&send_list, &data);
    println!("U line {:?} \nV line {:?}", u_line, v_line);
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

    gammas = get_gammas(&data, &send_list, &u_line, &v_line);
    _print_2d("Gammas", &gammas);

    let price = get_price(&data, &send_list);
    println!("Price {:#?}", price);

    let optimal = is_optimal(&gammas);
    let alternative = has_alternative(&gammas);
    println!("Optimal {:#?}", optimal);
    println!("Alternative {:#?}", alternative);

    if optimal {
      let path = get_path(
        &send_list,
        &production_place_initial,
        &production_need_initial,
        part_index,
      );
      _print_2d("Path", &path);
    }

    return (send_list, gammas, optimal, price, alternative);
  }

  let mut optimal;
  let mut price;
  let mut index = 0;
  let mut alternative;

  let mut gammas = Vec::new();
  let mut send_list = Vec::new();

  loop {
    (send_list, gammas, optimal, price, alternative) = go_cycle(
      &data,
      &production_place,
      &production_need,
      &production_place_initial,
      &production_need_initial,
      &send_list,
      &gammas,
      index,
      &Vec::new(),
      part_index,
    );

    index += 1;

    if optimal {
      break;
    }
  }

  if alternative {
    let send_list_alternative = send_list.clone();
    let gammas_alternative = gammas.clone();
    let mut alternative_indexes = get_alternative_indexes(&gammas);
    alternative_indexes.reverse();

    let mut alternative_index = 0;

    println!("Alternative indexes {:?}", alternative_indexes);

    println!("---Alternative---");

    loop {
      let skipped_matrix_indexes = alternative_indexes[0..alternative_index as usize].to_vec();
      go_cycle(
        &data,
        &production_place,
        &production_need,
        &production_place_initial,
        &production_need_initial,
        &send_list_alternative,
        &gammas_alternative,
        index,
        &skipped_matrix_indexes,
        part_index,
      );
      index += 1;
      alternative_index += 1;

      if skipped_matrix_indexes.len() + 1 == alternative_indexes.len() {
        break;
      }
    }
  }

  assert_eq!(price, result_price, "Error result");

  let duration = start.elapsed();
  println!("---Duration--- {:?}", duration);
}

fn get_data_based_double(
  price_to_time: &Vec<Vec<i32>>,
  price_from_time: &Vec<Vec<i32>>,
  a_to_b: &Vec<i32>,
  a_n: Vec<i32>,
  d_n: Vec<i32>,
  b_n: Vec<i32>,
) -> (Vec<Vec<i32>>, Vec<i32>, Vec<i32>) {
  let mut data = price_to_time.clone();
  for inx in 0..data.len() {
    for el in a_to_b.iter() {
      if inx == 0 {
        data[inx].push(*el);
      } else {
        data[inx].push(M);
      }
    }
  }

  let next_index = data.len();
  for i in 0..price_from_time.len() {
    data.push(Vec::new());
    for j in 0..price_to_time[0].len() {
      data[next_index + i].push(if j == i { 0 } else { M });
    }
  }

  for i in 0..price_from_time.len() {
    for j in 0..price_from_time[i].len() {
      data[next_index + i].push(price_from_time[i][j]);
    }
  }

  let mut production_place = a_n.clone();
  production_place.extend(d_n.clone());

  let mut production_need = d_n.clone();
  production_need.extend(b_n.clone());

  (data, production_place, production_need)
}

fn main() {
  fn task_00() {
    let data = vec![vec![1, 3, 2, 4], vec![2, 1, 4, 3], vec![3, 5, 6, 1]];
    let production_place = vec![35, 50, 15];
    let production_need = vec![30, 10, 20, 40];

    println!("\nTask youtube closed https://www.youtube.com/watch?v=1jBa_2IYDNY");

    transport_task(
      data,
      production_place,
      production_need,
      [[[1, 0], [0, 0], [0, 2], [1, 2]].to_vec()],
      185,
      None,
    );
  }

  fn task_0() {
    let data = vec![vec![2, 4, 1, 6], vec![2, 2, 4, 3], vec![1, 2, 3, 2]];
    let production_place = vec![70, 80, 50];
    let production_need = vec![40, 60, 25, 75];

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
      None,
    );
  }

  // degenerated
  fn task_1() {
    let data = vec![vec![2, 4, 1, 6], vec![2, 2, 4, 3], vec![1, 2, 3, 2]];
    let production_place = vec![50, 20, 30];
    let production_need = vec![30, 10, 25, 35];

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
      None,
    );
  }

  fn task_2() {
    let data = vec![vec![4, 3, 5], vec![8, 7, 6], vec![4, 5, 9], vec![10, 9, 7]];
    let production_place = vec![60, 70, 30, 70];
    let production_need = vec![90, 90, 50];

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
      None,
    );
  }

  // opened/not degenerated
  fn task_000() {
    let data = vec![vec![4, 6, 3], vec![5, 5, 2]];
    let production_place = vec![2000, 2500];
    let production_need = vec![1000, 1300, 1200];

    println!(
      "\n=====> Task youtube opened not degenerated https://www.youtube.com/watch?v=qz3Vu1TR4Jk"
    );
    transport_task(data, production_place, production_need, [], 12900, None);
  }

  // opened
  fn task_3() {
    let data = vec![vec![700, 400], vec![200, 300], vec![100, 200]];
    let production_place = vec![4500, 3200, 5500];
    let production_need = vec![4700, 6500];

    println!("\n=====> Task 3");
    transport_task(data, production_place, production_need, [], 2590000, None);
  }

  // opened
  fn task_4() {
    let data = vec![
      vec![700, 400, 500],
      vec![200, 300, 400],
      vec![100, 200, 100],
    ];
    let production_place = vec![4000, 3000, 5000];
    let production_need = vec![4700, 6500, 3500];

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
        [[1, 0], [1, 1], [0, 1], [0, 2], [2, 2], [2, 0]].to_vec(),
        [[3, 1], [3, 0], [1, 0], [1, 1]].to_vec(),
      ],
      2700000,
      None,
    );
  }

  fn task_5() {
    let data = vec![vec![1, 3, 1, 2], vec![2, 1, 4, 3], vec![5, 3, 2, 3]];
    let production_place = vec![100, 130, 270];
    let production_need = vec![80, 120, 240, 100];

    println!("\n=====> Task 4");

    transport_task(data, production_place, production_need, [], 840, None);
  }

  fn task_6_d() {
    let price_to_time = vec![vec![3, 6], vec![6, 9]];
    let price_from_time = vec![vec![4, 7, 10], vec![10, 13, 16]];
    let a_to_b = vec![1, 2, 3];
    let a_n = vec![15, 105];
    let d_n = vec![80, 60];
    let b_n = vec![45, 45, 45];

    // [
    //   [3, 6, 1, 2, 3,    15],
    //   [6, 9, M, M, M,    105],
    //   [0, M, 4, 7, 10,   80],
    //   [M, 0, 10, 13, 16, 60]
    //   [80, 60, 45, 45, 45 ]
    // ]

    let (data, production_place, production_need) =
      get_data_based_double(&price_to_time, &price_from_time, &a_to_b, a_n, d_n, b_n);

    println!("\n=====> Task 6 double");

    transport_task(
      data,
      production_place,
      production_need,
      [],
      1785,
      Some([price_to_time.len(), a_to_b.len()]),
    );

    // ! A1 =>(15) B3
    // ! A2 =>(80) D1
    // ! A2 =>(25) D2
    // ! A3 =>(15) D2
    // ! D1 =>(35) B1
    // ! D1 =>(45) B2
    // ! D2 =>(10) B1
    // ! D2 =>(30) B3

    // [
    // [0, 0, 0, 0, 15]
    // [80, 25, 0, 0, 0]
    // [0, 15, 0, 0, 0]
    // [0, 0, 35, 45, 0]
    // [0, 20, 10, 0, 30]
    // ]
  }

  fn task_rgr() {
    let data = vec![vec![1, 3, 1, 2], vec![2, 1, 4, 3], vec![5, 3, 2, 3]];
    let production_place = vec![100, 130, 270];
    let production_need = vec![80, 120, 240, 100];

    println!("\n=====> Task Rgr 0");

    transport_task(data, production_place, production_need, [], 840, None);
  }

  fn task_rgr_d() {
    let price_to_time = vec![vec![3, 7], vec![11, 8], vec![5, 2]];
    let price_from_time = vec![vec![4, 6], vec![9, 10]];
    let a_to_b = vec![1, M];
    let a_n = vec![25, 35, 75];
    let d_n = vec![100, 40];
    let b_n = vec![20, 20];

    // [
    //   [3, 7, 1, M], 25
    //   [11, 8, M, M], 35
    //   [5, 2, M, M], 75
    //   [0, M, 4, 6], 100
    //   [M, 0, 9, 10], 40
    //   100, 40, 20, 20
    // ]

    let (data, production_place, production_need) =
      get_data_based_double(&price_to_time, &price_from_time, &a_to_b, a_n, d_n, b_n);

    println!("\n=====> Task Rgr double");

    transport_task(
      data,
      production_place,
      production_need,
      [],
      795,
      Some([price_to_time.len(), a_to_b.len()]),
    );

    // A1 =>(5) D1
    // A1 =>(20) B1
    // A2 =>(35) D1
    // A3 =>(35) D1
    // A3 =>(40) D2
    // D1 =>(20) B2
    // D1 =>(55) B3
    // D2 =>(40) B3

    // [
    // [5, 0, 20, 0, 0]
    // [35, 0, 0, 0, 0]
    // [35, 40, 0, 0, 0]
    // [25, 0, 0, 20, 55]
    // [0, 0, 0, 0, 40]
    // ]
  }

  // task_00();
  // task_0();
  // task_1();
  // task_2();
  // task_000();
  // task_3();
  // task_4();
  // task_6_d();
  // task_rgr();
  task_rgr_d();
}
