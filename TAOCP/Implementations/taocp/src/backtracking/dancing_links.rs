// An implementation of Knuth's Algorithm X: Dancing links from
// TAOCP Volume 4B 7.2.2.1.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::num::NonZeroU16;

pub trait ProblemOption<ItemType> {
    type IteratorType: Iterator<Item = ItemType>;
    type BuilderType: ProblemOptionBuilder<ItemType, ProblemOptionType = Self>;

    fn primary_items(&self) -> Self::IteratorType;
    fn secondary_items(&self) -> Self::IteratorType;

    fn builder() -> Self::BuilderType;
}

pub trait ProblemOptionBuilder<ItemType> {
    type ProblemOptionType: ProblemOption<ItemType>;

    fn add_primary(&mut self, item: &ItemType) -> &mut Self;
    fn add_secondary(&mut self, item: &ItemType) -> &mut Self;

    fn build(self) -> Self::ProblemOptionType;
}

#[derive(Debug, PartialEq, Eq)]
pub struct DancingLinksError(String);

impl DancingLinksError {
    pub(crate) fn new<S: Into<String>>(message: S) -> DancingLinksError {
        DancingLinksError(message.into())
    }
}

impl From<String> for DancingLinksError {
    fn from(value: String) -> Self {
        DancingLinksError(value)
    }
}

impl From<&str> for DancingLinksError {
    fn from(value: &str) -> Self {
        DancingLinksError(value.into())
    }
}

#[derive(Debug)]
pub struct DancingLinksIterator<ItemType, OptionType> {
    state: IteratorState<ItemType>,
    iterator_item_type: PhantomData<OptionType>,
}

#[derive(Debug)]
enum IteratorState<ItemType> {
    DONE,
    NEW(InitialState<ItemType>),
    READY {
        x: Vec<u16>,
        solution_state: Box<SolutionState<ItemType>>,
    },
}

impl<ItemType, OptionType> DancingLinksIterator<ItemType, OptionType>
where
    ItemType: Ord,
    OptionType: ProblemOption<ItemType>,
{
    pub fn new(options: Vec<OptionType>) -> Result<Self, DancingLinksError> {
        if options.is_empty() {
            return Ok(DancingLinksIterator {
                state: IteratorState::DONE,
                iterator_item_type: PhantomData,
            });
        }
        Ok(DancingLinksIterator {
            state: IteratorState::NEW(InitialState::new(options)?),
            iterator_item_type: PhantomData,
        })
    }
}

impl<ItemType, OptionType> Iterator for DancingLinksIterator<ItemType, OptionType>
where
    OptionType: ProblemOption<ItemType>,
{
    type Item = Vec<OptionType>;
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
                        == solution_state.option_nodes
                            [solution_state.option_nodes[xl as usize].top as usize]
                            .ulink
                    {
                        // The last x was pointing at the final option for this item,
                        // so we have to backtrack.
                        solution_state.unapply_move(xl);
                        solution_state.uncover(solution_state.option_nodes[xl as usize].top as u16);
                        x.pop();
                        if x.is_empty() {
                            // Terminate.
                            self.state = IteratorState::DONE;
                            return None;
                        }
                        xl = x[x.len() - 1];
                    }

                    solution_state.unapply_move(xl);
                    // We know there is a next value because of the backtracking
                    // loop above.
                    xl = solution_state.option_nodes[xl as usize].dlink;
                    solution_state.apply_move(xl);
                    let x_idx = x.len() - 1;
                    x[x_idx] = xl;

                    // Done?
                    if solution_state.item_nodes[0].rlink == 0 {
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

impl<ItemType, OptionType> std::iter::FusedIterator for DancingLinksIterator<ItemType, OptionType> where
    OptionType: ProblemOption<ItemType>
{
}

// InitialState is the initial info needed to create SolutionState.
#[derive(Debug)]
struct InitialState<ItemType> {
    items: Vec<ItemType>,   // The sorted items, with the primary items first.
    num_primary_items: u16, // The number of primary items.
    options: Vec<InitialStateOption>, // The options.
    num_nodes: u16,
}

#[derive(Debug)]
struct InitialStateOption {
    item_indices: Vec<u16>,
}

impl<ItemType> InitialState<ItemType>
where
    ItemType: Ord,
{
    pub fn new<OptionType: ProblemOption<ItemType>>(
        options: Vec<OptionType>,
    ) -> Result<Self, DancingLinksError> {
        let mut primary_items = BTreeSet::new();
        let mut secondary_items = BTreeSet::new();
        for option in &options {
            primary_items.extend(option.primary_items());
            secondary_items.extend(option.secondary_items());
        }

        if primary_items.is_empty() {
            return Err(DancingLinksError::new("No primary items in options"));
        }

        // Make sure there are no overlaps.
        {
            let mut primary_and_secondary = primary_items.intersection(&secondary_items).peekable();
            if primary_and_secondary.peek().is_some() {
                return Err(DancingLinksError::new(
                    "Can't have items that are both primary and secondary",
                ));
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

        // Convert each option into indices into the ordered list of items.
        let item_to_index: BTreeMap<&ItemType, u16> = primary_items
            .iter()
            .chain(secondary_items.iter())
            .enumerate()
            .map(|(idx, item)| (item, idx as u16))
            .collect();
        let num_primary_items = primary_items.len();
        let initial_state_options: Vec<InitialStateOption> = options
            .into_iter()
            .map(|option| InitialStateOption {
                item_indices: option
                    .primary_items()
                    .chain(option.secondary_items())
                    .map(|item| item_to_index[&item])
                    .collect(),
            })
            .collect();

        let num_nodes: usize = n_items
            + initial_state_options.len()
            + initial_state_options
                .iter()
                .map(|option| option.item_indices.len())
                .sum::<usize>()
            + 1;
        if num_nodes > u16::MAX as usize {
            // The node points will run out of range.
            return Err(DancingLinksError::new(format!(
                "Too many total nodes required: {} > {}",
                num_nodes,
                u16::MAX
            )));
        }

        Ok(InitialState {
            items: primary_items
                .into_iter()
                .chain(secondary_items.into_iter())
                .collect(),
            num_primary_items: num_primary_items as u16,
            options: initial_state_options,
            num_nodes: num_nodes as u16,
        })
    }
}

impl<ItemType> Default for InitialState<ItemType> {
    fn default() -> Self {
        InitialState {
            items: vec![],
            num_primary_items: 0,
            options: vec![],
            num_nodes: 0,
        }
    }
}

// SolutionState is the internal representation of the dancing links state.
#[derive(Clone, Debug, Eq, PartialEq)]
struct SolutionState<ItemType> {
    items: Vec<ItemType>,
    num_primary_items: u16,
    item_nodes: Vec<ItemNode>,
    option_nodes: Vec<OptionNode>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ItemNode {
    // The number of active options that involve this item.
    len: u16,
    // The link to the preceding item.
    llink: u16,
    // The link to the next item.
    rlink: u16,
    // The index of the item in SolutionState::items, or MAX if not used.
    item_index: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct OptionNode {
    // Which item node this node is associated with.  Non-positive values are
    // spacer nodes.
    top: i16,
    // Link to the link above this one.
    ulink: u16,
    // Link to the node below this one.
    dlink: u16,
}

impl<ItemType> SolutionState<ItemType> {
    fn initiate(initial_state: InitialState<ItemType>) -> Self {
        // Build the nodes for each item.
        // First the header node for the primary items.
        let mut item_nodes = std::iter::once(ItemNode {
            len: 0,
            llink: initial_state.num_primary_items,
            rlink: 1,
            item_index: u16::MAX, // Not used.
        })
        // Then all the items.
        .chain(
            (1..=initial_state.items.len())
                .into_iter()
                .map(|idx| ItemNode {
                    len: 0,
                    llink: idx as u16 - 1,
                    rlink: idx as u16 + 1,
                    item_index: idx as u16 - 1,
                }),
        )
        // Then the header for the secondary items.
        .chain(std::iter::once(ItemNode {
            len: 0,
            llink: initial_state.items.len() as u16,
            rlink: (initial_state.num_primary_items + 1),
            item_index: u16::MAX, // Not used
        }))
        .collect::<Vec<_>>();
        // Link the primary items and secondary items into their own
        // circular lists, so that we never try to select a primary.
        item_nodes[initial_state.num_primary_items as usize].rlink = 0;
        item_nodes[(initial_state.num_primary_items + 1) as usize].llink =
            (initial_state.items.len() + 1) as u16;

        // Now build the option nodes.
        let mut option_nodes = Vec::with_capacity(initial_state.num_nodes as usize);
        // Create the item header nodes.
        option_nodes.push(OptionNode::default()); // Unused 'spacer'
        for idx in 1..=(initial_state.items.len() as u16) {
            option_nodes.push(OptionNode {
                top: idx as i16,
                ulink: idx,
                dlink: idx,
            });
        }

        // The first (real) spacer.
        option_nodes.push(OptionNode::default());
        let mut last_spacer_idx = option_nodes.len() - 1;

        // Now the real options.
        let mut spacer_top = 0;
        for input_option in initial_state.options.into_iter() {
            for item_index in input_option.item_indices.into_iter() {
                // The header option node this points to.  +1 for the spacer.
                let header_node_index = item_index as usize + 1;
                // The index the new node will have.
                let new_node_index = option_nodes.len() as u16;

                // Update the previous last occurence of this item to downpoint
                // to the new one we are about to add.
                let prev_tail_idx = option_nodes[header_node_index].ulink as usize;
                option_nodes[prev_tail_idx].dlink = new_node_index;

                // Push the new one.
                option_nodes.push(OptionNode {
                    top: header_node_index as i16,
                    ulink: prev_tail_idx as u16,
                    dlink: header_node_index as u16,
                });

                // Update the header to upoint to this new node.
                option_nodes[header_node_index].ulink = new_node_index;

                // Update the len in the item nodes.
                item_nodes[header_node_index].len += 1;
            }

            // Update the spacer before this option to point at the last node
            // from this option.
            option_nodes[last_spacer_idx].dlink = (option_nodes.len() - 1) as u16;
            // Add a new spacer.
            spacer_top -= 1;
            option_nodes.push(OptionNode {
                top: spacer_top,
                ulink: last_spacer_idx as u16 + 1,
                dlink: 0,
            });
            last_spacer_idx = option_nodes.len() - 1;
        }

        SolutionState {
            items: initial_state.items,
            num_primary_items: initial_state.num_primary_items,
            item_nodes,
            option_nodes,
        }
    }
    fn cover(&mut self, item: u16) {
        let mut p = self.option_nodes[item as usize].dlink;
        while p != item {
            self.hide(p);
            p = self.option_nodes[p as usize].dlink;
        }
        let l = self.item_nodes[item as usize].llink;
        let r = self.item_nodes[item as usize].rlink;
        self.item_nodes[l as usize].rlink = r;
        self.item_nodes[r as usize].llink = l;
    }

    fn hide(&mut self, p: u16) {
        let mut q = p + 1;
        while q != p {
            let x = self.option_nodes[q as usize].top;
            let u = self.option_nodes[q as usize].ulink;
            if x <= 0 {
                // Spacer node.
                q = u;
            } else {
                let d = self.option_nodes[q as usize].dlink;
                self.option_nodes[u as usize].dlink = d;
                self.option_nodes[d as usize].ulink = u;
                self.item_nodes[x as usize].len -= 1;
                q += 1;
            }
        }
    }

    fn uncover(&mut self, item: u16) {
        let l = self.item_nodes[item as usize].llink;
        let r = self.item_nodes[item as usize].rlink;
        self.item_nodes[l as usize].rlink = item;
        self.item_nodes[r as usize].llink = item;

        let mut p = self.option_nodes[item as usize].ulink;
        while p != item {
            self.unhide(p);
            p = self.option_nodes[p as usize].ulink;
        }
    }

    fn unhide(&mut self, p: u16) {
        let mut q = p - 1;
        while q != p {
            let x = self.option_nodes[q as usize].top;
            let d = self.option_nodes[q as usize].dlink;
            if x <= 0 {
                // Spacer node.
                q = d;
            } else {
                let u = self.option_nodes[q as usize].ulink;
                self.option_nodes[u as usize].dlink = q;
                self.option_nodes[d as usize].ulink = q;
                self.item_nodes[x as usize].len += 1;
                q -= 1;
            }
        }
    }

    fn apply_move(&mut self, xl: u16) {
        let mut p = xl + 1;
        while p != xl {
            let j = self.option_nodes[p as usize].top;
            if j <= 0 {
                // Spacer.
                p = self.option_nodes[p as usize].ulink;
            } else {
                self.cover(j as u16);
                p += 1;
            }
        }
    }

    fn unapply_move(&mut self, xl: u16) {
        if xl <= self.num_primary_items {
            return;
        }
        let mut p = xl - 1;
        while p != xl {
            let j = self.option_nodes[p as usize].top;
            if j <= 0 {
                // Spacer.
                p = self.option_nodes[p as usize].dlink;
            } else {
                self.uncover(j as u16);
                p -= 1;
            }
        }
    }

    // Choose the next item using the MRV heuristic.  Returns None if there is
    // no move.
    fn choose_next_item(&self) -> Option<NonZeroU16> {
        let mut current_item_idx = self.item_nodes[0].rlink;
        if current_item_idx == 0 {
            return None;
        }

        let mut best_len = self.item_nodes[current_item_idx as usize].len;
        if best_len == 0 {
            return None;
        }
        let mut best_item = current_item_idx;

        current_item_idx = self.item_nodes[current_item_idx as usize].rlink;
        while current_item_idx != 0 && current_item_idx <= self.num_primary_items {
            let current_len = self.item_nodes[current_item_idx as usize].len;
            if current_len == 0 {
                // Item has no choices.
                best_item = 0;
                break;
            }
            if current_len < best_len {
                best_len = current_len;
                best_item = current_item_idx;
            }

            current_item_idx = self.item_nodes[current_item_idx as usize].rlink;
        }
        NonZeroU16::new(best_item)
    }

    fn to_option<PO: ProblemOption<ItemType>>(&self, mut x: usize) -> PO {
        let mut builder = PO::builder();
        while self.option_nodes[x - 1].top > 0 {
            x -= 1;
        }
        // x now points at the first non-spacer node of the solution.
        let n_primary = self.num_primary_items as i16;
        while self.option_nodes[x].top > 0 && self.option_nodes[x].top <= n_primary {
            builder.add_primary(&self.items[self.option_nodes[x].top as usize - 1]);
            x += 1;
        }
        while self.option_nodes[x].top > 0 {
            builder.add_secondary(&self.items[self.option_nodes[x].top as usize - 1]);
            x += 1;
        }
        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::backtracking::dancing_links::{
        DancingLinksError, DancingLinksIterator, IteratorState, ProblemOption,
        ProblemOptionBuilder, SolutionState,
    };

    // A simple string option type for tests.
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
    struct StringOption {
        primary_items: Vec<String>,
        secondary_items: Vec<String>,
    }

    impl StringOption {
        fn new(primary: &[&str], secondary: &[&str]) -> Self {
            Self {
                primary_items: primary.iter().map(|s| s.to_string()).collect(),
                secondary_items: secondary.iter().map(|s| s.to_string()).collect(),
            }
        }
    }

    impl ProblemOption<String> for StringOption {
        type IteratorType = std::vec::IntoIter<String>;
        type BuilderType = Self;

        fn primary_items(&self) -> Self::IteratorType {
            self.primary_items.clone().into_iter()
        }

        fn secondary_items(&self) -> Self::IteratorType {
            self.secondary_items.clone().into_iter()
        }

        fn builder() -> Self::BuilderType {
            StringOption {
                primary_items: vec![],
                secondary_items: vec![],
            }
        }
    }

    impl ProblemOptionBuilder<String> for StringOption {
        type ProblemOptionType = Self;

        fn add_primary(&mut self, item: &String) -> &mut Self {
            self.primary_items.push(item.clone());
            self
        }

        fn add_secondary(&mut self, item: &String) -> &mut Self {
            self.secondary_items.push(item.clone());
            self
        }

        fn build(self) -> Self::ProblemOptionType {
            self
        }
    }

    fn create_solution_state(
        options: Vec<StringOption>,
    ) -> Result<SolutionState<String>, DancingLinksError> {
        let iterator = DancingLinksIterator::new(options)?;
        match iterator.state {
            IteratorState::DONE => Err(DancingLinksError::new("Iterator created in DONE state")),
            IteratorState::READY { .. } => {
                Err(DancingLinksError::new("Iterator created in READY state"))
            }
            IteratorState::NEW(initial_state) => Ok(SolutionState::initiate(initial_state)),
        }
    }

    mod initialize_iterator {
        use super::StringOption;
        use crate::backtracking::dancing_links::{DancingLinksIterator, IteratorState};
        use claim::assert_ok;

        #[test]
        fn no_options_is_done_iterator() {
            let it = assert_ok!(DancingLinksIterator::<String, StringOption>::new(vec![]));
            assert!(matches!(it.state, IteratorState::DONE));
        }

        #[test]
        fn single_option_is_in_new_state() {
            let it = assert_ok!(DancingLinksIterator::<String, StringOption>::new(vec![
                StringOption::new(
                    /*primary_items=*/ &["a"],
                    /*secondary_items=*/ &["b"]
                )
            ]));
            assert!(matches!(it.state, IteratorState::NEW { .. }));
        }

        #[test]
        fn item_that_is_primary_and_secondary_is_error() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["c"],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["c"],
                /*secondary_items=*/ &[],
            );
            let res = DancingLinksIterator::new(vec![option1, option2]);

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("both primary and secondary"));
        }
    }

    mod initialize_solution_state {
        use super::{create_solution_state, StringOption};
        use crate::backtracking::dancing_links::{ItemNode, OptionNode, SolutionState};
        use claim::assert_ok_eq;

        #[test]
        fn single_primary_item_initializes() {
            let option = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );

            assert_ok_eq!(
                create_solution_state(vec![option]),
                SolutionState {
                    items: vec!["a".to_string()],
                    num_primary_items: 1,
                    item_nodes: vec![
                        ItemNode {
                            len: 0,
                            llink: 1,
                            rlink: 1,
                            item_index: u16::MAX,
                        },
                        ItemNode {
                            len: 1,
                            llink: 0,
                            rlink: 0,
                            item_index: 0,
                        },
                        ItemNode {
                            len: 0,
                            llink: 2,
                            rlink: 2,
                            item_index: u16::MAX,
                        }
                    ],
                    option_nodes: vec![
                        // Node 0: empty
                        OptionNode::default(),
                        // Node 1: header node for only item.
                        OptionNode {
                            top: 1,
                            ulink: 3,
                            dlink: 3
                        },
                        // Node 2: Spacer node.
                        OptionNode {
                            top: 0,
                            ulink: 0,
                            dlink: 3
                        },
                        // Node 3: Only item node.
                        OptionNode {
                            top: 1,
                            ulink: 1,
                            dlink: 1
                        },
                        // Node 4: Spacer node.
                        OptionNode {
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
            let option1 = StringOption::new(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            );

            assert_ok_eq!(
                create_solution_state(vec![option1, option2]),
                SolutionState {
                    items: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    num_primary_items: 2,
                    item_nodes: vec![
                        ItemNode {
                            len: 0,
                            llink: 2,
                            rlink: 1,
                            item_index: u16::MAX
                        },
                        // Primary
                        ItemNode {
                            len: 1,
                            llink: 0,
                            rlink: 2,
                            item_index: 0
                        },
                        ItemNode {
                            len: 2,
                            llink: 1,
                            rlink: 0,
                            item_index: 1
                        },
                        // Secondary
                        ItemNode {
                            len: 1,
                            llink: 4,
                            rlink: 4,
                            item_index: 2
                        },
                        ItemNode {
                            len: 0,
                            llink: 3,
                            rlink: 3,
                            item_index: u16::MAX
                        }
                    ],
                    option_nodes: vec![
                        // Node 0: empty
                        OptionNode::default(),
                        // Node 1: header node for 'a'
                        OptionNode {
                            top: 1,
                            ulink: 5,
                            dlink: 5
                        },
                        // Node 2: header node for 'b'
                        OptionNode {
                            top: 2,
                            ulink: 8, // ProblemOption b, c
                            dlink: 6  // ProblemOption a, b
                        },
                        // Node 3: header node for 'c'
                        OptionNode {
                            top: 3,
                            ulink: 9, // ProblemOption b, c
                            dlink: 9  // ProblemOption b, c
                        },
                        // Node 4: Spacer node.
                        OptionNode {
                            top: 0,
                            ulink: 0, // unused.
                            dlink: 6  // Last node in option a, b
                        },
                        // Node 5: Option a, b item a
                        OptionNode {
                            top: 1,
                            ulink: 1,
                            dlink: 1
                        },
                        // Node 6: Option a, b item b
                        OptionNode {
                            top: 2,
                            ulink: 2,
                            dlink: 8
                        },
                        // Node 7: Spacer between a, b and b, c
                        OptionNode {
                            top: -1,
                            ulink: 5, // First node in option a, b
                            dlink: 9  // Last node in option b, c
                        },
                        // Node 8: Option b, c item b
                        OptionNode {
                            top: 2,
                            ulink: 6, // ProblemOption a, b
                            dlink: 2, // Header for b
                        },
                        // Node 9: Option b, c item c
                        OptionNode {
                            top: 3,
                            ulink: 3, // Header for c
                            dlink: 3, // Header for c
                        },
                        // Node 10: final spacer
                        OptionNode {
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
            let option1 = StringOption::new(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["g"],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b", "c"],
                /*secondary_items=*/ &["f"],
            );
            let option4 = StringOption::new(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["f"],
            );
            let option5 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["g"],
            );
            let option6 = StringOption::new(
                /*primary_items=*/ &["d", "e"],
                /*secondary_items=*/ &["g"],
            );

            assert_ok_eq!(
                create_solution_state(vec![option1, option2, option3, option4, option5, option6]),
                SolutionState {
                    items: ["a", "b", "c", "d", "e", "f", "g"]
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect(),
                    num_primary_items: 5,
                    item_nodes: vec![
                        // Primary items.
                        ItemNode {
                            len: 0,
                            llink: 5,
                            rlink: 1,
                            item_index: u16::MAX,
                        },
                        ItemNode {
                            len: 2,
                            llink: 0,
                            rlink: 2,
                            item_index: 0
                        },
                        ItemNode {
                            len: 2,
                            llink: 1,
                            rlink: 3,
                            item_index: 1
                        },
                        ItemNode {
                            len: 2,
                            llink: 2,
                            rlink: 4,
                            item_index: 2
                        },
                        ItemNode {
                            len: 3,
                            llink: 3,
                            rlink: 5,
                            item_index: 3
                        },
                        ItemNode {
                            len: 2,
                            llink: 4,
                            rlink: 0,
                            item_index: 4
                        },
                        // Secondary items
                        ItemNode {
                            len: 2,
                            llink: 8,
                            rlink: 7,
                            item_index: 5
                        },
                        ItemNode {
                            len: 3,
                            llink: 6,
                            rlink: 8,
                            item_index: 6
                        },
                        ItemNode {
                            len: 0,
                            llink: 7,
                            rlink: 6,
                            item_index: u16::MAX,
                        }
                    ],
                    option_nodes: vec![
                        // Node 0: empty
                        OptionNode::default(),
                        // Node 1: header node for 'a'
                        OptionNode {
                            top: 1,
                            ulink: 20, // Option a d
                            dlink: 12, // Option a d g
                        },
                        // Node 2: header node for 'b'
                        OptionNode {
                            top: 2,
                            ulink: 24, // Option b, g
                            dlink: 16  // Option b, c, f
                        },
                        // Node 3: header node for 'c'
                        OptionNode {
                            top: 3,
                            ulink: 17, // Option b, c, f
                            dlink: 9   // Option c, e
                        },
                        // Node 4: header node for 'd'
                        OptionNode {
                            top: 4,
                            ulink: 27, // Option d e g
                            dlink: 13, // Option a d g
                        },
                        // Node 5: header node for 'e'
                        OptionNode {
                            top: 5,
                            ulink: 28, // Option d e g
                            dlink: 10  // Option c e
                        },
                        // Node 6: header node for 'f'
                        OptionNode {
                            top: 6,
                            ulink: 22, // Option a d f
                            dlink: 18  // Option b c f
                        },
                        // Node 7: header node for 'g'
                        OptionNode {
                            top: 7,
                            ulink: 29, // Option d e g
                            dlink: 14  // Option a d g
                        },
                        // Node 8: Spacer node.
                        OptionNode {
                            top: 0,
                            ulink: 0,  // unused.
                            dlink: 10  // Last node in option c, e
                        },
                        // Nodes 9-10: Option c e
                        // Node 9: item c
                        OptionNode {
                            top: 3,
                            ulink: 3,  // Header
                            dlink: 17, // Option b c f
                        },
                        // Node 10: item e
                        OptionNode {
                            top: 5,
                            ulink: 5,  // Header
                            dlink: 28  // Option d e g
                        },
                        // Node 11: Spacer
                        OptionNode {
                            top: -1,
                            ulink: 9,
                            dlink: 14
                        },
                        // Nodes 12-14: Option a d g
                        // Node 12: item a
                        OptionNode {
                            top: 1,
                            ulink: 1,  // Header
                            dlink: 20  // Option a d f
                        },
                        // Node 13: item d
                        OptionNode {
                            top: 4,
                            ulink: 4,  // Header
                            dlink: 21  // Option a d f
                        },
                        // Node 14: item g
                        OptionNode {
                            top: 7,
                            ulink: 7,  // Header
                            dlink: 25, // Option b g
                        },
                        // Node 15: Spacer
                        OptionNode {
                            top: -2,
                            ulink: 12,
                            dlink: 18
                        },
                        // Nodes 16-18: Option b c f
                        // Node 16: item b
                        OptionNode {
                            top: 2,
                            ulink: 2,  // Header
                            dlink: 24  // Option b g
                        },
                        // Node 17: item c
                        OptionNode {
                            top: 3,
                            ulink: 9, // Option c e
                            dlink: 3  // Header
                        },
                        // Node 18: item f
                        OptionNode {
                            top: 6,
                            ulink: 6,  // Header
                            dlink: 22  // Option a d f
                        },
                        // Node 19: spacer
                        OptionNode {
                            top: -3,
                            ulink: 16,
                            dlink: 22
                        },
                        // Nodes 20-22: Option a d f
                        // Node 20: item a
                        OptionNode {
                            top: 1,
                            ulink: 12, // Option a d g
                            dlink: 1   // Header
                        },
                        // Node 21: item d
                        OptionNode {
                            top: 4,
                            ulink: 13, // Option a d g
                            dlink: 27  // Option d e g
                        },
                        // Node 22: item f
                        OptionNode {
                            top: 6,
                            ulink: 18, // Option b c f
                            dlink: 6   // Header
                        },
                        // Node 23: spacer
                        OptionNode {
                            top: -4,
                            ulink: 20,
                            dlink: 25
                        },
                        // Nodes 24-25: Option b g
                        // Node 24: item b
                        OptionNode {
                            top: 2,
                            ulink: 16, // Option b c f
                            dlink: 2   // Header
                        },
                        // Node 25: item g
                        OptionNode {
                            top: 7,
                            ulink: 14, // Option a d g
                            dlink: 29  // Option d e g
                        },
                        // Node 26: spacer
                        OptionNode {
                            top: -5,
                            ulink: 24,
                            dlink: 29
                        },
                        // Node 27-29: Option d e g
                        // Node 27: item d
                        OptionNode {
                            top: 4,
                            ulink: 21, // Option a d f
                            dlink: 4   // Header
                        },
                        // Node 28: item e
                        OptionNode {
                            top: 5,
                            ulink: 10, // Option c e
                            dlink: 5   // Header
                        },
                        // Node 29: item g
                        OptionNode {
                            top: 7,
                            ulink: 25, // Option b g
                            dlink: 7   // Header
                        },
                        // Node 30: final spacer
                        OptionNode {
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
        use super::{create_solution_state, StringOption};
        use crate::backtracking::dancing_links::SolutionState;
        use claim::{assert_ok, assert_some, assert_some_eq};
        use std::collections::BTreeSet;

        fn item_name(solution_state: &SolutionState<String>, idx: u16) -> Option<&String> {
            let idx_u = idx as usize;
            if idx_u >= solution_state.item_nodes.len() {
                None
            } else {
                Some(&solution_state.items[solution_state.item_nodes[idx_u].item_index as usize])
            }
        }

        #[test]
        fn single_primary_item_chooses_only_option() {
            let option = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );
            let solution_state = assert_ok!(create_solution_state(vec![option]));

            assert_some_eq!(
                solution_state
                    .choose_next_item()
                    .and_then(|v| item_name(&solution_state, v.get())),
                "a"
            );
        }

        #[test]
        fn chooses_item_with_fewest_options() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a", "b", "c"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a", "c"],
                /*secondary_items=*/ &[],
            );

            let solution_state = assert_ok!(create_solution_state(vec![option1, option2]));

            assert_eq!(solution_state.num_primary_items, 3);
            // Check the LEN vars are what we expect.
            assert_eq!(
                solution_state
                    .item_nodes
                    .iter()
                    .skip(1)
                    .take(3)
                    .map(|item| item.len)
                    .collect::<Vec<_>>(),
                vec![2, 1, 2]
            );
            assert_eq!(
                solution_state
                    .item_nodes
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
                    .and_then(|v| item_name(&solution_state, v.get())),
                "b"
            );
        }

        #[test]
        fn secondary_item_is_not_chosen_even_if_it_has_fewest_available_options() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &["b"],
            );
            let solution_state = assert_ok!(create_solution_state(vec![option1, option2]));

            assert_some_eq!(
                solution_state
                    .choose_next_item()
                    .and_then(|v| item_name(&solution_state, v.get())),
                "a"
            );
        }

        #[test]
        fn large_test_case_chooses_next_item() {
            // This is the example from TAOCP 7.2.2.1 (6), except that ag have
            // been made secondary.
            let option1 = StringOption::new(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["d"],
                /*secondary_items=*/ &["a", "g"],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b", "c", "f"],
                /*secondary_items=*/ &[],
            );
            let option4 = StringOption::new(
                /*primary_items=*/ &["d", "f"],
                /*secondary_items=*/ &["a"],
            );
            let option5 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["g"],
            );
            let option6 = StringOption::new(
                /*primary_items=*/ &["d", "e"],
                /*secondary_items=*/ &["g"],
            );

            let solution_state = assert_ok!(create_solution_state(vec![
                option1, option2, option3, option4, option5, option6
            ]));
            assert_eq!(solution_state.num_primary_items, 5);

            // b, c, e, f are all acceptable choices.  d is not because
            // it has 3 items, a and g are not because they are secondary.
            let choice = assert_some!(solution_state
                .choose_next_item()
                .and_then(|v| item_name(&solution_state, v.get())));
            assert!(["b", "c", "e", "f"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<BTreeSet<_>>()
                .contains(choice));
        }
    }

    mod cover {
        use super::{create_solution_state, StringOption};
        use claim::assert_ok;

        #[test]
        fn single_primary_item_cover() {
            let option = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );

            let mut state = assert_ok!(create_solution_state(vec![option]));

            // For a single item, cover only affects the item links.
            let mut expected_state = state.clone();
            expected_state.item_nodes[0].llink = 0;
            expected_state.item_nodes[0].rlink = 0;

            state.cover(1);
            assert_eq!(state, expected_state);
        }

        #[test]
        fn large_test_case_cover() {
            // This is the example from TAOCP 7.2.2.1 (6), updated following
            // exercise 11 from that section.
            let option1 = StringOption::new(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a", "d", "g"],
                /*secondary_items=*/ &[],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b", "c", "f"],
                /*secondary_items=*/ &[],
            );
            let option4 = StringOption::new(
                /*primary_items=*/ &["a", "d", "f"],
                /*secondary_items=*/ &[],
            );
            let option5 = StringOption::new(
                /*primary_items=*/ &["b", "g"],
                /*secondary_items=*/ &[],
            );
            let option6 = StringOption::new(
                /*primary_items=*/ &["d", "e", "g"],
                /*secondary_items=*/ &[],
            );

            let mut state = assert_ok!(create_solution_state(vec![
                option1, option2, option3, option4, option5, option6
            ]));

            // cover(1).
            let mut expected_state = state.clone();
            // ItemNode 1 ('a') will be unlinked.
            expected_state.item_nodes[0].rlink = 2;
            expected_state.item_nodes[2].llink = 0;
            // From hide(12)
            expected_state.option_nodes[4].dlink = 2;
            expected_state.option_nodes[21].ulink = 4;
            expected_state.item_nodes[4].len = 2;
            expected_state.option_nodes[7].dlink = 25;
            expected_state.option_nodes[25].ulink = 7;
            expected_state.item_nodes[7].len = 2;
            // From hide(20)
            expected_state.option_nodes[4].dlink = 27;
            expected_state.option_nodes[27].ulink = 4;
            expected_state.item_nodes[4].len = 1;
            expected_state.option_nodes[18].dlink = 6;
            expected_state.option_nodes[6].ulink = 18;
            expected_state.item_nodes[6].len = 1;

            state.cover(1);
            assert_eq!(state, expected_state);

            // cover(4).
            expected_state.item_nodes[5].llink = 3;
            expected_state.item_nodes[3].rlink = 5;
            expected_state.option_nodes[10].dlink = 5;
            expected_state.option_nodes[5].ulink = 10;
            expected_state.item_nodes[5].len = 1;
            expected_state.option_nodes[25].dlink = 7;
            expected_state.option_nodes[7].ulink = 25;
            expected_state.item_nodes[7].len = 1;

            state.cover(4);
            assert_eq!(state, expected_state);

            // cover(7)
            expected_state.item_nodes[6].rlink = 0;
            expected_state.item_nodes[0].llink = 6;
            expected_state.option_nodes[16].dlink = 2;
            expected_state.option_nodes[2].ulink = 16;
            expected_state.item_nodes[2].len = 1;

            state.cover(7);
            assert_eq!(state, expected_state);

            // cover(2)
            expected_state.item_nodes[3].llink = 0;
            expected_state.item_nodes[0].rlink = 3;
            expected_state.option_nodes[3].ulink = 9;
            expected_state.option_nodes[9].dlink = 3;
            expected_state.item_nodes[3].len = 1;
            expected_state.option_nodes[6].ulink = 6;
            expected_state.option_nodes[6].dlink = 6;
            expected_state.item_nodes[6].len = 0;

            state.cover(2);
            assert_eq!(state, expected_state);
        }

        #[test]
        fn single_primary_item_uncover() {
            // Check that uncover undoes what we tested in single_primary_item_cover
            let option = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );
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
            let option1 = StringOption::new(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a", "d", "g"],
                /*secondary_items=*/ &[],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b", "c", "f"],
                /*secondary_items=*/ &[],
            );
            let option4 = StringOption::new(
                /*primary_items=*/ &["a", "d", "f"],
                /*secondary_items=*/ &[],
            );
            let option5 = StringOption::new(
                /*primary_items=*/ &["b", "g"],
                /*secondary_items=*/ &[],
            );
            let option6 = StringOption::new(
                /*primary_items=*/ &["d", "e", "g"],
                /*secondary_items=*/ &[],
            );

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
        use super::{create_solution_state, StringOption};
        use claim::assert_ok;

        #[test]
        fn small_test_case_get_options() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            );

            let state = assert_ok!(create_solution_state(vec![
                option1.clone(),
                option2.clone()
            ]));

            assert_eq!(state.to_option::<StringOption>(5), option1);
            assert_eq!(state.to_option::<StringOption>(6), option1);
            assert_eq!(state.to_option::<StringOption>(8), option2);
        }

        #[test]
        fn large_test_case_get_options() {
            // This is the example from TAOCP 7.2.2.1 (6), except that fg have
            // been made secondary.
            let option1 = StringOption::new(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["g"],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b", "c"],
                /*secondary_items=*/ &["f"],
            );
            let option4 = StringOption::new(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["f"],
            );
            let option5 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["g"],
            );
            let option6 = StringOption::new(
                /*primary_items=*/ &["d", "e"],
                /*secondary_items=*/ &["g"],
            );

            let state = assert_ok!(create_solution_state(vec![
                option1.clone(),
                option2,
                option3.clone(),
                option4,
                option5,
                option6
            ]));

            assert_eq!(state.to_option::<StringOption>(9), option1);
            assert_eq!(state.to_option::<StringOption>(10), option1);
            assert_eq!(state.to_option::<StringOption>(16), option3);
        }
    }

    mod iterates_over_solutions {
        use super::StringOption;
        use crate::backtracking::dancing_links::DancingLinksIterator;
        use claim::{assert_none, assert_ok, assert_some_eq};
        use std::collections::HashSet;

        #[test]
        fn single_item_one_option_solution() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );
            let mut iterator = assert_ok!(DancingLinksIterator::new(vec![option1.clone()]));

            assert_some_eq!(iterator.next(), vec![option1]);
            assert_none!(iterator.next());
        }

        #[test]
        fn very_small_test_case_iterator() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            );

            let mut iterator =
                assert_ok!(DancingLinksIterator::new(vec![option1.clone(), option2]));

            assert_some_eq!(iterator.next(), vec![option1]);
            assert_none!(iterator.next());
        }

        #[test]
        fn small_test_case_iterator() {
            let option1 = StringOption::new(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["c"],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a"],
                /*secondary_items=*/ &[],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["c"],
            );

            let mut iterator = assert_ok!(DancingLinksIterator::new(vec![
                option1.clone(),
                option2.clone(),
                option3.clone()
            ]));

            assert_some_eq!(
                iterator
                    .next()
                    .map(move |v| v.into_iter().collect::<HashSet<_>>()),
                HashSet::from([option1])
            );
            assert_some_eq!(
                iterator
                    .next()
                    .map(move |v| v.into_iter().collect::<HashSet<_>>()),
                HashSet::from([option2, option3])
            );
            assert_none!(iterator.next());
        }

        #[test]
        fn large_test_case_iterator() {
            // This is the example from TAOCP 7.2.2.1 (6), except that fg have
            // been made secondary.
            let option1 = StringOption::new(
                /*primary_items=*/ &["c", "e"],
                /*secondary_items=*/ &[],
            );
            let option2 = StringOption::new(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["g"],
            );
            let option3 = StringOption::new(
                /*primary_items=*/ &["b", "c"],
                /*secondary_items=*/ &["f"],
            );
            let option4 = StringOption::new(
                /*primary_items=*/ &["a", "d"],
                /*secondary_items=*/ &["f"],
            );
            let option5 = StringOption::new(
                /*primary_items=*/ &["b"],
                /*secondary_items=*/ &["g"],
            );
            let option6 = StringOption::new(
                /*primary_items=*/ &["d", "e"],
                /*secondary_items=*/ &["g"],
            );

            let mut iterator = assert_ok!(DancingLinksIterator::new(vec![
                option1.clone(),
                option2,
                option3,
                option4.clone(),
                option5.clone(),
                option6
            ]));

            assert_some_eq!(
                iterator
                    .next()
                    .map(move |v| v.into_iter().collect::<HashSet<_>>()),
                HashSet::from([option1, option4, option5])
            );
            assert_none!(iterator.next());
        }
    }
}
