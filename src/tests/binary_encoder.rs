use crate::binary_encoder::encode_ion_value;
use crate::{IonParser, IonValue};

#[test]
fn encode_integer_2019() {
    #[allow(overflowing_literals)]
    let values: Vec<i64> = vec![
        0b_0000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_1000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
        0b_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
        -9271905709435714,
        -23874324,
        -234,
        -1,
        0,
        1,
        41234,
        12342151456,
        123165237231415,
        0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0001,
        0b_0100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
        0b_0111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111,
    ];

    for ion_value in values {
        let ion_value = IonValue::Integer(ion_value);

        let bytes = encode_ion_value(&ion_value);

        let resulting_ion_value = IonParser::new(&bytes[..]).consume_value().unwrap().0;

        assert_eq!(ion_value, resulting_ion_value);
    }
}
