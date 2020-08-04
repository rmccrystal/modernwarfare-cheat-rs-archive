# This script generates a character_info_t struct using the offsets
from dataclasses import dataclass


@dataclass
class Offset:
    name: str
    offset: int
    type: str
    type_size: int


def generate_struct(offsets: [Offset], struct_size: int):
    offsets.sort(key=lambda a: a.offset)
    current_offset = 0
    unk = 1
    print("pub struct character_info\n{")
    for o in offsets:
        buffer = o.offset - current_offset
        if buffer > 0:
            print(f"unk{unk}: [u8; {hex(buffer)}],")

        print(f"pub {o.name}: {o.type},")
        current_offset += o.type_size + buffer
        unk += 1

    buffer = struct_size - current_offset
    if buffer > 0:
        print(f"unk{unk}: [u8; {hex(buffer)}],")

    print("}} // Size: {}".format(hex(struct_size)))


type_cache = {
    "i32": 4,
    "u32": 4,
    "CharacterStance": 4,
    "Address": 8,
    "u64": 8
}

struct_size_input = int(input("Enter struct size: "), 0)
offsets_input: [Offset] = []

# get input
while True:
    name = input("Enter struct member name: ")
    if name == "":
        break

    offset = int(input(f"Enter offset for {name}: "), 0)
    type = input(f"Enter type of {name}: ")
    if type in type_cache:
        type_size = type_cache[type]
    else:
        type_size = int(input(f"Enter size in bytes {name}: "), 0)

    print()

    offsets_input.append(Offset(name, offset, type, type_size))

generate_struct(offsets_input, struct_size_input)
