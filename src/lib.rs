pub mod dual;

use dual::Dual;
pub use num_traits::{real::Real, NumCast};

pub fn diff<T: Real, F: Fn(Dual<T>) -> Dual<T>>(f: F, x: T) -> T {
    let x = Dual { r: x, d: T::one() };

    let fx = f(x);

    fx.d
}

pub fn make_diff_fn<T: Real, F: Fn(Dual<T>) -> Dual<T>>(
    f: &F,
) -> impl Fn(T) -> T + '_ {
    move |x| diff(f, x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff1() {
        let x = 1.0;

        fn f<T: Real>(x: T) -> T {
            (x - T::one()) / (x + T::one())
        }

        let d = diff(f, x);
        println!("{}", d);
    }

    #[test]
    fn diff_fn() {
        fn f<T: Real>(x: T) -> T {
            (x - T::one()) / (x + T::one())
        }

        let df = make_diff_fn(&f);

        let d = df(1.0);
        println!("{}", d);
    }

    #[test]
    fn diff2() {
        fn f<T: Real>(x: T) -> T {
            (x - T::one()) / (x + T::one())
        }

        let df = make_diff_fn(&f);
        let ddf = make_diff_fn(&df);

        println!("{}", ddf(0.0));
        println!("{}", ddf(0.5));
        println!("{}", ddf(1.0));
    }

    #[test]
    fn diff3() {
        fn f<T: Real>(x: T) -> T {
            (x - T::one()) / (x + T::one())
        }

        let d1f = make_diff_fn(&f);
        let d2f = make_diff_fn(&d1f);
        let d3f = make_diff_fn(&d2f);
        let d4f = make_diff_fn(&d3f);
        let d5f = make_diff_fn(&d4f);
        let d6f = make_diff_fn(&d5f);
        let d7f = make_diff_fn(&d6f);
        let d8f = make_diff_fn(&d7f);
        let d9f = make_diff_fn(&d8f);
        println!("{}", d9f(0.0));
    }

    #[test]
    fn diff_fancy() {
        fn f<T: Real>(x: T) -> T {
            // sin(e^(x * cos(x / 3)) * ln(cos(x) + 1.5))
            ((x * (x / <T as NumCast>::from(3).unwrap()).cos()).exp()
                * (x.cos() + <T as NumCast>::from(1.5).unwrap()).ln())
            .sin()
        }

        let d1f = make_diff_fn(&f);
        let d2f = make_diff_fn(&d1f);
        let d3f = make_diff_fn(&d2f);
        let d4f = make_diff_fn(&d3f);

        println!("{}", d4f(4.0));
    }
}
