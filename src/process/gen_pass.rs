use rand::seq::{IndexedRandom, SliceRandom};
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_,.?";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("大写字符不会空"));
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("小写字符不会空"));
    }
    if numbers {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("数字字符不会空"));
    }
    if symbols {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("特殊字符不会空"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).expect("在这种情况下，字符不会空");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;
    println!("{}", password);

    let estimate = zxcvbn(&password, &[]);
    eprintln!("密码强度: {}", estimate.score());

    Ok(())
}
