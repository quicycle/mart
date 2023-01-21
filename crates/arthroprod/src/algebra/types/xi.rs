use std::{cmp, collections::HashMap, fmt, ops};

use crate::algebra::{types::alpha::ALLOWED_ALPHA_STRINGS, Form};

#[derive(Hash, Eq, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Xi {
    value: Option<String>, // None for non-leaf nodes
    partials: Vec<Form>,
    child_num: Vec<Xi>,
    child_den: Vec<Xi>,
}

impl Xi {
    /// Construct a new symbolic Xi value
    pub fn new(value: &str) -> Xi {
        Xi {
            value: Some(value.to_string()),
            partials: Vec::new(),
            child_num: Vec::new(),
            child_den: Vec::new(),
        }
    }

    pub fn empty() -> Xi {
        Xi {
            value: None,
            partials: Vec::new(),
            child_num: Vec::new(),
            child_den: Vec::new(),
        }
    }

    /// The multiplicative inverse of this Xi value
    /// Swaps child_num and child_den
    pub fn inverse(&self) -> Xi {
        Xi {
            value: self.value.clone(),
            partials: self.partials.clone(),
            child_num: self.child_den.clone(),
            child_den: self.child_num.clone(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Xi::empty()
    }

    /// Add a single partial derivative to this Xi
    pub fn add_partial(&mut self, wrt: &Form) {
        self.partials.push(*wrt);
        self.partials.sort();
    }

    /// Replace the current set of partial derivatives
    pub fn set_partials(&mut self, partials: Vec<Form>) {
        self.partials = partials;
        self.partials.sort();
    }

    /// Construct a new Xi by forming the product of existing Xis
    /// NOTE: we pull up children from empty_parents so that we don't stack empty
    /// nodes on top of one another
    pub fn merge(xis: &[Xi]) -> Xi {
        fn is_empty_parent(x: &Xi) -> bool {
            x.value.is_none() && x.partials.is_empty()
        }

        let mut child_num = vec![];
        let mut child_den = vec![];

        for x in xis.iter() {
            if is_empty_parent(x) {
                child_num.extend(x.child_num.clone());
                child_den.extend(x.child_den.clone());
            } else {
                child_num.push(x.clone());
            }
        }
        child_num.sort();
        child_den.sort();

        Xi {
            value: None,
            partials: Vec::new(),
            child_num,
            child_den,
        }
    }

    /// Represent this Xi as a dotted string of terms
    pub fn dotted_string(&self) -> String {
        let partials = partial_str(&self.partials);

        let with_partials = |s: String| -> String {
            match partials.len() {
                0 => s,
                _ => format!("{}({})", partials, s),
            }
        };

        let power_notation = |xis: &Vec<Xi>| -> String {
            let exp_str = |(xi, count): (Xi, usize)| -> String {
                if count == 1 {
                    xi.dotted_string()
                } else {
                    format!("{}^{}", xi.dotted_string(), count)
                }
            };

            let mut groups: HashMap<Xi, Vec<Xi>> = HashMap::new();
            xis.iter().cloned().for_each(|c| {
                groups.entry(c.clone()).or_default().push(c);
            });

            let mut powers: Vec<(Xi, usize)> = groups.drain().map(|(k, v)| (k, v.len())).collect();
            powers.sort_by(|a, b| a.0.cmp(&b.0));
            powers
                .iter()
                .map(|p| exp_str(p.clone()))
                .collect::<Vec<String>>()
                .join(".")
        };

        match self.value.clone() {
            Some(val) => format!("{}ξ{}", partials, val),
            None => match (self.child_num.len(), self.child_den.len()) {
                (0, 0) => panic!("Empty Xi"),
                (_, 0) => with_partials(power_notation(&self.child_num)),
                (0, _) => with_partials(format!("1/{}", power_notation(&self.child_num))),
                (_, _) => with_partials(format!(
                    "{}/{}",
                    power_notation(&self.child_num),
                    power_notation(&self.child_den)
                )),
            },
        }
    }
}

impl fmt::Display for Xi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dotted_string())
    }
}

impl ops::Mul for Xi {
    type Output = Self;

    fn mul(self, rhs: Xi) -> Self::Output {
        Xi::merge(&[self, rhs])
    }
}

impl cmp::Ord for Xi {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        fn cmp_ix(left: &Option<String>, right: &Option<String>) -> cmp::Ordering {
            if let (Some(l), Some(r)) = (left, right) {
                let opt_i1 = ALLOWED_ALPHA_STRINGS.iter().position(|f| f == l);
                let opt_i2 = ALLOWED_ALPHA_STRINGS.iter().position(|f| f == r);

                // Compare indices if both are present, otherwise place indexed
                // first and non-indexed in alphanumeric ordering after
                match (opt_i1, opt_i2) {
                    (Some(i1), Some(i2)) => i1.cmp(&i2),
                    (Some(_), None) => cmp::Ordering::Less,
                    (None, Some(_)) => cmp::Ordering::Greater,
                    (None, None) => l.cmp(r),
                }
            } else {
                cmp::Ordering::Equal
            }
        }
        cmp_ix(&self.value, &other.value)
            .then(self.child_num.cmp(&other.child_num))
            .then(self.child_den.cmp(&other.child_den))
            .then(self.partials.cmp(&other.partials))
    }
}

impl cmp::PartialOrd for Xi {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Concatenate the forms represeneting the partial derivatives applied to this Xi
pub(super) fn partial_str(partials: &[Form]) -> String {
    partials
        .iter()
        .fold(String::new(), |acc, p| acc + &format!("∂{}", p))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _xi(s: &str) -> Xi {
        Xi::new(s)
    }

    fn xi(value: Option<&str>, partials: Vec<Form>, child_num: Vec<Xi>, child_den: Vec<Xi>) -> Xi {
        Xi {
            value: value.map(|s| s.to_string()),
            partials,
            child_num,
            child_den,
        }
    }

    test_cases! {
        merge_no_top_level_empty;
        args: (xis: Vec<Xi>, expected: Xi);

        case: two_simple => (
            vec![_xi("2"), _xi("1")],
            xi(None, vec![], vec![_xi("1"), _xi("2")], vec![])
        );
        case: three_simple => (
            vec![_xi("2"), _xi("3"), _xi("1")],
            xi(None, vec![], vec![_xi("1"), _xi("2"), _xi("3")], vec![])
        );
        case: two_simple_matching => (
            vec![_xi("1"), _xi("1")],
            xi(None, vec![], vec![_xi("1"), _xi("1")], vec![])
        );
        case: two_empty => (
            vec![Xi::empty(), Xi::empty()],
            Xi::empty()
        );

        body: {
            assert_eq!(Xi::merge(&xis), expected);
        }
    }

    test_cases! {
        merge_with_top_level_empty;
        args: (xis: Vec<Xi>, expected: Xi);

        case: two => (
            vec![
                xi(None, vec![], vec![_xi("2")], vec![]),
                xi(None, vec![], vec![_xi("1")], vec![]),
            ],
            xi(None, vec![], vec![_xi("1"), _xi("2")], vec![])
        );
        case: should_flatten => (
            vec![
                xi(None, vec![], vec![_xi("2")], vec![]),
                Xi::merge(&[_xi("3"), _xi("1")])
            ],
            xi(None, vec![], vec![_xi("1"), _xi("2"), _xi("3")], vec![])
        );

        body: {
            assert_eq!(Xi::merge(&xis), expected);
        }
    }

    test_cases! {
        merge_with_top_level_partials;
        args: (xis: Vec<Xi>, expected: Xi);

        case: two => (
            vec![
                xi(Some("2"), vec![form!(0)], vec![], vec![]),
                xi(Some("1"), vec![], vec![], vec![]),
            ],
            xi(None, vec![], vec![
                xi(Some("1"), vec![], vec![], vec![]),
                xi(Some("2"), vec![form!(0)], vec![], vec![]),
            ], vec![])
        );
        case: two_matching_value => (
            vec![
                xi(Some("1"), vec![form!(0)], vec![], vec![]),
                xi(Some("1"), vec![], vec![], vec![]),
            ],
            xi(None, vec![], vec![ _xi("1"), xi(Some("1"), vec![form!(0)], vec![], vec![]), ], vec![])
        );

        body: {
            assert_eq!(Xi::merge(&xis), expected);
        }
    }

    test_cases! {
        merge_with_child_partials;
        args: (xis: Vec<Xi>, expected: Xi);

        case: two => (
            vec![
                xi(None, vec![], vec![_xi("1")], vec![]),
                xi(None, vec![], vec![xi(Some("2"), vec![form!(0)], vec![], vec![])], vec![]),
            ],
            xi(None, vec![], vec![ _xi("1"), xi(Some("2"), vec![form!(0)], vec![], vec![]), ], vec![])
        );

        body: {
            assert_eq!(Xi::merge(&xis), expected);
        }
    }
}
