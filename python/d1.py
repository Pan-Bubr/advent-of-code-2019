import sys
import math

def get_input():
    input_file = sys.argv[1]
    f = open(input_file)
    ret = f.read()
    f.close()
    return ret

def calculate_fuel(mass):
    fuel = float(mass) / 3.0
    fuel = math.floor(fuel)
    fuel -= 2
    return fuel if fuel > 0 else 0

input = get_input()

total_fuel = 0

for mass in input.splitlines():
    total_fuel += calculate_fuel(mass)

print("[Part One] Fuel needed: {total_fuel}".format(
    total_fuel=total_fuel
))

total_fuel = 0

for mass in input.splitlines():
    while int(mass) > 0:
        fuel = calculate_fuel(mass)
        mass = fuel
        total_fuel += fuel

print("[Part Two] Fuel needed: {total_fuel}".format(
    total_fuel=total_fuel
))