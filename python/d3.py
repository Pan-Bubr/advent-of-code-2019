import sys
import math

def get_input():
    input_file = sys.argv[1]
    f = open(input_file)
    ret = f.read()
    f.close()
    return ret

class Pointer:
    def __init__(self, x, y):
        self.x = x
        self.y = y

class Instruction:
    def __init__(self, instruction):
        self.direction = instruction[0]
        self.steps = int(instruction[1:])

class Plane:
    manhattan = 99999
    def __init__(self, size):
        self.size = size
        self.plane = [[0 for y in range(size)] for x in range(size)]
    
    def print(self):
        for i in self.plane:
            print(i)

    def draw_cell(self, x, y, mark):
        
        if self.plane[y][x] != 0 and self.plane[y][x] != mark:
            manhattan = abs(x - self.size/2) + abs(y - self.size/2)
            if manhattan < self.manhattan:
                self.manhattan = manhattan
        self.plane[y][x] = mark
    
    def draw_line(self, i, mark):
        for _ in range(i.steps):
            if i.direction == "R":
                self.pointer.x += 1
            elif i.direction == "L":
                self.pointer.x -= 1
            elif i.direction == "U":
                self.pointer.y -= 1
            elif i.direction == "D":
                self.pointer.y += 1
            self.draw_cell(self.pointer.x, self.pointer.y, mark)
            

    def draw_wire(self, instructions, mark):
        self.pointer = Pointer(int(self.size/2), int(self.size/2))
        for i in instructions:
            self.draw_line(i, mark)



# Part 1
input = get_input()

instructions = input.split('\n')

instructionListA = [Instruction(i) for i in instructions[0].split(',')]
instructionListB = [Instruction(i) for i in instructions[1].split(',')]

plane = Plane(25000)

plane.draw_wire(instructionListA, "A")
plane.draw_wire(instructionListB, "B")

print("[Part One] Manhattan distance of closed wire crossing:", int(plane.manhattan))