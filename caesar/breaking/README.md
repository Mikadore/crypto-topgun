# Breaking the caesar cipher

## General

There are fundamentally two different scenarios where one might attempt to
decode the caesar cipher without knowledge of the shift parameter.

We're going to examine the case, where we know some variation of the caesar cipher has been used.
For the case where we don't, please refer to the section about frequency analysis. (TODO)

## Approach

Given the caesar cipher's simplicity, we're going to brute force our way to reveal the secret message we've been given.
The search space for the caesar cipher is incredibly small. Assuming we're dealing only with the english alphabet,
there are only 25 different possible decodings of a message. We might add some bits of entropy by considering different
schemas, such as an alphabet where capital and lower case letters are interchanged, adding 51 new possible ways to decode a message.
In the case of byte level shifting, there's 255 possible decodings. This is a bigger number, but even if we assume it takes a human
10 seconds to check each possibility, he'd still get the right answer well within an hour. Human brute forcing should only become infeasible
once we're dealing with very exotic variations of the caesar cipher. That's not to say we can't automate the guessing with some assumptions
about the given message even in the simple(r) case.

## Example

Imagine you're a british navy officer in 1917, and have been handed this [telegram](telegram.enc):
```
PXBGMXGWMHUXZBGHGMAXYBKLMHYYXUKNTKRNGKXLMKBVMXWLNUFTKBGXPTKYTKXPXLATEEXGWXTOHKBGLIBMXHYMABLMHDXXIMAXNGBMXWLMTMXLHYTFXKBVTGXNMKTEBGMAXXOXGMHYMABLGHMLNVVXXWBGZPXFTDXFXQBVHTIKHIHLTEHYTEEBTGVXHGMAXYHEEHPBGZUTLBLFTDXPTKMHZXMAXKFTDXIXTVXMHZXMAXKZXGXKHNLYBGTGVBTELNIIHKMTGWTGNGWXKLMTGWBGZHGHNKITKMMATMFXQBVHBLMHKXVHGJNXKMAXEHLMMXKKBMHKRBGMXQTLGXPFXQBVHTGWTKBSHGTMAXLXMMEXFXGMBGWXMTBEBLEXYMMHRHNRHNPBEEBGYHKFMAXIKXLBWXGMHYMAXTUHOXFHLMLXVKXMERTLLHHGTLMAXHNMUKXTDHYPTKPBMAMAXNGBMXWLMTMXLHYTFXKBVTBLVXKMTBGTGWTWWMAXLNZZXLMBHGMATMAXLAHNEWHGABLHPGBGBMBTMBOXBGOBMXCTITGMHBFFXWBTMXTWAXKXGVXTGWTMMAXLTFXMBFXFXWBTMXUXMPXXGCTITGTGWHNKLXEOXLIEXTLXVTEEMAXIKXLBWXGMLTMMXGMBHGMHMAXYTVMMATMMAXKNMAEXLLXFIEHRFXGMHYHNKLNUFTKBGXLGHPHYYXKLMAXIKHLIXVMHYVHFIXEEBGZXGZETGWBGTYXPFHGMALMHFTDXIXTVXLBZGXWSBFFXKFTGG
```

In general, when breaking a cipher we must consider what we expect to find. In this case, we know this is some sort of corespondence,
very probably consisting of letters. 

In my terminal, I'm going to run the following code (I use xonsh, it allows me to essentially mix shell and python):
```py
for i in range(26):
    # clear the screen
    clear
    # show which key is being used for the current iteration
    print(f"k = {i}")
    # decode the telegram using caesar's and print the output
    cargo run -q --manifest-path ../Cargo.toml -- -i telegram.enc -k @(i) -d
    # wait for user input until next iteration
    input()
```

After several iterations, we get this output:
```
k = 19
WEINTENDTOBEGINONTHEFIRSTOFFEBRUARYUNRESTRICTEDSUBMARINEWARFAREWESHALLENDEAVORINSPITEOFTHISTOKEEPTHEUNITEDSTATESOFAMERICANEUTRALINTHEEVENTOFTHISNOTSUCCEEDINGWEMAKEMEXICOAPROPOSALOFALLIANCEONTHEFOLLOWINGBASISMAKEWARTOGETHERMAKEPEACETOGETHERGENEROUSFINANCIALSUPPORTANDANUNDERSTANDINGONOURPARTTHATMEXICOISTORECONQUERTHELOSTTERRITORYINTEXASNEWMEXICOANDARIZONATHESETTLEMENTINDETAILISLEFTTOYOUYOUWILLINFORMTHEPRESIDENTOFTHEABOVEMOSTSECRETLYASSOONASTHEOUTBREAKOFWARWITHTHEUNITEDSTATESOFAMERICAISCERTAINANDADDTHESUGGESTIONTHATHESHOULDONHISOWNINITIATIVEINVITEJAPANTOIMMEDIATEADHERENCEANDATTHESAMETIMEMEDIATEBETWEENJAPANANDOURSELVESPLEASECALLTHEPRESIDENTSATTENTIONTOTHEFACTTHATTHERUTHLESSEMPLOYMENTOFOURSUBMARINESNOWOFFERSTHEPROSPECTOFCOMPELLINGENGLANDINAFEWMONTHSTOMAKEPEACESIGNEDZIMMERMANNW
```
Which is clearly english text. After reformatting the text, we can read it with ease:

> We intend to begin on the first of February unrestricted submarine warfare. We shall endeavor in spite of this to keep the United States of America neutral. In the event of this not succeeding, we make Mexico a proposal of alliance on the following basis: make war together, make peace together, generous financial support and an understanding on our part that Mexico is to reconquer the lost territory in Texas, New Mexico, and Arizona. The settlement in detail is left to you. You will inform the President of the above most secretly as soon as the outbreak of war with the United States of America is certain, and add the suggestion that he should, on his own initiative, invite Japan to immediate adherence and at the same time mediate between Japan and ourselves. Please call the President's attention to the fact that the ruthless employment of our submarines now offers the prospect of compelling England in a few months to make peace.
> Signed, ZIMMERMANN

This is in fact the famous [Zimmerman Telegram](https://en.wikipedia.org/wiki/Zimmermann_Telegram). 
Now, the actual telegram used a more complex code (called Code 13040), but we can use this contrived example
to get a sense of how a simple cipher might work, or rather, fail.

## Exercise for the reader

Knowing all of this, try decoding a simple message on your own.
I've encoded the [audio.enc](audio.enc) file using the `-b` option in the CLI.
The original format was a WAVE file playing a sine wave. Can you find the correct
`-k` parameter to get back `sine.wav`? Look at [sound.py](sound.py) for a solution.
