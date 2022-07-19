pub mod dual;

// have this as feature?
mod jacobian;
pub use jacobian::*;

use dual::Dual;
pub use num_traits::{real::Real, Zero};

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

pub fn grad<T: Real, F: Fn(&[Dual<T>]) -> Dual<T>>(
    f: F,
    x: &[T],
    g: &mut [T],
    buf: &mut [Dual<T>],
) {
    for (b, &x) in buf.iter_mut().zip(x.iter()) {
        *b = Dual::new(x);
    }

    for i in 0..x.len() {
        buf[i].d = T::one();

        let fx = f(buf);

        g[i] = fx.d;

        buf[i].d = T::zero();
    }
}

pub fn make_grad_fn<'a, T: Real, F: Fn(&[Dual<T>]) -> Dual<T>>(
    f: &'a F,
    buf: &'a mut [Dual<T>],
) -> impl FnMut(&[T], &mut [T]) + 'a {
    move |x, g| grad(f, x, g, buf)
}

pub fn grad_static<const N: usize, T: Real, F: Fn(&[Dual<T>]) -> Dual<T>>(
    f: F,
    x: &[T; N],
) -> [T; N] {
    let mut g = [T::zero(); N];
    let mut buf = [Dual::zero(); N];

    grad(&f, x, &mut g, &mut buf);

    g
}

pub fn make_grad_fn_static<
    const N: usize,
    T: Real,
    F: Fn(&[Dual<T>]) -> Dual<T>,
>(
    f: &F,
) -> impl Fn(&[T; N]) -> [T; N] + '_ {
    move |x| grad_static(f, x)
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
            ((x * (x / T::from(3).unwrap()).cos()).exp()
                * (x.cos() + T::from(1.5).unwrap()).ln())
            .sin()
        }

        let d1f = make_diff_fn(&f);
        let d2f = make_diff_fn(&d1f);
        let d3f = make_diff_fn(&d2f);
        let d4f = make_diff_fn(&d3f);

        println!("{}", d4f(4.0));
    }

    fn gf<T: Real>(x: &[T]) -> T {
        x[0] * (x[0] * x[1]).exp()
    }

    #[test]
    fn grad1() {
        let x = [1.0, 1.0];
        let mut g = [0.0; 2];
        let mut buf = [Dual::zero(); 2];

        grad(gf, &x, &mut g, &mut buf);

        println!("{:?}", g);
    }

    #[test]
    fn grad1_fn() {
        let mut g = [0.0; 2];
        let mut buf = [Dual::zero(); 2];

        let mut df = make_grad_fn(&gf, &mut buf);

        df(&[1.0, 1.0], &mut g);

        println!("{:?}", g);
    }

    #[test]
    fn grad1_static() {
        let x = [1.0, 1.0];

        let g = grad_static(gf, &x);

        println!("{:?}", g);
    }

    #[test]
    fn grad1_fn_static() {
        let df = make_grad_fn_static(&gf);

        let g = df(&[1.0, 1.0]);
        println!("{:?}", g);
    }
}
