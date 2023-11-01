from ast import literal_eval

data_file = open("test", "r")
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

list_of_correct_packets = []


def compare(pair, index) -> bool:
    print("Porównywanie pary: ", end="")
    print(pair)

    if type(pair[0]) is type(1) and type(pair[1]) is type(1):
        print("Porownoje liczby")
        if pair[0] == pair[1]:
            return None
        elif pair[0] < pair[1]:
            list_of_correct_packets.append((index, pair))
            return True
        else:
            index = 0
            return False

    elif type(pair[0]) is type([]) and type(pair[1]) is type([]):
        for i in range(max(len(pair[0]), len(pair[1]))):
            if i >= len(pair[0]):
                print("Po lewej skonczyl sie arr")
                list_of_correct_packets.append((index, pair))
                return True
            if i >= len(pair[1]):
                print("Po prawej skonczyl sie arr")
                index = 0
                return False

            result = compare([pair[0][i], pair[1][i]], index)
            if result is not None:
                return result

    elif type(pair[0]) is type([]) and type(pair[1]) is type(1):
        if compare([pair[0], [pair[1]]], index):
            return True

    elif type(pair[0]) is type(1) and type(pair[1]) is type([]):
        if compare([[pair[0]], pair[1]], index):
            return True

    return None


sum = 0
for i, pair in enumerate(data):
    if compare(pair, i + 1) is True:
        sum += i + 1

print(f"\n\n{sum}")


sum = 0
for packet in list_of_correct_packets:
    print(packet)
    sum += packet[0]

print(f"\n\nSum from list: {sum}")
