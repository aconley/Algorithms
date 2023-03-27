// An implementation of Knuth's Algorithm X: Dancing links from
// TAOCP Volume 4B 7.2.2.1.

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Option {
    primary_items: Vec<String>,
    secondary_items: Vec<String>,
}

#[derive(Debug)]
pub struct DancingLinksError(String);

impl DancingLinksError {
    fn new(message: &str) -> DancingLinksError {
        DancingLinksError(message.to_string())
    }
}

impl Option {
    pub fn new(
        primary_items: Vec<String>,
        secondary_items: Vec<String>,
    ) -> Result<Option, DancingLinksError> {
        // Reject duplicates, or items that are both primary and secondary.
        if primary_items.is_empty() {
            return Err(DancingLinksError::new(
                "Option must contain at least one primary item",
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
        Ok(Option {
            primary_items,
            secondary_items,
        })
    }

    pub fn new_from_str(
        primary_items: &[&str],
        secondary_items: &[&str],
    ) -> Result<Option, DancingLinksError> {
        Option::new(
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
}

struct Item {
    // The link to the preceding item.
    llink: u16,
    // The link to the next item.
    rlink: u16,
    // The name of the item.
    name: String,
}

struct Node {
    // Which item this node represents.  Negative numbers are spacer nodes,
    // and in header nodes this represents the number of active items.
    top: i16,
    // Link to the link above this one.
    ulink: u16,
    // Link to the node below this one.
    dlink: u16,
}

struct SolutionState {
    num_items: u16,
    last_spacer_address: u16,
    items: Vec<Item>,
    nodes: Vec<Node>,
}

impl SolutionState {
    fn initiate(options: &[Option]) -> SolutionState {
        assert!(!options.is_empty(), "No options");
        todo!("Unimplemented")
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
        let mut p = self.items[0].rlink;
        debug_assert!(p != 0);

        let mut current = self.nodes[p as usize].top;
        if current == 0 {
            return p;
        }
        let mut best_value = current;
        let mut best_item = p;

        p = self.items[p as usize].rlink;
        while p != 0 {
            current = self.nodes[p as usize].top;
            if current == 0 {
                return p;
            }
            if current < best_value {
                best_value = current;
                best_item = p;
                p = self.items[p as usize].rlink;
            }
        }
        return best_item;
    }
}

#[cfg(test)]
mod tests {
    mod options {
        use crate::backtracking::dancing_links::Option;

        #[test]
        fn option_has_expected_items() {
            let res = Option::new_from_str(
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
            let res = Option::new_from_str(
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
            let res = Option::new_from_str(
                /*primary_items=*/ &[],
                /*secondary_items=*/ &["a"],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("at least one primary item"));
        }

        #[test]
        fn duplicate_primary_is_error() {
            let res = Option::new_from_str(
                /*primary_items=*/ &["a", "b", "c", "b"],
                /*secondary_items=*/ &[],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("primary_items"));
        }

        #[test]
        fn duplicate_secondary_is_error() {
            let res = Option::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["d", "e", "e"],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("secondary_items"));
        }

        #[test]
        fn item_that_is_both_primary_and_secondary_is_error() {
            let res = Option::new_from_str(
                /*primary_items=*/ &["a", "b"],
                /*secondary_items=*/ &["c", "a", "e"],
            );

            assert!(res.is_err());
            assert!(res.unwrap_err().0.contains("overlap"));
        }
    }
}
