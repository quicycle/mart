use std::{cmp, fmt};

use itertools::Itertools;

use crate::algebra::{Grade, Index, Sign};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Form {
    pub(crate) zet: Zet,
    pub(crate) orientation: Orientation,
}

impl Form {
    pub fn new(z: Zet, o: Orientation) -> Self {
        Self {
            zet: z,
            orientation: o,
        }
    }

    pub fn zet(&self) -> Zet {
        self.zet
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    #[inline(always)]
    pub fn grade(&self) -> Grade {
        use Grade::*;
        use Orientation::*;
        use Zet::*;

        match (self.orientation, self.zet) {
            // { p 23 31 12 }
            (e, B) => Zero,
            (_, B) => Two,
            // { 0 023 031 012 }
            (e, T) => One,
            (_, T) => Three,
            // { 123 1 2 3 }
            (e, A) => Three,
            (_, A) => One,
            // { 0123 01 02 03 }
            (e, E) => Four,
            (_, E) => Two,
        }
    }

    pub fn iter() -> FormIterator {
        FormIterator { index: 0 }
    }

    /// Attempt to construct a Form from an arbitrary vector of [`Index`] values.
    pub fn try_from_indices(ixs: &[Index]) -> Result<Self, String> {
        use Index::*;
        use Orientation::*;
        use Zet::*;

        match *ixs {
            [] => Ok(Form::new(B, e)),
            [Two, Three] => Ok(Form::new(B, i)),
            [Three, One] => Ok(Form::new(B, j)),
            [One, Two] => Ok(Form::new(B, k)),
            [Zero] => Ok(Form::new(T, e)),
            [Zero, Two, Three] => Ok(Form::new(T, i)),
            [Zero, Three, One] => Ok(Form::new(T, j)),
            [Zero, One, Two] => Ok(Form::new(T, k)),
            [One, Two, Three] => Ok(Form::new(A, e)),
            [One] => Ok(Form::new(A, i)),
            [Two] => Ok(Form::new(A, j)),
            [Three] => Ok(Form::new(A, k)),
            [Zero, One, Two, Three] => Ok(Form::new(E, e)),
            [Zero, One] => Ok(Form::new(E, i)),
            [Zero, Two] => Ok(Form::new(E, j)),
            [Zero, Three] => Ok(Form::new(E, k)),
            _ => Err(format!("Invalid component indices {:?}", ixs)),
        }
    }

    #[inline(always)]
    pub fn as_indices(&self) -> Vec<Index> {
        use Index::*;
        use Orientation::*;
        use Zet::*;

        match (self.orientation, self.zet) {
            // { p 23 31 12 }
            (e, B) => vec![],
            (i, B) => vec![Two, Three],
            (j, B) => vec![Three, One],
            (k, B) => vec![One, Two],
            // { 0 023 031 012 }
            (e, T) => vec![Zero],
            (i, T) => vec![Zero, Two, Three],
            (j, T) => vec![Zero, Three, One],
            (k, T) => vec![Zero, One, Two],
            // { 123 1 2 3 }
            (e, A) => vec![One, Two, Three],
            (i, A) => vec![One],
            (j, A) => vec![Two],
            (k, A) => vec![Three],
            // { 0123 01 02 03 }
            (e, E) => vec![Zero, One, Two, Three],
            (i, E) => vec![Zero, One],
            (j, E) => vec![Zero, Two],
            (k, E) => vec![Zero, Three],
        }
    }

    // Decompose into singlet & orientation representation, which may include a sign change
    #[inline(always)]
    fn decompose(&self) -> (Sign, Singlet, Orientation) {
        use Orientation::*;
        use Sign::*;
        use Zet::*;

        // XXX Under +---/jk there is a sign change for the A and E triads
        let sign = match self.zet {
            A | E if self.orientation != e => Neg,
            _ => Pos,
        };

        (sign, self.zet.singlet(), self.orientation)
    }

    // Product of forms under their K4 group structure, tracking sign change
    //
    // XXX The set of sign changes described below are valid for +---/jk
    //     See the test cases below that compute the results using ar_product in order to
    //     ensure that we are always valid for the metric and alphas in use.
    //
    //   I) Decompose into sign, singlet and orientation
    //  II) Combine singlets under K4, tracking sign
    // III) Combine orientations according to quaternion multiplication
    //  IV) Recompose singlet and orientation, tracking sign change
    //   V) Combine all sign changes
    pub fn product(&self, rhs: &Self) -> (Sign, Self) {
        use Orientation::*;
        use Sign::*;
        use Singlet::*;
        use Zet::*;

        let (l_sign, l_singlet, l_orient) = self.decompose();
        let (r_sign, r_singlet, r_orient) = rhs.decompose();

        let (s_sign, singlet) = l_singlet.compose(&r_singlet);
        let (o_sign, orientation) = l_orient.compose(&r_orient);
        let (z_sign, zet) = match singlet {
            p => (Pos, B),
            t => (Pos, T),
            h if orientation == e => (Pos, A),
            h => (Neg, A),
            q if orientation == e => (Pos, E),
            q => (Neg, E),
        };

        let sign = l_sign * r_sign * s_sign * o_sign * z_sign;

        (sign, Form { zet, orientation })
    }
}

impl fmt::Display for Form {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let (Zet::B, Orientation::e) = (self.zet, self.orientation) {
            write!(f, "p")
        } else {
            let index_str = self.as_indices().iter().map(|i| i.to_string()).join("");
            write!(f, "{}", index_str)
        }
    }
}

impl cmp::Ord for Form {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.zet
            .cmp(&other.zet)
            .then(self.orientation.cmp(&other.orientation))
    }
}

impl cmp::PartialOrd for Form {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone)]
pub struct FormIterator {
    index: usize,
}

impl Iterator for FormIterator {
    type Item = Form;

    fn next(&mut self) -> Option<Self::Item> {
        use Orientation::*;
        use Zet::*;

        let item = match self.index {
            0 => Some(Form::new(B, e)),
            1 => Some(Form::new(B, i)),
            2 => Some(Form::new(B, j)),
            3 => Some(Form::new(B, k)),
            4 => Some(Form::new(T, e)),
            5 => Some(Form::new(T, i)),
            6 => Some(Form::new(T, j)),
            7 => Some(Form::new(T, k)),
            8 => Some(Form::new(A, e)),
            9 => Some(Form::new(A, i)),
            10 => Some(Form::new(A, j)),
            11 => Some(Form::new(A, k)),
            12 => Some(Form::new(E, e)),
            13 => Some(Form::new(E, i)),
            14 => Some(Form::new(E, j)),
            15 => Some(Form::new(E, k)),
            _ => None,
        };
        self.index += 1;
        item
    }
}

macro_rules! __signed_k4 {
    (
        $t:tt => $e:tt, $i:tt, $j:tt, $k:tt;
        Negatives: $(($l:tt, $r:tt)),*;
    ) => {
        #[inline(always)]
        fn compose(&self, right: &$t) -> (Sign, $t) {
            use $t::*;
            use Sign::*;

            let sign = match (self, right) {
                $(($l, $r) => Neg,)*
                _ => Pos,
            };

            let elem = match (self, right) {
                ($e, other) | (other, $e) => *other,
                ($i, $i) | ($j, $j) | ($k, $k) => $e,
                ($i, $j) | ($j, $i) => $k,
                ($j, $k) | ($k, $j) => $i,
                ($k, $i) | ($i, $k) => $j,
            };

            (sign, elem)
        }
    };
}

#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Zet {
    B,
    T,
    A,
    E,
}

impl Zet {
    #[inline(always)]
    fn singlet(&self) -> Singlet {
        use Singlet::*;
        use Zet::*;

        match self {
            B => p,
            T => t,
            A => h,
            E => q,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
enum Singlet {
    p,
    t,
    h,
    q,
}

impl Singlet {
    __signed_k4!(
        Singlet => p, t, h, q;
        Negatives: (q, q), (h, t), (h, q), (q, t);
    );
}

#[allow(non_camel_case_types)]
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Orientation {
    e,
    i,
    j,
    k,
}

impl Orientation {
    __signed_k4!(
        Orientation => e, i, j, k;
        Negatives: (i, i), (j, j), (k, k), (j, i), (k, j), (i, k);
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algebra::{operations::ar_product, Alpha};

    #[test]
    fn product_agrees_with_full_prod() {
        use Orientation::*;
        use Sign::*;
        use Zet::*;

        for zl in [B, T, A, E].iter() {
            for ol in [e, i, j, k].iter() {
                let fl = Form::new(*zl, *ol);
                let al = Alpha::try_from_indices(Pos, &fl.as_indices()).unwrap();

                for zr in [B, T, A, E].iter() {
                    for or in [e, i, j, k].iter() {
                        let fr = Form::new(*zr, *or);
                        let ar = Alpha::try_from_indices(Pos, &fr.as_indices()).unwrap();

                        let a_prod = ar_product(&al, &ar);
                        let res_a = (a_prod.sign(), a_prod.form().as_indices());

                        let (f_sgn, f_form) = fl.product(&fr);
                        let f_ixs = f_form.as_indices();
                        let res_f = (f_sgn, f_ixs);

                        assert_eq!(res_a, res_f, "{:?}{:?} {:?}{:?}", ol, zl, or, zr);
                    }
                }
            }
        }
    }
}
