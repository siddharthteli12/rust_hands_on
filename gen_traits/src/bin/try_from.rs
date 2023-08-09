struct Num(i16);

#[derive(PartialEq, Debug)]
struct OddNum(i16);

impl TryFrom<Num> for OddNum {
    type Error = String;
    fn try_from(value: Num) -> Result<Self, Self::Error> {
        if value.0 % 2 == 0 {
            Err(String::from("Num should be odd"))
        } else {
            Ok(OddNum(value.0))
        }
    }
}

fn main() {
    let num_odd = Num(11);
    let num_even = Num(10);

    assert_eq!(11, OddNum::try_from(num_odd).unwrap().0);
    assert_eq!(
        Err(String::from("Num should be odd")),
        OddNum::try_from(num_even)
    );
}
