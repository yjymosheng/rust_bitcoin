#![feature(random)]

pub mod block;
mod tools;
pub mod chain;

pub use block::Transaction; // 导出 Transaction 结构
