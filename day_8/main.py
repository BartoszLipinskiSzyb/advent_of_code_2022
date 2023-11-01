file = open("input", "r")

data = []

for line in file.readlines():
    data.append(list(line.strip("\n")))

file.close()

trees_seen = 0

directions = [
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0]
        ]


def is_seen(data, x, y):
    my_height = data[y][x]
    for d in directions:
        step = 1

        seen = True
        while y + d[1] * step >= 0 and y + d[1] * step < len(data) and x + d[0] * step >= 0 and x + d[0] * step < len(data[0]):
            if data[y + d[1] * step][x + d[0] * step] >= my_height:
                seen = False
                break
            step += 1

        if seen:
            return True

    return False


for y in range(len(data)):
    for x in range(len(data[0])):
        if is_seen(data, x, y):
            trees_seen += 1

print(trees_seen)
