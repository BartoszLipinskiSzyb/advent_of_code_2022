from ast import literal_eval

data_file = open("input", "r")
data = data_file.read()
data_file.close()

data = data.split("\n\n")
for pair in range(len(data)):
    data[pair] = data[pair].split("\n")
    for arr in range(len(data[pair])):
        if not data[pair][arr] == "":
            try:
                data[pair][arr] = literal_eval(data[pair][arr])
            except Exception:
                print("Tu jest blad")
                print(data[pair][arr])


# print(data)

# False - bad order
# True - good order
# None - don't know yet, check next


def compare(pair) -> bool:
    if type(pair[0]) is type(1) and type(pair[1]) is type(1):
        if pair[0] == pair[1]:
            return None
        return pair[0] < pair[1]

    elif type(pair[0]) is type([]) and type(pair[1]) is type([]):
        for i in range(max(len(pair[0]), len(pair[1]))):
            if i >= len(pair[0]):
                return True
            if i >= len(pair[1]):
                return False
            result = compare([pair[0][i], pair[1][i]])
            if result is not None:
                return result

    elif type(pair[0]) is type([]) and type(pair[1]) is type(1):
        return compare([pair[0], [pair[1]]])
    elif type(pair[0]) is type(1) and type(pair[1]) is type([]):
        return compare([[pair[0]], pair[1]])


sum = 0
for i, pair in enumerate(data):
    if compare(pair) is True:
        sum += i + 1

print(f"\n\n{sum}")
