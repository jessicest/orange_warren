#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pos<T> {
    x: T,
    y: T,
}

pub struct Square<T> {
    nw: Pos<T>,
    width: T,
}

type ZoneId = String;

pub struct Zone<T> {
    area: T,
    name: ZoneId,
    parents: Vec<Box<Zone<T>>>,
    children: Vec<Box<Zone<T>>>,
    neighbors: Vec<Box<Zone<T>>>,
}
