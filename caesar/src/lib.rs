/// Takes a string and encodes it using the caesar cipher (english ASCII letters only)
pub fn caesar_encode(s: &str, shift: u8) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (b'a' + (c as u8 - b'a' + shift) % 26) as char,
            'A'..='Z' => (b'A' + (c as u8 - b'A' + shift) % 26) as char,
            _ => c,
        })
        .collect()
}

/// Takes a string and decodes it using the caesar cipher (english ASCII letters only)
pub fn caesar_decode(s: &str, shift: u8) -> String {
    caesar_encode(s, 26 - shift % 26)
}

/// Takes a string and encodes it using the caesar cipher (czech alphabet, treating ch as 2 separate letters)
/// Note: for small shifts this is even less secure than the english caesar cipher, since neighboring letters
/// will often look similar ('c' vs 'č' for example)
pub fn cz_caesar_encode(s: &str, shift: u8) -> String {
    let mut lowercase = [
        'a', 'á', 'b', 'c', 'č', 'd', 'ď', 'e', 'é', 'ě', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l',
        'm', 'n', 'ň', 'o', 'ó', 'p', 'q', 'r', 'ř', 's', 'š', 't', 'ť', 'u', 'ú', 'ů', 'v', 'w',
        'x', 'y', 'ý', 'z', 'ž',
    ];
    let mut uppercase = [
        'A', 'Á', 'B', 'C', 'Č', 'D', 'Ď', 'E', 'É', 'Ě', 'F', 'G', 'H', 'I', 'Í', 'J', 'K', 'L',
        'M', 'N', 'Ň', 'O', 'Ó', 'P', 'Q', 'R', 'Ř', 'S', 'Š', 'T', 'Ť', 'U', 'Ú', 'Ů', 'V', 'W',
        'X', 'Y', 'Ý', 'Z', 'Ž',
    ];

    uppercase.rotate_left(shift as usize % 41);
    lowercase.rotate_left(shift as usize % 41);

    s.chars()
        .map(|c| {
            // We'll just let the compiler figure out an optimized version of this
            // In nomine Patris et Filii et Spiritus Sancti
            match c {
                'A' => uppercase[0],
                'Á' => uppercase[1],
                'B' => uppercase[2],
                'C' => uppercase[3],
                'Č' => uppercase[4],
                'D' => uppercase[5],
                'Ď' => uppercase[6],
                'E' => uppercase[7],
                'É' => uppercase[8],
                'Ě' => uppercase[9],
                'F' => uppercase[10],
                'G' => uppercase[11],
                'H' => uppercase[12],
                'I' => uppercase[13],
                'Í' => uppercase[14],
                'J' => uppercase[15],
                'K' => uppercase[16],
                'L' => uppercase[17],
                'M' => uppercase[18],
                'N' => uppercase[19],
                'Ň' => uppercase[20],
                'O' => uppercase[21],
                'Ó' => uppercase[22],
                'P' => uppercase[23],
                'Q' => uppercase[24],
                'R' => uppercase[25],
                'Ř' => uppercase[26],
                'S' => uppercase[27],
                'Š' => uppercase[28],
                'T' => uppercase[29],
                'Ť' => uppercase[30],
                'U' => uppercase[31],
                'Ú' => uppercase[32],
                'Ů' => uppercase[33],
                'V' => uppercase[34],
                'W' => uppercase[35],
                'X' => uppercase[36],
                'Y' => uppercase[37],
                'Ý' => uppercase[38],
                'Z' => uppercase[39],
                'Ž' => uppercase[40],
                // By Allah, this is maddening
                // Maybe I'm missing some easier/more obvious solution cus I haven't slept in 20 hours
                'a' => lowercase[0],
                'á' => lowercase[1],
                'b' => lowercase[2],
                'c' => lowercase[3],
                'č' => lowercase[4],
                'd' => lowercase[5],
                'ď' => lowercase[6],
                'e' => lowercase[7],
                'é' => lowercase[8],
                'ě' => lowercase[9],
                'f' => lowercase[10],
                'g' => lowercase[11],
                'h' => lowercase[12],
                'i' => lowercase[13],
                'í' => lowercase[14],
                'j' => lowercase[15],
                'k' => lowercase[16],
                'l' => lowercase[17],
                'm' => lowercase[18],
                'n' => lowercase[19],
                'ň' => lowercase[20],
                'o' => lowercase[21],
                'ó' => lowercase[22],
                'p' => lowercase[23],
                'q' => lowercase[24],
                'r' => lowercase[25],
                'ř' => lowercase[26],
                's' => lowercase[27],
                'š' => lowercase[28],
                't' => lowercase[29],
                'ť' => lowercase[30],
                'u' => lowercase[31],
                'ú' => lowercase[32],
                'ů' => lowercase[33],
                'v' => lowercase[34],
                'w' => lowercase[35],
                'x' => lowercase[36],
                'y' => lowercase[37],
                'ý' => lowercase[38],
                'z' => lowercase[39],
                'ž' => lowercase[40],
                _ => c
            }
        })
        .collect()
}

/// Takes a string and decodes it using the caesar cipher (czech alphabet, treating ch as 2 separate letters)
pub fn cz_caesar_decode(s: &str, shift: u8) -> String {
    cz_caesar_encode(s, 41 - shift % 41)
}

#[cfg(test)]
mod test;