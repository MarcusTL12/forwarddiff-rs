use super::*;
use ndarray::prelude::*;

pub fn jacobian<T: Real, F: FnMut(&[Dual<T>], &mut [Dual<T>])>(
    mut f: F,
    x: &[T],
    buf_in: &mut [Dual<T>],
    buf_out: &mut [Dual<T>],
    jac: Option<Array2<T>>,
) -> Array2<T> {
    let n_in = buf_in.len();
    let n_out = buf_out.len();

    assert_eq!(x.len(), n_in);

    let mut jac = if let Some(jac) = jac {
        // Assert matrix size
        jac
    } else {
        Array2::zeros((n_out, n_in))
    };

    for i in 0..n_in {
        buf_in[i] = Dual::new(x[i]);
    }

    for i in 0..n_in {
        buf_in[i].d = T::one();
        f(buf_in, buf_out);
        buf_in[i].d = T::zero();

        for j in 0..n_out {
            jac[(j, i)] = buf_out[j].d;
        }
    }

    jac
}

//pub fn make_jacobian_fn<T: Real, F: FnMut(&[Dual<T>], &mut [Dual<T>])>

pub fn hessian<T: Real, F: Fn(&[Dual<Dual<T>>]) -> Dual<Dual<T>>>(
    f: F,
    x: &[T],
    buf_grad: &mut [Dual<Dual<T>>],
    buf_in: &mut [Dual<T>],
    buf_out: &mut [Dual<T>],
    hess: Option<Array2<T>>,
) -> Array2<T> {
    let g_fn = make_grad_fn(&f, buf_grad);

    jacobian(g_fn, x, buf_in, buf_out, hess)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn f1<T: Real>(x: &[T], y: &mut [T]) {
        let x2my2 = x[0].powi(2) - x[1].powi(2);
        let xy = x[0] * x[1];
        y[0] = x2my2 * xy.sin();
        y[1] = xy * x2my2.cos();
    }

    #[test]
    fn jacobian1() {
        let x = [1.0, 2.0];
        let mut buf_in = [Dual::zero(); 2];
        let mut buf_out = buf_in;

        let j = jacobian(f1, &x, &mut buf_in, &mut buf_out, None);
        println!("{j:6.3?}");
    }

    fn gf<T: Real>(x: &[T]) -> T {
        x[0] * (x[0] * x[1]).exp()
    }

    #[test]
    fn hessian1() {
        let x = [1.0, 2.0];
        let mut buf_grad = [Dual::zero(); 2];
        let mut buf_in = [Dual::zero(); 2];
        let mut buf_out = [Dual::zero(); 2];

        let h = hessian(gf, &x, &mut buf_grad, &mut buf_in, &mut buf_out, None);
        println!("{h:8.4?}");
    }
}
