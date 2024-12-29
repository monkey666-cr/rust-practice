use core::fmt;
use std::ops::{Add, AddAssign, Mul};

use anyhow::{Ok, Result};

#[derive(Debug)]
pub struct Matrix<T: fmt::Debug> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Matrix<T>
where
    T: fmt::Debug,
{
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug + Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.col != b.row {
        panic!("Matrix dimensions are not compatible for multiplication");
    }

    let mut data: Vec<T> = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}
