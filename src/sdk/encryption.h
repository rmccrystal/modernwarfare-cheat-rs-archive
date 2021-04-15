#include <cstdint>
#include <stdlib.h>
#include <stdio.h>

typedef unsigned __int64 QWORD;

auto decrypt_client_info(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t;
auto decrypt_client_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t;
auto decrypt_bone_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t;
auto get_bone_index(uint64_t index) -> uint64_t;
