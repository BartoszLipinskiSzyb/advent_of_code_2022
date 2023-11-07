path = ""
separator = "/"
parent = ".."
change_dir = "$ cd "

paths = {}
path = []


def all_ancestor_paths(path):
    ancestors = []

    for i in range(0, len(path)+1):
        ancestors.append(path[0:len(path)-i])

    return ancestors


def main():
    file = open("input", "r")
    for line in file.readlines():
        line = line.strip("\n")
        if change_dir in line:
            direction = line.replace(change_dir, "")

            if direction == separator:
                path = []
            elif direction == parent:
                path.pop()
            else:
                path.append(direction)

        str_path = separator.join(path)
        if str_path not in paths:
            paths[str_path] = 0

        if line.split(" ")[0].isdigit():
            for ancestor in all_ancestor_paths(path):
                paths[separator.join(ancestor)] += int(line.split(" ")[0])

    file.close()

    system_space = 70_000_000
    needed_space = 30_000_000
    min_delete = needed_space - (system_space - paths[""])

    candidates_to_delete = []
    for p in paths:
        if paths[p] >= min_delete:
            candidates_to_delete.append(p)

    minim = system_space
    minim_name = ""

    for cand in candidates_to_delete:
        if paths[cand] < minim:
            minim = paths[cand]
            minim_name = cand

    print(paths[minim_name])

if __name__ == "__main__":
    main()
