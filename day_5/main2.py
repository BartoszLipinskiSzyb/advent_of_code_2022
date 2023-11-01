import re

state_file = open("state")
state = state_file.readlines()

stacks = []
for i in range(9):
    stacks.append([])

for line in state:
    for stack_index in range(9):
        crate = list(line)[(stack_index * 4) + 1]
        if crate != " " and not crate.isdigit():
            stacks[stack_index].insert(0, crate)

state_file.close()


def use_crane(my_from: int, my_to: int, my_count: int):
    my_from -= 1
    my_to -= 1

    buffer = []
    for i in range(my_count):
        buffer.append(stacks[my_from].pop())

    for i in range(my_count):
        stacks[my_to].append(buffer.pop())


instructions_file = open("instructions")
instructions = instructions_file.readlines()

for line in instructions:
    numbers = re.findall(r"(?<= )\d{1,2}", line)
    use_crane(int(numbers[1]), int(numbers[2]), int(numbers[0]))

result = ""
for stack in stacks:
    result += stack[-1]

print(result)
