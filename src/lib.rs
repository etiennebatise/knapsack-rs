use chrono::prelude::*;
use num_rational::Rational32;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

type Value = i32;
type Weight = i32;
// type UpperBound = Rational32;
type UpperBound = i32;

struct Item {
    index: usize,
    value: Value,
    weight: Weight,
}

fn compute_upper_bound<T>(items: &[(T, Value, Weight)], slack: Weight, value: Value) -> UpperBound {
    if slack == 0 || items.len() == 0 {
        return value;
    }
    let mut slack = slack;
    let mut res = value;
    for (_, v, w) in items {
        if slack >= *w {
            res += v;
            slack -= w;
        } else {
            res += (*v / *w) * slack;
            break;
        }
    }
    res
}

#[derive(Debug, Clone)]
struct Node {
    items: Vec<Weak<usize>>,
    value: Value,
    slack: Weight,
    level: usize,
}

fn crawl(items: &Vec<(Rc<usize>, Value, Weight)>, slack: Weight) -> (Vec<usize>, Value, Weight) {
    // println!("{:?}", items);
    let mut queue: VecDeque<Node> = VecDeque::new();
    let init = Node {
        items: vec![],
        value: 0,
        slack: slack,
        level: 0,
    };
    let mut best_node = Node {
        items: vec![],
        value: 0,
        slack: slack,
        level: 0,
    };
    queue.push_back(init);
    loop {
        match queue.pop_front() {
            None => break,
            Some(n) => {
                if n.value > best_node.value {
                    // println!("Best node : {:?} {}", n.level, n.value);
                    best_node.items = n.items.to_vec();
                    best_node.value = n.value;
                    best_node.slack = n.slack;
                    best_node.level = n.level;
                }
                if n.level == items.len() {
                    continue;
                }
                let tail = &items[n.level..];
                let local_upper_bound = compute_upper_bound(tail, n.slack, n.value);
                if (local_upper_bound) > best_node.value {
                    let new_node = Node {
                        items: n.items.to_vec(),
                        slack: n.slack,
                        value: n.value,
                        level: n.level + 1,
                    };
                    queue.push_front(new_node);

                    let (i, v, w) = &tail[0];
                    if *w <= n.slack {
                        let new_node_items = &mut n.items.to_vec();
                        new_node_items.push(Rc::downgrade(&i));
                        let new_node = Node {
                            items: new_node_items.to_vec(),
                            slack: n.slack - w,
                            value: n.value + v,
                            level: n.level + 1,
                        };
                        queue.push_front(new_node);
                    }
                }
            }
        }
    }
    let mut resi: Vec<usize> = vec![];
    for i in best_node.items.iter() {
        let y = &mut i.upgrade().unwrap();
        let x = Rc::make_mut(y);
        resi.push(*x);
    }

    (resi, best_node.value, best_node.slack)
}

pub fn solve(items: &Vec<(Value, Weight)>, target: Weight) -> (Value, Weight, Vec<bool>) {
    let mut sorted_items = items
        .iter()
        .enumerate()
        .map(|(i, (v, w))| (i, *v, *w, v / w))
        .collect::<Vec<_>>();
    sorted_items.sort_by(
        |(_, l_v, _, l_price_per_weight), (_, r_v, _, r_price_per_weight)| {
            (*r_price_per_weight, *r_v).cmp(&(*l_price_per_weight, *l_v))
            // (*l_price_per_weight, *l_v).cmp(&(*r_price_per_weight, *r_v))
        },
    );
    let sorted_items = sorted_items
        .iter()
        .map(|(i, v, w, _)| (Rc::new(*i), *v, *w))
        .collect::<Vec<_>>();
    let (best_node_items, value, _) = crawl(&sorted_items, target);
    let mut total_weight = 0;
    for i in best_node_items.iter() {
        let (_, w) = items.get(*i).unwrap();
        total_weight += *w;
    }
    let res = items
        .iter()
        .enumerate()
        .map(|(i, _)| best_node_items.contains(&i))
        .collect::<Vec<_>>();

    (value, total_weight, res)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solve_with_a_simple_test() {
        let target = 200;
        let items = vec![
            (92, 92),
            (86, 86),
            (16, 16),
            (20, 20),
            (48, 48),
            (85, 85),
            (49, 49),
            (73, 73),
            (94, 94),
            (10, 10),
        ];
        let (v, w, res) = solve(&items, target);
        assert_eq!(v, 200);
        assert_eq!(w, 200);
        // assert_eq!(
        //     res,
        //     vec![false, true, false, true, false, false, false, false, true, false]
        // );
    }

    #[test]
    fn test_crawl_with_empty_items() {
        let items = vec![];
        let (_, v, _) = crawl(&items, 0);
        // assert_eq!(best_node.items, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_crawl_with_too_heavy_items() {
        let items = vec![(Rc::new(1), 1, 2), (Rc::new(2), 1, 2), (Rc::new(3), 1, 2)];
        let slack = 1;
        let (_, v, _) = crawl(&items, slack);
        // assert_eq!(best_node.items, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    // Is this test even useful?
    fn test_crawl_with_worthless_items() {
        let items = vec![(Rc::new(1), 0, 2), (Rc::new(2), 0, 2), (Rc::new(3), 0, 2)];
        let slack = 10;
        let (_, v, _) = crawl(&items, slack);
        // assert_eq!(best_node.items, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_crawl_with_better_branch() {
        // Items left to check
        // weight is irrelevant for this check
        let items = vec![(Rc::new(1), 5, 0), (Rc::new(2), 6, 0)];
        let slack = 1;
        let (_, v, _) = crawl(&items, slack);
        // We expect the best_node hasn't changed
        // assert_eq!(best_node.items, vec![1, 2]);
        assert_eq!(v, 11);
    }

    #[test]
    fn test_compute_upper_bound_when_slack_is_0() {
        let res = compute_upper_bound::<usize>(&vec![], 0, 0);
        assert_eq!(res, Rational32::from(0));
    }

    #[test]
    fn test_compute_upper_bound_when_vec_is_empty() {
        let res = compute_upper_bound::<usize>(&vec![], 1, 1);
        assert_eq!(res, Rational32::from(1));
    }

    #[test]
    fn test_compute_upper_bound() {
        // Items must be sorted by price per weight
        // Here usizes are not important
        let items = vec![(1, 1, 1), (2, 1, 1), (3, 1, 1)];
        // Only 1 weight unit remaining, value is 1
        let res = compute_upper_bound(&items, 1, 1);
        // expected upper bound is 2
        assert_eq!(res, Rational32::from(2));
    }
}
