impl Solution {
  pub fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
    let n = cost.len();

    for i in (0..n-2).rev() {
      cost[i] += i32::min(cost[i + 1], cost[i + 2]);
    }

    i32::min(cost[1], cost[0])
  }
}
