#include "encryption.h"
#include <iostream>
#include <intrin.h>

#pragma intrinsic(_umul128)

extern void interop_read_bytes(uint64_t address, uint64_t size, size_t buf);

template<typename T>
auto read(uint64_t address) -> T {
    T result;
    interop_read_bytes(address, sizeof(T), (size_t) &result);
    return result;
}

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
    rcx ^= read<QWORD>(baseModuleAddr + 0x5C5A110);
    rax ^= rdx;
    rdx = 0xF1437CC9A0155F3;
    rax ^= rdx;
    rdx = 0xCA39C7DAB1A5DD89;
    rax *= rdx;
    rcx = _byteswap_uint64(rcx);
    rax += r8;
    rbx = read<QWORD>(rcx + 0x13);
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
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A121);
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0; // new
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rax *= read<QWORD>(rcx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rdx = read<QWORD>(rbp + 0xf8);
            rcx = r11 + rax * 1;
//            rdx -= rbx;
            rax = (baseModuleAddr + 0xE3EE);
            rcx += rax;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = (~rdx);
            rax = read<QWORD>(rdx + 0x19);
            rax *= rcx;
            rcx = 0xC490B2C8238CFF77;
            rax ^= rcx;
            rcx = 0x862C569C4827D98F;
            rax *= rcx;
            return rax;
            break;
        }

        case 2: {
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            r14 = (baseModuleAddr + 0x19528CF7);
            rcx = 0xEB6C0C4437042F3B;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x17;
            rax ^= rcx;
//            rdx = read<QWORD>(rbp + 0xf8);
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
            rax = read<QWORD>(rdx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read<QWORD>(rcx + 0x19);
            rax += r11;
            return rax;
            break;
        }

        case 4: {
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x3480EDD9);
            rax ^= r11;
            rcx = 0x754B686CCE9886A;
            rax += rcx;
            rcx = 0x9798AF0DFEE346C7;
            rax *= rcx;
            rax -= r11;
            rcx = rax;
//            rdx = read<QWORD>(rbp + 0xf8);
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
            rax = read<QWORD>(rdx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read<QWORD>(rcx + 0x19);
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
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rcx = read<QWORD>(rcx + 0x19);
            rcx *= 0xDC2DB627D9887CF1;
            rax *= rcx;
            return rax;
            break;
        }

        case 7: {
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read<QWORD>(rcx + 0x19);
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
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rcx = read<QWORD>(rcx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rdx = read<QWORD>(rbp + 0xf8);
//            rdx -= rbx;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rcx = r11;
            rdx ^= r10;
            rcx ^= rax;
            rdx = (~rdx);
            rax = read<QWORD>(rdx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rcx = read<QWORD>(rcx + 0x19);
            rax *= rcx;
            rcx = 0x95595DB74CDD1E73;
            rax *= rcx;
            rcx = baseModuleAddr;
            rax += rcx;
            return rax;
            break;
        }

        case 11: {
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
            rbx = (baseModuleAddr + 0xB4E);
            r15 = (baseModuleAddr + 0x3D81);
            rax -= r11;
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rdx = read<QWORD>(rcx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rcx = read<QWORD>(rcx + 0x19);
            rcx *= 0xD0D4E21D12B341CD;
            rax *= rcx;
            rcx = 0x7F8AD31BCC9F69A;
            rax += rcx;
            return rax;
            break;
        }

        case 13: {
            rbx = (baseModuleAddr + 0xB4E);
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A121);
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
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = (~rcx);
            rax *= read<QWORD>(rcx + 0x19);
            rcx = baseModuleAddr;
            rax += rcx;
            return rax;
            break;
        }

        case 14: {
            rbx = (baseModuleAddr + 0xB4E);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
            rcx = r11;
            rcx *= 0x7FF848674351;
            rax += rcx;
            rcx = 0x32C02A0B0C8367DF;
            rax -= rcx;
//            rcx = read<QWORD>(rbp + 0xf8);
//            rcx -= rbx;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax *= read<QWORD>(rcx + 0x19);
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
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A121);
//            rdx = read<QWORD>(rbp + 0xf8);
            rax -= r11;
//            rdx -= rbx;
//            rdx &= 0xffffffffc0000000;
            rdx = 0;
            rdx <<= 0x10;
            rcx = r14 + rax * 1;
            rdx ^= r10;
            rdx = (~rdx);
            rax = read<QWORD>(rdx + 0x19);
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
    uint64_t rax, rbx, rcx, rdx, r8, rdi, rsi, r9, r10, r11, r12, r13, r14 = 0;

    const auto Peb = peb;
    const auto baseModuleAddr = game_base_address;

    const auto enc_case = _rotr64(Peb, 0x15) & 0xf;

    r8 = encrypted_address;

    rbx = peb;

    switch (enc_case) {
        case 0: {
            rbx = 0xA5D6FA6FC6D397EA;
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rdi = (baseModuleAddr + 0x125);
            r8 ^= rbx;
            rax = (baseModuleAddr + 0x432CA097);
            r8 ^= rax;
            rax = r8;
            rax >>= 0xF;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1E;
            r8 ^= rax;
//            rcx = read<QWORD>(rbp + 0x78);
//            rcx -= rdi;
//            rcx &= 0xffffffffc0000000;
            rax = r8;
            rcx = 0;
            rcx <<= 0x10;
            rax >>= 0x3C;
            rcx ^= r10;
            rax ^= r8;
            rcx = (~rcx);
            r8 = read<QWORD>(rcx + 0x7);
            r8 *= rax;
            rax = 0xCC9FA0CE0E35512B;
            rax *= r8;
            r8 = 0xE12D63925DEFD0A9;
            rax ^= rbx;
            r8 = 0x3C230020E95DD0F7;
            r8 = 0xD4FE02CE6A7EACDC;
            r8 = rax;
            r8 >>= 0x26;
            r8 ^= rax;
            rax = 0xE4B46422D644E542;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
            return r8;
            break;
        }

        case 1: {
            rdi = (baseModuleAddr + 0x125);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            r8 -= rbx;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r10;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            r8 ^= rbx;
            rax = r8;
            rax >>= 0xA;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x14;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
            rcx = rbx;
            rcx = (~rcx);
            rax = (baseModuleAddr + 0x2040);
            rax = (~rax);
            rcx *= rax;
            rax = 0xE0F739F5C5E76B3D;
            rax *= r8;
            r8 = 0x5BAC544604A30922;
            rax ^= rbx;
            r8 ^= rax;
            r8 += rcx;
            return r8;
            break;
        }

        case 2: {
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rdi = (baseModuleAddr + 0x125);
            rax = (baseModuleAddr + 0x4737875B);
            r8 += rax;
            rcx = rbx;
            rax = (baseModuleAddr + 0x77FD33FE);
            rcx ^= rax;
            rax = r8;
            r8 = 0xDE1D4CD500518BDD;
            r8 *= rax;
            r8 += rcx;
            rax = r8;
            rax >>= 0xC;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x18;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x30;
            r8 ^= rax;
            rax = 0x65B0F82E119FD3D0;
            r8 ^= rax;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r10;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            rax = 0x71CAF2496F010833;
            r8 -= rax;
            return r8;
            break;
        }

        case 3: {
            rdi = (baseModuleAddr + 0x125);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            r8 *= 0x23D8125EA0B64CD;
            rax = rbx;
//            rcx = read<QWORD>(rbp + 0x78);
            rax *= 0x7FF65FCA6B56;
//            rcx -= rdi;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rax ^= r8;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            r8 = read<QWORD>(rcx + 0x7);
            r8 *= rax;
            rax = 0x5A2478E907BD6DAB;
            r8 -= rax;
            rax = (baseModuleAddr + 0x272ACCF7);
            rax = (~rax);
            rax ^= rbx;
            r8 += rax;
            rax = r8;
            rax >>= 0xF;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1E;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3C;
            r8 ^= rax;
            rax = 0x379B4DBE5204FBFA;
            r8 += rax;
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

        case 4: {
            rdi = (baseModuleAddr + 0x125);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = (baseModuleAddr + 0x86BE);
            rax = (~rax);
            rax -= rbx;
            r8 ^= rax;
            rax = 0x4ACDEA38870E4D86;
            r8 ^= rax;
            rax = 0x338A15F527DAD9C9;
            r8 *= rax;
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
            r8 -= rbx;
            rax = r8;
            rax >>= 0x12;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x24;
            r8 ^= rax;
            rax = 0xCF5620578B65CBB7;
            r8 ^= rax;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r10;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            return r8;
            break;
        }

        case 5: {
            rdi = (baseModuleAddr + 0x125);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = 0x45E18ABD94081706;
            r8 += rax;
            rax = r8;
            rax >>= 0xD;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1A;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x34;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 += rax;
            r8 -= rbx;
            rax = r8;
            rax >>= 0x9;
            r8 ^= rax;
//            rcx = read<QWORD>(rbp + 0x78);
//            rcx -= rdi;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rax = r8;
            rax >>= 0x12;
            r8 ^= rax;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = (~rcx);
            rax = r8;
            rax >>= 0x24;
            rax ^= r8;
            r8 = read<QWORD>(rcx + 0x7);
            r8 *= rax;
            rax = 0x8E2D92BE2C4874E9;
            r8 *= rax;
            rax = 0x7C25B2A2CE6F9E99;
            r8 += rax;
            return r8;
            break;
        }

        case 6: {
            rdi = (baseModuleAddr + 0x125);
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = baseModuleAddr;
            r8 ^= rax;
            r8 -= rbx;
            r8 ^= rbx;
            rax = 0x97886542CA363D19;
            r8 *= rax;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r9;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            rax = 0x7C11257D56B3AE81;
            r8 ^= rax;
            rax = 0x418101B0441EAE5B;
            r8 *= rax;
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
            rdi = (baseModuleAddr + 0x125);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = r8;
            rax >>= 0x18;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x30;
            r8 ^= rax;
            rax = 0x939CEEFDF1772E43;
            r8 *= rax;
            rax = baseModuleAddr;
            r8 += rax;
            rax = r8;
            rax >>= 0x14;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
//            rcx = read<QWORD>(rbp + 0x78);
//            rcx -= rdi;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rax = rbx;
            rcx <<= 0x10;
            rax = (~rax);
            rcx ^= r10;
            rcx = (~rcx);
            rcx = read<QWORD>(rcx + 0x7);
            rcx *= r8;
            r8 = (baseModuleAddr + 0x31B2B997);
            r8 ^= rax;
            r8 += rcx;
            rax = rbx;
            rax *= 0x7FF65FCA5093;
            r8 += rax;
            rax = rbx;
            rax = (~rax);
            rax += 0xFFFFFFFFBD16E289;
            r8 += rax;
            return r8;
            break;
        }

        case 8: {
            rdi = (baseModuleAddr + 0x125);
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = 0xFFFFFFFFC0CEB1E2;
            rax -= rbx;
            r8 += rax;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r9;
            rax = (~rax);
            rax = read<QWORD>(rax + 0x7);
            rax *= 0xFDB56606FB71B80B;
            r8 *= rax;
            rax = r8;
            rax >>= 0x13;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x26;
            r8 ^= rax;
            rax = 0x55D8769D677585D4;
            r8 -= rax;
            rax = 0x71D744FD38C4AFBB;
            r8 ^= rax;
            return r8;
            break;
        }

        case 9: {
            rdi = (baseModuleAddr + 0x125);
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r10;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            rax = r8;
            rax >>= 0x10;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x20;
            r8 ^= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            rax = 0x417A3AD9CEE94077;
            r8 ^= rax;
            rcx = rbx;
            rcx = (~rcx);
            rax = (baseModuleAddr + 0x27338CC5);
            rcx *= rax;
            rax = r8;
            r8 = baseModuleAddr;
            rax -= r8;
            r8 = rcx;
            r8 ^= rax;
            rax = 0x721F05432AC03F33;
            r8 *= rax;
            rax = 0x95E000BA10AFB48B;
            r8 ^= rax;
            return r8;
            break;
        }

        case 10: {
            r11 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rdi = (baseModuleAddr + 0x125);
            rdx = (baseModuleAddr + 0x8304);
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r11;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            rcx = rbx;
            rcx = (~rcx);
            rax = (baseModuleAddr + 0x5220);
            rax = (~rax);
            rcx += rax;
            rax = r8;
            rax >>= 0x24;
            rcx ^= rax;
            r8 ^= rcx;
            rax = (baseModuleAddr + 0x2BEA);
            rax = (~rax);
            rax *= rbx;
            r8 += rax;
            rax = rdx;
            rax -= rbx;
            r8 += rax;
            rax = 0xC6697C3201D16831;
            r8 *= rax;
            rax = 0xDEB8C087365B11DF;
            r8 *= rax;
            rax = 0x53B4B4D6EA2D421;
            r8 -= rax;
            return r8;
            break;
        }

        case 11: {
            rdi = (baseModuleAddr + 0x125);
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r9;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            rax = 0xFFFFFFFFFFFF430E;
            r8 += rax;
            rax = r8;
            rax >>= 0x27;
            r8 ^= rax;
            r8 -= rbx;
            rax = 0x2BE71A29B7359995;
            r8 *= rax;
            rax = 0x8A5CA30EC2ABDDE3;
            r8 *= rax;
            rax = (baseModuleAddr + 0xB35F);
            r8 -= rbx;
            rax = (~rax);
            r8 += rax;
            return r8;
            break;
        }

        case 12: {
            rbx = 0x4F7068D3729F37D3;
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rdi = (baseModuleAddr + 0x125);
//            rcx = read<QWORD>(rbp + 0x78);
//            rcx -= rdi;
//            rcx &= 0xffffffffc0000000;
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rax = rbx;
            rax = (~rax);
            rcx = (~rcx);
            rax ^= r8;
            r8 = (baseModuleAddr + 0x222E);
            rax ^= r8;
            r8 = read<QWORD>(rcx + 0x7);
            r8 *= rax;
            rax = baseModuleAddr;
            r8 -= rax;
            rax = r8;
            rax >>= 0x14;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x28;
            r8 ^= rax;
            rax = 0x4D476575EDEA0097;
            r8 *= rax;
            rax = 0x29E7CF34D82E0AE6;
            r8 += rax;
            r8 -= rbx;
            rax = baseModuleAddr;
            r8 -= rax;
            r8 -= 0x6E80;
            r8 ^= rbx;
            return r8;
            break;
        }

        case 13: {
            rdi = (baseModuleAddr + 0x125);
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = r8;
            rax >>= 0xB;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x16;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x2C;
            r8 ^= rax;
            rax = 0x43A4091A1FD69E66;
            r8 ^= rax;
            rax = 0xE37AF9E54D4D1B01;
            r8 *= rax;
            rax = r8;
            rax >>= 0x9;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x12;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x24;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x1D;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x3A;
            r8 ^= rax;
            r8 -= rbx;
            rax = r8;
            rax >>= 0x26;
            r8 ^= rax;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r9;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            return r8;
            break;
        }

        case 14: {
            rdi = (baseModuleAddr + 0x125);
            r11 = (baseModuleAddr + 0x82D2);
            r9 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rax = baseModuleAddr;
            r8 ^= rax;
            rax = 0xC8A8124C0418124F;
            r8 *= rax;
            rax = r8;
            rax >>= 0x25;
            rax ^= r8;
            r8 = rbx;
            rax += rbx;
            r8 ^= r11;
            r8 += rax;
            rax = 0xC8CB23E7D6644921;
            r8 *= rax;
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r9;
            rax = (~rax);
            rax = read<QWORD>(rax + 0x7);
            r8 *= rax;
            rax = 0x28025DABC97ACDB6;
            r8 += rax;
            return r8;
            break;
        }

        case 15: {
            r10 = read<QWORD>(baseModuleAddr + 0x5C5A1E8);
            rdi = (baseModuleAddr + 0x125);
            r12 = (baseModuleAddr + 0x49D04898);
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
//            rax = read<QWORD>(rbp + 0x78);
//            rax -= rdi;
//            rax &= 0xffffffffc0000000;
            rax = 0;
            rax <<= 0x10;
            rax ^= r10;
            rax = (~rax);
            r8 *= read<QWORD>(rax + 0x7);
            rax = r8;
            rax >>= 0x18;
            r8 ^= rax;
            rax = r8;
            rax >>= 0x30;
            r8 ^= rax;
            rax = (baseModuleAddr + 0xDADB);
            rax = (~rax);
            r8 -= rbx;
            r8 += rax;
            rax = rbx;
            rax *= r12;
            r8 ^= rax;
            rax = 0x74EC5FDF65D49351;
            r8 *= rax;
            rax = 0x1CC086208DEE8BC1;
            r8 += rax;
            rax = 0x7F682B1A85A68D41;
            r8 ^= rax;
            return r8;
            break;
        }
    }
    return 0;
}

auto get_bone_index(uint64_t index, uint64_t game_base_address) -> uint64_t {
    uint64_t RAX = 0, RBX = 0, RCX = 0, RDX = 0, R8 = 0, RDI = 0, R9 = 0, R10 = 0, R11 = 0, R12 = 0, R13 = 0, R14 = 0, RSI = 0, RSP = 0, RBP = 0;

    RBX = index;
    RCX = RBX * 0x13C8;
    RAX = 0xE16108FF1793EEB9;
    R11 = game_base_address;
    RAX = _umul128(RAX, RCX, &RDX);
    R10 = 0x8A63AB88AA8DD5E7;
    RDX >>= 0xD;
    RAX = RDX * 0x2459;
    RCX -= RAX;
    RAX = 0xBB4776D52876E0DD;
    R8 = RCX * 0x2459;
    RAX = _umul128(RAX, R8, &RDX);
    RDX >>= 0xD;
    RAX = RDX * 0x2BBE;
    R8 -= RAX;
    RAX = 0x889ABF4CB4E4EB53;
    RAX = _umul128(RAX, R8, &RDX);
    RAX = 0xCCCCCCCCCCCCCCCD;
    RDX >>= 0xA;
    RCX = RDX * 0x77F;
    RAX = _umul128(RAX, R8, &RDX);
    RDX >>= 0x3;
    RCX += RDX;
    RAX = RCX + RCX * 0x4;
    RAX <<= 0x2;
    RCX = R8 * 0x16;
    RCX -= RAX;
    RAX = read<uint16_t>(RCX + R11 + 0x5C5FB60);
    R8 = RAX * 0x13C8;
    RAX = R10;
    RAX = _umul128(RAX, R8, &RDX);
    RAX = R10;
    RDX >>= 0xC;
    RCX = RDX * 0x1D99;
    R8 -= RCX;
    R9 = R8 * 0x2B81;
    RAX = _umul128(RAX, R9, &RDX);
    RDX >>= 0xC;
    RAX = RDX * 0x1D99;
    R9 -= RAX;
    RAX = 0xBFA02FE80BFA02FF;
    RAX = _umul128(RAX, R9, &RDX);
    RAX = 0xFC0FC0FC0FC0FC1;
    RDX >>= 0x7;
    RCX = RDX * 0xAB;
    RAX = _umul128(RAX, R9, &RDX);
    RDX >>= 0x2;
    RCX += RDX;
    RAX = RCX * 0x82;
    RCX = R9 * 0x84;
    RCX -= RAX;
    RSI = read<uint16_t>(RCX + R11 + 0x5C69150);
    return RSI;
}