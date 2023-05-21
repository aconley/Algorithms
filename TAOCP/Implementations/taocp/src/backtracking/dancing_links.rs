// An implementation of Knuth's Algorithm X: Dancing links from
// TAOCP Volume 4B 7.2.2.1.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

const EMPTY_ITEM_STRING: &str = "EMPTY ITEM";

#[derive(Debug, PartialEq, Eq)]
pub struct ProblemOption {
    primary_items: Vec<String>,
    secondary_items: Vec<String>,
}

#[derive(Debug)]
pub struct DancingLinksError(String);

impl DancingLinksError {
    fn new<S: Into<String>>(message: S) -> DancingLinksError {
        DancingLinksError(message.into())
    }
}

impl ProblemOption {
    pub fn new(
        primary_items: Vec<String>,
        secondary_items: Vec<String>,
    ) -> Result<ProblemOption, DancingLinksError> {
        // Reject duplicates, or items that are both primary and secondary.
        if primary_items.is_empty() {
            return Err(DancingLinksError::new(
                "ProblemOption must contain at least one primary item",
            ));
        }
        let primary_set = primary_items.iter().collect::<HashSet<&String>>();
        if primary_set.len() < primary_items.len() {
            return Err(DancingLinksError::new(
                "primary_items contained at least one duplicate",
            ));
        }
        if !secondary_items.is_empty() {
            let secondary_set = secondary_items.iter().collect::<HashSet<&String>>();
            if secondary_set.len() < secondary_items.len() {
                return Err(DancingLinksError::new(
                    "secondary_items contained at least one duplicate",
                ));
            }
            if primary_set.intersection(&secondary_set).next().is_some() {
                return Err(DancingLinksError::new(
                    "Primary and secondary items overlap",
                ));
            }
        }
        Ok(ProblemOption {
            primary_items,
            secondary_items,
        })
    }

    pub fn new_from_str(
        primary_items: &[&str],
        secondary_items: &[&str],
    ) -> Result<ProblemOption, DancingLinksError> {
        ProblemOption::new(
            primary_items
                .into_iter()
                .map(|item| String::from(*item))
                .collect(),
            secondary_items
                .into_iter()
                .map(|item| String::from(*item))
                .collect(),
        )
    }

    fn len(&self) -> usize {
        self.primary_items.len() + self.secondary_items.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    // The link to the preceding item.
    llink: u16,
    // The link to the next item.
    rlink: u16,
    // The name of the item.
    name: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Node {
    // Which item this node represents.  Non-positive values for top are spacer
    // nodes, and in header nodes top represents the number of active items.
    top: i16,
    // Link to the link above this one.
    ulink: u16,
    // Link to the node below this one.
    dlink: u16,
}

#[derive(Debug, Eq, PartialEq)]
struct SolutionState {
    num_primary_items: u16,
    items: Vec<Item>,
    nodes: Vec<Node>,
}

impl SolutionState {
    pub fn initiate(options: Vec<ProblemOption>) -> Result<SolutionState, DancingLinksError> {
        if options.is_empty() {
            return Err(DancingLinksError::new(
                "At least one ProblemOption must be provided",
            ));
        }

        // Build up an ordered list of the primary and secondary items.
        let primary_items: BTreeSet<String> = options
            .iter()
            .flat_map(|option| option.primary_items.clone())
            .collect::<BTreeSet<String>>();
        let secondary_items: BTreeSet<String> = options
            .iter()
            .flat_map(|option| option.secondary_items.clone())
            .collect::<BTreeSet<String>>();
        if primary_items.is_empty() {
            return Err(DancingLinksError::new("No primary items in options"));
        }

        // Make sure there are no overlaps.
        {
            let mut primary_and_secondary = primary_items.intersection(&secondary_items).peekable();
            if primary_and_secondary.peek().is_some() {
                return Err(DancingLinksError::new(format!(
                    "Can't have items that are both primary and secondary: {:?}",
                    primary_and_secondary
                )));
            }
        }

        let n_items = primary_items.len() + secondary_items.len();
        if n_items > i16::MAX as usize {
            return Err(DancingLinksError::new(format!(
                "Too many items: {} > {}",
                n_items,
                i16::MAX
            )));
        }
        let num_primary = primary_items.len() as u16;

        let mut items = std::iter::once(Item {
            llink: n_items as u16,
            rlink: 1,
            name: EMPTY_ITEM_STRING.to_string(),
        })
        .chain(
            primary_items
                .into_iter()
                .chain(secondary_items.into_iter())
                .enumerate()
                .map(|(idx, name)| Item {
                    llink: idx as u16,
                    rlink: idx as u16 + 2,
                    name: name,
                }),
        )
        .collect::<Vec<_>>();
        items[n_items].rlink = 0;

        let name_to_item_index = items
            .iter()
            .enumerate()
            .skip(1)
            .map(|(idx, item)| (&item.name, idx))
            .collect::<HashMap<&String, usize>>();

        let num_nodes: usize = items.len()
            + options.len()
            + options.iter().map(|option| option.len()).sum::<usize>()
            + 1;
        if num_nodes > u16::MAX as usize {
            // The node points will run out of range.
            return Err(DancingLinksError::new(format!(
                "Too many total nodes required: {} > {}",
                num_nodes,
                u16::MAX
            )));
        }
        let mut nodes = Vec::with_capacity(num_nodes);
        // Create the item header nodes.
        nodes.push(Node::default()); // Unused 'spacer'
        for idx in 1..(items.len() as u16) {
            nodes.push(Node {
                top: 0, // Len in headers.
                ulink: idx,
                dlink: idx,
            });
        }

        // The first (real) spacer.
        nodes.push(Node::default());
        let mut last_spacer_idx = nodes.len() - 1;

        // Now the nodes representing the options.
        let mut spacer_top = 0;
        for option in options.into_iter() {
            for item_name in option
                .primary_items
                .iter()
                .chain(option.secondary_items.iter())
            {
                let item_idx = name_to_item_index[&item_name];

                // Update the previous last occurrence of this item.
                let node_idx = nodes.len();
                let prev_tail_idx = nodes[item_idx].ulink as usize;
                nodes[prev_tail_idx].dlink = node_idx as u16;
                nodes.push(Node {
                    top: item_idx as i16,
                    ulink: prev_tail_idx as u16,
                    dlink: item_idx as u16,
                });

                // Update the header
                nodes[item_idx].top += 1; // Len in header nodes.
                nodes[item_idx].ulink = node_idx as u16;
            }
            // Update the spacer before this option to point at the last node
            // from this option.
            nodes[last_spacer_idx].dlink = (nodes.len() - 1) as u16;
            // Add a new spacer.
            spacer_top -= 1;
            nodes.push(Node {
                top: spacer_top,
                ulink: last_spacer_idx as u16 + 1,
                dlink: 0,
            });
            last_spacer_idx = nodes.len() - 1;
        }

        Ok(SolutionState {
            num_primary_items: num_primary,
            items: items,
            nodes: nodes,
        })
    }

    fn item_name(&self, idx: u16) -> Option<&String> {
        if idx == 0 || idx >= self.items.len() as u16 {
            None
        } else {
            Some(&self.items[idx as usize].name)
        }
    }

    fn cover(&mut self, i: u16) {
        let mut p = self.nodes[i as usize].dlink;
        while p != i {
            self.hide(p);
            p = self.nodes[p as usize].dlink;
        }
        let l = self.items[i as usize].llink;
        let r = self.items[i as usize].rlink;
        self.items[l as usize].rlink = r;
        self.items[r as usize].llink = l;
    }

    fn hide(&mut self, p: u16) {
        let mut q = p + 1;
        while q != p {
            let x = self.nodes[q as usize].top;
            let u = self.nodes[q as usize].ulink;
            if x <= 0 {
                // Spacer node.
                q = u;
            } else {
                let d = self.nodes[q as usize].dlink;
                self.nodes[u as usize].dlink = d;
                self.nodes[d as usize].ulink = u;
                self.nodes[x as usize].top -= 1;
                q += 1;
            }
        }
    }

    fn uncover(&mut self, i: u16) {
        let l = self.items[i as usize].llink;
        let r = self.items[i as usize].rlink;
        self.items[l as usize].rlink = i;
        self.items[r as usize].llink = i;

        let mut p = self.nodes[i as usize].ulink;
        while p != i {
            self.unhide(p);
            p = self.nodes[p as usize].ulink;
        }
    }

    fn unhide(&mut self, p: u16) {
        let mut q = p - 1;
        while q != p {
            let x = self.nodes[q as usize].top;
            let d = self.nodes[q as usize].dlink;
            if x <= 0 {
                // Spacer node.
                q = d;
            } else {
                let u = self.nodes[q as usize].ulink;
                self.nodes[u as usize].dlink = q;
                self.nodes[d as usize].ulink = q;
                self.nodes[x as usize].top += 1;
                q -= 1;
            }
        }
    }

    // Chose the next item using the MRV heuristic.  Assumes there is at
    // least one uncovered item.
    fn chose_next_item_mrv(&self) -> u16 {
        let mut current_item_idx = self.items[0].rlink;
        debug_assert!(current_item_idx != 0);

        let mut best_len = self.nodes[current_item_idx as usize].top;
        if best_len == 0 {
            return current_item_idx;
        }
        let mut best_item = current_item_idx;

        current_item_idx = self.items[current_item_idx as usize].rlink;
        while current_item_idx != 0 && current_item_idx <= self.num_primary_items {
            let current_len = self.nodes[current_item_idx as usize].top;
            if current_len == 0 {
                // This item has no choices, we should abort.
                return current_item_idx;
            }
            if current_len < best_len {
                best_len = current_len;
                best_item = current_item_idx;
            }

            current_item_idx = self.items[current_item_idx as usize].rlink;
        }
        return best_item;
    }
}

#[cfg(test)]
mod tests {
    mod options {
        use crate::backtracking::dancing_links::ProblemOption;

        #[test]
        fn option_has_expected_items() {
            let res = ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["c", "d"],
            );

            assert!(res.is_ok());
            let opt = res.unwrap();
            assert_eq!(opt.primary_items, vec!["a", "b"]);
            assert_eq!(opt.secondary_items, vec!["c", "d"]);
        }

        #[test]
        fn option_with_no_secondary_items_is_allowed() {
            let res = ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            );

            assert!(res.is_ok());
            let opt = res.unwrap();
            assert_eq!(opt.primary_items, vec!["a", "b"]);
            assert!(opt.secondary_items.is_empty());
        }

        #[test]
        fn option_with_no_primary_items_is_error() {
            let res = ProblemOption::new_from_str(
                /*primary_items=*/ &[],
                /*secondary_items=*/ &["a"],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("at least one primary item"));
        }

        #[test]
        fn duplicate_primary_is_error() {
            let res = ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b", "c", "b"],
                /*secondary_items=*/ &[],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("primary_items"));
        }

        #[test]
        fn duplicate_secondary_is_error() {
            let res = ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["d", "e", "e"],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("secondary_items"));
        }

        #[test]
        fn item_that_is_both_primary_and_secondary_is_error() {
            let res = ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["c", "a", "e"],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("overlap"));
        }
    }

    mod initialization {
        use crate::backtracking::dancing_links::{
            Item, Node, ProblemOption, SolutionState, EMPTY_ITEM_STRING,
        };
        use claim::{assert_ok, assert_ok_eq};

        #[test]
        fn single_primary_item_initializes() {
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));

            assert_ok_eq!(
                SolutionState::initiate(vec![option]),
                SolutionState {
                    num_primary_items: 1,
                    items: vec![
                        Item {
                            llink: 1,
                            rlink: 1,
                            name: EMPTY_ITEM_STRING.to_string()
                        },
                        Item {
                            llink: 0,
                            rlink: 0,
                            name: "a".to_string()
                        }
                    ],
                    nodes: vec![
                        // Node 0: empty
                        Node::default(),
                        // Node 1: header node for only item.
                        Node {
                            top: 1,
                            ulink: 3,
                            dlink: 3
                        },
                        // Node 2: Spacer node.
                        Node {
                            top: 0,
                            ulink: 0,
                            dlink: 3
                        },
                        // Node 3: Only item node.
                        Node {
                            top: 1,
                            ulink: 1,
                            dlink: 1
                        },
                        // Node 4: Spacer node.
                        Node {
                            top: -1,
                            ulink: 3,
                            dlink: 0
                        }
                    ],
                }
            );
        }

        #[test]
        fn small_test_case_initializes() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            ));

            assert_ok_eq!(
                SolutionState::initiate(vec![option1, option2]),
                SolutionState {
                    num_primary_items: 2,
                    items: vec![
                        Item {
                            llink: 3,
                            rlink: 1,
                            name: EMPTY_ITEM_STRING.to_string()
                        },
                        Item {
                            llink: 0,
                            rlink: 2,
                            name: "a".to_string()
                        },
                        Item {
                            llink: 1,
                            rlink: 3,
                            name: "b".to_string()
                        },
                        Item {
                            llink: 2,
                            rlink: 0,
                            name: "c".to_string()
                        }
                    ],
                    nodes: vec![
                        // Node 0: empty
                        Node::default(),
                        // Node 1: header node for 'a'
                        Node {
                            top: 1,
                            ulink: 5,
                            dlink: 5
                        },
                        // Node 2: header node for 'b'
                        Node {
                            top: 2,
                            ulink: 8, // ProblemOption b, c
                            dlink: 6  // ProblemOption a, b
                        },
                        // Node 3: header node for 'c'
                        Node {
                            top: 1,
                            ulink: 9, // ProblemOption b, c
                            dlink: 9  // ProblemOption b, c
                        },
                        // Node 4: Spacer node.
                        Node {
                            top: 0,
                            ulink: 0, // unused.
                            dlink: 6  // Last node in option a, b
                        },
                        // Node 5: ProblemOption a, b item a
                        Node {
                            top: 1,
                            ulink: 1,
                            dlink: 1
                        },
                        // Node 6: ProblemOption a, b item b
                        Node {
                            top: 2,
                            ulink: 2,
                            dlink: 8
                        },
                        // Node 7: Spacer between a, b and b, c
                        Node {
                            top: -1,
                            ulink: 5, // First node in option a, b
                            dlink: 9  // Last node in option b, c
                        },
                        // Node 8: ProblemOption b, c item b
                        Node {
                            top: 2,
                            ulink: 6, // ProblemOption a, b
                            dlink: 2, // Header for b
                        },
                        // Node 9: ProblemOption b, c item c
                        Node {
                            top: 3,
                            ulink: 3, // Header for c
                            dlink: 3, // Header for c
                        },
                        // Node 10: final spacer
                        Node {
                            top: -2,
                            ulink: 8, // First node of option b, c
                            dlink: 0  // unused
                        }
                    ],
                }
            );
        }

        #[test]
        fn large_test_case_initializes() {
            // This is the example from TAOCP 7.2.2.1 (6), except that fg have
            // been made secondary.
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["g"],
            ));
            let option3 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b", "c"],
                /*secondary_items=*/ &["f"],
            ));
            let option4 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["f"],
            ));
            let option5 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["g"],
            ));
            let option6 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["d", "e"],
                /*secondary_items=*/ &["g"],
            ));

            assert_ok_eq!(
                SolutionState::initiate(vec![option1, option2, option3, option4, option5, option6]),
                SolutionState {
                    num_primary_items: 5,
                    items: vec![
                        // Primary items.
                        Item {
                            llink: 7,
                            rlink: 1,
                            name: EMPTY_ITEM_STRING.to_string()
                        },
                        Item {
                            llink: 0,
                            rlink: 2,
                            name: "a".to_string()
                        },
                        Item {
                            llink: 1,
                            rlink: 3,
                            name: "b".to_string()
                        },
                        Item {
                            llink: 2,
                            rlink: 4,
                            name: "c".to_string()
                        },
                        Item {
                            llink: 3,
                            rlink: 5,
                            name: "d".to_string()
                        },
                        Item {
                            llink: 4,
                            rlink: 6,
                            name: "e".to_string()
                        },
                        // Secondary items
                        Item {
                            llink: 5,
                            rlink: 7,
                            name: "f".to_string(),
                        },
                        Item {
                            llink: 6,
                            rlink: 0,
                            name: "g".to_string(),
                        }
                    ],
                    nodes: vec![
                        // Node 0: empty
                        Node::default(),
                        // Node 1: header node for 'a'
                        Node {
                            top: 2,
                            ulink: 20, // ProblemOption a d
                            dlink: 12, // ProblemOption a d g
                        },
                        // Node 2: header node for 'b'
                        Node {
                            top: 2,
                            ulink: 24, // ProblemOption b, g
                            dlink: 16  // ProblemOption b, c, f
                        },
                        // Node 3: header node for 'c'
                        Node {
                            top: 2,
                            ulink: 17, // ProblemOption b, c, f
                            dlink: 9   // ProblemOption c, e
                        },
                        // Node 4: header node for 'd'
                        Node {
                            top: 3,
                            ulink: 27, // ProblemOption d e g
                            dlink: 13, // ProblemOption a d g
                        },
                        // Node 5: header node for 'e'
                        Node {
                            top: 2,
                            ulink: 28, // ProblemOption d e g
                            dlink: 10  // ProblemOption c e
                        },
                        // Node 6: header node for 'f'
                        Node {
                            top: 2,
                            ulink: 22, // ProblemOption a d f
                            dlink: 18  // ProblemOption b c f
                        },
                        // Node 7: header node for 'g'
                        Node {
                            top: 3,
                            ulink: 29, // ProblemOption d e g
                            dlink: 14  // ProblemOption a d g
                        },
                        // Node 8: Spacer node.
                        Node {
                            top: 0,
                            ulink: 0,  // unused.
                            dlink: 10  // Last node in option c, e
                        },
                        // Nodes 9-10: ProblemOption c e
                        // Node 9: item c
                        Node {
                            top: 3,
                            ulink: 3,  // Header
                            dlink: 17, // ProblemOption b c f
                        },
                        // Node 10: item e
                        Node {
                            top: 5,
                            ulink: 5,  // Header
                            dlink: 28  // ProblemOption d e g
                        },
                        // Node 11: Spacer
                        Node {
                            top: -1,
                            ulink: 9,
                            dlink: 14
                        },
                        // Nodes 12-14: ProblemOption a d g
                        // Node 12: item a
                        Node {
                            top: 1,
                            ulink: 1,  // Header
                            dlink: 20  // ProblemOption a d f
                        },
                        // Node 13: item d
                        Node {
                            top: 4,
                            ulink: 4,  // Header
                            dlink: 21  // ProblemOption a d f
                        },
                        // Node 14: item g
                        Node {
                            top: 7,
                            ulink: 7,  // Header
                            dlink: 25, // ProblemOption b g
                        },
                        // Node 15: Spacer
                        Node {
                            top: -2,
                            ulink: 12,
                            dlink: 18
                        },
                        // Nodes 16-18: ProblemOption b c f
                        // Node 16: item b
                        Node {
                            top: 2,
                            ulink: 2,  // Header
                            dlink: 24  // ProblemOption b g
                        },
                        // Node 17: item c
                        Node {
                            top: 3,
                            ulink: 9, // ProblemOption c e
                            dlink: 3  // Header
                        },
                        // Node 18: item f
                        Node {
                            top: 6,
                            ulink: 6,  // Header
                            dlink: 22  // ProblemOption a d f
                        },
                        // Node 19: spacer
                        Node {
                            top: -3,
                            ulink: 16,
                            dlink: 22
                        },
                        // Nodes 20-22: ProblemOption a d f
                        // Node 20: item a
                        Node {
                            top: 1,
                            ulink: 12, // ProblemOption a d g
                            dlink: 1   // Header
                        },
                        // Node 21: item d
                        Node {
                            top: 4,
                            ulink: 13, // ProblemOption a d g
                            dlink: 27  // ProblemOption d e g
                        },
                        // Node 22: item f
                        Node {
                            top: 6,
                            ulink: 18, // ProblemOption b c f
                            dlink: 6   // Header
                        },
                        // Node 23: spacer
                        Node {
                            top: -4,
                            ulink: 20,
                            dlink: 25
                        },
                        // Nodes 24-25: ProblemOption b g
                        // Node 24: item b
                        Node {
                            top: 2,
                            ulink: 16, // ProblemOption b c f
                            dlink: 2   // Header
                        },
                        // Node 25: item g
                        Node {
                            top: 7,
                            ulink: 14, // ProblemOption a d g
                            dlink: 29  // ProblemOption d e g
                        },
                        // Node 26: spacer
                        Node {
                            top: -5,
                            ulink: 24,
                            dlink: 29
                        },
                        // Node 27-29: ProblemOption d e g
                        // Node 27: item d
                        Node {
                            top: 4,
                            ulink: 21, // ProblemOption a d f
                            dlink: 4   // Header
                        },
                        // Node 28: item e
                        Node {
                            top: 5,
                            ulink: 10, // ProblemOption c e
                            dlink: 5   // Header
                        },
                        // Node 29: item g
                        Node {
                            top: 7,
                            ulink: 25, // ProblemOption b g
                            dlink: 7   // Header
                        },
                        // Node 30: final spacer
                        Node {
                            top: -6,
                            ulink: 27,
                            dlink: 0 // unused
                        }
                    ],
                }
            );
        }
    }

    mod chose_next {
        use crate::backtracking::dancing_links::{ProblemOption, SolutionState};
        use claim::{assert_ok, assert_some, assert_some_eq};
        use std::collections::HashSet;

        #[test]
        fn single_primary_item_choses_only_option() {
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));
            let solution_state = assert_ok!(SolutionState::initiate(vec![option]));

            assert_some_eq!(
                solution_state.item_name(solution_state.chose_next_item_mrv()),
                "a"
            );
        }

        #[test]
        fn choses_item_with_fewest_options() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b", "c"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "c"],
                /*secondary_items=*/ &[],
            ));

            let solution_state = assert_ok!(SolutionState::initiate(vec![option1, option2]));

            assert_eq!(solution_state.num_primary_items, 3);
            assert_eq!(
                solution_state
                    .nodes
                    .iter()
                    .skip(1)
                    .take(3)
                    .map(|node| node.top)
                    .collect::<Vec<_>>(),
                vec![2, 1, 2]
            );
            assert_eq!(
                solution_state
                    .items
                    .iter()
                    .skip(1)
                    .map(|item| item.rlink)
                    .collect::<Vec<_>>(),
                vec![2, 3, 0]
            );

            assert_some_eq!(
                solution_state.item_name(solution_state.chose_next_item_mrv()),
                "b"
            );
        }

        #[test]
        fn secondary_item_is_not_chosen_even_if_it_has_fewest_available_options() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &["b"],
            ));
            let solution_state = assert_ok!(SolutionState::initiate(vec![option1, option2]));

            assert_some_eq!(
                solution_state.item_name(solution_state.chose_next_item_mrv()),
                "a"
            );
        }

        #[test]
        fn large_test_case_choses_next_item() {
            // This is the example from TAOCP 7.2.2.1 (6), except that ag have
            // been made secondary.
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["d"],
                /*secondary_items=*/ &["a", "g"],
            ));
            let option3 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b", "c", "f"],
                /*secondary_items=*/ &[],
            ));
            let option4 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["d", "f"],
                /*secondary_items=*/ &["a"],
            ));
            let option5 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["g"],
            ));
            let option6 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["d", "e"],
                /*secondary_items=*/ &["g"],
            ));

            let solution_state = assert_ok!(SolutionState::initiate(vec![
                option1, option2, option3, option4, option5, option6
            ]));
            assert_eq!(solution_state.num_primary_items, 5);

            // b, c, e, f are all acceptable choices.  d is not because
            // it has 3 items, a and g are not because they are secondary.
            let choice =
                assert_some!(solution_state.item_name(solution_state.chose_next_item_mrv()));
            assert!(["b", "c", "e", "f"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>()
                .contains(choice));
        }
    }
}
