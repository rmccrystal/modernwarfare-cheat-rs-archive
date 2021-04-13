#include "encryption.h"
#include <iostream>

extern auto read_qword(uint64_t address) -> uint64_t;

auto decrypt_client_info(uint64_t encrypted_address, uint64_t game_base_address, uint64_t _last_key,
                         uint64_t peb) -> uint64_t {
    const auto Peb = peb;
    const auto baseModuleAddr = game_base_address;
    uint64_t rax, rbx, rcx = 0, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14 = 0;

    rdx = baseModuleAddr; //new
    rbx = encrypted_address;

    r8 = Peb;
    rax = rbx;
    rax >>= 0x1C;
    rbx ^= rax;
    rax = (baseModuleAddr + 0x80D);
//    rcx -= rax;
    rax = rbx;
    rax >>= 0x38;
    rcx &= 0xffffffffc0000000;
    rax ^= rbx;
    rcx <<= 0x10;
    rcx ^= read_qword(baseModuleAddr + 0x5C5A110);
    rax ^= rdx;
    rdx = 0xF1437CC9A0155F3;
    rax ^= rdx;
    rdx = 0xCA39C7DAB1A5DD89;
    rax *= rdx;
    rcx = _byteswap_uint64(rcx);
    rax += r8;
    rbx = read_qword(rcx + 0x13);
    rbx *= rax;

    return rbx;
}

typedef uint64_t UINT64;

auto decrypt_client_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    // Default decl
    uint64_t rax, rbx, rcx, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14, r15 = 0;
    const auto Peb = peb;
    const auto baseModuleAddr = game_base_address;

    r11 = ~peb;
    rax = encrypted_address;

    const auto clientBaseSwitch = _rotr64(~Peb, 0x17) & 0xf;;

    const auto rbp = 0;


    switch (clientBaseSwitch) {
        case 0: {
            rbx = (baseModuleAddr + 0xB4E);
            r9 = read_qword(baseModuleAddr + 0x5C5A121);
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0; // new
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rax *= read_qword(rcx + 0x19);
            rcx = baseModuleAddr;
            rcx += 0xBDE9;
            rcx += r11;
            rax ^= rcx;
            rcx = 0x312EFF07E41E1414;
            rax -= rcx;
            rax -= r11;
            rcx = 0x5CDB44B162572C63;
            rax *= rcx;
            rcx = 0x1C858CBD8EDD640B;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x27;
            rax ^= rcx;
            rcx = r11;
            rcx -= 0xF27F;
            rax ^= rcx;
            return rax;
            break;
        }

        case 1: {
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x2B8B);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = rax;
            rcx >>= 0x24;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x9;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x12;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x24;
            rax ^= rcx;
            rcx = r11;
            rcx = (~rcx);
            rax ^= rcx;
            rax ^= r15;
            rcx = 0x555DDC944D0D943D;
            rax *= rcx;
//            rdx = read_qword(rbp + 0xf8);
            rcx = r11 + rax * 1;
//            rdx -= rbx;
            rax = (baseModuleAddr + 0xE3EE);
            rcx += rax;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = (~rdx);
            rax = read_qword(rdx + 0x19);
            rax *= rcx;
            rcx = 0xC490B2C8238CFF77;
            rax ^= rcx;
            rcx = 0x862C569C4827D98F;
            rax *= rcx;
            return rax;
            break;
        }

        case 2: {
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            r14 = (baseModuleAddr + 0x19528CF7);
            rcx = 0xEB6C0C4437042F3B;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x17;
            rax ^= rcx;
//            rdx = read_qword(rbp + 0xf8);
//            rdx -= rbx;
            rcx = rax;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rcx >>= 0x2E;
            rcx ^= rax;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = (~rdx);
            // rdx is a valid ptr here
            rax = read_qword(rdx + 0x19);
            rax *= rcx;
            rcx = r11;
            rcx *= r14;
            rax -= rcx;
            rcx = baseModuleAddr;
            rax -= rcx;
            rcx = 0x6D5686851A27372B;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax ^= rcx;
            rcx = 0x7FCFB874870925E5;
            rax *= rcx;
            return rax;
            break;
        }

        case 3: {
            rbx = (baseModuleAddr + 0xB4E);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = baseModuleAddr;
            rax -= rcx;
            rcx = 0x4618B6F51371DD2F;
            rax -= rcx;
            rcx = rax;
            rcx >>= 0x28;
            rax ^= rcx;
            rcx = 0xE9B49C5D532C63E3;
            rax *= rcx;
            rcx = (baseModuleAddr + 0x323313DB);
            rcx = (~rcx);
            rdx = r11;
            rax += rcx;
            rdx = (~rdx);
            rax += rdx;
            rcx = 0x419974F479D310C4;
            rax ^= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read_qword(rcx + 0x19);
            rax += r11;
            return rax;
            break;
        }

        case 4: {
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x3480EDD9);
            rax ^= r11;
            rcx = 0x754B686CCE9886A;
            rax += rcx;
            rcx = 0x9798AF0DFEE346C7;
            rax *= rcx;
            rax -= r11;
            rcx = rax;
//            rdx = read_qword(rbp + 0xf8);
            rcx >>= 0x13;
//            rdx -= rbx;
            rax ^= rcx;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = (~rdx);
            rcx = rax;
            rcx >>= 0x26;
            rcx ^= rax;
            rax = read_qword(rdx + 0x19);
            rax *= rcx;
            rcx = 0x87AC0291A4EF2062;
            rax ^= rcx;
            rdx = r11;
            rdx = (~rdx);
            rcx = r15;
            rcx = (~rcx);
            rdx *= rcx;
            rax += rdx;
            return rax;
            break;
        }

        case 5: {
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            rcx = rax;
            rcx >>= 0x1F;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x3E;
            rax ^= rcx;
            rcx = 0x1A324B89D41EFCF7;
            rax += rcx;
            rax ^= r11;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read_qword(rcx + 0x19);
            rcx = 0x6D3CA144C8E0194F;
            rax *= rcx;
            rcx = 0xE2189F0341B1958B;
            rax *= rcx;
            rax -= r11;
            rcx = baseModuleAddr;
            rax -= rcx;
            return rax;
            break;
        }

        case 6: {
            rbx = (baseModuleAddr + 0xB4E);
            r14 = (baseModuleAddr + 0x16610662);
            r9 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = rax;
            rcx >>= 0x15;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2A;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax += rcx;
            rcx = rax;
            rcx >>= 0xF;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1E;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x3C;
            rax ^= rcx;
            rcx = 0x2CDBAB6696583E9D;
            rax -= rcx;
            rcx = 0x74917DFAEF91EF6D;
            rax *= rcx;
            rcx = r11 + 0x1;
            rcx *= r14;
            rax += rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rcx = read_qword(rcx + 0x19);
            rcx *= 0xDC2DB627D9887CF1;
            rax *= rcx;
            return rax;
            break;
        }

        case 7: {
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            rcx = rax;
            rcx >>= 0x26;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rcx += 0x933D;
            rcx += r11;
            rax += rcx;
            rcx = rax;
            rcx >>= 0xB;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x16;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2C;
            rax ^= rcx;
            rcx = 0x96A3856562FB6B0E;
            rax ^= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read_qword(rcx + 0x19);
            rcx = 0xAEE1F5679ADB4BD7;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0xB;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x16;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2C;
            rax ^= rcx;
            return rax;
            break;
        }

        case 8: {
            rbx = (baseModuleAddr + 0xB4E);
            r9 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = rax;
            rcx >>= 0x1F;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x3E;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax += rcx;
            rcx = rax;
            rcx >>= 0x20;
            rax ^= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rcx = read_qword(rcx + 0x19);
            rax *= rcx;
            rcx = 0x2C58D03E65DBC973;
            rax *= rcx;
            rax += r11;
            rcx = 0xCB04FF56455D4FF1;
            rax *= rcx;
            rcx = 0x4A64C87346644F7F;
            rax ^= rcx;
            return rax;
            break;
        }

        case 9: {
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x3A23AC81);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = rax;
            rcx >>= 0x23;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax += rcx;
            rcx = r11;
            rcx ^= r15;
            rax -= rcx;
            rcx = 0x9F11BDA0B31163A1;
            rax *= rcx;
//            rdx = read_qword(rbp + 0xf8);
//            rdx -= rbx;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rcx = r11;
            rdx ^= r10;
            rcx ^= rax;
            rdx = (~rdx);
            rax = read_qword(rdx + 0x19);
            rax *= rcx;
            rcx = 0x9581FD4BB02282FC;
            rax += rcx;
            rcx = 0x872BD95E21C644BD;
            rax ^= rcx;
            return rax;
            break;
        }

        case 10: {
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x39DFECD6);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = r11;
            rcx *= r15;
            rax -= rcx;
            rcx = 0x9003227C3DA86DC9;
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
            rcx = rax;
            rcx >>= 0x28;
            rax ^= rcx;
            rcx = 0x2DE793EC9F63ECCA;
            rax -= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rcx = read_qword(rcx + 0x19);
            rax *= rcx;
            rcx = 0x95595DB74CDD1E73;
            rax *= rcx;
            rcx = baseModuleAddr;
            rax += rcx;
            return rax;
            break;
        }

        case 11: {
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x3D81);
            rax -= r11;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rdx = read_qword(rcx + 0x19);
            rcx = r11;
            rdx *= rax;
            rax = r15;
            rcx = (~rcx);
            rax = (~rax);
            rcx *= rax;
            rdx += rcx;
            rax = rdx;
            rax >>= 0x22;
            rax ^= rdx;
            rax += r11;
            rcx = 0xD8E6EAEB0E028D9F;
            rax ^= rcx;
            rcx = 0xA62360698DAE335F;
            rax *= rcx;
            rcx = 0x479F90BDA19F9C41;
            rax -= rcx;
            return rax;
            break;
        }

        case 12: {
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x7BC74523);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rdx = (baseModuleAddr + 0x6C0D6474);
            rcx = baseModuleAddr;
            rcx += 0x96EE;
            rcx += r11;
            rax += rcx;
            rcx = rax;
            rcx >>= 0x1A;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x34;
            rax ^= rcx;
            rcx = r11;
            rcx ^= r15;
            rax += rcx;
            rcx = r11;
            rcx ^= rdx;
            rax -= rcx;
            rcx = 0xAC21E3686611AE00;
            rax ^= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rcx = read_qword(rcx + 0x19);
            rcx *= 0xD0D4E21D12B341CD;
            rax *= rcx;
            rcx = 0x7F8AD31BCC9F69A;
            rax += rcx;
            return rax;
            break;
        }

        case 13: {
            rbx = (baseModuleAddr + 0xB4E);
            r9 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = 0x85ADB6C74D90A65;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax -= rcx;
            rax += 0xFFFFFFFFFFFF1483;
            rax += r11;
            rcx = rax;
            rcx >>= 0x11;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x22;
            rax ^= rcx;
            rcx = 0xA1A4DCF9196B7240;
            rax ^= rcx;
            rcx = 0x2458AD0B6D6E8353;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0xF;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1E;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x3C;
            rax ^= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rax *= read_qword(rcx + 0x19);
            rcx = baseModuleAddr;
            rax += rcx;
            return rax;
            break;
        }

        case 14: {
            rbx = (baseModuleAddr + 0xB4E);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
            rcx = r11;
            rcx *= 0x7FF848674351;
            rax += rcx;
            rcx = 0x32C02A0B0C8367DF;
            rax -= rcx;
//            rcx = read_qword(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read_qword(rcx + 0x19);
            rcx = rax;
            rcx >>= 0x21;
            rax ^= rcx;
            rcx = r11;
            rcx = (~rcx);
            rcx *= 0x7FF7D9616053;
            rax += rcx;
            rcx = 0x406DF273BB5E023;
            rax *= rcx;
            rcx = baseModuleAddr;
            rcx *= 0x74B3D215816E053;
            rax -= rcx;
            return rax;
            break;
        }

        case 15: {
            rbx = (baseModuleAddr + 0xB4E);
            r14 = (baseModuleAddr + 0x62F2C5B2);
            r10 = read_qword(baseModuleAddr + 0x5C5A121);
//            rdx = read_qword(rbp + 0xf8);
            rax -= r11;
//            rdx -= rbx;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rcx = r14 + rax * 1;
            rdx ^= r10;
            rdx = (~rdx);
            rax = read_qword(rdx + 0x19);
            rax *= rcx;
            rax += r11;
            rcx = rax;
            rcx >>= 0xA;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x14;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x28;
            rax ^= rcx;
            rcx = baseModuleAddr;
            rax ^= rcx;
            rdx = r11;
            rdx = (~rdx);
            rcx = (baseModuleAddr + 0x48FF);
            rcx = (~rcx);
            rdx += rcx;
            rax ^= rdx;
            rcx = 0x87213BE2FB9E9B93;
            rax *= rcx;
            rcx = baseModuleAddr;
            rax += rcx;
            return rax;
            break;
        }
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
