import numpy as np

def main():
    print("Solution for question 3\n-----------------------------")
    # setting up hyperparamters for multiple cases
    size = 10
    low = -50000
    high = 50000
    cases = [str(num) for num in np.random.uniform(low, high, size)]

    # annoying conversion from decimal to binary (not my algo)
    for i in range(len(cases)):
        c = cases[i]
        d = c.split(".")
        lb = bin(int(d[0]))
        b_idx = lb.find('b')
        lb = lb[b_idx+1:]
        rb = bin(int(d[1]))
        b_idx = rb.find('b')
        rb = rb[b_idx+1:]
        cases[i] = lb + "." + rb

    print("Beginning solution to 3.a")
    print("Converting Binary to Decimal code for the following cases:")
    for c in cases:
        print(c + '\n')
    bin_cases = []
    for i, c in enumerate(cases):
        print(f'\n---|Case: {i+1}, Binary number: {c} |---')
        bin_cases.append(convert_bin_to_dec(c))
        print(bin_cases[i])
    print("---DONE---\n")

    print("Beginning solution to 3.b")
    print("Converting Decimal to Binary code for the following cases:")
    # TODO: convert decimal to binary

def convert_bin_to_dec(bin_code: str) -> str:
    result = 0
    bins = bin_code.split(".")
    left_bits, lnum = bins[0], len(bins[0])
    right_bits, rnum = bins[1], len(bins[1])
    for i in range(lnum):
        if left_bits[i] == "1":
            result += 2**(lnum - (i+1))
    for i in range(rnum):
        if right_bits[i] == "1":
            result += 2**(-(i+1))
    return str(result)

def convert_dec_to_bin(dec_code):
    pass

if __name__ == '__main__':
    main()

