use std::collections::VecDeque;

type Value = usize;
type Weight = usize;
type UpperBound = usize;

fn bound<It>(items: It, mut slack: Weight, mut res: Value) -> UpperBound
where
    It: IntoIterator<Item = (Value, Weight)>,
{
    if slack != 0 {
        for (v, w) in items.into_iter() {
            if slack < w {
                res += (v / w) * slack;
                break;
            }

            res += v;
            slack -= w;
        }
    }

    res
}

#[derive(Debug, Clone)]
struct Node {
    items: Vec<usize>,
    profit: Value,
    slack: Weight,
    level: usize,
}

impl Node {
    pub fn to_tuple(self) -> (Vec<usize>, Value, Weight) {
        (self.items, self.profit, self.slack)
    }
}

fn crawl(items: &Vec<(usize, Value, Weight)>, slack: Weight) -> (Vec<usize>, Value, Weight) {
    let init = Node {
        items: vec![],
        profit: 0,
        slack: slack,
        level: 0,
    };

    let mut best_node = init.clone();
    let mut queue: VecDeque<Node> = VecDeque::new();

    queue.push_back(init);

    while let Some(mut n) = queue.pop_front() {
        if n.profit > best_node.profit {
            best_node = n.clone();
        }

        if n.level == items.len() {
            continue;
        }

        let tail = &items[n.level..];
        let upper_bound = bound(
            tail.iter().copied().map(|(_, v, w)| (v, w)),
            n.slack,
            n.profit,
        );

        if (upper_bound) > best_node.profit {
            n.level += 1;
            queue.push_front(n.clone());
            let (i, v, w) = &tail[0];

            if *w <= n.slack {
                n.items.push(*i);
                n.slack -= w;
                n.profit += v;

                queue.push_front(n);
            }
        }
    }

    best_node.to_tuple()
}

pub trait Item {
    fn value(&self) -> Value;
    fn weight(&self) -> Weight;

    fn price_per_weight(&self) -> usize {
        self.value() / self.weight()
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

pub fn sort<I: Item>(items: &Vec<I>) -> Vec<(usize, Value, Weight)> {
    let mut sorted_items = items.into_iter().enumerate().collect::<Vec<_>>();

    sorted_items.sort_by(|(_, l), (_, r)| {
        (r.price_per_weight(), r.value()).cmp(&(l.price_per_weight(), l.value()))
    });

    sorted_items
        .into_iter()
        .map(|(pos, i)| (pos, i.value(), i.weight()))
        .collect()
}

pub fn solve<I: Item>(items: &Vec<I>, slack: Weight) -> (Value, Weight, Vec<bool>) {
    let sorted_items = sort(items);
    let (best_node_items, value, _) = crawl(&sorted_items, slack);
    let mut total_weight = 0;
    for i in best_node_items.iter() {
        let item = items.get(*i).unwrap();
        total_weight += item.weight();
    }
    let res = items
        .into_iter()
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
        let (v, w, res) = solve(&items, target);
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
        let items = vec![(1, 1, 2), (2, 1, 2), (3, 1, 2)];
        let slack = 1;
        let (i, v, _) = crawl(&items, slack);
        assert_eq!(i, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    // Is this test even useful?
    fn test_crawl_with_worthless_items() {
        let items = vec![(1, 0, 2), (2, 0, 2), (3, 0, 2)];
        let slack = 10;
        let (i, v, _) = crawl(&items, slack);
        assert_eq!(i, vec![]);
        assert_eq!(v, 0);
    }

    #[test]
    fn test_crawl_with_better_branch() {
        // weight is irrelevant for this check
        let items = vec![(1, 5, 0), (2, 6, 0)];
        let slack = 1;
        let (i, v, _) = crawl(&items, slack);
        assert_eq!(i, vec![1, 2]);
        assert_eq!(v, 11);
    }

    #[test]
    fn test_compute_upper_bound_when_slack_is_0() {
        let res = bound(vec![], 0, 0);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_compute_upper_bound_when_vec_is_empty() {
        let res = bound(vec![], 1, 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_compute_upper_bound() {
        // Items must be sorted by price per weight
        // Here usizes are not important
        let items = vec![(1, 1), (1, 1), (1, 1)];
        // Only 1 weight unit remaining, value is 1
        let res = bound(items, 1, 1);
        // expected upper bound is 2
        assert_eq!(res, 2);
    }
}
