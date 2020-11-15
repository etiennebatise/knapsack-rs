use num_rational::Rational32;

type Value = Rational32;
type Weight = Rational32;
// type Item = (Value, Weight);
// type PricePerWeigh = Rational32;

fn compute_upper_bound(items: &Vec<(usize, Value, Weight)>, slack: Weight, value: Value) -> Value {
    if slack == Rational32::from(0) || items.len() == 0 {
        return value;
    }
    let mut slack = slack;
    let mut res = value;
    for (_, v, w) in items.iter() {
        if slack >= *w {
            res += v;
            slack -= w;
        } else {
            res += (v / w) * slack;
        }
    }
    res
}

#[derive(Debug)]
struct Node {
    items: Vec<usize>,
    value: Value,
}

fn crawl(
    items: &Vec<(usize, Value, Weight)>,
    slack: Weight,
    current_node: Node,
    best_node: &mut Node,
) {
    let local_upper_bound = compute_upper_bound(&items, slack, current_node.value);
    if local_upper_bound > best_node.value {
        match items.split_first() {
            None => {
                best_node.value = current_node.value;
                best_node.items = current_node.items.clone();
            }
            Some(((i, v, w), t)) => {
                if slack >= *w {
                    let items = &mut current_node.items.to_vec();
                    items.extend(vec![i]);
                    let n = Node {
                        value: current_node.value + v,
                        items: items.to_vec(),
                    };
                    crawl(&Vec::from(t), slack - w, n, best_node);
                }
                crawl(&Vec::from(t), slack, current_node, best_node);
            }
        }
    }
}

pub fn solve(items: &Vec<(Value, Weight)>, target: Weight) -> (Value, Weight, Vec<bool>) {
    let mut sorted_items = items
        .iter()
        .enumerate()
        .map(|(i, (v, w))| (i, *v, *w, v / w))
        .collect::<Vec<_>>();
    sorted_items.sort_by(
        |(_, _, _, l_price_per_weight), (_, _, _, r_price_per_weight)| {
            l_price_per_weight.partial_cmp(r_price_per_weight).unwrap()
        },
    );
    let sorted_items = sorted_items
        .iter()
        .map(|(i, v, w, _)| (*i, *v, *w))
        .collect::<Vec<_>>();
    let best_node = &mut Node {
        items: vec![],
        value: Rational32::from(0),
    };
    let init = Node {
        items: vec![],
        value: Rational32::from(0),
    };
    crawl(&sorted_items, target, init, best_node);
    let mut total_weight = Rational32::from(0);
    for i in best_node.items.iter() {
        let (_, w) = items.get(*i).unwrap();
        total_weight += *w;
    }
    let res = items
        .iter()
        .enumerate()
        .map(|(i, _)| best_node.items.contains(&i))
        .collect::<Vec<_>>();

    (best_node.value, total_weight, res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_with_a_simple_test() {
        let target = Rational32::from(200);
        let items = vec![
            (Rational32::from(92), Rational32::from(92)),
            (Rational32::from(86), Rational32::from(86)),
            (Rational32::from(16), Rational32::from(16)),
            (Rational32::from(20), Rational32::from(20)),
            (Rational32::from(48), Rational32::from(48)),
            (Rational32::from(85), Rational32::from(85)),
            (Rational32::from(49), Rational32::from(49)),
            (Rational32::from(73), Rational32::from(73)),
            (Rational32::from(94), Rational32::from(94)),
            (Rational32::from(10), Rational32::from(10)),
        ];
        let (v, w, res) = solve(&items, target);
        assert_eq!(v, Rational32::from(200));
        assert_eq!(w, Rational32::from(200));
        assert_eq!(
            res,
            vec![false, true, false, true, false, false, false, false, true, false]
        );
    }

    #[test]
    fn test_crawl_with_empty_items() {
        let mut best_node = Node {
            items: vec![],
            value: Rational32::from(0),
        };
        let current_node = Node {
            items: vec![],
            value: Rational32::from(0),
        };
        crawl(&vec![], Rational32::from(0), current_node, &mut best_node);
        assert_eq!(best_node.items, vec![]);
        assert_eq!(best_node.value, Rational32::from(0));
    }

    #[test]
    fn test_crawl_with_too_heavy_items() {
        let mut best_node = Node {
            items: vec![],
            value: Rational32::from(0),
        };
        let current_node = Node {
            value: Rational32::from(0),
            items: vec![],
        };
        let items = vec![
            (1, Rational32::from(1), Rational32::from(2)),
            (2, Rational32::from(1), Rational32::from(2)),
            (3, Rational32::from(1), Rational32::from(2)),
        ];
        let slack = Rational32::from(1);
        crawl(&items, slack, current_node, &mut best_node);
        assert_eq!(best_node.items, vec![]);
        assert_eq!(best_node.value, Rational32::from(0));
    }

    #[test]
    fn test_crawl_with_worthless_items() {
        let mut best_node = Node {
            items: vec![],
            value: Rational32::from(10),
        };
        let current_node = Node {
            items: vec![],
            value: Rational32::from(0),
        };
        let items = vec![
            (1, Rational32::from(0), Rational32::from(2)),
            (2, Rational32::from(0), Rational32::from(2)),
            (3, Rational32::from(0), Rational32::from(2)),
        ];
        let slack = Rational32::from(10);
        crawl(&items, slack, current_node, &mut best_node);
        // We expect the best_node hasn't changed
        assert_eq!(best_node.items, vec![]);
        assert_eq!(best_node.value, Rational32::from(10));
    }

    #[test]
    fn test_crawl_with_better_branch() {
        // We've had a best value of 10 in anothercurrent_node
        let mut best_node = Node {
            items: vec![],
            value: Rational32::from(10),
        };
        // Current current_node has a value of 0
        let current_node = Node {
            items: vec![],
            value: Rational32::from(0),
        };
        // Items left to check
        // weight is irrelevant for this check
        let items = vec![
            (1, Rational32::from(5), Rational32::from(0)),
            (2, Rational32::from(6), Rational32::from(0)),
        ];
        let slack = Rational32::from(1);
        crawl(&items, slack, current_node, &mut best_node);
        // We expect the best_node hasn't changed
        assert_eq!(best_node.items, vec![1, 2]);
        assert_eq!(best_node.value, Rational32::from(11));
    }

    #[test]
    fn test_compute_upper_bound_when_slack_is_0() {
        let res = compute_upper_bound(&vec![], Rational32::from(0), Rational32::from(0));
        assert_eq!(res, Rational32::from(0));
    }

    #[test]
    fn test_compute_upper_bound_when_vec_is_empty() {
        let res = compute_upper_bound(&vec![], Rational32::from(1), Rational32::from(1));
        assert_eq!(res, Rational32::from(1));
    }

    #[test]
    fn test_compute_upper_bound() {
        // Items must be sorted by price per weight
        // Here usizes are not important
        let items = vec![
            (1, Rational32::from(1), Rational32::from(1)),
            (2, Rational32::from(1), Rational32::from(1)),
            (3, Rational32::from(1), Rational32::from(1)),
        ];
        // Only 1 weight unit remaining, value is 1
        let res = compute_upper_bound(&items, Rational32::from(1), Rational32::from(1));
        // expected upper bound is 2
        assert_eq!(res, Rational32::from(2));
    }
}
