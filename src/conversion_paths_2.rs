use std::iter::Iterator;

type GoalId = u32;

#[derive(Debug, PartialEq)]
pub struct ConversionPaths {
    pub goal_id: u32,
    pub count: usize,
    pub children: Vec<ConversionPaths>,
}

impl ConversionPaths {
    // precondition: all elements have non-empty elements and the slices are sorted (not their elements)
    fn chunk_by_head<T: PartialEq + Copy>(xs: Vec<&[T]>) -> Vec<Vec<&[T]>> {
        let mut res = Vec::new();
        let mut current = Vec::new();
        let mut current_head = None;
        for x in xs {
            if Some(x[0]) == current_head {
                current.push(x);
            } else {
                // flush
                if current_head.is_some() {
                    res.push(current);
                }
                current_head = Some(x[0]);
                current = vec![x];
            }
        }
        res.push(current);
        res
    }

    // precondition: all sessions have the same first element
    // and the sessions are sorted (not their element)
    pub fn build(sessions: &Vec<&[GoalId]>) -> ConversionPaths {
        let goal_id = sessions[0][0];
        let count = sessions.len();
        let children_sessions: Vec<&[GoalId]> =
            sessions.into_iter()
                .filter_map(|s| {
                    if s.len() < 2 {
                        None
                    } else {
                        let v: &[GoalId] = &s[1..];
                        Some(v)
                    }
                }).collect();
        let chunked =
            if children_sessions.len() > 0 {
                ConversionPaths::chunk_by_head(children_sessions)
            } else {
                Vec::new()
            };
        let children = chunked.iter().map(|s| ConversionPaths::build(s)).collect();

        ConversionPaths {
            goal_id: goal_id,
            count: count,
            children: children,
        } 
    }
}

#[cfg(test)]
mod test {
    use conversion_paths_2::ConversionPaths;

    #[test]
    fn build_test() {
        let mut sessions: Vec<&[u32]> = Vec::new();
        sessions.push(&[1, 2]);
        sessions.push(&[1, 2, 3]);
        sessions.push(&[1, 3]);

        assert_eq!(
            ConversionPaths {
                goal_id: 1,
                count: 3,
                children: vec![
                    ConversionPaths {
                        goal_id: 2,
                        count: 2,
                        children: vec![
                            ConversionPaths {
                                goal_id: 3,
                                count: 1,
                                children: vec![]
                            }   
                        ],
                    },
                    ConversionPaths {
                        goal_id: 3,
                        count: 1,
                        children: vec![]
                    }
                ],
            },
            ConversionPaths::build(&sessions)
        )
    }
}