use ibig::{ibig, IBig};

/// Trait ExtGcd is used to define the extended Euclidean algorithm
/// method ext_gcd for data type IBig
pub trait ExtGcd {
    /// Find the greatest common divisor (gcd)
    /// between the two numbers self and rhs & return it
    /// alongside the coefficients of Bézout's identity.
    ///
    /// # Arguments
    ///
    /// * `rhs` - An IBig integer  
    ///
    /// # Returns
    ///
    /// * `(IBig, IBig, IBig)` - A tuple containing the gcd & coefficients of Bézout's identity.
    fn ext_gcd(self, rhs: IBig) -> (IBig, IBig, IBig);
}

impl ExtGcd for IBig {
    /// Realize the extended Euclidean algorithm to find
    /// the greatest common divisor (gcd) between self and y & return it
    /// alongside the coefficients of Bézout's identity.
    ///
    /// # Arguments
    ///
    /// * `y` - An IBig integer  
    ///
    /// # Returns
    ///
    /// * `(IBig, IBig, IBig)` - A tuple containing gcd and coefficients of Bézout's identity.
    fn ext_gcd(mut self, mut y: IBig) -> (IBig, IBig, IBig) {
        let mut a1 = ibig!(0);
        let mut b2 = ibig!(0);

        let mut b1 = ibig!(1);
        let mut a2 = ibig!(1);

        while y != ibig!(0) {
            let q = &self / &y;
            let r = self - &q * &y;

            let a = a2 - &q * &a1;
            let b = b2 - &q * &b1;

            self = y;
            y = r;
            a2 = a1;
            a1 = a;
            b2 = b1;
            b1 = b;
        }

        (self, a2, b2)
    }
}
