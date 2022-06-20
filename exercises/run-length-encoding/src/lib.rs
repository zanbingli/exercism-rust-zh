pub fn encode(source: &str) -> String {
    let (a, b, mut c) =
        source.chars().fold((0, None, vec![]), |mut ve, x| {
            if ve.0 == 0 {
                ve.0 = 1;
                ve.1 = Some(x);
            } else if ve.1.is_some() && ve.1.unwrap() == x {
                ve.0 += 1;
            } else {
                if ve.0 == 1 {
                    ve.2.push(format!("{}", ve.1.unwrap()));
                } else {
                    ve.2.push(format!("{}{}", ve.0, ve.1.unwrap()));
                }
                ve.0 = 1;
                ve.1 = Some(x);
            }
            ve
        });
    //收尾
    if a == 1 {
        c.push(format!("{}", b.unwrap()));
    } else if a > 1 {
        c.push(format!("{}{}", a, b.unwrap()));
    }
    c.iter().flat_map(|x| x.chars()).collect()
}

pub fn decode(source: &str) -> String {
    let (_a, c) =
        source.chars().fold((0, String::new()), |mut ve, x| {
            if x.is_digit(10) {
                ve.0 = ve.0 * 10 + x.to_digit(10).unwrap();
            } else {
                if ve.0 == 0 {
                    ve.1.push(x);
                } else {
                    ve.1 += (0..ve.0).map(|_| x).collect::<String>().as_str();
                }
                ve.0 = 0;
            }
            ve
        });
    c
}
