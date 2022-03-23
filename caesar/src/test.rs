use super::{caesar_decode, caesar_encode, caesar_rot, cz_caesar_decode, cz_caesar_encode};

#[test]
fn identity_properties() {
    let texts = [
        "Could've used some fancy fuzzer",
        "Or really just the rand crate",
        "But why put in so much effort",
        "If I can write down test cases manually",
        "the quick brown fox jumps over the lazy dog",
        "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG",
    ];

    for text in texts {
        assert_eq!(
            caesar_encode(text, 0),
            text,
            "A 0-shift must be equivalent to identity"
        );
        assert_eq!(
            caesar_decode(text, 0),
            text,
            "A 0-shift must be equivalent to identity"
        );
        assert_eq!(
            caesar_encode(text, 26),
            text,
            "A 26-shift must be equivalent to identity"
        );
        assert_eq!(
            caesar_decode(text, 26),
            text,
            "A 26-shift must be equivalent to identity"
        );
        assert_eq!(
            caesar_encode(&caesar_encode(text, 13), 13),
            text,
            "ROT13(ROT13(x)) = x"
        );
        for shift in 1..=4 * 26 {
            assert_eq!(
                caesar_decode(&caesar_encode(text, shift), shift),
                text,
                "Decoding the encoded string must result in the original"
            );
            assert_eq!(
                caesar_encode(text, shift),
                caesar_encode(text, shift + 26),
                "Shifts are observable only in mod 26"
            );
            assert_eq!(
                caesar_encode(text, shift),
                caesar_encode(text, shift + 52),
                "Shifts are observable only in mod 26"
            );
            assert_eq!(
                caesar_encode(text, shift),
                caesar_encode(text, shift + 78),
                "Shifts are observable only in mod 26"
            );
        }
    }
}

#[test]
fn abc_shift() {
    assert_eq!(
        caesar_encode("abcdefghijklmnopqrstuvwxyz", 1),
        "bcdefghijklmnopqrstuvwxyza"
    );
    assert_eq!(
        caesar_encode("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 1),
        "BCDEFGHIJKLMNOPQRSTUVWXYZA"
    );

    let lowercase_alphabet = b"abcdefghijklmnopqrstuvwxyz";
    let uppercase_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let lowercase_string = std::str::from_utf8(lowercase_alphabet).unwrap();
    let uppercase_string = std::str::from_utf8(uppercase_alphabet).unwrap();

    for shift in 0..26 {
        let mut low = lowercase_alphabet.clone();
        low.rotate_left(shift);
        let low = std::str::from_utf8(&low).unwrap();
        let mut upp = uppercase_alphabet.clone();
        upp.rotate_left(shift);
        let upp = std::str::from_utf8(&upp).unwrap();

        assert_eq!(
            caesar_encode(lowercase_string, shift as u8),
            low,
            "Shift by {0} == Rotation {0}",
            shift
        );
        assert_eq!(
            caesar_encode(uppercase_string, shift as u8),
            upp,
            "Shift by {0} == Rotation {0}",
            shift
        );
    }
}

#[test]
fn cz_identity_properties() {
    let texts = [
        "Strč prst skrz krk",
        "Nechť již hříšné saxofony ďáblů rozezvučí síň úděsnými tóny waltzu, tanga a quickstepu.",
        "Příliš žluťoučký kůň úpěl ďábelské ódy.",
        "the quick brown fox jumps over the lazy dog",
        "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG",
    ];

    for text in texts {
        assert_eq!(
            cz_caesar_encode(text, 0),
            text,
            "A 0-shift must be equivalent to identity"
        );
        assert_eq!(
            cz_caesar_decode(text, 0),
            text,
            "A 0-shift must be equivalent to identity"
        );
        assert_eq!(
            cz_caesar_encode(text, 41),
            text,
            "A 41-shift must be equivalent to identity"
        );
        assert_eq!(
            cz_caesar_decode(text, 41),
            text,
            "A 41-shift must be equivalent to identity"
        );
        for shift in 1..=3 * 41 {
            assert_eq!(
                cz_caesar_decode(&cz_caesar_encode(text, shift), shift),
                text,
                "Decoding the encoded string must result in the original"
            );
            assert_eq!(
                cz_caesar_encode(text, shift),
                cz_caesar_encode(text, shift + 41),
                "Shifts are observable only in mod 41"
            );
        }
    }
}

#[test]
fn cz_abc_shift() {
    assert_eq!(
        cz_caesar_encode("aábcčdďeéěfghiíjklmnňoópqrřsštťuúůvwxyýzž", 4),
        "čdďeéěfghiíjklmnňoópqrřsštťuúůvwxyýzžaábc"
    );
    assert_eq!(
        cz_caesar_encode("AÁBCČDĎEÉĚFGHIÍJKLMNŇOÓPQRŘSŠTŤUÚŮVWXYÝZŽ", 4),
        "ČDĎEÉĚFGHIÍJKLMNŇOÓPQRŘSŠTŤUÚŮVWXYÝZŽAÁBC"
    );

    let lowercase_alphabet = [
        'a', 'á', 'b', 'c', 'č', 'd', 'ď', 'e', 'é', 'ě', 'f', 'g', 'h', 'i', 'í', 'j', 'k', 'l',
        'm', 'n', 'ň', 'o', 'ó', 'p', 'q', 'r', 'ř', 's', 'š', 't', 'ť', 'u', 'ú', 'ů', 'v', 'w',
        'x', 'y', 'ý', 'z', 'ž',
    ];
    let uppercase_alphabet = [
        'A', 'Á', 'B', 'C', 'Č', 'D', 'Ď', 'E', 'É', 'Ě', 'F', 'G', 'H', 'I', 'Í', 'J', 'K', 'L',
        'M', 'N', 'Ň', 'O', 'Ó', 'P', 'Q', 'R', 'Ř', 'S', 'Š', 'T', 'Ť', 'U', 'Ú', 'Ů', 'V', 'W',
        'X', 'Y', 'Ý', 'Z', 'Ž',
    ];

    let lowercase_string = "aábcčdďeéěfghiíjklmnňoópqrřsštťuúůvwxyýzž";
    let uppercase_string = "AÁBCČDĎEÉĚFGHIÍJKLMNŇOÓPQRŘSŠTŤUÚŮVWXYÝZŽ";

    for shift in 0..=41 {
        let mut low = lowercase_alphabet.clone();
        low.rotate_left(shift);
        let low: String = low.iter().collect();
        let mut upp = uppercase_alphabet.clone();
        upp.rotate_left(shift);
        let upp: String = upp.iter().collect();

        assert_eq!(
            cz_caesar_encode(lowercase_string, shift as u8),
            low,
            "Shift by {0} == Rotation {0}",
            shift
        );
        assert_eq!(
            cz_caesar_encode(uppercase_string, shift as u8),
            upp,
            "Shift by {0} == Rotation {0}",
            shift
        );
    }
}

#[test]
fn rot_test() {
    let caesar = b"abcdef";

    assert_eq!(
        caesar_encode(std::str::from_utf8(caesar).unwrap(), 13).as_bytes(),
        caesar_rot(caesar, 13),
        "Due to ascii, there result of these operations in this case should be the same"
    );

    let iota = (0..=u8::MAX).collect::<Vec<u8>>();
    let shifted = {
        let mut copy = iota.clone();
        copy.rotate_right(1);
        copy
    };

    assert_eq!(caesar_rot(&iota, u8::MAX), shifted);
    assert_eq!(caesar_rot(&shifted, 1), iota);
}
