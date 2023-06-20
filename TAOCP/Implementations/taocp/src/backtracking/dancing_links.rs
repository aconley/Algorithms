// An implementation of Knuth's Algorithm X: Dancing links from
// TAOCP Volume 4B 7.2.2.1.

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::num::NonZeroU16;

const PRIMARY_LIST_HEAD: &str = "PRIMARY LIST HEAD";
const SECONDARY_LIST_HEAD: &str = "SECONDARY LIST HEAD";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProblemOption {
    primary_items: Vec<String>,
    secondary_items: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct DancingLinksIterator {
    state: IteratorState,
}

#[derive(Debug)]
enum IteratorState {
    DONE,
    NEW(InitialState),
    READY {
        x: Vec<u16>,
        solution_state: Box<SolutionState>,
    },
}

impl DancingLinksIterator {
    pub fn new(options: Vec<ProblemOption>) -> Result<Self, DancingLinksError> {
        if options.is_empty() {
            return Ok(DancingLinksIterator {
                state: IteratorState::DONE,
            });
        }

        let primary_items: BTreeSet<String> = options
            .iter()
            .flat_map(|option| option.primary_items.clone())
            .collect();
        let secondary_items: BTreeSet<String> = options
            .iter()
            .flat_map(|option| option.secondary_items.clone())
            .collect();

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

        let num_nodes: usize =
            n_items + options.len() + options.iter().map(|option| option.len()).sum::<usize>() + 1;
        if num_nodes > u16::MAX as usize {
            // The node points will run out of range.
            return Err(DancingLinksError::new(format!(
                "Too many total nodes required: {} > {}",
                num_nodes,
                u16::MAX
            )));
        }

        Ok(DancingLinksIterator {
            state: IteratorState::NEW(InitialState {
                primary_items,
                secondary_items,
                options,
                num_nodes: num_nodes as u16,
            }),
        })
    }
}

impl Iterator for DancingLinksIterator {
    type Item = Vec<ProblemOption>;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.state {
            IteratorState::DONE => None,
            IteratorState::NEW(ref mut initial_state) => {
                let mut solution_state =
                    Box::new(SolutionState::initiate(std::mem::take(initial_state)));
                match solution_state.choose_next_item() {
                    None => {
                        // No possible moves immediately.
                        self.state = IteratorState::DONE;
                        None
                    }
                    Some(next_item) => {
                        solution_state.cover(next_item.get());
                        self.state = IteratorState::READY {
                            x: vec![next_item.get()],
                            solution_state,
                        };
                        self.next()
                    }
                }
            }
            IteratorState::READY {
                ref mut x,
                ref mut solution_state,
            } => {
                loop {
                    // Backtrack.
                    let mut xl = x[x.len() - 1];
                    while xl
                        == solution_state.nodes[solution_state.nodes[xl as usize].top as usize]
                            .ulink
                    {
                        // The last x was pointing at the final option for this item,
                        // so we have to backtrack.
                        solution_state.unapply_move(xl);
                        solution_state.uncover(solution_state.nodes[xl as usize].top as u16);
                        x.pop();
                        if x.is_empty() {
                            // Terminate.
                            self.state = IteratorState::DONE;
                            return None;
                        }
                        xl = x[x.len() - 1];
                    }

                    // Try the next value of xl.
                    let xl_idx = x.len() - 1;
                    x[xl_idx] = solution_state.nodes[x[xl_idx] as usize].dlink;
                    solution_state.apply_move(x[xl_idx]);

                    // Done?
                    if solution_state.items[0].rlink == 0 {
                        return Some(
                            x.iter()
                                .map(|&x| solution_state.to_option(x as usize))
                                .collect(),
                        );
                    }

                    // Not done, select the next item we will try.
                    match solution_state.choose_next_item() {
                        None => (), // No choice was possible, let the loop try the next value.
                        Some(next_item) => {
                            solution_state.cover(next_item.get());
                            x.push(next_item.get());
                        }
                    }
                }
            }
        }
    }
}

// InitialState is the initial info needed to create SolutionState.
#[derive(Debug, Default)]
struct InitialState {
    primary_items: BTreeSet<String>,
    secondary_items: BTreeSet<String>,
    options: Vec<ProblemOption>,
    num_nodes: u16,
}

// SolutionState is the internal representation of the dancing links state.
#[derive(Clone, Debug, Eq, PartialEq)]
struct SolutionState {
    num_primary_items: u16,
    items: Vec<Item>,
    nodes: Vec<Node>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Item {
    // The number of active options that involve this item.
    len: u16,
    // The link to the preceding item.
    llink: u16,
    // The link to the next item.
    rlink: u16,
    // The name of the item.
    name: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Node {
    // Which item this node represents.  Non-positive values for top are spacer
    // nodes.
    top: i16,
    // Link to the link above this one.
    ulink: u16,
    // Link to the node below this one.
    dlink: u16,
}

impl SolutionState {
    fn initiate(initial_state: InitialState) -> Self {
        let n_primary = initial_state.primary_items.len();
        let n_secondary = initial_state.secondary_items.len();

        let mut items = std::iter::once(Item {
            len: 0,
            llink: n_primary as u16,
            rlink: 1,
            name: PRIMARY_LIST_HEAD.to_string(),
        })
        .chain(
            initial_state
                .primary_items
                .into_iter()
                .chain(initial_state.secondary_items.into_iter())
                .enumerate()
                .map(|(idx, name)| Item {
                    len: 0,
                    llink: idx as u16,
                    rlink: idx as u16 + 2,
                    name: name,
                }),
        )
        .chain(std::iter::once(Item {
            len: 0,
            llink: (n_primary + n_secondary) as u16,
            rlink: (n_primary + 1) as u16,
            name: SECONDARY_LIST_HEAD.to_string(),
        }))
        .collect::<Vec<_>>();
        items[n_primary as usize].rlink = 0;
        items[(n_primary + 1) as usize].llink = (n_primary + n_secondary + 1) as u16;

        let name_to_item_index = items
            .iter()
            .enumerate()
            .skip(1)
            .take(n_primary + n_secondary)
            .map(|(idx, item)| (&item.name, idx))
            .collect::<HashMap<&String, usize>>();

        let mut nodes = Vec::with_capacity(initial_state.num_nodes as usize);
        // Create the item header nodes.
        nodes.push(Node::default()); // Unused 'spacer'
        for idx in 1..(items.len() as u16 - 1) {
            nodes.push(Node {
                top: idx as i16,
                ulink: idx,
                dlink: idx,
            });
        }

        // The first (real) spacer.
        nodes.push(Node::default());
        let mut last_spacer_idx = nodes.len() - 1;

        // Now the nodes representing the options.
        let mut spacer_top = 0;
        for option in initial_state.options.into_iter() {
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

        // Count the number of times each item is used.
        drop(name_to_item_index);
        for (item_idx, item) in items
            .iter_mut()
            .enumerate()
            .skip(1)
            .take(n_primary + n_secondary)
        {
            let mut curr_idx = nodes[item_idx].dlink as usize;
            while curr_idx != item_idx {
                curr_idx = nodes[curr_idx].dlink as usize;
                item.len += 1;
            }
        }

        SolutionState {
            num_primary_items: n_primary as u16,
            items: items,
            nodes: nodes,
        }
    }

    fn item_name(&self, idx: u16) -> Option<&String> {
        let idx_u = idx as usize;
        if idx_u >= self.items.len() {
            None
        } else {
            Some(&self.items[idx_u].name)
        }
    }

    fn to_option(&self, mut x: usize) -> ProblemOption {
        while self.nodes[x - 1].top > 0 {
            x -= 1;
        }
        let mut res = ProblemOption {
            primary_items: Vec::default(),
            secondary_items: Vec::default(),
        };
        // x now points at the first non-spacer node of the solution.
        let n_primary = self.num_primary_items as i16;
        while self.nodes[x].top > 0 && self.nodes[x].top <= n_primary {
            res.primary_items
                .push(self.items[self.nodes[x].top as usize].name.clone());
            x += 1;
        }
        while self.nodes[x].top > 0 {
            res.secondary_items
                .push(self.items[self.nodes[x].top as usize].name.clone());
            x += 1;
        }
        res
    }

    fn cover(&mut self, item: u16) {
        let mut p = self.nodes[item as usize].dlink;
        while p != item {
            self.hide(p);
            p = self.nodes[p as usize].dlink;
        }
        let l = self.items[item as usize].llink;
        let r = self.items[item as usize].rlink;
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
                self.items[x as usize].len -= 1;
                q += 1;
            }
        }
    }

    fn uncover(&mut self, item: u16) {
        let l = self.items[item as usize].llink;
        let r = self.items[item as usize].rlink;
        self.items[l as usize].rlink = item;
        self.items[r as usize].llink = item;

        let mut p = self.nodes[item as usize].ulink;
        while p != item {
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
                self.items[x as usize].len += 1;
                q -= 1;
            }
        }
    }

    fn apply_move(&mut self, xl: u16) {
        let mut p = xl + 1;
        while p != xl {
            let j = self.nodes[p as usize].top;
            if j <= 0 {
                // Spacer.
                p = self.nodes[p as usize].ulink;
            } else {
                self.cover(j as u16);
                p += 1;
            }
        }
    }

    fn unapply_move(&mut self, xl: u16) {
        let mut p = xl + 1;
        while p != xl {
            let j = self.nodes[p as usize].top;
            if j <= 0 {
                // Spacer.
                p = self.nodes[p as usize].ulink;
            } else {
                self.uncover(j as u16);
                p += 1;
            }
        }
    }

    // Choose the next item using the MRV heuristic.  Returns None if there is
    // no move.
    fn choose_next_item(&self) -> Option<NonZeroU16> {
        let mut current_item_idx = self.items[0].rlink;
        if current_item_idx == 0 {
            return None;
        }

        let mut best_len = self.items[current_item_idx as usize].len;
        if best_len == 0 {
            return None;
        }
        let mut best_item = current_item_idx;

        current_item_idx = self.items[current_item_idx as usize].rlink;
        while current_item_idx != 0 && current_item_idx <= self.num_primary_items {
            let current_len = self.items[current_item_idx as usize].len;
            if current_len == 0 {
                // Item has no choices.
                best_item = 0;
                break;
            }
            if current_len < best_len {
                best_len = current_len;
                best_item = current_item_idx;
            }

            current_item_idx = self.items[current_item_idx as usize].rlink;
        }
        NonZeroU16::new(best_item)
    }
}

#[cfg(test)]
mod tests {
    use crate::backtracking::dancing_links::{
        DancingLinksError, DancingLinksIterator, IteratorState, ProblemOption, SolutionState,
    };

    fn create_solution_state(
        options: Vec<ProblemOption>,
    ) -> Result<SolutionState, DancingLinksError> {
        let iterator = DancingLinksIterator::new(options)?;
        match iterator.state {
            IteratorState::DONE => Err(DancingLinksError::new("Iterator created in DONE state")),
            IteratorState::READY { .. } => {
                Err(DancingLinksError::new("Iterator created in READY state"))
            }
            IteratorState::NEW(initial_state) => Ok(SolutionState::initiate(initial_state)),
        }
    }

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

    mod initialize_iterator {
        use crate::backtracking::dancing_links::{
            DancingLinksIterator, IteratorState, ProblemOption,
        };
        use claim::assert_ok;

        #[test]
        fn no_options_is_done_iterator() {
            let it = assert_ok!(DancingLinksIterator::new(vec![]));
            assert!(matches!(it.state, IteratorState::DONE));
        }

        #[test]
        fn single_option_is_in_new_state() {
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &["b"],
            ));

            let it = assert_ok!(DancingLinksIterator::new(vec![option]));
            assert!(matches!(it.state, IteratorState::NEW { .. }));
        }

        #[test]
        fn item_that_is_primary_and_secondary_is_error() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["c"],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["c"],
                /*secondary_items=*/ &[],
            ));
            let res = DancingLinksIterator::new(vec![option1, option2]);

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("both primary and secondary"));
        }
    }

    mod initialize_solution_state {
        use super::create_solution_state;
        use crate::backtracking::dancing_links::{
            Item, Node, ProblemOption, SolutionState, PRIMARY_LIST_HEAD, SECONDARY_LIST_HEAD,
        };
        use claim::{assert_ok, assert_ok_eq};

        #[test]
        fn single_primary_item_initializes() {
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));

            assert_ok_eq!(
                create_solution_state(vec![option]),
                SolutionState {
                    num_primary_items: 1,
                    items: vec![
                        Item {
                            len: 0,
                            llink: 1,
                            rlink: 1,
                            name: PRIMARY_LIST_HEAD.to_string()
                        },
                        Item {
                            len: 1,
                            llink: 0,
                            rlink: 0,
                            name: "a".to_string()
                        },
                        Item {
                            len: 0,
                            llink: 2,
                            rlink: 2,
                            name: SECONDARY_LIST_HEAD.to_string()
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
                create_solution_state(vec![option1, option2]),
                SolutionState {
                    num_primary_items: 2,
                    items: vec![
                        Item {
                            len: 0,
                            llink: 2,
                            rlink: 1,
                            name: PRIMARY_LIST_HEAD.to_string()
                        },
                        // Primary
                        Item {
                            len: 1,
                            llink: 0,
                            rlink: 2,
                            name: "a".to_string()
                        },
                        Item {
                            len: 2,
                            llink: 1,
                            rlink: 0,
                            name: "b".to_string()
                        },
                        // Secondary
                        Item {
                            len: 1,
                            llink: 4,
                            rlink: 4,
                            name: "c".to_string()
                        },
                        Item {
                            len: 0,
                            llink: 3,
                            rlink: 3,
                            name: SECONDARY_LIST_HEAD.to_string()
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
                            top: 3,
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
                create_solution_state(vec![option1, option2, option3, option4, option5, option6]),
                SolutionState {
                    num_primary_items: 5,
                    items: vec![
                        // Primary items.
                        Item {
                            len: 0,
                            llink: 5,
                            rlink: 1,
                            name: PRIMARY_LIST_HEAD.to_string()
                        },
                        Item {
                            len: 2,
                            llink: 0,
                            rlink: 2,
                            name: "a".to_string()
                        },
                        Item {
                            len: 2,
                            llink: 1,
                            rlink: 3,
                            name: "b".to_string()
                        },
                        Item {
                            len: 2,
                            llink: 2,
                            rlink: 4,
                            name: "c".to_string()
                        },
                        Item {
                            len: 3,
                            llink: 3,
                            rlink: 5,
                            name: "d".to_string()
                        },
                        Item {
                            len: 2,
                            llink: 4,
                            rlink: 0,
                            name: "e".to_string()
                        },
                        // Secondary items
                        Item {
                            len: 2,
                            llink: 8,
                            rlink: 7,
                            name: "f".to_string(),
                        },
                        Item {
                            len: 3,
                            llink: 6,
                            rlink: 8,
                            name: "g".to_string(),
                        },
                        Item {
                            len: 0,
                            llink: 7,
                            rlink: 6,
                            name: SECONDARY_LIST_HEAD.to_string()
                        }
                    ],
                    nodes: vec![
                        // Node 0: empty
                        Node::default(),
                        // Node 1: header node for 'a'
                        Node {
                            top: 1,
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
                            top: 3,
                            ulink: 17, // ProblemOption b, c, f
                            dlink: 9   // ProblemOption c, e
                        },
                        // Node 4: header node for 'd'
                        Node {
                            top: 4,
                            ulink: 27, // ProblemOption d e g
                            dlink: 13, // ProblemOption a d g
                        },
                        // Node 5: header node for 'e'
                        Node {
                            top: 5,
                            ulink: 28, // ProblemOption d e g
                            dlink: 10  // ProblemOption c e
                        },
                        // Node 6: header node for 'f'
                        Node {
                            top: 6,
                            ulink: 22, // ProblemOption a d f
                            dlink: 18  // ProblemOption b c f
                        },
                        // Node 7: header node for 'g'
                        Node {
                            top: 7,
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

    mod choose_next {
        use super::create_solution_state;
        use crate::backtracking::dancing_links::ProblemOption;
        use claim::{assert_ok, assert_some, assert_some_eq};
        use std::collections::HashSet;

        #[test]
        fn single_primary_item_chooses_only_option() {
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));
            let solution_state = assert_ok!(create_solution_state(vec![option]));

            assert_some_eq!(
                solution_state
                    .choose_next_item()
                    .and_then(|v| solution_state.item_name(v.get())),
                "a"
            );
        }

        #[test]
        fn chooses_item_with_fewest_options() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b", "c"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "c"],
                /*secondary_items=*/ &[],
            ));

            let solution_state = assert_ok!(create_solution_state(vec![option1, option2]));

            assert_eq!(solution_state.num_primary_items, 3);
            // Check the LEN vars are what we expect.
            assert_eq!(
                solution_state
                    .items
                    .iter()
                    .skip(1)
                    .take(3)
                    .map(|item| item.len)
                    .collect::<Vec<_>>(),
                vec![2, 1, 2]
            );
            assert_eq!(
                solution_state
                    .items
                    .iter()
                    .skip(1)
                    .take(3)
                    .map(|item| item.rlink)
                    .collect::<Vec<_>>(),
                vec![2, 3, 0]
            );

            assert_some_eq!(
                solution_state
                    .choose_next_item()
                    .and_then(|v| solution_state.item_name(v.get())),
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
            let solution_state = assert_ok!(create_solution_state(vec![option1, option2]));

            assert_some_eq!(
                solution_state
                    .choose_next_item()
                    .and_then(|v| solution_state.item_name(v.get())),
                "a"
            );
        }

        #[test]
        fn large_test_case_chooses_next_item() {
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

            let solution_state = assert_ok!(create_solution_state(vec![
                option1, option2, option3, option4, option5, option6
            ]));
            assert_eq!(solution_state.num_primary_items, 5);

            // b, c, e, f are all acceptable choices.  d is not because
            // it has 3 items, a and g are not because they are secondary.
            let choice = assert_some!(solution_state
                .choose_next_item()
                .and_then(|v| solution_state.item_name(v.get())));
            assert!(["b", "c", "e", "f"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<HashSet<_>>()
                .contains(choice));
        }
    }

    mod cover {
        use super::create_solution_state;
        use crate::backtracking::dancing_links::ProblemOption;
        use claim::assert_ok;

        #[test]
        fn single_primary_item_cover() {
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));

            let mut state = assert_ok!(create_solution_state(vec![option]));

            // For a single item, cover only affects the item links.
            let mut expected_state = state.clone();
            expected_state.items[0].llink = 0;
            expected_state.items[0].rlink = 0;

            state.cover(1);
            assert_eq!(state, expected_state);
        }

        #[test]
        fn large_test_case_cover() {
            // This is the example from TAOCP 7.2.2.1 (6), updated following
            // exercise 11 from that section.
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "d", "g"],
                /*secondary_items=*/ &[],
            ));
            let option3 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b", "c", "f"],
                /*secondary_items=*/ &[],
            ));
            let option4 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "d", "f"],
                /*secondary_items=*/ &[],
            ));
            let option5 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b", "g"],
                /*secondary_items=*/ &[],
            ));
            let option6 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["d", "e", "g"],
                /*secondary_items=*/ &[],
            ));

            let mut state = assert_ok!(create_solution_state(vec![
                option1, option2, option3, option4, option5, option6
            ]));

            // cover(1).
            let mut expected_state = state.clone();
            // Item 1 ('a') will be unlinked.
            expected_state.items[0].rlink = 2;
            expected_state.items[2].llink = 0;
            // From hide(12)
            expected_state.nodes[4].dlink = 2;
            expected_state.nodes[21].ulink = 4;
            expected_state.items[4].len = 2;
            expected_state.nodes[7].dlink = 25;
            expected_state.nodes[25].ulink = 7;
            expected_state.items[7].len = 2;
            // From hide(20)
            expected_state.nodes[4].dlink = 27;
            expected_state.nodes[27].ulink = 4;
            expected_state.items[4].len = 1;
            expected_state.nodes[18].dlink = 6;
            expected_state.nodes[6].ulink = 18;
            expected_state.items[6].len = 1;

            state.cover(1);
            assert_eq!(state, expected_state);

            // cover(4).
            expected_state.items[5].llink = 3;
            expected_state.items[3].rlink = 5;
            expected_state.nodes[10].dlink = 5;
            expected_state.nodes[5].ulink = 10;
            expected_state.items[5].len = 1;
            expected_state.nodes[25].dlink = 7;
            expected_state.nodes[7].ulink = 25;
            expected_state.items[7].len = 1;

            state.cover(4);
            assert_eq!(state, expected_state);

            // cover(7)
            expected_state.items[6].rlink = 0;
            expected_state.items[0].llink = 6;
            expected_state.nodes[16].dlink = 2;
            expected_state.nodes[2].ulink = 16;
            expected_state.items[2].len = 1;

            state.cover(7);
            assert_eq!(state, expected_state);

            // cover(2)
            expected_state.items[3].llink = 0;
            expected_state.items[0].rlink = 3;
            expected_state.nodes[3].ulink = 9;
            expected_state.nodes[9].dlink = 3;
            expected_state.items[3].len = 1;
            expected_state.nodes[6].ulink = 6;
            expected_state.nodes[6].dlink = 6;
            expected_state.items[6].len = 0;

            state.cover(2);
            assert_eq!(state, expected_state);
        }

        #[test]
        fn single_primary_item_uncover() {
            // Check that uncover undoes what we tested in single_primary_item_cover
            let option = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));
            let initial_state = assert_ok!(create_solution_state(vec![option]));

            let mut state = initial_state.clone();
            state.cover(1);
            state.uncover(1);

            assert_eq!(state, initial_state);
        }

        #[test]
        fn large_test_case_uncover() {
            // This is the example from TAOCP 7.2.2.1 (6), updated following
            // exercise 11 from that section.
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "d", "g"],
                /*secondary_items=*/ &[],
            ));
            let option3 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b", "c", "f"],
                /*secondary_items=*/ &[],
            ));
            let option4 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "d", "f"],
                /*secondary_items=*/ &[],
            ));
            let option5 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b", "g"],
                /*secondary_items=*/ &[],
            ));
            let option6 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["d", "e", "g"],
                /*secondary_items=*/ &[],
            ));

            let initial_state = assert_ok!(create_solution_state(vec![
                option1, option2, option3, option4, option5, option6
            ]));

            let mut state = initial_state.clone();
            state.cover(1);
            let state_before_cover_4 = state.clone();
            state.cover(4);
            let state_before_cover_7 = state.clone();
            state.cover(7);
            let state_before_cover_2 = state.clone();
            state.cover(2);
            let state_before_cover_3 = state.clone();
            state.cover(3);

            state.uncover(3);
            assert_eq!(
                state, state_before_cover_3,
                "uncover(3) did not reverse cover(3)"
            );
            state.uncover(2);
            assert_eq!(
                state, state_before_cover_2,
                "uncover(2) did not reverse cover(2)"
            );
            state.uncover(7);
            assert_eq!(
                state, state_before_cover_7,
                "uncover(7) did not reverse cover(7)"
            );
            state.uncover(4);
            assert_eq!(
                state, state_before_cover_4,
                "uncover(4) did not reverse cover(4)"
            );
            state.uncover(1);
            assert_eq!(state, initial_state, "uncover(1) did not reverse cover(1)");
        }
    }

    mod to_solution {
        use super::create_solution_state;
        use crate::backtracking::dancing_links::{ProblemOption, SolutionState};
        use claim::assert_ok;

        #[test]
        fn small_test_case_get_options() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            ));

            let state: SolutionState = assert_ok!(create_solution_state(vec![
                option1.clone(),
                option2.clone()
            ]));

            assert_eq!(state.to_option(5), option1);
            assert_eq!(state.to_option(6), option1);
            assert_eq!(state.to_option(8), option2);
        }

        #[test]
        fn large_test_case_get_options() {
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

            let state = assert_ok!(create_solution_state(vec![
                option1.clone(),
                option2,
                option3.clone(),
                option4,
                option5,
                option6
            ]));

            assert_eq!(state.to_option(9), option1);
            assert_eq!(state.to_option(10), option1);
            assert_eq!(state.to_option(16), option3);
        }
    }

    mod iterates_over_solutions {
        use crate::backtracking::dancing_links::{DancingLinksIterator, ProblemOption};
        use claim::{assert_none, assert_ok, assert_some_eq};

        #[test]
        fn single_item_one_option_solution() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            ));
            let mut iterator = assert_ok!(DancingLinksIterator::new(vec![option1.clone()]));

            assert_some_eq!(iterator.next(), vec![option1]);
            assert_none!(iterator.next());
        }

        #[test]
        fn small_test_case_iterator() {
            let option1 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            ));
            let option2 = assert_ok!(ProblemOption::new_from_str(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            ));

            let mut iterator =
                assert_ok!(DancingLinksIterator::new(vec![option1.clone(), option2]));

            assert_some_eq!(iterator.next(), vec![option1]);
            assert_none!(iterator.next());
        }

        #[test]
        fn large_test_case_iterator() {
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

            let mut iterator = assert_ok!(DancingLinksIterator::new(vec![
                option1.clone(),
                option2,
                option3,
                option4.clone(),
                option5.clone(),
                option6
            ]));

            assert_some_eq!(iterator.next(), vec![option1, option4, option5]);
            assert_none!(iterator.next());
        }
    }
}
