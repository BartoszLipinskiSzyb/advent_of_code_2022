file = open("input", "r")

data = []

for line in file.readlines():
    data.append(list(line.strip("\n")))

file.close()

directions = [
    [0, -1],
    [1, 0],
    [0, 1],
    [-1, 0]
        ]


def scenic_score(data, x, y):
    my_height = data[y][x]

    the_score = 1

    for d in directions:
        step = 1

        while True:
            if y + d[1] * step < 0 or y + d[1] * step >= len(data) or x + d[0] * step < 0 or x + d[0] * step >= len(data[0]):
                the_score *= step - 1
                break

            if data[y + d[1] * step][x + d[0] * step] >= my_height:
                the_score *= step
                break

            step += 1

    return the_score


max = 0

for y in range(len(data)):
    for x in range(len(data[0])):
        the_score = scenic_score(data, x, y)
        if the_score > max:
            max = the_score

print(max)
