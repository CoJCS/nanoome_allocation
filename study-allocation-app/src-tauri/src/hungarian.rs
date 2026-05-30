use pathfinding::prelude::{kuhn_munkres_min, Matrix};

/// Square assignment problem (minimum cost). Returns column index for each row.
pub fn solve_assignment(cost: &[Vec<i32>]) -> Vec<usize> {
    let n = cost.len();
    assert!(n > 0);
    assert!(cost.iter().all(|row| row.len() == n));

    let matrix = Matrix::from_rows(cost.to_vec()).expect("square cost matrix");
    let (_, cols) = kuhn_munkres_min(&matrix);
    cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimizes_preference_cost() {
        // Prefer low-cost slots: row0->col0 (0), row1->col1 (0)
        let cost = vec![vec![0, 10], vec![10, 0]];
        let cols = solve_assignment(&cost);
        assert_eq!(cols, vec![0, 1]);
    }

    #[test]
    fn solves_diagonal_matrix() {
        let cost = vec![vec![1, 2, 3], vec![2, 4, 6], vec![3, 6, 9]];
        let cols = solve_assignment(&cost);
        assert_eq!(cols.len(), 3);
        let total: i32 = (0..3).map(|i| cost[i][cols[i]]).sum();
        assert_eq!(total, 10);
    }
}
