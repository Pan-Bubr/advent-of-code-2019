import sys
import math

def get_input():
    input_file = sys.argv[1]
    f = open(input_file)
    ret = f.read()
    f.close()
    return ret

def calculate(orders):
    i = 0
    while i < len(orders):
        if orders[i] == 1:
            first_digit_address = orders[i+1]
            second_digit_address = orders[i+2]
            destination = orders[i+3]
            orders[destination] = orders[first_digit_address] + orders[second_digit_address]
            print("A", first_digit_address, second_digit_address, orders[destination])
            i += 4
        elif orders[i] == 2:
            first_digit_address = orders[i+1]
            second_digit_address = orders[i+2]
            destination = orders[i+3]
            i += 4
            orders[destination] = orders[first_digit_address] * orders[second_digit_address]
            print("M", first_digit_address, second_digit_address, orders[destination])
        else:
            break
    return orders[0]

# Part 1
input = get_input()
orders = [int(i) for i in input.split(',')]

orders[1] = 1
orders[2] = 12
solution = calculate(orders)

print("[Part One] State of the computer: {solution}".format(
    solution=solution
))

# for i in range(100):
#     for j in range(100):
#         input = get_input()
#         orders = [int(i) for i in input.split(',')]
        
#         orders[1] = i
#         orders[2] = j
#         if (calculate(orders) == 19690720):
#             print("[Part Two] State of the computer: noun={noun} verb={verb}, solution={solution}".format(
#                 noun=i, verb=j, solution=100*i+j
#             ))
#             break