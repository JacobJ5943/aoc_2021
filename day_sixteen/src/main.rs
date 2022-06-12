use anyhow::anyhow;
use anyhow::Result;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq, Eq)]
enum Bit {
    Zero,
    One,
}

fn bit_slice_to_usize(bit_vec: &[&Bit]) -> Result<usize> {
    if bit_vec.is_empty() {
        Err(anyhow!("bit vec must contain Bits"))
    } else {
        bit_vec
            .iter()
            .map(|b| match b {
                Bit::One => 1_usize,
                Bit::Zero => 0_usize,
            })
            .reduce(|acc, cur_digit| acc * 2 + cur_digit)
            .ok_or_else(|| anyhow!("Failed ot convert bitmap to usize"))
    }
}

fn bit_vec_to_usize(bit_vec: Vec<Bit>) -> Result<usize> {
    if bit_vec.is_empty() {
        Err(anyhow!("bit vec must contain Bits"))
    } else {
        bit_vec
            .iter()
            .map(|b| match b {
                Bit::One => 1_usize,
                Bit::Zero => 0_usize,
            })
            .reduce(|acc, cur_digit| acc * 2 + cur_digit)
            .ok_or_else(|| anyhow!("Failed ot convert bitmap to usize"))
    }
}

fn hex_to_bit_vec(input: String) -> Result<Vec<Bit>> {
    // It's not the right capacity, but eh
    let mut working_vector: Vec<Bit> = Vec::with_capacity(input.len());
    for char in input.chars() {
        match char {
            '0' => working_vector.append(&mut vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero]),
            '1' => working_vector.append(&mut vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::One]),
            '2' => working_vector.append(&mut vec![Bit::Zero, Bit::Zero, Bit::One, Bit::Zero]),
            '3' => working_vector.append(&mut vec![Bit::Zero, Bit::Zero, Bit::One, Bit::One]),
            '4' => working_vector.append(&mut vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero]),
            '5' => working_vector.append(&mut vec![Bit::Zero, Bit::One, Bit::Zero, Bit::One]),
            '6' => working_vector.append(&mut vec![Bit::Zero, Bit::One, Bit::One, Bit::Zero]),
            '7' => working_vector.append(&mut vec![Bit::Zero, Bit::One, Bit::One, Bit::One]),
            '8' => working_vector.append(&mut vec![Bit::One, Bit::Zero, Bit::Zero, Bit::Zero]),
            '9' => working_vector.append(&mut vec![Bit::One, Bit::Zero, Bit::Zero, Bit::One]),
            'A' => working_vector.append(&mut vec![Bit::One, Bit::Zero, Bit::One, Bit::Zero]),
            'B' => working_vector.append(&mut vec![Bit::One, Bit::Zero, Bit::One, Bit::One]),
            'C' => working_vector.append(&mut vec![Bit::One, Bit::One, Bit::Zero, Bit::Zero]),
            'D' => working_vector.append(&mut vec![Bit::One, Bit::One, Bit::Zero, Bit::One]),
            'E' => working_vector.append(&mut vec![Bit::One, Bit::One, Bit::One, Bit::Zero]),
            'F' => working_vector.append(&mut vec![Bit::One, Bit::One, Bit::One, Bit::One]),
            _ => panic!(),
        }
    }

    Ok(working_vector)
}

fn handle_4_literal<I>(bits: &mut I) -> Result<usize>
where
    I: Iterator<Item = Bit>,
{
    let mut working_sum = 0;

    // What I want to do here is loop until I have a group of 5 which start with a 0
    while let (Some(first), Some(second), Some(third), Some(fourth), Some(fifth)) = (
        bits.next(),
        bits.next(),
        bits.next(),
        bits.next(),
        bits.next(),
    ) {
        match first {
            Bit::One => {
                working_sum *= 2 * 2 * 2 * 2;
                working_sum += bit_slice_to_usize(&[&second, &third, &fourth, &fifth])?;
            }
            Bit::Zero => {
                working_sum *= 2 * 2 * 2 * 2;
                working_sum += bit_slice_to_usize(&[&second, &third, &fourth, &fifth])?;
                break;
            }
        }
    }

    Ok(working_sum)
}

fn part_one(input_hex_string: String) -> Result<usize> {
    let bit_vec = hex_to_bit_vec(input_hex_string)?;
    if bit_vec.len() < 6 {
        return Err(anyhow!("Len must be greater than 6"));
    }

    let mut bit_vec_iter = bit_vec.into_iter();
    let result = count_version_numbers(&mut bit_vec_iter)?;
    match result {
        None => return Err(anyhow!("Not enough bits")),
        Some(result) => Ok(result),
    }
}

fn part_two(input_hex_string: String) -> Result<usize> {
    let bit_vec = hex_to_bit_vec(input_hex_string)?;
    if bit_vec.len() < 6 {
        return Err(anyhow!("Len must be greater than 6"));
    }

    let mut bit_vec_iter = bit_vec.into_iter();
    let result = handle_packet(&mut bit_vec_iter)?;
    match result {
        None => return Err(anyhow!("Not enough bits")),
        Some(result) => Ok(result),
    }
}

fn count_version_numbers<I>(current_bit_iterator: &mut I) -> Result<Option<usize>>
where
    I: Iterator<Item = Bit>,
{
    let version = match (
        current_bit_iterator.next(),
        current_bit_iterator.next(),
        current_bit_iterator.next(),
    ) {
        (Some(one), Some(two), Some(three)) => vec![one, two, three],
        (None, None, None) => return Ok(None),
        _ => return Err(anyhow!("Not enough bits")),
    };

    let type_bits = if let (Some(one), Some(two), Some(three)) = (
        current_bit_iterator.next(),
        current_bit_iterator.next(),
        current_bit_iterator.next(),
    ) {
        vec![one, two, three]
    } else {
        return Err(anyhow!("len must be greater than 6"));
    };

    match bit_slice_to_usize(&type_bits.iter().collect::<Vec<&Bit>>())? {
        4 => {
            // This isn't needed as it won't contain a version
            // read 5 at a time until the first bit is 0

            while let (Some(Bit::One), _, _, _, _) = (
                current_bit_iterator.next(),
                current_bit_iterator.next(),
                current_bit_iterator.next(),
                current_bit_iterator.next(),
                current_bit_iterator.next(),
            ) {}

            Ok(Some(bit_vec_to_usize(version)?))
        }
        _operator_type => {
            // This means that it's an operator of some sort
            match current_bit_iterator.next() {
                Some(Bit::Zero) => {
                    let mut length_of_sub_packet_vec = Vec::with_capacity(15);
                    for _ in 0..15 {
                        length_of_sub_packet_vec.push(
                            current_bit_iterator
                                .next()
                                .ok_or_else(|| anyhow!("not enough bits in initial 15"))?,
                        );
                    }

                    let sub_packet_length = bit_vec_to_usize(length_of_sub_packet_vec)?;

                    let mut sub_packets: Vec<Bit> = Vec::with_capacity(sub_packet_length);
                    for last_bit_read in 0..sub_packet_length {
                        sub_packets.push(current_bit_iterator.next().ok_or_else(|| {
                            anyhow!(
                                "not enough bits.  Expected {} got {}",
                                sub_packet_length,
                                last_bit_read
                            )
                        })?);
                    }

                    // I am going to assume that there are the perfrect number of bits and no padding on the end
                    let mut sub_packets = sub_packets.into_iter();

                    let mut sub_version_total = bit_vec_to_usize(version)?;

                    // handle_packet will always take from the iterator so eventually it will run out
                    loop {
                        //sub_packets = dbg!(sub_packets.collect::<Vec<Bit>>()).into_iter();
                        let sub_packet_result = count_version_numbers(&mut sub_packets)?;
                        match sub_packet_result {
                            None => break,
                            Some(literal_value) => sub_version_total += literal_value,
                        }
                    }
                    Ok(Some(sub_version_total))
                }
                Some(Bit::One) => {
                    let mut sub_packet_bit_count_vec = Vec::with_capacity(11);
                    for _ in 0..11 {
                        sub_packet_bit_count_vec.push(
                            current_bit_iterator
                                .next()
                                .ok_or_else(|| anyhow!("not enough bits"))?,
                        );
                    }

                    let sub_packet_bit_count = bit_vec_to_usize(sub_packet_bit_count_vec)?;

                    let mut sub_version_total = bit_vec_to_usize(version)?;
                    for _ in 0..sub_packet_bit_count {
                        let sub_packet_result = count_version_numbers(current_bit_iterator)?;
                        match sub_packet_result {
                            None => return Err(anyhow!("Not enough bits")),
                            Some(sub_packet_result) => sub_version_total += sub_packet_result,
                        }
                    }
                    Ok(Some(sub_version_total))
                }
                None => Err(anyhow!("Ran out of bits")),
            }
        }
    }
}

fn handle_packet<I>(current_bit_iterator: &mut I) -> Result<Option<usize>>
where
    I: Iterator<Item = Bit>,
{
    let _version = match (
        current_bit_iterator.next(),
        current_bit_iterator.next(),
        current_bit_iterator.next(),
    ) {
        (Some(one), Some(two), Some(three)) => vec![one, two, three],
        (None, None, None) => return Ok(None),
        _ => return Err(anyhow!("Not enough bits")),
    };

    let type_bits = if let (Some(one), Some(two), Some(three)) = (
        current_bit_iterator.next(),
        current_bit_iterator.next(),
        current_bit_iterator.next(),
    ) {
        vec![one, two, three]
    } else {
        return Err(anyhow!("len must be greater than 6"));
    };

    match bit_slice_to_usize(&type_bits.iter().collect::<Vec<&Bit>>())? {
        4 => {
            let result = handle_4_literal(current_bit_iterator)?;
            Ok(Some(result))
        }
        operator_type => match operator_type {
            0 => handle_operator(current_bit_iterator, usize::wrapping_add),
            1 => handle_operator(current_bit_iterator, usize::wrapping_mul),
            2 => handle_operator(current_bit_iterator, usize::min),
            3 => handle_operator(current_bit_iterator, usize::max),
            5 => handle_operator(current_bit_iterator, |a, b| if a > b { 1 } else { 0 }),
            6 => handle_operator(current_bit_iterator, |a, b| if a < b { 1 } else { 0 }),
            7 => handle_operator(current_bit_iterator, |a, b| if a == b { 1 } else { 0 }),
            _ => Err(anyhow!("{} not a valid operator", operator_type)),
        },
    }
}

// operator function should be treated as a reduce
fn handle_operator<I, F>(
    current_bit_iterator: &mut I,
    operator_function: F,
) -> Result<Option<usize>>
where
    I: Iterator<Item = Bit>,
    F: Fn(usize, usize) -> usize,
{
    // This means that it's an operator of some sort
    match current_bit_iterator.next() {
        Some(Bit::Zero) => {
            let mut sub_packet_bit_count_vec = Vec::with_capacity(15);
            for _ in 0..15 {
                sub_packet_bit_count_vec.push(
                    current_bit_iterator
                        .next()
                        .ok_or_else(|| anyhow!("not enough bits in initial 15"))?,
                );
            }

            let sub_packet_bit_count = bit_vec_to_usize(sub_packet_bit_count_vec)?;

            let mut sub_packets: Vec<Bit> = Vec::with_capacity(sub_packet_bit_count);
            for last_bit_read in 0..sub_packet_bit_count {
                sub_packets.push(current_bit_iterator.next().ok_or_else(|| {
                    anyhow!(
                        "not enough bits.  Expected {} got {}",
                        sub_packet_bit_count,
                        last_bit_read
                    )
                })?);
            }

            let mut sub_packets = sub_packets.into_iter();

            let mut current_packet_result = None;

            loop {
                let sub_packet_result = handle_packet(&mut sub_packets)?;
                match sub_packet_result {
                    None => break,
                    Some(sub_packet_result) => match current_packet_result {
                        Some(previou_value) => {
                            current_packet_result =
                                Some(operator_function(previou_value, sub_packet_result))
                        }
                        None => current_packet_result = Some(sub_packet_result),
                    },
                }
            }
            if current_packet_result.is_none() {
                Err(anyhow!("didn't find sub results"))
            } else {
                Ok(current_packet_result)
            }
        }
        Some(Bit::One) => {
            let mut sub_packet_count_vec = Vec::with_capacity(11);
            for _ in 0..11 {
                sub_packet_count_vec.push(
                    current_bit_iterator
                        .next()
                        .ok_or_else(|| anyhow!("not enough bits"))?,
                );
            }

            let sub_packet_count = bit_vec_to_usize(sub_packet_count_vec)?;

            let mut current_packet_result = None;
            for _ in 0..sub_packet_count {
                let sub_packet_result = handle_packet(current_bit_iterator)?;
                match sub_packet_result {
                    None => return Err(anyhow!("Not enough bits")),
                    Some(sub_packet_result) => match current_packet_result {
                        Some(previous_value) => {
                            current_packet_result =
                                Some(operator_function(previous_value, sub_packet_result))
                        }
                        None => current_packet_result = Some(sub_packet_result),
                    },
                }
            }

            if current_packet_result.is_none() {
                Err(anyhow!("didn't find sub results"))
            } else {
                Ok(current_packet_result)
            }
        }
        None => Err(anyhow!("Ran out of bits")),
    }
}

#[cfg(test)]
mod tests {
    use crate::{bit_slice_to_usize, handle_4_literal, hex_to_bit_vec, part_one, part_two, Bit};

    #[test]
    fn test_hex_to_bit_vec() {
        assert_eq!(
            hex_to_bit_vec("D2FE28".to_string()).expect("Should return vec"),
            vec![
                Bit::One,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::One,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::One,
                Bit::Zero,
                Bit::Zero,
                Bit::Zero
            ]
        );
    }

    #[test]
    fn test_bit_vec_to_usize() {
        assert!(bit_slice_to_usize(&[]).is_err());
        assert_eq!(
            bit_slice_to_usize(&[&Bit::One, &Bit::One, &Bit::One]).expect("-esult to be Ok"),
            7
        );
        assert_eq!(
            bit_slice_to_usize(&[&Bit::One, &Bit::Zero, &Bit::One]).expect("Result to be Ok"),
            5
        );
        assert_eq!(
            bit_slice_to_usize(&[&Bit::Zero, &Bit::One, &Bit::One]).expect("Result to be Ok"),
            3
        );
        assert_eq!(
            bit_slice_to_usize(&[&Bit::One]).expect("Result to be Ok"),
            1
        );
        assert_eq!(
            bit_slice_to_usize(&[&Bit::Zero]).expect("Result to be Ok"),
            0
        );
    }

    #[test]
    fn test_handle_four_literal() {
        assert_eq!(
            handle_4_literal(
                &mut [
                    Bit::One,
                    Bit::Zero,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::One,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::Zero,
                    Bit::One,
                    Bit::Zero,
                    Bit::One
                ]
                .into_iter()
            )
            .expect("Expect result ok"),
            2021
        );
    }
    #[test]
    fn test_part_one() {
        assert_eq!(part_one("D2FE28".to_string()).expect("result ok"), 6);
        assert_eq!(
            part_one("38006F45291200".to_string()).expect("ok_result"),
            9
        );
        assert_eq!(
            part_one("EE00D40C823060".to_string()).expect("ok_result"),
            14
        );
        assert_eq!(
            part_one("8A004A801A8002F478".to_string()).expect("ok_result"),
            16
        );
        assert_eq!(
            part_one("620080001611562C8802118E34".to_string()).expect("ok_result"),
            12
        );
        assert_eq!(
            part_one("C0015000016115A2E0802F182340".to_string()).expect("ok_result"),
            23
        );
        assert_eq!(
            part_one("A0016C880162017C3686B18A3D4780".to_string()).expect("ok_result"),
            31
        );
    }

    #[test]
    fn part_one_my_input() {
        assert_eq!(part_one("C20D7900A012FB9DA43BA00B080310CE3643A0004362BC1B856E0144D234F43590698FF31D249F87B8BF1AD402389D29BA6ED6DCDEE59E6515880258E0040A7136712672454401A84CE65023D004E6A35E914BF744E4026BF006AA0008742985717440188AD0CE334D7700A4012D4D3AE002532F2349469100708010E8AD1020A10021B0623144A20042E18C5D88E6009CF42D972B004A633A6398CE9848039893F0650048D231EFE71E09CB4B4D4A00643E200816507A48D244A2659880C3F602E2080ADA700340099D0023AC400C30038C00C50025C00C6015AD004B95002C400A10038C00A30039C0086002B256294E0124FC47A0FC88ACE953802F2936C965D3005AC01792A2A4AC69C8C8CA49625B92B1D980553EE5287B3C9338D13C74402770803D06216C2A100760944D8200008545C8FB1EC80185945D9868913097CAB90010D382CA00E4739EDF7A2935FEB68802525D1794299199E100647253CE53A8017C9CF6B8573AB24008148804BB8100AA760088803F04E244480004323BC5C88F29C96318A2EA00829319856AD328C5394F599E7612789BC1DB000B90A480371993EA0090A4E35D45F24E35D45E8402E9D87FFE0D9C97ED2AF6C0D281F2CAF22F60014CC9F7B71098DFD025A3059200C8F801F094AB74D72FD870DE616A2E9802F800FACACA68B270A7F01F2B8A6FD6035004E054B1310064F28F1C00F9CFC775E87CF52ADC600AE003E32965D98A52969AF48F9E0C0179C8FE25D40149CC46C4F2FB97BF5A62ECE6008D0066A200D4538D911C401A87304E0B4E321005033A77800AB4EC1227609508A5F188691E3047830053401600043E2044E8AE0008443F84F1CE6B3F133005300101924B924899D1C0804B3B61D9AB479387651209AA7F3BC4A77DA6C519B9F2D75100017E1AB803F257895CBE3E2F3FDE014ABC".to_string()).expect("ok_result"),
         960);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("C200B40A82".to_string()).expect("result ok"), 3);
        assert_eq!(part_two("04005AC33890".to_string()).expect("result ok"), 54);
        assert_eq!(
            part_two("880086C3E88112".to_string()).expect("result ok"),
            7
        );
        assert_eq!(
            part_two("CE00C43D881120".to_string()).expect("result ok"),
            9
        );
        assert_eq!(part_two("D8005AC2A8F0".to_string()).expect("result ok"), 1);
        assert_eq!(part_two("F600BC2D8F".to_string()).expect("result ok"), 0);
        assert_eq!(part_two("9C005AC2F8F0".to_string()).expect("result ok"), 0);
        assert_eq!(
            part_two("9C0141080250320F1802104A08".to_string()).expect("result ok"),
            1
        );
    }

    #[test]
    fn part_two_my_input() {
        assert_eq!(part_two("C20D7900A012FB9DA43BA00B080310CE3643A0004362BC1B856E0144D234F43590698FF31D249F87B8BF1AD402389D29BA6ED6DCDEE59E6515880258E0040A7136712672454401A84CE65023D004E6A35E914BF744E4026BF006AA0008742985717440188AD0CE334D7700A4012D4D3AE002532F2349469100708010E8AD1020A10021B0623144A20042E18C5D88E6009CF42D972B004A633A6398CE9848039893F0650048D231EFE71E09CB4B4D4A00643E200816507A48D244A2659880C3F602E2080ADA700340099D0023AC400C30038C00C50025C00C6015AD004B95002C400A10038C00A30039C0086002B256294E0124FC47A0FC88ACE953802F2936C965D3005AC01792A2A4AC69C8C8CA49625B92B1D980553EE5287B3C9338D13C74402770803D06216C2A100760944D8200008545C8FB1EC80185945D9868913097CAB90010D382CA00E4739EDF7A2935FEB68802525D1794299199E100647253CE53A8017C9CF6B8573AB24008148804BB8100AA760088803F04E244480004323BC5C88F29C96318A2EA00829319856AD328C5394F599E7612789BC1DB000B90A480371993EA0090A4E35D45F24E35D45E8402E9D87FFE0D9C97ED2AF6C0D281F2CAF22F60014CC9F7B71098DFD025A3059200C8F801F094AB74D72FD870DE616A2E9802F800FACACA68B270A7F01F2B8A6FD6035004E054B1310064F28F1C00F9CFC775E87CF52ADC600AE003E32965D98A52969AF48F9E0C0179C8FE25D40149CC46C4F2FB97BF5A62ECE6008D0066A200D4538D911C401A87304E0B4E321005033A77800AB4EC1227609508A5F188691E3047830053401600043E2044E8AE0008443F84F1CE6B3F133005300101924B924899D1C0804B3B61D9AB479387651209AA7F3BC4A77DA6C519B9F2D75100017E1AB803F257895CBE3E2F3FDE014ABC".to_string()).expect("ok_result"),
         12301926782560);
    }
}
