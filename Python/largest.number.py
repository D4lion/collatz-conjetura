from collatz import collatz as cll


def largestNumber(ls):
    maxnum = 0
    for i in ls:
        if i > maxnum:
            maxnum = i
    return maxnum


if __name__ == '__main__':
    print(largestNumber(cll(27)))  # You can change 27 for any number
