#include "encryption.h"
#include <iostream>

auto decrypt_client_info(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t
{
    encrypted_address -= 0x04B2BD3ECA30D631;
    encrypted_address ^= (encrypted_address >> 0x05);
    encrypted_address ^= (encrypted_address >> 0x0A);
    encrypted_address ^= (encrypted_address >> 0x14);
    encrypted_address ^= (encrypted_address >> 0x28);
    encrypted_address *= last_key;
    encrypted_address ^= (encrypted_address >> 0x20);
    encrypted_address *= 0xA0CF0117F8C2EB8D;
    encrypted_address ^= (encrypted_address >> 0x0E);
    encrypted_address ^= (encrypted_address >> 0x1C);
    encrypted_address ^= (encrypted_address >> 0x38);

    return encrypted_address;
}

auto decrypt_client_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t
{
    auto not_peb = ~peb;
    if (encrypted_address && last_key) {
        auto rax = encrypted_address;
        QWORD rbx, rcx, rdx, r10, r11, r14 = 0;

        auto index = _rotr64(not_peb, 0xC) & 0xF;

        switch(index) {
            case 0: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = rax;
                rcx = (rcx >> 0x11);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x22);
                rax ^= rcx;
                rcx = 0x2FEFB83350467A05;
                rax *= rcx;
                rcx = r11;
                rdx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rdx *= rax;
                rax = game_base_address + 0xF6DE;
                rcx *= rax;
                rax = rdx;
                rax ^= rcx;
                rbx = game_base_address;
                rax -= rbx;
                rcx = game_base_address + 0xA83B;
                rcx += rax;
                rax = rcx + r11 * 2 + 0x0;
                rcx = rax;
                rcx = (rcx >> 0x21);
                rax ^= rcx;
                break;
            case 1: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rdx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rdx *= rax;
                rax = r11;
                rax = ~rax;
                rax += rdx;
                rbx = game_base_address + 0x4A86C79B;
                rax += rbx;
                rcx = rax;
                rcx = (rcx >> 0xC);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x18);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x30);
                rax ^= rcx;
                rcx = 0x3E02A01F4E446E89;
                rax -= rcx;
                rcx = rax;
                rax = 0xA7CB507928B779F7;
                rcx *= rax;
                rdx = game_base_address + 0x1F28BFFE;
                rdx = ~rdx;
                rdx *= r11;
                rax = rdx;
                rax ^= rcx;
                rcx = 0x507A5FFD9AA6B7AF;
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x25);
                rax ^= rcx;
                break;
            case 2: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = r11;
                rcx *= game_base_address + 0xD9AF;
                rbx = game_base_address;
                rcx -= rbx;
                rax += rcx;
                rcx = rax;
                rcx = (rcx >> 0x13);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x26);
                rax ^= rcx;
                rdx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rdx;
                r14 = game_base_address + 0x2AD7946E;
                rcx = r14;
                rcx -= r11;
                rax ^= rcx;
                rcx = 0xFC8723C1EFF6530F;
                rax *= rcx;
                rcx = game_base_address + 0x12545C8B;
                rcx += r11;
                rax += rcx;
                rax -= rbx;
                break;
            case 3: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = 0x6392D61853126CC3;
                rax -= rcx;
                rdx = game_base_address + 0x3D9E;
                rdx -= r11;
                rax ^= rdx;
                rcx = game_base_address + 0x57E2;
                rcx = ~rcx;
                rcx ^= r11;
                rax -= rcx;
                rcx = rax;
                rcx = (rcx >> 0x18);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x30);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x28);
                rax ^= rcx;
                rax *= last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64;
                rcx = 0xDFA076C93F68778B;
                rax *= rcx;
                rbx = game_base_address;
                rax -= rbx;
                rax += 0xFFFFFFFFE0F38B10;
                rax += r11;
                break;
            case 4:
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = 0x39D009C4DB835389;
                rax *= rcx;
                rcx = rax;
                rcx = (rcx >> 0x9);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x12);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x24);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x1F);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x3E);
                rax ^= rcx;
                rcx = r11;
                rcx ^= rax;
                rax = 0x56450F07CEA1DA39;
                rcx *= rax;
                rax = 0x6AEEFF9E3A8F6526;
                rcx -= rax;
                rax = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rcx;
                rax += r11;
                break;
            case 5: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = rax;
                rcx = (rcx >> 0x21);
                rax ^= rcx;
                r14 = game_base_address + 0x29675457;
                rcx = r14;
                rcx = ~rcx;
                rcx *= r11;
                rax += rcx;
                rcx = 0xA1EA456E8AD2421B;
                rax *= rcx;
                rcx = 0x4F46F66FE0D5A8D;
                rax -= rcx;
                rcx = rax;
                rcx = (rcx >> 0x11);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x22);
                rax ^= rcx;
                rcx = r11;
                rcx ^= rax;
                rax = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rcx;
                rcx = 0x4A5A84C27B9FF295;
                rax *= rcx;
                break;
            case 6: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = rax;
                rcx = (rcx >> 0x5);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0xA);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x14);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x28);
                rax ^= rcx;
                rdx = game_base_address + 0x10448FE0;
                rcx = rdx;
                rcx -= r11;
                rax ^= rcx;
                rbx = game_base_address;
                rax ^= rbx;
                rax -= r11;
                rcx = rax;
                rcx = (rcx >> 0x1F);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x3E);
                rcx ^= rax;
                rax = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rcx;
                rcx = 0x865ACBE65A13EB55;
                rax *= rcx;
                rcx = 0x14A6B47063247CF1;
                rax -= rcx;
                break;
            case 7: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = rax;
                rcx = (rcx >> 0x13);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x26);
                rax ^= rcx;
                rax ^= r11;
                rcx = rax;
                rcx = (rcx >> 0x1B);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x36);
                rcx ^= rax;
                rdx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rdx *= rcx;
                rax = rdx;
                rax = (rax >> 0x1B);
                rdx ^= rax;
                rax = rdx;
                rax = (rax >> 0x36);
                rax ^= rdx;
                rcx = 0x2D24C5810A6D67F5;
                rax ^= rcx;
                rcx = 0x2DCB82D1575111E7;
                rax += rcx;
                rcx = 0x58BD2C7DBCB08C1D;
                rax *= rcx;
                break;
            case 8: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rax *= last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64;
                rcx = rax;
                rcx = (rcx >> 0x1);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x2);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x4);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x8);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x10);
                rax ^= rcx;
                rdx = rax;
                rdx = (rdx >> 0x20);
                rdx ^= rax;
                rcx = r11;
                rax = game_base_address + 0x7E9;
                rcx ^= rax;
                rax = rdx;
                rax -= rcx;
                rcx = 0x6CE3B54FB335CD6A;
                rax ^= rcx;
                rcx = 0x7A9AD9218FE5EB1;
                rax *= rcx;
                rcx = r11;
                r14 = game_base_address + 0x5485;
                rcx ^= r14;
                rax += rcx;
                rcx = 0x2291C3826EF3D7A3;
                rax *= rcx;
                rcx = rax;
                rcx = (rcx >> 0x28);
                rax ^= rcx;
                break;
            case 9: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rax *= last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64;
                rax += r11;
                rax ^= r11;
                r14 = game_base_address + 0x34D9;
                rax ^= r14;
                rcx = r11;
                rcx *= game_base_address + 0x7C38E180;
                rdx = 0x9B8EAD398078E7F5;
                rcx += rdx;
                rax += rcx;
                rcx = r11;
                rbx = game_base_address;
                rcx -= rbx;
                rcx -= 0xB84E;
                rax ^= rcx;
                rcx = 0x8C19FEAC69800C3F;
                rax *= rcx;
                rcx = rax;
                rcx = (rcx >> 0x15);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x2A);
                rax ^= rcx;
                break;
            case 10: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rdx = r11;
                rdx = ~rdx;
                rbx = game_base_address;
                rdx -= rbx;
                rdx -= 0xED38;
                rdx ^= rax;
                rax = game_base_address + 0xB19F;
                rax = ~rax;
                rax *= r11;
                rax += rdx;
                rcx = rax;
                rcx = (rcx >> 0xF);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x1E);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x3C);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x12);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x24);
                rax ^= rcx;
                rcx = 0xA75AE220A11CAE83;
                rcx *= rax;
                rax = 0x618E549EE3829055;
                rcx ^= rax;
                rax = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rcx;
                rcx = 0x8980A9D964A4C47;
                rax *= rcx;
                break;
            case 11: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rcx *= 0x3E93BFD6C42E9C31;
                rax *= rcx;
                rcx = rax;
                rcx = (rcx >> 0x11);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x22);
                rax ^= rcx;
                rbx = game_base_address;
                rax ^= rbx;
                rcx = r11;
                r10 = game_base_address + 0x9308;
                rcx ^= r10;
                rax -= rcx;
                rax -= r11;
                rax ^= r11;
                rcx = game_base_address + 0x38A65EC4;
                rcx = ~rcx;
                rcx *= r11;
                rax += rcx;
                break;
            case 12: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rcx;
                rcx = 0x468F0549833F76D3;
                rax += rcx;
                rbx = game_base_address;
                rax += rbx;
                rcx = 0x589145FA85F152F7;
                rax ^= rcx;
                rax += r11;
                rcx = 0x3F1D9EF06F57F6CB;
                rax *= rcx;
                rcx = rax;
                rcx = (rcx >> 0x23);
                rax ^= rcx;
                rax -= r11;
                rax -= rbx;
                rax -= 0x15F34726;
                break;
            case 13: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = 0x2A36CE34734F5CFE;
                rcx += rax;
                rax = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rax *= rcx;
                rcx = rax;
                rcx = (rcx >> 0xB);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x16);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x2C);
                rax ^= rcx;
                rcx = 0xC533752EB9DEEFA7;
                rax *= rcx;
                rcx = 0x48DD2203387288CF;
                rax += rcx;
                rcx = game_base_address + 0x5D8C352E;
                rcx += r11;
                rax += rcx;
                rax -= r11;
                rbx = game_base_address;
                rax ^= rbx;
                break;
            case 14: // tested good
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rbx = game_base_address;
                rax -= rbx;
                rcx = 0x3CA536BA062888B1;
                rax *= rcx;
                rax -= r11;
                rcx = rax;
                rcx = (rcx >> 0x9);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x12);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x24);
                rax ^= rcx;
                rcx = last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64
                rcx *= rax;
                rax = r11 - 0x514BA695;
                rax += rcx;
                rcx = game_base_address + 0x11D3;
                rcx -= r11;
                rax ^= rcx;
                break;
            case 15:
                // rax = BASE + 0x97B98;
                r11 = not_peb;
                rcx = 0x5E676767C11503B5;
                rax -= rcx;
                rax *= last_key; // DISPLACEMENT = 0x15, REVERSED_ADDRESS = 0x4D47115, _byteswap_uint64;
                rax += r11;
                rcx = 0x644FE6C253667683;
                rax -= rcx;
                rcx = r11;
                rcx ^= game_base_address + 0x443EB233;
                rax += rcx;
                rcx = rax;
                rcx = (rcx >> 0xB);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x16);
                rax ^= rcx;
                rcx = rax;
                rcx = (rcx >> 0x2C);
                rax ^= rcx;
                rcx = 0xF19F079F1DEC5C59;
                rax *= rcx;
                rcx = 0xFFFFFFFFFFFFC7E1;
                rcx -= r11;
                rbx = game_base_address;
                rcx -= rbx;
                rax += rcx;
                break;
        }

        return rax;
    }

    return 0;
}

auto decrypt_bone_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key, uint64_t peb) -> uint64_t
{
    auto not_peb = ~peb;

    switch ((not_peb >> 0xC) & 0xF)
    {
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
            encrypted_address ^= ~(not_peb)+(game_base_address + 0x531AEA66);
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
