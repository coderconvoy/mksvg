
use std::fmt::Display;
use num;

pub trait CDNum:num::Num+num::NumCast+ Copy + Display{}



//impl CDNum for integer{}
impl CDNum for usize{}
impl CDNum for i32{}
impl CDNum for i64{}
impl CDNum for u8{}
impl CDNum for f32{}
impl CDNum for f64{}

