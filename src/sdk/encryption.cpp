#include "encryption.h"
#include <iostream>

extern auto read_qword(uint64_t address) -> uint64_t;

auto decrypt_client_info(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    const auto Peb = peb;
    const auto baseModuleAddr = game_base_address;
    uint64_t rax, rbx, rcx, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14 = 0;

    r8 = baseModuleAddr;

    rbx = read_qword(baseModuleAddr + 0x164FE708);

    // mole tool output
    rdx = Peb;
    rax = (baseModuleAddr + 0xDCB);
    rcx -= rax;
    rax = 0xCC18A31EF579748B;
    rax *= rbx;
    rcx &= 0xffffffffc0000000;
    rax -= rdx;
    rcx <<= 0x10;
    rcx ^= read_qword(baseModuleAddr + 0x5BDB0D2);
    rax -= r8;
    rax -= 0xB37F;
    rcx = (~rcx);
    rax ^= r8;
    rbx = read_qword(rcx + 0x17);
    rbx *= rax;
    rax = (baseModuleAddr + 0x234F);
    rax = (~rax);
    rax *= rdx;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x26;
    rbx ^= rax;
    return rbx;
}

typedef uint64_t UINT64;

auto decrypt_client_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    uint64_t rax, rbx, rcx, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14 = 0;
    const auto Peb = peb;
    const auto baseModuleAddr = game_base_address;

    if (encrypted_address) {
        UINT64 r11 = peb;
        UINT64 rax = encrypted_address;
        UINT64 clientBaseSwitch = _rotr64(peb, 0xF) & 0xF;
        switch (clientBaseSwitch) {
            case 0: {
                rax = (baseModuleAddr + 0xDCB);
                rcx -= rax;
                rax = 0xCC18A31EF579748B;
                rax *= rbx;
                rcx &= 0xffffffffc0000000;
                rax -= rdx;
                rcx <<= 0x10;
                rcx ^= read_qword(baseModuleAddr + 0x5BDB0D2);
                rax -= r8;
                rax -= 0xB37F;
                rcx = (~rcx);
                rax ^= r8;
                rbx = read_qword(rcx + 0x17);
                rbx *= rax;
                rax = (baseModuleAddr + 0x234F);
                rax = (~rax);
                rax *= rdx;
                rbx ^= rax;
                rax = rbx;
                rax >>= 0x26;
                rbx ^= rax;
                rdi = (baseModuleAddr + 0x4C7);
                r8 = 0xC182479671FCB70B;
                r9 = 0xB60DE1AD5F04434D;
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdx = 0x81C9D564;
                r8 *= r9;
                rcx = 0xD50D54717B0AD987;
                r9 ^= rcx;
                rcx = 0xC7B9B2E4B976DDF4;
                r8 ^= rcx;
                rcx = 0xF8F7901679E63766;
                rax ^= rcx;
                rax -= rbx;
                r8 *= r9;
                rcx = 0x8235BCD3D92700A;
                r8 ^= rcx;
                rcx = 0x77B7505FA95F3AE4;
                r9 ^= rcx;
                rcx = 0xFFFFFFFF8465CE8C;
                rcx -= rbx;
                rax += rcx;
                r8 *= r9;
                rcx = 0xC232559676D4BBB6;
                r8 ^= rcx;
                rcx = 0x779240561BA778E;
                r9 ^= rcx;
                r8 *= r9;
                rcx = 0xD77C78DAF3EB203A;
                r9 ^= rcx;
                rcx = 0x325FFA6CAE82EF5D;
                r8 ^= rcx;
                r8 *= r9;
                rcx = 0xAEA248079C212200;
                r8 ^= rcx;
                rcx = 0x6CD3F0EB6E3F426D;
                r9 ^= rcx;
                rcx = baseModuleAddr;
                rcx += 0x3A53;
                rcx += rbx;
                rax += rcx;
                r8 *= r9;
                rcx = 0x8E70ECE7E7E46147;
                r9 ^= rcx;
                rcx = 0x9705A79DE5AC3934;
                r8 ^= rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rcx = read_qword(rcx + 0x11);
                rcx *= 0x5B86AB55B605BE97;
                rax *= rcx;
                r8 *= r9;
                rcx = rax;
                rcx >>= 0x10;
                rax ^= rcx;
                rcx = 0xC3D201B924B35410;
                r9 ^= rcx;
                rcx = 0x3889539B89E45806;
                r8 ^= rcx;
                r8 *= r9;
                rcx = rax;
                rcx >>= 0x20;
                rax ^= rcx;
                rcx = 0x7F7EA947112EB147;
                rax *= rcx;
                return rax;
                break;
            }

            case 1: {
                r11 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdi = (baseModuleAddr + 0x4C7);
                rax += rbx;
                rcx = rax;
                rcx >>= 0x21;
                rax ^= rcx;
                rcx = 0xA0BC991616047162;
                rax += rcx;
                rcx = 0x9853A34E765E1306;
                rax ^= rcx;
//                r8 = read_qword(rbp + 0xe8);
                rdx = (baseModuleAddr + 0x8849);
//                r8 -= rdi;
//                r8 &= 0xffffffffc0000000;
                r8 = 0;
                r8 <<= 0x10;
                r8 ^= r11;
                rcx = rbx;
                r8 = (~r8);
                rcx *= rdx;
                rdx = rax;
                rax = read_qword(r8 + 0x11);
                rdx -= rcx;
                rax *= rdx;
                rcx = 0x476EC20EF1BD52AD;
                rax *= rcx;
                rcx = rax;
                rcx >>= 0xC;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x18;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x30;
                rax ^= rcx;
                return rax;
                break;
            }

            case 2: {
                rdi = (baseModuleAddr + 0x4C7);
                r11 = (baseModuleAddr + 0x52D9);
                r9 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = rbx;
                rcx = (~rcx);
                rcx ^= r11;
                rax -= rcx;
                rcx = rax;
                rcx >>= 0xB;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x16;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x2C;
                rax ^= rcx;
                rax += rbx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r9;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rax ^= rbx;
                rcx = 0x741F82748099EB56;
                rax ^= rcx;
                rcx = 0xA2D472E6BCD55AF9;
                rax *= rcx;
                return rax;
                break;
            }

            case 3: {
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdi = (baseModuleAddr + 0x4C7);
                r14 = (baseModuleAddr + 0x578C81D1);
                rcx = baseModuleAddr;
                rax -= rcx;
                rcx = rax;
                rcx >>= 0x1F;
                rax ^= rcx;
                rdx = r14;
                rdx -= rbx;
                rcx = rax;
                rcx >>= 0x3E;
                rdx ^= rcx;
                rcx = 0x33768ED5DA5407FF;
                rdx ^= rax;
                rax = (baseModuleAddr + 0xD3D5);
                rax ^= rbx;
                rax += rdx;
                rax *= rcx;
                rcx = 0x65C9930CB8E2BCB5;
                rax += rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rcx = read_qword(rcx + 0x11);
                rcx *= 0xF65D1909FCAC691;
                rax *= rcx;
                return rax;
                break;
            }

            case 4: {
                rdi = (baseModuleAddr + 0x4C7);
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = baseModuleAddr;
                rax -= rcx;
                rcx = baseModuleAddr;
                rcx += 0x43832E1F;
                rcx += rbx;
                rax += rcx;
                rcx = rax;
                rcx >>= 0x1E;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x3C;
                rax ^= rcx;
                rcx = 0x1AA8A03DDAB2D0D4;
                rax ^= rcx;
                rcx = 0x68EA9674D2249399;
                rax *= rcx;
                rcx = baseModuleAddr;
                rcx += 0x2597BC81;
                rcx += rbx;
                rax += rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rcx = read_qword(rcx + 0x11);
                rax *= rcx;
                rcx = 0x5FEF85AD6D119735;
                rax -= rcx;
                return rax;
                break;
            }

            case 5: {
                rdi = (baseModuleAddr + 0x4C7);
                r14 = (baseModuleAddr + 0x623CBA2A);
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = r14;
                rcx -= rbx;
                rax ^= rcx;
                rcx = 0xE119FC793E29DC07;
                rax *= rcx;
                rcx = 0x3CB0E3CE3AEEA352;
                rax -= rcx;
                rcx = baseModuleAddr;
                rax -= rcx;
//                rdx = read_qword(rbp + 0xe8);
                rcx = rbx;
                rcx ^= read_qword(baseModuleAddr + 0x17B84D8);
//                rdx -= rdi;
                rax -= rcx;
//                rdx &= 0xffffffffc0000000;
                rdx = 0;
                rdx <<= 0x10;
                rdx ^= r10;
                rdx = (~rdx);
                rcx = read_qword(rdx + 0x11);
                rcx *= rax;
                rax = 0x73780622BCE1D54;
                rax = 0xFADB2E1C46DB368C;
                rax = rcx;
                rax >>= 0x20;
                rax ^= rcx;
                rax -= rbx;
                return rax;
                break;
            }

            case 6: {
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdi = (baseModuleAddr + 0x4C7);
//                rdx = read_qword(rbp + 0xe8);
//                rdx -= rdi;
//                rdx &= 0xffffffffc0000000;
                rdx = 0;
                rcx = rax;
                rdx <<= 0x10;
                rax = baseModuleAddr;
                rcx -= rax;
                rdx ^= r10;
                rdx = (~rdx);
                rax = read_qword(rdx + 0x11);
                rax *= rcx;
                rcx = rax;
                rcx >>= 0x17;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x2E;
                rax ^= rcx;
                rcx = 0x787687AD404D7227;
                rax *= rcx;
                rcx = 0x74F58D8A9B2FCC81;
                rax += rcx;
                rcx = 0x7B7A98B5A384D9E7;
                rax *= rcx;
                rcx = baseModuleAddr;
                rax -= rcx;
                rax -= rbx;
                return rax;
                break;
            }

            case 7: {
                rdi = (baseModuleAddr + 0x4C7);
                r11 = (baseModuleAddr + 0xD926);
                r9 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = rbx;
                rcx = (~rcx);
                rax ^= rcx;
                rax ^= r11;
                rcx = rax;
                rcx >>= 0x1F;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x3E;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x1E;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x3C;
                rax ^= rcx;
                rcx = 0xB723D34F12F5135B;
                rax *= rcx;
                rcx = 0xE3DAD9F52D02EBFB;
                rax ^= rcx;
                rcx = baseModuleAddr;
                rax += rcx;
                rcx = 0x6777E5AF0F853137;
                rax *= rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r9;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                return rax;
                break;
            }

            case 8: {
                rdi = (baseModuleAddr + 0x4C7);
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = 0x21234FA3E7D72725;
                rax *= rcx;
//                rdx = read_qword(rbp + 0xe8);
                rcx = rbx;
//                rdx -= rdi;
//                rdx &= 0xffffffffc0000000;
                rdx = 0;
                rdx <<= 0x10;
                rdx ^= r10;
                rdx = (~rdx);
                rdx = read_qword(rdx + 0x11);
                rdx *= rax;
                rax = (baseModuleAddr + 0xE932);
                rcx *= rax;
                rax = rdx;
                rax ^= rcx;
                rcx = baseModuleAddr;
                rax ^= rcx;
                rdx = rbx;
                rcx = (baseModuleAddr + 0x6A05);
                rdx ^= rcx;
                rcx = 0x244DEDEB88859B6A;
                rcx -= rdx;
                rax += rcx;
                rcx = rax;
                rcx >>= 0x8;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x10;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x20;
                rax ^= rcx;
                rcx = 0x17C5C34FF2F3057B;
                rax *= rcx;
                return rax;
                break;
            }

            case 9: {
                rdi = (baseModuleAddr + 0x4C7);
                r14 = (baseModuleAddr + 0x60BB);
                r9 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = rbx;
                rcx ^= r14;
                rax += rcx;
                rax -= rbx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r9;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rcx = 0x2A536ADAEA2376F1;
                rax *= rcx;
                rcx = rax;
                rcx >>= 0x1F;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x3E;
                rax ^= rcx;
                rax -= rbx;
                rcx = 0xBBDA5BEFB492B750;
                rax ^= rcx;
                return rax;
                break;
            }

            case 10: {
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdi = (baseModuleAddr + 0x4C7);
                r14 = (baseModuleAddr + 0xD262);
                rcx = rax;
                rcx >>= 0x16;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x2C;
                rax ^= rcx;
                rcx = baseModuleAddr;
                rax -= rcx;
                rcx = 0x24BCDDA13866EEF;
                rax ^= rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rcx = 0x354D4A473EC1CDF5;
                rax += rcx;
                rcx = 0xB17DCDF93B68E0C3;
                rax *= rcx;
                rcx = rbx;
                rcx *= r14;
                rax -= rcx;
                rcx = baseModuleAddr;
                rax -= rcx;
                return rax;
                break;
            }

            case 11: {
                rdi = (baseModuleAddr + 0x4C7);
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = baseModuleAddr;
                rax += rcx;
//                rdx = read_qword(rbp + 0xe8);
//                rdx -= rdi;
//                rdx &= 0xffffffffc0000000;
                rdx = 0;
                rdx <<= 0x10;
                rdx ^= r10;
                rcx = rax;
                rdx = (~rdx);
                rcx >>= 0x23;
                rcx ^= rax;
                rax = read_qword(rdx + 0x11);
                rdx = (baseModuleAddr + 0x1DCB);
                rax *= rcx;
                rcx = 0x3C5AA5CA0E88C9E;
                rax -= rcx;
                rcx = rbx;
                rcx *= rdx;
                rax -= rcx;
                rcx = (baseModuleAddr + 0x166C38A5);
                rcx = (~rcx);
                rcx *= rbx;
                rax ^= rcx;
                rax += rbx;
                rcx = 0x500D3489887A521;
                rax *= rcx;
                return rax;
                break;
            }

            case 12: {
                rdi = (baseModuleAddr + 0x4C7);
                r9 = read_qword(baseModuleAddr + 0x5BDB10E);
                rcx = rax;
                rcx >>= 0x27;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x28;
                rax ^= rcx;
                rcx = baseModuleAddr;
                rcx += 0x88C1;
                rcx += rbx;
                rax += rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r9;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rax -= rbx;
                rcx = 0xF25ECD745F9442AD;
                rax *= rcx;
                rcx = 0x9D8367A95ABB7EA9;
                rax *= rcx;
                rcx = 0x4DCADE4615FD36A6;
                rax -= rcx;
                return rax;
                break;
            }

            case 13: {
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdi = (baseModuleAddr + 0x4C7);
                rcx = rax;
                rcx >>= 0x11;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x22;
                rcx ^= rbx;
                rax ^= rcx;
                rcx = baseModuleAddr;
                rax ^= rcx;
                rcx = 0xB82C3FC7537C161;
                rax ^= rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rcx = (baseModuleAddr + 0x1544);
                rdx = rbx;
                rdx = (~rdx);
                rdx ^= rcx;
                rcx = rax;
                rax = 0x64B09EDE42DA74F;
                rax *= rcx;
                rax += rdx;
                rcx = rax;
                rcx >>= 0x6;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0xC;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x18;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x30;
                rax ^= rcx;
                return rax;
                break;
            }

            case 14: {
                rdi = (baseModuleAddr + 0x4C7);
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rcx = rax;
                rcx >>= 0x1A;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x34;
                rax ^= rcx;
                rcx = 0x563A85A278B3031E;
                rax -= rcx;
                rcx = baseModuleAddr;
                rax ^= rcx;
                rcx = (baseModuleAddr + 0x744E);
                rax += rbx;
                rax += rcx;
                rcx = 0x7203C00954F8CCCB;
                rax *= rcx;
                rcx = rax;
                rcx >>= 0x15;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x2A;
                rax ^= rcx;
                rax ^= rbx;
                return rax;
                break;
            }

            case 15: {
                r10 = read_qword(baseModuleAddr + 0x5BDB10E);
                rdi = (baseModuleAddr + 0x4C7);
                r12 = (baseModuleAddr + 0x79178BD6);
                rdx = (baseModuleAddr + 0xA278);
                rcx = rbx;
                rcx *= 0x7FF788C0E1B1;
                rax += rcx;
//                rcx = read_qword(rbp + 0xe8);
//                rcx -= rdi;
//                rcx &= 0xffffffffc0000000;
                rcx = 0;
                rcx <<= 0x10;
                rcx ^= r10;
                rcx = (~rcx);
                rax *= read_qword(rcx + 0x11);
                rcx = rax;
                rcx >>= 0x1A;
                rax ^= rcx;
                rcx = rax;
                rcx >>= 0x34;
                rax ^= rcx;
                rcx = rbx;
                rcx = (~rcx);
                rcx *= r12;
                rax += rcx;
                rcx = rbx;
                rcx *= rdx;
                rax += rcx;
                rcx = 0xC41F90241AA4D77B;
                rax ^= rcx;
                rcx = 0xC5E3D9BCA1CA54D3;
                rax *= rcx;
                rcx = 0x57FD2D20C86FE978;
                rax += rcx;
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
    /*
    uint64_t rax, rbx, rcx, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14 = 0;

    const auto enc_case = _rotl64(~peb, 0x2b);
    rbx = peb;

    const auto baseModuleAddr = game_base_address;

    switch (enc_case) {
        case 0: {
            rbx = 0x5A47CA78BD2E409F;
            rdi = (baseModuleAddr + 0xB50);
            r9 = read_qword(baseModuleAddr + 0x5C71205);
            r8 ^= rbx;
            rax = 0xBFB56F29257246C7;
            r8 *= rax;
            rax = r8;
            rax >>= 0x5;
            r8 ^= rax;
            rax = r8;
            rax >>= 0xA;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x14;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 ^= rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r9;
            _byteswap_uint64(rax);
            r8 *= read_qword(rax + 0xf);
            rax = r8;
            rax >>= 0x1A;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x34;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 += rax;
            rax = r8;
            rax >>= 0x15;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x2A;
            r8 ^= rax;
            return r8;
            break;
        }

        case 1: {
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            r8 -= rbx;
            rax = 0xE2EE731C3F6427FD;
            r8 ^= rax;
            rax = 0x41BF37D9B9A71EBD;
            r8 ^= rax;
            rax = 0xA3A9C39B828066C5;
            r8 *= rax;
            rax = r8;
            rax >>= 0xC;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x18;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x30;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            rcx = 0;
            rax = baseModuleAddr;
            rcx -= rdi;
            rax += 0xA8C8;
            rcx &= 0xffffffffc0000000;
            rax += rbx;
            rcx <<= 0x10;
            rcx ^= r10;
            _byteswap_uint64(rcx);
            rcx = read_qword(rcx + 0xf);
            r8 *= rcx;
            r8 ^= rax;
            return r8;
            break;
        }

        case 2: {
            r11 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = r8;
            rax >>= 0x24;
            r8 ^= rax;
            rax = rbx;
            rax *= 0x7FF6DBE850DE;
            r8 += rax;
            rax = r8;
            rax >>= 0x16;
            r8 ^= rax;
            rcx = r8;
            rcx >>= 0x2C;
            rdx = read_qword(rbp + 0x88);
            rdx -= rdi;
            rax = (baseModuleAddr + 0x53859A6A);
            rdx &= 0xffffffffc0000000;
            rax = (~rax);
            rdx <<= 0x10;
            rcx ^= rax;
            rdx ^= r11;
            rcx ^= rbx;
            rcx ^= r8;
            _byteswap_uint64(rdx);
            r8 = read_qword(rdx + 0xf);
            r8 *= rcx;
            rax = baseModuleAddr;
            r8 -= rax;
            rax = 0xEF6985F605E2E50B;
            r8 *= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            return r8;
            break;
        }

        case 3: {
            rdi = (baseModuleAddr + 0xB50);
            r9 = read_qword(baseModuleAddr + 0x5C71205);
            rax = rbx;
            rax = (~rax);
            rax += 0xFFFFFFFFD48933C7;
            r8 += rax;
            r8 += rbx;
            rax = r8;
            rax >>= 0xF;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1E;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3C;
            r8 ^= rax;
            rax = 0xAED6D6C9EEE9EDBD;
            r8 *= rax;
            rax = 0x5674923517CD4D3B;
            r8 += rax;
            r8 -= rbx;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r9;
            _byteswap_uint64(rax);
            rax = read_qword(rax + 0xf);
            rax *= 0x369964A79B497CF7;
            r8 *= rax;
            return r8;
            break;
        }

        case 4: {
            rdi = (baseModuleAddr + 0xB50);
            r9 = read_qword(baseModuleAddr + 0x5C71205);
            rax = r8;
            rax >>= 0x1E;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3C;
            r8 ^= rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r9;
            _byteswap_uint64(rax);
            r8 *= read_qword(rax + 0xf);
            rax = r8;
            rax >>= 0xD;
            r8 ^= rax;
            return r8;
            break;
        }

        case 5: {
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = r8;
            rax >>= 0x1D;
            r8 ^= rax;
            rax = rbx;
            rcx = r8;
            rax = (~rax);
            rcx >>= 0x3A;
            rcx ^= r8;
            r8 = (baseModuleAddr + 0x8065);
            r8 *= rax;
            r8 += rcx;
            rax = baseModuleAddr;
            r8 -= rax;
            r8 += 0xFFFFFFFF9B389A02;
            r8 += rbx;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r10;
            _byteswap_uint64(rax);
            rax = read_qword(rax + 0xf);
            r8 *= rax;
            rax = 0xC44D9B99FC06CFD3;
            r8 *= rax;
            rax = 0x644A746A68E06381;
            r8 += rax;
            rax = r8;
            rax >>= 0x1D;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3A;
            r8 ^= rax;
            rax = 0xDEEC93F64EB70DAD;
            r8 *= rax;
            return r8;
            break;
        }

        case 6: {
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = 0x88118B047DE4997B;
            r8 *= rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r10;
            _byteswap_uint64(rax);
            r8 *= read_qword(rax + 0xf);
            rax = baseModuleAddr;
            r8 += rax;
            rcx = rbx;
            rcx = (~rcx);
            rax = (baseModuleAddr + 0xDF47);
            r8 += rax;
            r8 += rcx;
            rax = r8;
            rax >>= 0x24;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x20;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x6;
            r8 ^= rax;
            rax = r8;
            rax >>= 0xC;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x18;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x30;
            r8 ^= rax;
            return r8;
            break;
        }

        case 7: {
            rbx = 0x152DA7067B059EC4;
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = (baseModuleAddr + 0x5A91F65B);
            rax = (~rax);
            rax *= rbx;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            rcx = read_qword(rbp + 0x88);
            rcx -= rdi;
            rcx &= 0xffffffffc0000000;
            rcx <<= 0x10;
            rax = rbx + r8 * 1;
            rcx ^= r10;
            r8 = (baseModuleAddr + 0x79995182);
            rax += r8;
            _byteswap_uint64(rcx);
            r8 = read_qword(rcx + 0xf);
            r8 *= rax;
            rax = r8;
            rax >>= 0x14;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
            rcx = rbx;
            rcx = (~rcx);
            rax = (baseModuleAddr + 0x5D4EB937);
            rcx += rax;
            rax = 0x8D2CA5D99810099F;
            rax *= r8;
            r8 = 0x59C6F64E9C256368;
            rax += r8;
            r8 = rcx;
            r8 ^= rax;
            return r8;
            break;
        }

        case 8: {
            r9 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = 0x92DC248512BB51C3;
            r8 *= rax;
            rax = r8;
            rax >>= 0x1C;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x38;
            r8 ^= rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r9;
            _byteswap_uint64(rax);
            r8 *= read_qword(rax + 0xf);
            r8 = r8 + rbx * 2;
            r10 = 0x78FBA8BA16DFCE36;
            rax = rbx;
            rax = (~rax);
            rax *= 0x7FF7271601AD;
            rax += r10;
            r8 += rax;
            rax = r8;
            rax >>= 0x8;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x10;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x20;
            r8 ^= rax;
            return r8;
            break;
        }

        case 9: {
            rbx = 0xC4606B84;
            rdi = (baseModuleAddr + 0xB50);
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rax = rbx;
            rax *= 0x7FF6DBE8A320;
            r8 += rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r10;
            _byteswap_uint64(rax);
            rax = read_qword(rax + 0xf);
            rax *= 0x30FFDE29FC4DD27;
            r8 *= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            rax = 0x2C27A049C97AA591;
            r8 += rax;
            rax = r8;
            rax >>= 0x22;
            r8 ^= rax;
            rcx = rbx;
            rcx = (~rcx);
            rax = (baseModuleAddr + 0x5CCB3A3D);
            rax = (~rax);
            rcx *= rax;
            r8 += rcx;
            rax = 0x508011E144D271F5;
            r8 += rax;
            return r8;
            break;
        }

        case 10: {
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rcx = read_qword(rbp + 0x88);
            rax = rbx + r8 * 1;
            rcx -= rdi;
            rcx &= 0xffffffffc0000000;
            r8 = 0x8B63BCC6617845DE;
            rax ^= r8;
            rcx <<= 0x10;
            rcx ^= r10;
            _byteswap_uint64(rcx);
            r8 = read_qword(rcx + 0xf);
            r8 *= rax;
            rax = (baseModuleAddr + 0x78F8);
            r8 -= rbx;
            r8 += rax;
            rax = 0x45D20BD1A23CD457;
            r8 *= rax;
            rcx = rbx;
            rax = (baseModuleAddr + 0x499E6F87);
            rcx *= rax;
            rax = r8;
            rax >>= 0x21;
            rcx ^= rax;
            r8 ^= rcx;
            return r8;
            break;
        }

        case 11: {
            rdi = (baseModuleAddr + 0xB50);
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rax = baseModuleAddr;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
            rax = 0xC3D5E66FA2C67E9;
            r8 *= rax;
            rax = r8;
            rax >>= 0x3;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x6;
            r8 ^= rax;
            rax = r8;
            rax >>= 0xC;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x18;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x30;
            r8 ^= rax;
            rax = 0xAEEBA5B9DB47C9C1;
            r8 ^= rax;
            rcx = read_qword(rbp + 0x88);
            rcx -= rdi;
            rcx &= 0xffffffffc0000000;
            rcx <<= 0x10;
            rax = rbx + r8 * 1;
            rcx ^= r10;
            _byteswap_uint64(rcx);
            r8 = read_qword(rcx + 0xf);
            r8 *= rax;
            rax = 0x1DF33D280C08C61F;
            r8 ^= rax;
            return r8;
            break;
        }

        case 12: {
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = (baseModuleAddr + 0x5A68103A);
            rax -= rbx;
            r8 += rax;
            rax = 0x588F124C72AE6C51;
            r8 *= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            r8 += 0xFFFFFFFFC484D99B;
            r8 += rbx;
            rax = 0xF2C25DC154BB0E35;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1E;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3C;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x26;
            r8 ^= rax;
            rax = r8;
            rax >>= 0xF;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1E;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3C;
            r8 ^= rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r10;
            _byteswap_uint64(rax);
            r8 *= read_qword(rax + 0xf);
            return r8;
            break;
        }

        case 13: {
            rbx += 0x2621EB5A;
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rdi = (baseModuleAddr + 0xB50);
            rax = 0xDF28CAC9996C9B9F;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x14;
            r8 ^= rax;
            rcx = r8;
            rcx >>= 0x28;
            r8 ^= rcx;
            rax = (baseModuleAddr + 0xD246);
            rax = (~rax);
            rax ^= rbx;
            r8 -= rax;
            rcx = read_qword(rbp + 0x88);
            rcx -= rdi;
            rcx &= 0xffffffffc0000000;
            rax = r8;
            rcx <<= 0x10;
            r8 = 0x103580C14316FBE8;
            rax ^= r8;
            rcx ^= r10;
            _byteswap_uint64(rcx);
            r8 = read_qword(rcx + 0xf);
            r8 *= rax;
            rax = 0x44B7974CFCA38DF;
            r8 *= rax;
            r8 ^= rbx;
            rax = r8;
            rax >>= 0x1A;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x34;
            r8 ^= rax;
            return r8;
            break;
        }

        case 14: {
            rdi = (baseModuleAddr + 0xB50);
            rcx = (baseModuleAddr + 0xB98F);
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rax = rbx;
            rax *= rcx;
            r8 -= rax;
            rax = r8;
            rax >>= 0x9;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x12;
            r8 ^= rax;
            rcx = read_qword(rbp + 0x88);
            rcx -= rdi;
            rcx &= 0xffffffffc0000000;
            rax = r8;
            rcx <<= 0x10;
            rcx ^= r10;
            rax >>= 0x24;
            rax ^= r8;
            _byteswap_uint64(rcx);
            r8 = read_qword(rcx + 0xf);
            r8 *= rax;
            rax = rbx;
            rax *= 0x7FF7169D408D;
            r8 -= rax;
            rax = 0x8AAF881E8ACB35D8;
            r8 ^= rax;
            rax = 0x1594DE324E7FBBE4;
            r8 += rax;
            rax = r8;
            rax >>= 0xB;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x16;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x2C;
            r8 ^= rax;
            rax = 0xBE03C46877C36BC7;
            r8 *= rax;
            return r8;
            break;
        }

        case 15: {
            rdi = (baseModuleAddr + 0xB50);
            r10 = read_qword(baseModuleAddr + 0x5C71205);
            rax = baseModuleAddr;
            rax += rbx;
            r8 -= rax;
            r8 ^= rbx;
            rax = 0xAEB396BE3D6F82B5;
            r8 *= rax;
rax = 0; // new
            rax <<= 0x10;
            rax ^= r10;
            _byteswap_uint64(rax);
            r8 *= read_qword(rax + 0xf);
            rax = 0x9123D2711F5ED54B;
            r8 *= rax;
            rax = r8;
            rax >>= 0x1F;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3E;
            r8 ^= rax;
            rax = 0x5AC7D35A3E5B70CA;
            r8 -= rax;
            return r8;
            break;
        }
    }
     */
    return 0;
}
