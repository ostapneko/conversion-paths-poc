use std::collections::HashMap;
use std::iter::Iterator;
use std::collections::BTreeSet;

type GoalId = u32;

#[derive(Debug, PartialEq)]
pub struct ConversionPaths {
    pub count: u64,
    pub children: HashMap<GoalId, ConversionPaths>,
}

impl ConversionPaths {
    pub fn build<T>(target: GoalId, sessions: T) -> ConversionPaths
    where
        T: Iterator<Item = BTreeSet<GoalId>>
    {
        let mut res = ConversionPaths {
            count: 0, children: HashMap::new()
        };

        for goals in sessions {
            if goals.contains(&target) {
                let until_target = goals.into_iter().rev().take_while(|g| g != &target);
                res.count += 1;
                res.add_paths(until_target);
            }
        }
        res
    }

    fn add_paths<T>(&mut self, mut goals: T)
    where
        T: Iterator<Item = GoalId>
    {
        match goals.next() {
            None =>
                (),
            Some(goal) =>
                if self.children.contains_key(&goal) {
                    let child = self.children.get_mut(&goal).unwrap();
                    child.count += 1;
                    child.add_paths(goals);
                } else {
                    self.children.insert(goal, ConversionPaths::from_goals(goals));
                },
        }
    }

    fn from_goals<T>(mut goals: T) -> ConversionPaths
    where
        T: Iterator<Item = GoalId>
    { 
        match goals.next() {
            None =>
                ConversionPaths::empty(),
            Some(goal) => {
                let mut children = HashMap::new();
                children.insert(goal, ConversionPaths::from_goals(goals));
                ConversionPaths { count: 1, children: children }
            },
        }
    }

    fn empty() -> ConversionPaths {
        ConversionPaths {
            count: 1,
            children: HashMap::new()
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use conversion_paths::ConversionPaths;
    use std::hash::Hash;
    use std::collections::BTreeSet;

    fn small_tree() -> ConversionPaths {
        let mut grand_children = HashMap::new();
        grand_children.insert(2, ConversionPaths::empty());

        let mut children = HashMap::new();

        children.insert(1, 
            ConversionPaths {
                count: 1,
                children: grand_children
            }
        );

        ConversionPaths {
            count: 1,
            children: children,
        }
    }

    #[test]
    fn from_goals_test() {
        assert_eq!(
            small_tree(),
            ConversionPaths::from_goals(vec![1, 2].into_iter())
        )
    }

    fn one_elem_hashmap<K: Hash + Eq, T>(k: K, t: T) -> HashMap<K, T> {
        let mut hm = HashMap::new();
        hm.insert(k, t);
        hm
    }

    #[test]
    fn add_paths_test() {
        let paths = vec![1, 2, 3];
        let mut actual = small_tree();
        actual.add_paths(paths.into_iter());

        let exp = ConversionPaths {
            count: 1,
            children: one_elem_hashmap(1, ConversionPaths {
                count:2,
                children: one_elem_hashmap(2, ConversionPaths {
                    count: 2,
                    children: one_elem_hashmap(3, ConversionPaths::empty())
                })
            })
        };

        assert_eq!(exp, actual);
    }

    #[test]
    fn build_test() {
        let mut sessions = vec![];
        let mut goals1 = BTreeSet::new();
        for g in [1, 2].iter() {
            goals1.insert(g);
        }

        let mut goals2 = BTreeSet::new();
        for g in [1, 2, 3].iter() {
            goals2.insert(g);
        }
        sessions.push(goals1);
        sessions.push(goals2);
    }
}