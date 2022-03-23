# Caesar cipher
The caesar cipher is a simple substitution cipher.
It "works" by shifting the input text alphabetically by N places.
E.g., if N=2 then A will be replaced with C, B with D, C with E, etc.

|A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|
|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|
|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z|A|B|

The reason I put "works" in quotes is that this is an example 
of a historic cipher - because of it's simplicity it's not secure at 
all against a knowledgeable attacker. In fact, a special case
of the caesar cipher, when N=13, called ROT13 is often used as 
a joke example for bad encryption.

## Definition

Given we have our ciphertext of english letters, where each letter coresponds to its position in the alphabet (that is in the range [0;25]), we apply a transformation function <!-- $E_n(c) = c + n \bmod 26$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=E_n(c)%20%3D%20c%20%2B%20n%20%5Cbmod%2026">, where n is the shift for each  letter c of the ciphertext.

To decrypt we reverse this process: <!-- $D_n(c) = c - n \bmod 26$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=D_n(c)%20%3D%20c%20-%20n%20%5Cbmod%2026"> (Given we use the floored definition of modulo, in my implementation I use a handy property of the caesar cipher to circumvent this).

## Properties

The most important property of the caesar cipher 
from a security perspective is, that it's the same character in
the input will always map to the same character in the output.

For the following properties, also check out the [unit tests](src/test.rs) that they work!

Since the actual mapping of each character uses modulo 26, any 
shift larger than 26 is equivalent:
<!-- $$
E_n(c) = E_{n\bmod26}
$$ --> 

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=E_n(c)%20%3D%20E_%7Bn%5Cbmod26%7D"></div>

For the same reason, any shift which is a multiple of 26 (including 0) 
is equivalent to the identity function:
<!-- $$
E_0(c) = E_{26n}(c) = c
$$ --> 

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=E_0(c)%20%3D%20E_%7B26n%7D(c)%20%3D%20c"></div>

ROT13, i.e. a shift of 13 is special, because encoding and decoding are 
the same operation:
<!-- $$
E_{13}(c) = D_{13}(c); E_{13}(E_{13}(c)) = c
$$ --> 

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=E_%7B13%7D(c)%20%3D%20D_%7B13%7D(c)%3B%20E_%7B13%7D(E_%7B13%7D(c))%20%3D%20c"></div>
(Because shifting by 13 places twice is equivalent to shifting by 26 places, which is just identity)

And lastly, given a shift n, we can use the encoding function with shift 26-n for decoding:
<!-- $$
E_{26 - n}(c) = D_n(c)
$$ --> 

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=E_%7B26%20-%20n%7D(c)%20%3D%20D_n(c)"></div>
(We essentially shift by n in the opposite direction, mathematically the statement <!-- $c + 26 - n \equiv c - n \mod 26$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=c%20%2B%2026%20-%20n%20%5Cequiv%20c%20-%20n%20%5Cmod%2026"> is trivially true)
This is helpful, as we can avoid dealing with negative shifts by expressing them as equivalent positive shifts.

## Sidenote

So far, we've describing the algorithm in numerical terms - adding/substracting modulo 26.
Equivalently, we can think of it more visually, as rotating an array of our sorted alphabet by N,
generating a lookup table. So, take:
```rust
let abc = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
let mut caesar2 = abc.clone();
caesar2.rotate_left(2);
```
For any letter at index `i`, we can now use `caesar2[i]` to get the corresponding letter
in the caesar encoding with N=2;

## Extending to arbitrary alphabets

All the examples so far were only concerned with the uppercase english alphabet.
However, we can define the caesar cipher for any arbitrary alphabet (ordered collection of symbols)
by assigning a number to each symbol (e.g. index in an array), and then applying the usual math.

Similarly, we can also define an encoding scheme for based on the caesar cipher to fit our needs.
In my implementation, for any given ciphertext, the encoded output only changes supported letters,
shifting them according to the caesar cipher. Uppercase and lowercase letters are essentially treated
as different alphabets. With N=2, `'a'` will be mapped to `c`, and `'A'` to `'C'`. 

## CLI

I've created a simple CLI for the caesar encoding/decoding:
```
caesar 
caesar is a cli utility for encoding/decoding text using the caesar cipher

USAGE:
    caesar [OPTIONS] --key <Key> <--decode|--encode>

OPTIONS:
    -c, --czech                Use the czech alphabet
    -d, --decode               Decode the input
    -e, --encode               Encode the input
    -h, --help                 Print help information
    -i, --in <Input File>      Use file as input
    -k, --key <Key>            The shift amount for the caesar cipher
    -o, --out <Output File>    Write output to file instead of stdout

The program encodes/decodes valid utf8 using the caesar cipher.You need to provide a key, i.e. the
amount to shift by.By default, the program reads from stdin and writes to stdout.Any non-alphabetic
character will not be changed by either decoding or encoding.Lowercase and uppercase characters are
encoded separately, i.e. with key = 1,'A' becomes 'B' and 'a' becomes 'b'.Use -c to enable the czech
alphabet. This will use czech letters in the order 'AÁBCČDĎEÉĚFGHIÍJKLMNŇOÓPQRŘSŠTŤUÚŮVWXYÝZŽ'(as
defined in unicode, same order for lowercase), exluding the letter 'ch'
```

Try it! Example:
```
$ cargo run -- --in samples/test.cz -k 13 -ce | diff - samples/test.cz.13
```
```
$ cargo run -- --in samples/test.en.13 -k 13 -d
```
```
$ printf "\0\1\2\3\4\5" | cargo run -- -bk 1 | hexdump -C
```