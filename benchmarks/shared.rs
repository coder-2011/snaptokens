pub(crate) fn williams_cycle(candidate_count: usize) -> Vec<Vec<usize>> {
    if candidate_count <= 1 {
        return vec![vec![0]];
    }

    let base = (0..candidate_count)
        .map(|position| {
            if position == 0 {
                0
            } else if position % 2 == 1 {
                position.div_ceil(2)
            } else {
                candidate_count - position / 2
            }
        })
        .collect::<Vec<_>>();
    let cycle_rows = if candidate_count % 2 == 0 {
        candidate_count
    } else {
        2 * candidate_count
    };
    let mut orders = Vec::with_capacity(cycle_rows);

    for shift in 0..candidate_count {
        orders.push(
            base.iter()
                .map(|value| (value + shift) % candidate_count)
                .collect(),
        );
    }
    if candidate_count % 2 == 1 {
        // Reversed companion rows remove odd-design carryover asymmetry.
        let reversed = base.iter().rev().copied().collect::<Vec<_>>();
        for shift in 0..candidate_count {
            orders.push(
                reversed
                    .iter()
                    .map(|value| (value + shift) % candidate_count)
                    .collect(),
            );
        }
    }
    orders
}

pub(crate) fn balanced_orders(candidate_count: usize, requested_rounds: usize) -> Vec<Vec<usize>> {
    let cycle = williams_cycle(candidate_count);
    let cycles = requested_rounds.div_ceil(cycle.len());
    (0..cycles).flat_map(|_| cycle.iter().cloned()).collect()
}
