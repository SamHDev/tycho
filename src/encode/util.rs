use crate::ident::Ident;

pub(crate) fn join_bytes(mut a: Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    a.extend_from_slice(&b);
    a
}

pub(crate) fn prefix_bytes(p: u8, mut b: Vec<u8>) -> Vec<u8> {
    b.insert(0, p);
    b
}

pub(crate) fn join_nibs(a: u8, b: u8) -> u8 {
    ((a & 0xF) << 4) + (b & 0xF)
}

pub(crate) fn join_idents<A: Ident<u8>, B: Ident<u8>>(a: A, b: B) -> u8 {
    join_nibs(a.ident(), b.ident())
}

pub(crate) fn one_ident<A: Ident<u8>>(a: A) -> u8 {
    join_nibs(a.ident(), 0x00)
}