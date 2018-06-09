extern crate num;


use std::fmt::Display;

pub trait CDNum:num::Num + Copy + Display{}



impl CDNum for usize{}
impl CDNum for i32{}
impl CDNum for i64{}
impl CDNum for u8{}
impl CDNum for f32{}
impl CDNum for f64{}

