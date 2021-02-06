#include "encryption.h"
#include <iostream>

extern auto read_qword(uint64_t address) -> uint64_t;

auto decrypt_client_info(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    auto var_3 = (encrypted_address) * (0x157d6db5fbec5a99);
    auto var_4 = var_3;
    auto var_5 = (var_4) >> 0xa;
    auto var_6 = (var_3) ^(var_5);
    auto var_7 = var_6;
    auto var_8 = (var_7) >> 0x14;
    auto var_9 = (var_6) ^(var_8);
    auto var_10 = var_9;
    auto var_11 = (var_10) >> 0x28;
    auto var_12 = (var_11) ^(var_9);
    auto var_13 = (var_12) - (peb);
    auto var_14 = (var_13) >> 0x27;
    auto var_15 = (var_14) ^((var_12) - (peb));
    auto var_16 = last_key;
    auto var_17 = (var_16) * (var_15);
    auto var_18 = (var_17) * (0x981c17f8a6efa8eb);
    return var_18;

}

typedef uint64_t UINT64;

auto decrypt_client_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    uint64_t rax, rbx, rcx, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14 = 0;

    if (encrypted_address)
    {
        UINT64 r11 = peb;
        UINT64 rax = encrypted_address;
        UINT64 clientBaseSwitch = _rotr64(peb, 0xF) & 0xF;
        switch (clientBaseSwitch)
        {
        case 0:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r9 = read_qword(game_base_address + 0x5C71122);
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0x9);
            rcx = rax;
            rcx >>= 0x21;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x24;
            rax ^= rcx;
            rcx = 0x38F6B6276DCF7425;
            rax -= rcx;
            rcx = rax;
            rcx >>= 0x4;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x8;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x10;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x20;
            rax ^= rcx;
            rax ^= rbx;
            rcx = 0xC01877C687D2B5F4;
            rax ^= rcx;
            rcx = 0xEF195D44C097B245;
            rax *= rcx;

            return rax;

            break;

        }

        case 1:
        {
            rdi = (game_base_address + 0xA7F);
            r10 = read_qword(game_base_address + 0x5C71122);
            rax += r11;
            rcx = (game_base_address + 0x8E69);
            rcx = (~rcx);
            rcx *= r11;
            rax += rcx;
            rcx = rax;
            rcx >>= 0x18;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x30;
            rax ^= rcx;
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0x9);
            rax -= r11;
            rcx = 0x77194237C7785C87;
            rax *= rcx;
            rcx = 0x13E307DB62628C8B;
            rax *= rcx;
            rcx = 0x1AF04960AB17733F;
            rax -= rcx;

            return rax;

            break;

        }

        case 2:
        {
            r9 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0x9);
            rax += r11;
            rcx = r11;
            rcx = (~rcx);
            rcx ^= read_qword(game_base_address + 0x17BC8E0);
            rax += rcx;
            rcx = 0xAD2855D5D9050BDF;
            rax *= rcx;
            rax -= rbx;
            rcx = rax;
            rcx >>= 0xC;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x18;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x30;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0xA;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x14;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x28;
            rax ^= rcx;
            rcx = 0x463FC32646AB8483;
            rax *= rcx;

            return rax;

            break;

        }

        case 3:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r9 = read_qword(game_base_address + 0x5C71122);
            rax += rbx;
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0x9);
            rcx = 0xEE4C9AC74466C667;
            rax *= rcx;
            rax -= r11;
            rcx = rax;
            rcx >>= 0x19;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x32;
            rax ^= rcx;
            rcx = 0x1BAB185822D798F4;
            rax ^= rcx;

            return rax;

            break;

        }

        case 4:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r9 = read_qword(game_base_address + 0x5C71122);
            rcx = 0x60E5FF2A3029BB8B;
            rax *= rcx;
            rcx = 0xA8D76FE9246D0DC;
            rax -= rcx;
            rcx = rax;
            rcx >>= 0x11;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x22;
            rax ^= rcx;
            rcx = r11 + rbx * 1;
            rax -= rcx;
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rcx = read_qword(rcx + 0x9);
            rax *= rcx;
            rax -= rbx;
            rax += rbx;

            return rax;

            break;

        }

        case 5:
        {
            r10 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            rcx = 0x713A7849491A1B3F;
            rax *= rcx;
            rax ^= rbx;
            rcx = rax;
            rcx >>= 0x26;
            rax ^= rcx;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            //rdx -= rdi;
            rdx = 0;
            rdx <<= 0x10;
            rdx ^= r10;
            rcx = 0xFFC7E09B778FFCB2;
            rcx += rax;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = 0xE87E7E1837DCC63;
            rax += rcx;
            rax ^= rbx;
            rax += rbx;

            return rax;

            break;

        }

        case 6:
        {
            r10 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            rcx = rax;
            rcx >>= 0x23;
            rax ^= rcx;
            rcx = 0x8248677CDD7B66F;
            rax *= rcx;
            rax ^= rbx;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            //rdx -= rdi;
            rcx = rax;
            rdx = 0;
            rax = 0x4A0A449EF54FB096;
            rcx ^= rax;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = 0x2009A549E21E477A;
            rax += rcx;
            rax ^= rbx;
            rcx = rax;
            rcx >>= 0x1B;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x36;
            rax ^= rcx;

            return rax;

            break;

        }

        case 7:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r9 = read_qword(game_base_address + 0x5C71122);
            rcx = 0x45FF433E6488119C;
            rcx -= r11;
            rax += rcx;
            rcx = rbx;
            rcx -= r11;
            rax += rcx;
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0x9);
            rcx = rax;
            rcx >>= 0x24;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1C;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x38;
            rax ^= rcx;
            rcx = 0x2A626BD1AE9DB971;
            rax *= rcx;

            return rax;

            break;

        }

        case 8:
        {
            rdi = (game_base_address + 0xA7F);
            r14 = (game_base_address + 0x28AC1A70);
            r10 = read_qword(game_base_address + 0x5C71122);
            rcx = rax;
            rcx >>= 0xF;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1E;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x3C;
            rax ^= rcx;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            //rdx -= rdi;
            rcx = rax;
            rdx = 0;
            rcx -= r11;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x12;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x24;
            rcx ^= r11;
            rax ^= rcx;
            rcx = 0x61C3059DB9F15C9B;
            rax ^= r14;
            rax *= rcx;
            rcx = 0x18C71AD0EF38A3FB;
            rax += rcx;

            return rax;

            break;

        }

        case 9:
        {
            r10 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            rcx = rax;
            rcx >>= 0x1;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x4;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x8;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x10;
            rax ^= rcx;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            rcx = rax;
            //rdx -= rdi;
            rcx >>= 0x20;
            rdx = 0;
            rcx ^= rax;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = 0xF8F2A91D0B389E33;
            rax *= rcx;
            rcx = 0x28A95820C535CC13;
            rax += rcx;
            rcx = rax;
            rcx >>= 0x7;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0xE;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1C;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x38;
            rax ^= rcx;
            rax ^= r11;
            rcx = (game_base_address + 0x9E5D);
            rax ^= rcx;
            rcx = 0x5ABEA3B89A7C265C;
            rax -= rcx;
            rcx = 0xFFFFFFFFFFFFA97A;
            rcx -= r11;
            rcx -= rbx;
            rax += rcx;

            return rax;

            break;

        }

        case 10:
        {
            r10 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            rdx = (game_base_address + 0x6B1B01C4);
            rcx = 0x880C7445899E176A;
            rax ^= rcx;
            rax ^= rbx;
            rcx = r11;
            rcx ^= rdx;
            rax -= rcx;
            rcx = 0xA258080472A3A6BB;
            rax *= rcx;
            rax ^= rbx;
            rcx = 0xE3D85F42E80A58B;
            rax *= rcx;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            rcx = rax;
            rcx >>= 0xE;
            //rdx -= rdi;
            rax ^= rcx;
            rdx = 0;
            rcx = rax;
            rdx <<= 0x10;
            rcx >>= 0x1C;
            rdx ^= r10;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x38;
            rcx ^= rax;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;

            return rax;

            break;

        }

        case 11:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r10 = read_qword(game_base_address + 0x5C71122);
            //rdx = RVPM<UINT64>(rbp + 0x118);
            //rdx -= rdi;
            rcx = rax;
            rdx = 0;
            rcx >>= 0x16;
            rax ^= rcx;
            rdx <<= 0x10;
            rcx = rax;
            rdx ^= r10;
            rcx >>= 0x2C;
            rcx ^= rax;
            rcx += rbx;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = 0xB23AB9E375BE2697;
            rax *= rcx;
            rcx = (game_base_address + 0x363A);
            rax += r11;
            rax += rcx;
            rdx = (game_base_address + 0x19194929);
            rax += rbx;
            rcx = (game_base_address + 0x5704B017);
            rcx -= r11;
            rax += rcx;
            rcx = r11;
            rcx *= rdx;
            rax += rcx;

            return rax;

            break;

        }

        case 12:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r10 = read_qword(game_base_address + 0x5C71122);
            rcx = 0x1D09E61FFEEA736D;
            rax *= rcx;
            rax += r11;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            rax -= r11;
            //rdx -= rdi;
            rax -= rbx;
            rdx = 0;
            rdx <<= 0x10;
            rdx ^= r10;
            rcx = rax + 0xffffffffffff7025;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = 0x6B515363DF94C679;
            rax *= rcx;
            rdx = rax;
            rdx >>= 0x20;
            rdx ^= rax;
            rcx = r11;
            rax = (game_base_address + 0x6C9C88EA);
            rcx ^= rax;
            rax = rdx;
            rax -= rcx;
            rcx = 0x5B9B9DB413DBCABD;
            rax *= rcx;

            return rax;

            break;

        }

        case 13:
        {
            r10 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            //rdx -= rdi;
            rdx = 0;
            rdx <<= 0x10;
            rcx = rax;
            rdx ^= r10;
            rcx -= rbx;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rcx = 0xD7B3BCFE65F0AECF;
            rax -= r11;
            rax ^= rcx;
            rcx = 0xCE5C9608FD3E64D1;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x18;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x30;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0xD;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1A;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x34;
            rax ^= rcx;

            return rax;

            break;

        }

        case 14:
        {
            r10 = read_qword(game_base_address + 0x5C71122);
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            rcx = (game_base_address + 0xDF03);
            rcx -= r11;
            rax += rcx;
            rax ^= rbx;
            //rdx = RVPM<UINT64>(rbp + 0x118);
            //rdx -= rdi;
            rdx = 0;
            rdx <<= 0x10;
            rcx = rax;
            rdx ^= r10;
            rcx >>= 0x23;
            rcx ^= rax;
            rax = 0x6DE50B2BD295E81F;
            rcx -= rax;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0x9);
            rax *= rcx;
            rax ^= rbx;
            rcx = rax;
            rcx >>= 0x20;
            rax ^= rcx;
            rcx = 0xDB499CA7A80C687B;
            rax *= rcx;

            return rax;

            break;

        }

        case 15:
        {
            rdi = (game_base_address + 0xA7F);
            rbx = game_base_address;
            r9 = read_qword(game_base_address + 0x5C71122);
            rcx = 0x620E629D5555CD43;
            rax *= rcx;
            rcx = 0x86FE45A8C6CF1067;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x4;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x8;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x10;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x20;
            rax ^= rcx;
            rax -= r11;
            rax += r11;
            rcx = 0x6DCC43ECB600673F;
            rax *= rcx;
            //rcx = RVPM<UINT64>(rbp + 0x118);
            //rcx -= rdi;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0x9);
            rcx = rbx + 0x4d3592a1;
            rcx += r11;
            rax ^= rcx;

            return rax;

            break;

        }
        }
        return 0;
    }
    return 0;
}

auto
decrypt_bone_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t {
    auto not_peb = ~peb;

    switch ((not_peb >> 0xC) & 0xF) {
        case 0:
            encrypted_address *= 0xDEBCFFB38B60F2EB;
            encrypted_address ^= (encrypted_address >> 0x22);
            encrypted_address ^= game_base_address;
            encrypted_address *= 0xDEBCFFB38B60F2EB;
            encrypted_address *= 0x8EC04574CAFA10A5;
            encrypted_address += (not_peb * 0x7996B5DB71ADB8B);
            encrypted_address -= game_base_address;
            encrypted_address *= last_key;
            break;
        case 1:

            encrypted_address ^= 0x2A06319E3AD132F6;
            encrypted_address += 0x1DC88FA3075A0187;
            encrypted_address ^= (encrypted_address >> 0x04);
            encrypted_address ^= (encrypted_address >> 0x08);
            encrypted_address ^= (encrypted_address >> 0x10);
            encrypted_address ^= (encrypted_address >> 0x20);
            encrypted_address ^= (encrypted_address >> 0x06);
            encrypted_address ^= (encrypted_address >> 0x0C);
            encrypted_address ^= (encrypted_address >> 0x18);
            encrypted_address ^= (encrypted_address >> 0x30);
            encrypted_address *= 0x6096E4C692110081;
            encrypted_address ^= game_base_address;
            encrypted_address -= game_base_address;
            encrypted_address += 0x0FFFFFFFF9D048A7E;
            encrypted_address += not_peb;
            encrypted_address *= last_key;
            break;
        case 2:
            encrypted_address *= last_key;
            encrypted_address ^= (encrypted_address >> 0x10);
            encrypted_address ^= (encrypted_address >> 0x20);
            encrypted_address += ((not_peb) ^ (game_base_address + 0x4880));
            encrypted_address += not_peb;
            encrypted_address *= 0xBFED8FEB52011319;
            encrypted_address += 0x072C8AC4A5B2E0C5;
            encrypted_address += (((~not_peb) ^ (game_base_address + 0x61FC)) + 0x3D9083DBBBBDB928);
            break;
        case 3:

            encrypted_address *= last_key;
            encrypted_address ^= (encrypted_address >> 0x02);
            encrypted_address ^= (encrypted_address >> 0x04);
            encrypted_address ^= (encrypted_address >> 0x08);
            encrypted_address ^= (encrypted_address >> 0x10);
            encrypted_address ^= (encrypted_address >> 0x20);
            encrypted_address -= not_peb;
            encrypted_address ^= (~(game_base_address + 0x1217) ^ not_peb);
            encrypted_address += (0xFFFFFFFFFFFF4638 - not_peb);
            encrypted_address ^= (encrypted_address >> 0x0D);
            encrypted_address ^= (encrypted_address >> 0x1A);
            encrypted_address ^= (encrypted_address >> 0x34);
            encrypted_address ^= 0x9996DA09AE776E69;
            encrypted_address *= 0x7D73DABAA37935BB;
            break;
        case 4:
            encrypted_address ^= (encrypted_address >> 0x18);
            encrypted_address ^= (encrypted_address >> 0x30);
            encrypted_address ^= (encrypted_address >> 0x06);
            encrypted_address ^= (encrypted_address >> 0x0C);
            encrypted_address ^= (encrypted_address >> 0x18);
            encrypted_address ^= (encrypted_address >> 0x30);
            encrypted_address *= last_key;
            encrypted_address += 0x2DDF4CB76563FFBD;
            encrypted_address *= 0x5FC37F9D32687B4D;
            encrypted_address ^= ((game_base_address + 0xB85D) + not_peb);
            encrypted_address -= game_base_address;
            encrypted_address += 0x7BA5E7ADA8B7098B;
            break;
        case 5:

            encrypted_address ^= (encrypted_address >> 0x0B);
            encrypted_address ^= (encrypted_address >> 0x16);
            encrypted_address ^= (encrypted_address >> 0x2C);
            encrypted_address += (~(game_base_address + 0x1A09E7B8) ^ not_peb);
            encrypted_address ^= (encrypted_address >> 0x0D);
            encrypted_address ^= (encrypted_address >> 0x1A);
            encrypted_address ^= (encrypted_address >> 0x34);
            encrypted_address -= 0x3F75FF30DE10A0E0;
            encrypted_address *= last_key;
            encrypted_address ^= (~(game_base_address + 0x58AF0018) + ~not_peb);
            encrypted_address *= 0x5CB40B64B00630C3;
            break;
        case 6:

            encrypted_address *= 0xAA0E33F563896B69;
            encrypted_address *= 0x221257687E9FBC0B;
            encrypted_address ^= 0x4DE244FBEA084E28;
            encrypted_address ^= (encrypted_address >> 0x1B);
            encrypted_address ^= (encrypted_address >> 0x36);
            encrypted_address += not_peb;
            encrypted_address += ~(game_base_address + 0x4772D463);
            encrypted_address += not_peb;
            encrypted_address *= last_key;
            encrypted_address ^= ((game_base_address + 0x12E5) * not_peb);
            break;
        case 7:

            encrypted_address ^= (encrypted_address >> 0x12);
            encrypted_address ^= (encrypted_address >> 0x24);
            encrypted_address *= last_key;
            encrypted_address ^= ~(not_peb) + (game_base_address + 0x531AEA66);
            encrypted_address *= 0x647A15C3556292C7;
            encrypted_address ^= (encrypted_address >> 0x19);
            encrypted_address ^= (encrypted_address >> 0x32);
            encrypted_address ^= 0x87E9A6344B31408F;
            encrypted_address ^= (encrypted_address >> 0x08);
            encrypted_address ^= (encrypted_address >> 0x10);
            encrypted_address ^= (encrypted_address >> 0x20);
            break;
        case 8:

            encrypted_address ^= (encrypted_address >> 0x04);
            encrypted_address ^= (encrypted_address >> 0x08);
            encrypted_address ^= (encrypted_address >> 0x10);
            encrypted_address ^= (encrypted_address >> 0x20);
            encrypted_address ^= (encrypted_address >> 0x12);
            encrypted_address ^= (encrypted_address >> 0x24);
            encrypted_address *= last_key;
            encrypted_address += ((not_peb) * (game_base_address + 0x3DCD7E13));
            encrypted_address *= 0x1CE8EDE9D203AAF7;
            encrypted_address += ~(game_base_address + 0x1CF7CA67);
            encrypted_address += (not_peb * -2);
            encrypted_address += (~not_peb) * 0x00007FF700E754F0;
            break;
        case 9:
            encrypted_address ^= (encrypted_address >> 0x18);
            encrypted_address ^= (encrypted_address >> 0x30);
            encrypted_address ^= (encrypted_address >> 0x22);
            encrypted_address *= last_key;
            encrypted_address += (~not_peb);
            encrypted_address += (game_base_address + 0x7555);
            encrypted_address ^= game_base_address;
            encrypted_address ^= 0xBEB6C558985317AA;
            encrypted_address *= 0xCE0CA0CD08CF28A1;
            encrypted_address -= 0x0CCDFDAC7A4C2932;
            break;
        case 10:
            encrypted_address -= ((not_peb) ^ (game_base_address + 0x22890803));
            encrypted_address *= 0x90DD57C3A9AF9CC9;
            encrypted_address ^= 0x2E5C093119ADD1E8;
            encrypted_address ^= (encrypted_address >> 0x09);
            encrypted_address ^= (encrypted_address >> 0x12);
            encrypted_address ^= (encrypted_address >> 0x24);
            encrypted_address ^= 0x07BEA95B628C8803;
            encrypted_address -= game_base_address;
            encrypted_address ^= (encrypted_address >> 0x14);
            encrypted_address ^= (encrypted_address >> 0x28);
            encrypted_address *= last_key;
            break;
        case 11:
            encrypted_address ^= 0x01AC23249E2BBB06;
            encrypted_address *= 0x7097A370ED9F960D;
            encrypted_address ^= ((~not_peb) + (game_base_address + 0x1F65F573));
            encrypted_address *= last_key;
            encrypted_address ^= 0x799B9920228527AE;
            encrypted_address += (~not_peb);
            encrypted_address += (game_base_address + 0x5480);
            encrypted_address ^= (encrypted_address >> 0x07);
            encrypted_address ^= (encrypted_address >> 0x0E);
            encrypted_address ^= (encrypted_address >> 0x1C);
            encrypted_address ^= (encrypted_address >> 0x38);
            encrypted_address ^= (not_peb - 0x2C3F542D);
            break;
        case 12:
            encrypted_address += 0x0436C40CB2221B2D;
            encrypted_address ^= 0x5FA983F79828AA2A;
            encrypted_address += not_peb;
            encrypted_address += (game_base_address + 0x3A7F);
            encrypted_address *= last_key;
            encrypted_address += game_base_address;
            encrypted_address *= 0xFC15C6AB3D765813;
            encrypted_address += ((not_peb) * (game_base_address + 0x1DEA));
            encrypted_address ^= (encrypted_address >> 0x23);
            break;
        case 13:
            encrypted_address += game_base_address;
            encrypted_address *= 0x498A770E897A8489;
            encrypted_address ^= not_peb;
            encrypted_address *= last_key;
            encrypted_address += not_peb;
            encrypted_address ^= (encrypted_address >> 0x14);
            encrypted_address ^= (encrypted_address >> 0x28);
            encrypted_address += 0x01D27F4740ED251E;
            encrypted_address -= 0x7F4057B170A77EB7;
            break;
        case 14:
            encrypted_address ^= not_peb;
            encrypted_address ^= (encrypted_address >> 0x0C);
            encrypted_address ^= (encrypted_address >> 0x18);
            encrypted_address ^= (encrypted_address >> 0x30);
            encrypted_address ^= game_base_address;
            encrypted_address -= game_base_address;
            encrypted_address *= last_key;
            encrypted_address -= 0x42BFF47C18EFE769;
            encrypted_address *= 0xDA080684F68088A7;
            break;
        default:
        case 15:
            encrypted_address ^= 0x6079DE628E69CEFB;
            encrypted_address -= 0x0D167679B3A9AC8C;
            encrypted_address *= 0x6EF6E004451EDFAB;
            encrypted_address ^= (encrypted_address >> 0x1E);
            encrypted_address ^= ((encrypted_address >> 0x3C) ^ (~not_peb * ~(game_base_address + 0x20BF4C2B)));
            encrypted_address ^= game_base_address;
            encrypted_address -= game_base_address;
            encrypted_address += (game_base_address - 0x321F);
            encrypted_address *= last_key;
            break;
    }

    return encrypted_address;

}
