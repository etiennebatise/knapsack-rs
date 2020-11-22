use std::collections::VecDeque;
use std::rc::{Rc, Weak};

type Value = usize;
type Weight = usize;
type UpperBound = usize;

fn bound<T>(items: &[(T, Value, Weight)], slack: Weight, value: Value) -> UpperBound {
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
    profit: Value,
    slack: Weight,
    level: usize,
}

fn crawl(items: &Vec<(Rc<usize>, Value, Weight)>, slack: Weight) -> (Vec<usize>, Value, Weight) {
    let init = Node {
        items: vec![],
        profit: 0,
        slack: slack,
        level: 0,
    };
    let mut best_node = Node {
        items: vec![],
        profit: 0,
        slack: slack,
        level: 0,
    };
    let mut queue: VecDeque<Node> = VecDeque::new();
    queue.push_back(init);
    loop {
        match queue.pop_front() {
            None => break,
            Some(n) => {
                if n.profit > best_node.profit {
                    best_node.items = n.items.to_vec();
                    best_node.profit = n.profit;
                    best_node.slack = n.slack;
                    best_node.level = n.level;
                }
                if n.level == items.len() {
                    continue;
                }
                let tail = &items[n.level..];
                let upper_bound = bound(tail, n.slack, n.profit);
                if (upper_bound) > best_node.profit {
                    let new_node = Node {
                        items: n.items.to_vec(),
                        slack: n.slack,
                        profit: n.profit,
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
                            profit: n.profit + v,
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

    (resi, best_node.profit, best_node.slack)
}

struct Foo {
    foo: usize,
    bar: usize,
}

pub trait Item {
    fn value(&self) -> Value;
    fn weight(&self) -> Weight;
}

impl Item for Foo {
    fn value(&self) -> Value {
        self.foo
    }
    fn weight(&self) -> Weight {
        self.bar
    }
}

impl Item for (usize, usize) {
    fn value(&self) -> Value {
        self.0
    }
    fn weight(&self) -> Weight {
        self.1
    }
}

pub fn solve(items: Vec<impl Item>, target: Weight) -> (Value, Weight, Vec<bool>) {
    let mut sorted_items = items
        .iter()
        .enumerate()
        .map(|(pos, i)| (pos, i.value(), i.weight(), i.value() / i.weight()))
        .collect::<Vec<_>>();
    sorted_items.sort_by(
        |(_, l_v, _, l_price_per_weight), (_, r_v, _, r_price_per_weight)| {
            (*r_price_per_weight, *r_v).cmp(&(*l_price_per_weight, *l_v))
        },
    );
    let sorted_items = sorted_items
        .iter()
        .map(|(i, v, w, _)| (Rc::new(*i), *v, *w))
        .collect::<Vec<_>>();
    let (best_node_items, value, _) = crawl(&sorted_items, target);
    let mut total_weight = 0;
    for i in best_node_items.iter() {
        let item = items.get(*i).unwrap();
        total_weight += item.weight();
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
        let items: Vec<(usize, usize)> = vec![
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
        let (v, w, res) = solve(items, target);
        assert_eq!(v, 200);
        assert_eq!(w, 200);
        assert_eq!(
            res,
            vec![false, true, false, true, false, false, false, false, true, false]
        );
    }

    #[test]
    fn test_crawl_with_empty_items() {
        let items = vec![];
        let (i, v, _) = crawl(&items, 0);
        assert_eq!(i, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_crawl_with_too_heavy_items() {
        let items = vec![(Rc::new(1), 1, 2), (Rc::new(2), 1, 2), (Rc::new(3), 1, 2)];
        let slack = 1;
        let (i, v, _) = crawl(&items, slack);
        assert_eq!(i, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    // Is this test even useful?
    fn test_crawl_with_worthless_items() {
        let items = vec![(Rc::new(1), 0, 2), (Rc::new(2), 0, 2), (Rc::new(3), 0, 2)];
        let slack = 10;
        let (i, v, _) = crawl(&items, slack);
        assert_eq!(i, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_crawl_with_better_branch() {
        // Items left to check
        // weight is irrelevant for this check
        let items = vec![(Rc::new(1), 5, 0), (Rc::new(2), 6, 0)];
        let slack = 1;
        let (i, v, _) = crawl(&items, slack);
        // We expect the best_node hasn't changed
        assert_eq!(i, vec![1, 2]);
        assert_eq!(v, 11);
    }

    #[test]
    fn test_compute_upper_bound_when_slack_is_0() {
        let res = bound::<usize>(&vec![], 0, 0);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_compute_upper_bound_when_vec_is_empty() {
        let res = bound::<usize>(&vec![], 1, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_compute_upper_bound() {
        // Items must be sorted by price per weight
        // Here usizes are not important
        let items = vec![(1, 1, 1), (2, 1, 1), (3, 1, 1)];
        // Only 1 weight unit remaining, value is 1
        let res = bound(&items, 1, 1);
        // expected upper bound is 2
        assert_eq!(res, 2);
    }
}
