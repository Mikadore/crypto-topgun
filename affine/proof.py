import numpy as np

def caesar(c, n, m):
    return (c + n) % m

# Adjust for wanted search space as wanted
for m in range(100):
    L0 = np.matmul(np.array([[*range(1, m + 1)]]).T, np.array([[*range(m)]])) 
    for n in range(m):
        Ln = caesar(L0, n, m)
        
        for a in range(1, m ):
            for c in range(m):
                # -1 Because of zero based indexing
                assert Ln[a-1,c] == (a*c + n) % m 
        
print("Correct!")
