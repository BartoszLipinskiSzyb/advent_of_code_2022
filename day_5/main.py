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


def move_crate(my_from, my_to):
    my_from -= 1
    my_to -= 1
    if len(stacks[my_from]) == 0:
        print(f"No crates to move from {my_from}")
        return None
    stacks[my_to].append(stacks[my_from].pop())


def use_crane(my_from, my_to, my_count):
    for i in range(int(my_count)):
        move_crate(int(my_from), int(my_to))


instructions_file = open("instructions")
instructions = instructions_file.readlines()

for line in instructions:
    numbers = re.findall(r"(?<= )\d{1,2}", line)
    use_crane(numbers[1], numbers[2], numbers[0])

result = ""
for stack in stacks:
    result += stack[-1]

print(result)
