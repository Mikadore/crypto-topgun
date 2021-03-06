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
        "Str?? prst skrz krk",
        "Nech?? ji?? h??????n?? saxofony ????bl?? rozezvu???? s???? ??d??sn??mi t??ny waltzu, tanga a quickstepu.",
        "P????li?? ??lu??ou??k?? k???? ??p??l ????belsk?? ??dy.",
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
        cz_caesar_encode("a??bc??d??e????fghi??jklmn??o??pqr??s??t??u????vwxy??z??", 4),
        "??d??e????fghi??jklmn??o??pqr??s??t??u????vwxy??z??a??bc"
    );
    assert_eq!(
        cz_caesar_encode("A??BC??D??E????FGHI??JKLMN??O??PQR??S??T??U????VWXY??Z??", 4),
        "??D??E????FGHI??JKLMN??O??PQR??S??T??U????VWXY??Z??A??BC"
    );

    let lowercase_alphabet = [
        'a', '??', 'b', 'c', '??', 'd', '??', 'e', '??', '??', 'f', 'g', 'h', 'i', '??', 'j', 'k', 'l',
        'm', 'n', '??', 'o', '??', 'p', 'q', 'r', '??', 's', '??', 't', '??', 'u', '??', '??', 'v', 'w',
        'x', 'y', '??', 'z', '??',
    ];
    let uppercase_alphabet = [
        'A', '??', 'B', 'C', '??', 'D', '??', 'E', '??', '??', 'F', 'G', 'H', 'I', '??', 'J', 'K', 'L',
        'M', 'N', '??', 'O', '??', 'P', 'Q', 'R', '??', 'S', '??', 'T', '??', 'U', '??', '??', 'V', 'W',
        'X', 'Y', '??', 'Z', '??',
    ];

    let lowercase_string = "a??bc??d??e????fghi??jklmn??o??pqr??s??t??u????vwxy??z??";
    let uppercase_string = "A??BC??D??E????FGHI??JKLMN??O??PQR??S??T??U????VWXY??Z??";

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
