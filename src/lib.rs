use ibig::{ibig, IBig};

pub trait ExtGcd {
    fn ext_gcd(self, rhs: IBig) -> (IBig, IBig, IBig);
}

impl ExtGcd for IBig {
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
