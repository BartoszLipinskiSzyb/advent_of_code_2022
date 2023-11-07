file = open("test", "r")


class File:
    name = None
    size = None

    def __init__(self, name, size):
        self.name = name
        self.size = size


class Dir:
    name = None
    files = []
    dirs = []

    def __init__(self, name):
        self.name = name

    def get_files(self):
        result = ""
        for file in self.files:
            result += file.name + " "
        return result

    def get_size(self):
        size = 0

        for file in self.files:
            size += file.size

        for dir in self.dirs:
            size += dir.get_size()

        return size


dir_paths = {}

path = ""
separator = "/"
for line in file.readlines():
    line = line.strip("\n")

    if "$ cd" in line:
        direction = line.split(" ")[2]
        if direction == "..":
            path = separator.join(path.split(separator)[0:-2]) + separator
        elif direction == separator:
            path = separator
        else:
            path += direction + separator

    if path not in dir_paths:
        dir_paths[path] = Dir(path)
        if path != separator:
            # print(path)
            dir_paths[separator.join(path.split(separator)[0:-2]) + separator].dirs.append(dir_paths[path])

    splitted = line.split(" ")
    if splitted[0].isdigit():
        dir_paths[path].files.append(File(splitted[1], int(splitted[0])))


file.close()

for dir in dir_paths:
    print(f"{dir} : {dir_paths[dir].get_files()}")
