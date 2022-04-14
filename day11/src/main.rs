fn next_password(password: &mut String) {
    let bytes = unsafe { password.as_bytes_mut() };
    for i in (0..bytes.len()).rev() {
        match bytes[i] {
            b'z' => bytes[i] = b'a',
            _ => {
                bytes[i] += 1;
                break;
            }
        }
    }
}

// Wanted do to password validation in one pass over the bytes, hence the imperative style
fn valid_password(password: &str) -> bool {
    let (mut window, mut prev, mut grp_a) = ([0; 3], 0, 0);
    let (mut straight, mut iol, mut two_groups) = (false, false, false);
    let bytes = password.as_bytes();

    for &byte in bytes {
        if byte == b'i' || byte == b'o' || byte == b'l' {
            iol = true;
        }
        if byte == prev && grp_a != byte {
            if grp_a == 0 {
                grp_a = byte;
            } else {
                two_groups = true;
            }
        }
        prev = byte;

        window.rotate_left(1);
        window[2] = byte;
        if window[1].saturating_sub(window[0]) == 1 && window[2].saturating_sub(window[1]) == 1 {
            straight = true;
        }
    }

    !iol && two_groups && straight
}

fn next_valid_passport(password: &mut String) {
    loop {
        next_password(password);
        if valid_password(password) {
            break;
        }
    }
}

fn main() {
    let mut pwd = String::from("vzbxkghb");

    next_valid_passport(&mut pwd);
    println!("Part 1: {}", pwd);

    next_valid_passport(&mut pwd);
    println!("Part 2: {}", pwd);
}
