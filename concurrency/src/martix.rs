use core::fmt;
use std::{
    ops::{Add, AddAssign, Mul, MulAssign},
    thread,
};

use anyhow::Result;

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, ", ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, "; ")?;
            }
        }

        write!(f, "}}")?;

        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: fmt::Debug
        + Copy
        + Default
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + MulAssign
        + Send
        + 'static,
{
    if a.col != b.row {
        panic!("Matrix dimensions are not compatible for multiplication");
    }

    // 定义异步线程的执行函数
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = std::sync::mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Failed to send result: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            // 返回结果的索引
            let idx = i * b.col + j;
            // 输入消息
            let input = MsgInput { idx, row, col };
            // 一次性结果返回
            let (tx, rx) = oneshot::channel();
            // 构造消息
            let msg = Msg::new(input, tx);
            // 判断是否提交失败
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Failed to send message: {:?}", e);
            }
            receivers.push(rx);
        }
    }

    // map/recude
    for rx in receivers {
        let output = rx.recv().unwrap();
        data[output.idx] = output.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T> Mul for Matrix<T>
where
    T: fmt::Debug
        + Copy
        + Default
        + Add<Output = T>
        + AddAssign
        + Mul<Output = T>
        + MulAssign
        + Send
        + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiplication failed")
    }
}
