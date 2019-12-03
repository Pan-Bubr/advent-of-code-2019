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
    def value(self):
        return "{x}/{y}".format(x=self.x, y=self.y)

class Instruction:
    def __init__(self, instruction):
        self.direction = instruction[0]
        self.steps = int(instruction[1:])

class Step:
    def __init__(self, x, y, A_steps, B_steps):
        self.x = int(x)
        self.y = int(y)
        self.A_steps = A_steps
        self.B_steps = B_steps
    
    def manhattan(self):
        return abs(self.x) + abs(self.y)
    
    def step_count(self):
        return self.A_steps + self.B_steps

class Plane:
    steps = 0
    crossings = []
    def __init__(self):
        self.plane = {}
    
    def print(self):
        for i in self.plane:
            print(i)

    def draw_cell(self, mark):
        value = self.pointer.value()
        self.steps += 1
        if mark == "A":
            if value not in self.plane:
                self.plane[value] = Step(self.pointer.x, self.pointer.y, self.steps, 0)
        elif mark == "B":
            if value not in self.plane:
                self.plane[value] = Step(self.pointer.x, self.pointer.y, 0, self.steps)
            elif self.plane[value].A_steps != 0:
                self.plane[value].B_steps = self.steps
                self.crossings.append(self.plane[value])
    
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
            self.draw_cell(mark)

    def draw_wire(self, instructions, mark):
        self.steps = 0
        self.pointer = Pointer(0, 0)
        for i in instructions:
            self.draw_line(i, mark)


# Part 1
input = get_input()

instructions = input.split('\n')

instructionListA = [Instruction(i) for i in instructions[0].split(',')]
instructionListB = [Instruction(i) for i in instructions[1].split(',')]

plane = Plane()

plane.draw_wire(instructionListA, "A")
plane.draw_wire(instructionListB, "B")

min_manhattan = 999999
min_step_count = 999999
for c in plane.crossings:
    if c.manhattan() < min_manhattan:
        min_manhattan = c.manhattan()
    if c.step_count() < min_step_count:
        min_step_count = c.step_count()

print("[Part One] Min. manhattan distance to crossing is:", min_manhattan)
print("[Part Two] Min. step count to crossing is:", min_step_count)
