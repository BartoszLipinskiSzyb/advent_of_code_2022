file = open("input", "r")
data = file.read()
file.close()

for i in range(len(data) - 13):
    if len(set(data[i:i+14])) == 14:
        print(i + 14)
        break
