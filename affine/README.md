# Affine cipher

## Definition

The affine cipher is another example of a simple substitution cipher.
It is a more generalized version of the [caesar cipher](../caesar).
Whereas in the caesar cipher we applied the function <!-- $E_n(c) = c + n \bmod 26$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=E_n(c)%20%3D%20c%20%2B%20n%20%5Cbmod%2026">,
in the affine cipher we use the following pair of functions for encoding and decoding:
<!-- $$
\begin{aligned}
E_{a,n}(c) &= (ac + n) \bmod m \\
D_{a,b}(c) &= a^{-1}(c - b) \bmod m
\end{aligned}
$$ --> 

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=%5Cbegin%7Baligned%7D%0AE_%7Ba%2Cn%7D(c)%20%26%3D%20(ac%20%2B%20n)%20%5Cbmod%20m%20%5C%5C%0AD_%7Ba%2Cb%7D(c)%20%26%3D%20a%5E%7B-1%7D(c%20-%20b)%20%5Cbmod%20m%0A%5Cend%7Baligned%7D"></div>

There are some things to unpack here. Firstly, 
I used `m` instead of `26`. Sine generally ciphers are defined
for any alphabet, which might contain more or less than 26 letters,
we can talk about the cipher in a general way without restricting us
to for example lowercase english letters only. So, `m` is the length 
of our alphabet (e.g. there are 26 english lowercase letters), and c 
is a letter in our alphabet 
(more accurately, it encodes the letter `alphabet[c]`).

A lot of cryptography is at least in part based in modular arithmetic.
It is helpful, if not required to have at least a basic understanding
of what this area of math is all about, however, this lies outside
the scope of this project. However, I'm going to try and explain the
mathematics behind every cipher we're going to talk about, in part 
to understand it myself.

### Modular multiplicative inverse

In the definition of our decryption function, we used the funny notation <!-- $a^{-1}$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=a%5E%7B-1%7D"> In this context, 
this does not mean <!-- $\frac{1}{a}$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=%5Cfrac%7B1%7D%7Ba%7D">, but signifies the 
[modular multiplicative inverse](https://en.wikipedia.org/wiki/Modular_multiplicative_inverse)
of `a` with respect to `m`.
Simply, <!-- $a^{-1}$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=a%5E%7B-1%7D"> is an integer, such that <!-- $a^{-1}a \bmod m = 1$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=a%5E%7B-1%7Da%20%5Cbmod%20m%20%3D%201">


## Visual intuition

Note: this is my own analysis, expect messy and incomple observations.
If you notice anything, feel free to send a PR ;)

### Approach

Whereas in the caesar cipher, we rotated the alphabet by `n` to get 
a mapping:

|0|1|2|
|-|-|-|
|1|2|0|

Let's rediscover the affine cipher by expanding this thinking into 2 dimensions:

|a|0|1|2|3|4|
|-|-|-|-|-|-|
|1|0|1|2|3|4|
|2|0|2|4|6|8|
|3|0|3|6|9|12|
|4|0|4|8|12|16|
|5|0|5|10|15|20|

Here, we have a table where each row represents a choice of a 
(in the interval `[1;m]`)., and each column represents the product `ac`,
essentially a multiplication table.

Now, to get the actual cipher's mapping we only need to 
apply our addition of `n` mod `m`. Doing this with `n = 1` 
for the above example:

|a|0|1|2|3|4|
|-|-|-|-|-|-|
|1|1|2|3|4|0|
|2|1|3|0|2|1|
|3|1|4|2|0|3|
|4|1|0|4|3|2|
|5|1|1|1|1|1|

Note: just taking the `%m` ofour first table gives us the affine cipher for n=0.

Mathematically, this is equivalent a simple matrix product and 
element-wise function application:
<!-- $$
L_n(m)=C_n\left(
\begin{bmatrix}
1 \\
\vdots \\
m
\end{bmatrix}
\times
\begin{bmatrix}
0 & \cdots & m-1
\end{bmatrix}
\right)
$$ --> 

<div align="center"><img style="background: white;" src="https://render.githubusercontent.com/render/math?math=L_n(m)%3DC_n%5Cleft(%0A%5Cbegin%7Bbmatrix%7D%0A1%20%5C%5C%0A%5Cvdots%20%5C%5C%0Am%0A%5Cend%7Bbmatrix%7D%0A%5Ctimes%0A%5Cbegin%7Bbmatrix%7D%0A0%20%26%20%5Ccdots%20%26%20m-1%0A%5Cend%7Bbmatrix%7D%0A%5Cright)"></div>
Where <!-- $C_n$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=C_n"> is the caesar cipher for `n`. 
Let's call it `L` for lookup.
(Actually, in the definition above, `L` is a function whose value is the lookup table for any alphabet `m`, but we can be a bit loose
in terminology here).

This is interesting, since we've expressed the affine cipher
as a series of caesar ciphers. More exactly, the encoding function
for the affine cipher is *equivalent* to <!-- $E_{a,n}(c)=L_n(m)_{a-1,c}$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=E_%7Ba%2Cn%7D(c)%3DL_n(m)_%7Ba-1%2Cc%7D"> (We substract 1 from `a` because we want to use zero based indexing consistently)

Essentially, in code, given we have computed a matrix `L` for the given
shift `n`, the encoding of any letter `c` is precisely:
```py
L[a-1][c]
```
Where `a` is the chosen parameter. We confirm this experimentally 
[here](proof.py).

Note:

This approach only looks at affine ciphers with the same parameter `n`.
If we wanted to encompass the whole set of possible affine ciphers over
the alphabet `m`, we would need a 3D table/matrix, with the z axis 
representing the shift. Similarly, to think about all caesar ciphers
we would need a 2D table/matrix, with the rows representing every 
possinble parameter `n`. However, I do believe this approach has merit,
as it essentially *extends* any caesar cipher to the affine cipher.

If we think about the caesar cipher as mechanically turning a wheel,
this in some sense is the physical equivalent for the affine cipher:


### Observations

The row `m` will always be composed of all `n`s. This is because in
the initial step, we essentially generate multiples of `m`. 
So, we get <!-- $\begin{bmatrix}0&m&\cdots&m(m-1)\end{bmatrix}$ --> <img style="transform: translateY(0.1em); background: white;" src="https://render.githubusercontent.com/render/math?math=%5Cbegin%7Bbmatrix%7D0%26m%26%5Ccdots%26m(m-1)%5Cend%7Bbmatrix%7D">, which 
in the next step is reduced to `0 + n`. Similarly, had we included a 
row 0, it would too end up consisting just of `n`.

The above example has been chosen nicely, such that each row,
except for row 5 is a valid affine cipher over `m`. Valid just means
that the encoding function  $E_{a,n}(c)$ is 
