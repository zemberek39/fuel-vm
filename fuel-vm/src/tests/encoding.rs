use fuel_vm::{
    consts::*,
    prelude::*,
};
use rand::{
    rngs::StdRng,
    Rng,
    SeedableRng,
};

use fuel_types::Word;
use std::{
    fmt,
    io::{
        self,
        Read,
        Write,
    },
};

pub fn assert_encoding_correct<T>(data: &[T])
where
    T: Read + Write + fmt::Debug + Clone + PartialEq,
{
    let mut buffer;

    for data in data.iter() {
        let mut d = data.clone();
        let mut d_p = data.clone();

        buffer = vec![0u8; 1024];
        let read_size = d.read(buffer.as_mut_slice()).expect("Failed to read");
        let write_size = d_p.write(buffer.as_slice()).expect("Failed to write");

        // Simple RW assertion
        assert_eq!(d, d_p);
        assert_eq!(read_size, write_size);

        buffer = vec![0u8; read_size];

        // Minimum size buffer assertion
        let _ = d.read(buffer.as_mut_slice()).expect("Failed to read");
        let _ = d_p.write(buffer.as_slice()).expect("Failed to write");
        assert_eq!(d, d_p);

        // No panic assertion
        loop {
            buffer.pop();

            let err = d
                .read(buffer.as_mut_slice())
                .expect_err("Insufficient buffer should fail!");
            assert_eq!(io::ErrorKind::UnexpectedEof, err.kind());

            let err = d_p
                .write(buffer.as_slice())
                .expect_err("Insufficient buffer should fail!");
            assert_eq!(io::ErrorKind::UnexpectedEof, err.kind());

            if buffer.is_empty() {
                break
            }
        }
    }
}

#[test]
fn call() {
    let rng = &mut StdRng::seed_from_u64(2322u64);

    assert_encoding_correct(
        (0..10)
            .map(|_| Call::new(rng.gen(), rng.gen(), rng.gen()))
            .collect::<Vec<Call>>()
            .as_slice(),
    );
}

#[test]
fn call_frame() {
    let rng = &mut StdRng::seed_from_u64(2322u64);

    assert_encoding_correct(
        (0..10)
            .map(|_| {
                CallFrame::new(
                    rng.gen(),
                    rng.gen(),
                    [rng.gen(); VM_REGISTER_COUNT],
                    200,
                    rng.gen(),
                    rng.gen(),
                )
            })
            .collect::<Vec<CallFrame>>()
            .as_slice(),
    );
}

#[test]
fn witness() {
    assert_encoding_correct(&[Witness::from(vec![0xef]), Witness::from(vec![])]);
}

#[test]
fn input() {
    assert_encoding_correct(&[
        Input::coin_signed(
            UtxoId::new([0xaa; 32].into(), 0),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            TxPointer::new(0x3802.into(), 0x28),
            0xff,
            (u32::MAX >> 1).into(),
        ),
        Input::coin_predicate(
            UtxoId::new([0xaa; 32].into(), 0),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            TxPointer::new(0x3802.into(), 0x28),
            (u32::MAX >> 1).into(),
            Word::MAX,
            vec![0xdd; 50],
            vec![0xee; 23],
        ),
        Input::coin_predicate(
            UtxoId::new([0xaa; 32].into(), 0),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            TxPointer::new(0x3802.into(), 0x28),
            (u32::MAX >> 1).into(),
            Word::MAX,
            vec![0xdd; 50],
            vec![],
        ),
        Input::message_coin_signed(
            [0xaa; 32].into(),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            0xff,
        ),
        Input::message_coin_predicate(
            [0xaa; 32].into(),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            Word::MAX,
            vec![0xee; 50],
            vec![0xff; 23],
        ),
        Input::message_coin_predicate(
            [0xaa; 32].into(),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            Word::MAX,
            vec![0xee; 50],
            vec![],
        ),
        Input::message_data_signed(
            [0xaa; 32].into(),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            0xff,
            vec![0xdd; 50],
        ),
        Input::message_data_predicate(
            [0xaa; 32].into(),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            Word::MAX,
            vec![0xdd; 50],
            vec![0xee; 50],
            vec![0xff; 23],
        ),
        Input::message_data_predicate(
            [0xaa; 32].into(),
            [0xbb; 32].into(),
            Word::MAX,
            [0xcc; 32].into(),
            Word::MAX,
            vec![0xdd; 50],
            vec![0xee; 50],
            vec![],
        ),
        Input::contract(
            UtxoId::new([0xaa; 32].into(), 0),
            [0xbb; 32].into(),
            [0xcc; 32].into(),
            TxPointer::new(0x3802.into(), 0x28),
            [0xdd; 32].into(),
        ),
    ]);
}

#[test]
fn output() {
    assert_encoding_correct(&[
        Output::coin([0xaa; 32].into(), Word::MAX >> 1, [0xbb; 32].into()),
        Output::contract(0xaa, [0xbb; 32].into(), [0xcc; 32].into()),
        Output::change([0xaa; 32].into(), Word::MAX >> 1, [0xbb; 32].into()),
        Output::variable([0xaa; 32].into(), Word::MAX >> 1, [0xbb; 32].into()),
        Output::contract_created([0xaa; 32].into(), [0xaa; 32].into()),
    ]);
}

#[test]
fn transaction() {
    let i = Input::contract(
        UtxoId::new([0xaa; 32].into(), 0),
        [0xbb; 32].into(),
        [0xcc; 32].into(),
        TxPointer::new(0xbeef.into(), 0xeaae),
        [0xdd; 32].into(),
    );
    let o = Output::coin([0xaa; 32].into(), Word::MAX >> 1, [0xbb; 32].into());
    let w = Witness::from(vec![0xbf]);

    assert_encoding_correct(&[
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![0xfa],
            vec![0xfb, 0xfc],
            vec![i.clone()],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![],
            vec![0xfb, 0xfc],
            vec![i.clone()],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![0xfa],
            vec![],
            vec![i.clone()],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![],
            vec![],
            vec![i.clone()],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![],
            vec![],
            vec![],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![w.clone()],
        ),
        Transaction::script(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
        ),
    ]);
    assert_encoding_correct(&[
        Transaction::create(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            0xba,
            [0xdd; 32].into(),
            vec![],
            vec![i],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::create(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            0xba,
            [0xdd; 32].into(),
            vec![],
            vec![],
            vec![o],
            vec![w.clone()],
        ),
        Transaction::create(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            0xba,
            [0xdd; 32].into(),
            vec![],
            vec![],
            vec![],
            vec![w],
        ),
        Transaction::create(
            Word::MAX >> 1,
            Word::MAX >> 2,
            (u32::MAX >> 3).into(),
            0xba,
            [0xdd; 32].into(),
            vec![],
            vec![],
            vec![],
            vec![],
        ),
    ]);
}
