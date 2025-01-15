import numpy as np

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

def create_cases(size, low, high):
    dec_code_cases = [str(num) for num in np.random.uniform(low, high, size)]
    bin_code_cases = []

    # annoying conversion from decimal to binary (not my algo)
    for i in range(len(dec_code_cases)):
        c = dec_code_cases[i]
        d = c.split(".")

        lb = bin(int(d[0]))
        b_idx = lb.find('b')
        lb = lb[b_idx+1:]

        rb = bin(int(d[1]))
        b_idx = rb.find('b')
        rb = rb[b_idx+1:]

        bin_code_cases.append(lb + "." + rb)

    return bin_code_cases, dec_code_cases

def main():
    # setting up hyperparamters for multiple cases
    size = 10
    low = -50000
    high = 50000
    bin_code_cases, dec_code_cases = create_cases(size, low, high)
    print("Solution for question 3\n-----------------------------")
    print("Beginning solution to 3.a")
    print("Converting Binary to Decimal code for the following cases:")
    for c in bin_code_cases:
        print(c + '\n')
    dec_from_bin = []
    for i, c in enumerate(bin_code_cases):
        print(f'\n---|Case: {i+1}, Binary number: {c} |---')
        dec_from_bin.append(convert_bin_to_dec(c))
        print(dec_from_bin[i])
    print("---DONE---\n")

    print("Beginning solution to 3.b")
    print("Converting Decimal to Binary code for the following cases:")
    # TODO: convert decimal to binary

if __name__ == '__main__':
    main()

