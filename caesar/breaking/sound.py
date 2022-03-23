contents = open("audio.enc", "rb").read()

def decode(message: list[int], shift: int):
    return [(b + shift) % 256 for b in message]

for key in range(256):
    attempt = decode(contents, key)

    # To see all the guesses:
    # print(f"{str(bytes(attempt[0:4]))}...{str(bytes(attempt[8:12]))}")
    
    # The binary layout of a WAVE file is:
    #   Bytes           Value
    #   0-3	            "RIFF" (ASCII)
    #   4-7	            File size (le integer)
    #   8-11	        "WAVE" (ASCII)
    #
    # So, we decode the message using `key`, and see if the file magic matches.
    # - Simple as that!
    # Use any knowledge of the message to your advantage! After all, knowledge is power.
    if bytes(attempt[0:4]) == b"RIFF" and bytes(attempt[8:12]) == b"WAVE":
        print(f"Correct signature at k = {key}")