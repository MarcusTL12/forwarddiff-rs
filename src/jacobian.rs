use super::*;
use ndarray::prelude::*;

pub fn jacobian<T: Real, F: Fn(&[Dual<T>], &mut [Dual<T>])>(
    f: F,
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
}
