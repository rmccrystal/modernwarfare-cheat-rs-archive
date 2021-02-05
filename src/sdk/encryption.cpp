#include "encryption.h"
#include <iostream>

extern auto read_qword(uint64_t address) -> uint64_t;

auto decrypt_client_info(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    uint64_t rax, rbx, rcx, rdx, r8, rdi, r9, r10, r11, r12, r13, r14 = 0;
    rbx = encrypted_address;

    rcx = (game_base_address + 0xDAE);
    //rax -= rcx;
    rcx = 0xCAE24248DB9B9E3D;
//    rax &= 0xffffffffc0000000;
//    rax <<= 0x10;
//    rax ^= RVPM<UINT64>(processBaseAddress + 0x5B430EA);
//    rax = (~rax);
//    rbx *= RVPM<UINT64>(rax + 0xd);
    rbx *= last_key;
    rax = rbx;
    rax >>= 0x21;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x7;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0xE;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x1C;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x38;
    rbx ^= rax;
    rax = 0x6E0942B8DC6C7F2A;
    rbx *= rcx;
    rbx += rax;
    rax = rbx;
    rax >>= 0x2;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x4;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x8;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x10;
    rbx ^= rax;
    rax = rbx;
    rax >>= 0x20;
    rbx ^= rax;

    return rbx;
}

auto decrypt_client_base(uint64_t encrypted_address, uint64_t game_base_address, uint64_t last_key,
                         uint64_t peb) -> uint64_t {
    QWORD rax, rbx, rcx, rdx, rdi, r8, r9, r10, r11, r12, r13, r14 = 0;

    rbx = peb; // OR ~peb?
    rax = encrypted_address;

    const auto g_decryptCase = _byteswap_uint64(peb << 0x24) & 0xf;

    switch (g_decryptCase) {
        case 0: {
            r10 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            rcx = rax;
            rcx >>= 0x23;
            rax ^= rcx;
            rcx = rbx;
            rcx = (~rcx);
            rcx -= rdi;
            rdx = rcx + 0xffffffffba07610c;
            rcx += 0xFFFFFFFFD979F6D5;
            rdx ^= rcx;
            rax ^= rdx;
            rcx = 0xCC0B2FC1F232E415;
            rax *= rcx;
            rcx = 0x673439B330235FEC;
            rax ^= rcx;
            rcx = 0x6B7C61D6B947965E;
            rax += rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rax ^= rbx;

            return rax;

            break;

        }

        case 1: {
            rdi = game_base_address;
            //nop
            r9 = read_qword(game_base_address + 0x5B4311C);
            rax ^= rdi;
            rcx = 0xA19EE1C1C83655FB;
            rax *= rcx;
            rcx = 0xC3A073EF2DF80321;
            rax *= rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rax ^= rbx;
            rcx = 0x3F975DD3813B0D89;
            rax += rcx;
            rcx = rax;
            rcx >>= 0x9;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x12;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x24;
            rax ^= rcx;
            rax -= rdi;

            return rax;

            break;

        }

        case 2: {
            r10 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            rdx = rbx;
            rdx = (~rdx);
            rcx = (game_base_address + 0x1B3162AB);
            rcx = (~rcx);
            rax += rcx;
            rax += rdx;
            rax -= rdi;
            rax += 0xFFFFFFFFA26593A7;
            rax += rbx;
            rcx = 0x4C5F6255CE87BC60;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1A;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x34;
            rax ^= rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = _byteswap_uint64(rcx);
            rcx = read_qword(rcx + 0xd);
            rax *= rcx;
            rcx = 0xEBF2F7F98A2ACD69;
            rax ^= rcx;
            rcx = 0x4991E273E0129B3B;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x1B;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x36;
            rax ^= rcx;

            return rax;

            break;

        }

        case 3: {
            rdi = game_base_address;
            //nop
            r11 = (game_base_address + 0x3A54E233);
            r9 = read_qword(game_base_address + 0x5B4311C);
            rcx = rdi + 0x389b6554;
            rcx += rbx;
            rax ^= rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rcx = rbx;
            rcx *= r11;
            rax += rcx;
            rcx = rbx;
            rcx = (~rcx);
            rcx ^= read_qword(game_base_address + 0x178175D);
            rax -= rcx;
            rcx = rax;
            rcx >>= 0x16;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2C;
            rax ^= rcx;
            rcx = 0x7FB262CD15417417;
            rax *= rcx;
            rcx = 0x617DF683E6D98F69;
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

            return rax;

            break;

        }

        case 4: {
            rdi = game_base_address;
            //nop
            r10 = read_qword(game_base_address + 0x5B4311C);
            rax ^= rdi;
            rcx = rax;
            rcx >>= 0x23;
            rax ^= rcx;
            rcx = rbx;
            rcx ^= read_qword(game_base_address + 0x1781A7F);
            rax -= rcx;
            rcx = 0xB240E0666218BBF6;
            rax ^= rcx;
            rcx = (game_base_address + 0x31CE17EA);
            rcx = (~rcx);
            rcx += rbx;
            rax ^= rcx;
            rcx = 0x7F554C7195F237B7;
            rax *= rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);

            return rax;

            break;

        }

        case 5: {
            r10 = read_qword(game_base_address + 0x5B4311C);
            //nop
            rdx = (game_base_address + 0x9B4E);
            rcx = rax;
            rcx >>= 0x28;
            rax ^= rcx;
            rcx = 0xA5F2421AAD0E251;
            rax *= rcx;
            rcx = 0x7874B358DFAA4832;
            rax += rcx;
            rcx = 0x79ED618DDDD08F2F;
            rax += rcx;
            rcx = rbx;
            rcx = (~rcx);
            rcx += rdx;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x12;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x24;
            rax ^= rcx;
            //rdx = read_qword(rbp + 0xe8);
            rcx = rbx;
            //rdx -= rsi;
            rcx = (~rcx);
            rdx = 0;
            rcx ^= rax;
            rdx <<= 0x10;
            rdx ^= r10;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0xd);
            rax *= rcx;

            return rax;

            break;

        }

        case 6: {
            r9 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            rcx = (game_base_address + 0xCACD);
            rcx = (~rcx);
            rcx *= rbx;
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
            rcx = rax;
            rcx >>= 0x15;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2A;
            rax ^= rcx;
            rcx = 0xF8A0C0CE66D9CE59;
            rax -= rdi;
            rax *= rcx;
            rcx = 0x3304E81FB2A659CE;
            rax += rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rcx = read_qword(rcx + 0xd);
            rax *= rcx;
            rcx = 0xFA53D6B40F712986;
            rax ^= rcx;

            return rax;

            break;

        }

        case 7: {
            r9 = read_qword(game_base_address + 0x5B4311C);
            //nop
            rcx = rax;
            rcx >>= 0x25;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1B;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x36;
            rax ^= rcx;
            rcx = 0x7DC722B365226A2C;
            rax ^= rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rcx = rax;
            rcx >>= 0x25;
            rax ^= rcx;
            rcx = 0x3B4B1A242FE71717;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x19;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x32;
            rax ^= rcx;
            rcx = 0xC8C297441A47011D;
            rax ^= rcx;

            return rax;

            break;

        }

        case 8: {
            r10 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            rdx = (game_base_address + 0x5DB4A9AF);
            rcx = rbx;
            rcx ^= rdx;
            rax += rcx;
            rcx = rax;
            rcx >>= 0x17;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2E;
            rax ^= rcx;
            rax += rdi;
            rax += rbx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rdx = rbx;
            rdx = (~rdx);
            rcx = (game_base_address + 0x4E84AAE1);
            rcx = (~rcx);
            rdx *= rcx;
            rcx = rax;
            rax = 0x296F71B4532510C1;
            rax *= rcx;
            rax += rdx;
            rax -= rdi;

            return rax;

            break;

        }

        case 9: {
            rdi = game_base_address;
            //nop
            r9 = read_qword(game_base_address + 0x5B4311C);
            rcx = rax;
            rcx >>= 0x22;
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
            rcx = rax;
            rcx >>= 0x20;
            rax ^= rcx;
            rax ^= rbx;
            rcx = 0xD47081853C209C9;
            rax -= rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rcx = 0x4B5E7DF7E48CB0FF;
            rax *= rcx;
            rcx = 0xE8485558E64A1835;
            rax += rcx;
            rax += rdi;

            return rax;

            break;

        }

        case 10: {
            rdi = game_base_address;
            //nop
            rdx = read_qword(game_base_address + 0x5B4311C);
            rcx = rax;
            rcx >>= 0x1B;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x36;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x11;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x22;
            rax ^= rcx;
            rcx = 0x1D00277E9810B328;
            rax ^= rcx;
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
            rcx = rax;
            rcx >>= 0x20;
            rax ^= rcx;
            rcx = rdi + 0x4abe5f34;
            rcx += rbx;
            rax += rcx;
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= rdx;
            rcx = _byteswap_uint64(rcx);
            rcx = read_qword(rcx + 0xd);
            rcx *= 0xD76BA1704984D279;
            rax *= rcx;

            return rax;

            break;

        }

        case 11: {
            r11 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            r8 = (game_base_address + 0x6D482CE3);
            rcx = r8;
            //rdx = read_qword(rbp + 0xe8);
            rcx = (~rcx);
            //rdx -= rsi;
            rcx += rbx;
            rcx ^= rax;
            rdx = 0;
            rdx <<= 0x10;
            rax = 0x52981C2050FA4F22;
            rcx ^= rax;
            rdx ^= r11;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0xd);
            rax *= rcx;
            rcx = 0x35AF355E6C020F;
            rax *= rcx;
            rcx = rax;
            rcx >>= 0x23;
            rax ^= rcx;
            rax += rbx;
            rcx = (game_base_address + 0x5879);
            rax += rcx;
            rax ^= rdi;

            return rax;

            break;

        }

        case 12: {
            rdi = game_base_address;
            //nop
            r9 = read_qword(game_base_address + 0x5B4311C);
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r9;
            rcx = _byteswap_uint64(rcx);
            rax *= read_qword(rcx + 0xd);
            rcx = rax;
            rcx >>= 0xD;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1A;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x34;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0xB;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x16;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2C;
            rax ^= rcx;
            rax ^= rdi;
            rcx = rbx;
            rcx ^= read_qword(game_base_address + 0x1783E99);
            rax -= rcx;
            rcx = 0x4DC3AD3F48A93C57;
            rax *= rcx;
            rcx = 0x1DF312C73486CBC0;
            rax += rcx;
            rax ^= rbx;
            rcx = (game_base_address + 0x5EAA7860);
            rax ^= rcx;

            return rax;

            break;

        }

        case 13: {
            r10 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            //nop
            //nop
            rcx = 0;
            rcx <<= 0x10;
            rcx ^= r10;
            rcx = _byteswap_uint64(rcx);
            rcx = read_qword(rcx + 0xd);
            rcx *= 0x447CBAE743A18DB5;
            rax *= rcx;
            rax ^= rbx;
            rax ^= rdi;
            rax += rbx;
            rcx = rax;
            rcx >>= 0xF;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x1E;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x3C;
            rax ^= rcx;
            rax += rbx;
            rax ^= rbx;

            return rax;

            break;

        }

        case 14: {
            rdi = game_base_address;
            //nop
            r10 = read_qword(game_base_address + 0x5B4311C);
            rcx = rax;
            rcx >>= 0x3;
            rax ^= rcx;
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
            rcx = rax;
            rcx >>= 0x16;
            rax ^= rcx;
            rcx = rax;
            rcx >>= 0x2C;
            rax ^= rcx;
            rcx = 0x3821D34108A8DD68;
            rax -= rcx;
            //rdx = read_qword(rbp + 0xe8);
            //rdx -= rsi;
            rdx = 0;
            rcx = 0xA493EA00C24E6085;
            rcx *= rax;
            rdx <<= 0x10;
            rcx -= rbx;
            rdx ^= r10;
            rcx -= rdi;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0xd);
            rax *= rcx;
            rcx = 0x592808E400697AB3;
            rax += rcx;

            return rax;

            break;

        }

        case 15: {
            r10 = read_qword(game_base_address + 0x5B4311C);
            rdi = game_base_address;
            //nop
            r12 = (game_base_address + 0x11499A40);
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
            //rdx = read_qword(rbp + 0xe8);
            rcx >>= 0x20;
            //rdx -= rsi;
            rcx ^= rax;
            rdx = 0;
            rcx += rbx;
            rdx <<= 0x10;
            rdx ^= r10;
            rax = (game_base_address + 0x7F2B5AE2);
            rcx += rax;
            rdx = _byteswap_uint64(rdx);
            rax = read_qword(rdx + 0xd);
            rax *= rcx;
            rcx = 0x2D383F6C0FA9B0A8;
            rax -= rcx;
            rdx = rbx;
            rdx ^= r12;
            rax += rdx;
            rcx = rdi + 0x6437;
            rcx += rbx;
            rax ^= rcx;
            rcx = 0xE46D884DA5594AD1;
            rax ^= rcx;
            rcx = 0x2FD844FCD06F92F7;
            rax *= rcx;

            return rax;

            break;
        }
    }
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
