#include "rounding_fpu.h"

unsigned int fpu_cw() {
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"fstcw %0;"
			:"=m"(cw)
			:
			:
	);
	return (cw);
}

void fpu_set_cw(unsigned int cw) {
	__asm__ __volatile__ (
			"fldcw %0;"
			:
			:"m"(cw)
			:
	);
}

void fpu_round_upward() { // Set rounding mode toward +infinity.
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"xor %%eax,%%eax;"
			"fstcw %0;"
			"mov %0,%%eax;"
			"and $0xf3ff,%%eax;"
			"or $0x800, %%eax;"
			"mov %%eax,%0;"
			"fldcw %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}

void fpu_round_downward() { // Set rounding mode toward -infinity.
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"xor %%eax,%%eax;"
			"fstcw %0;"
			"mov %0,%%eax;"
			"and $0xf3ff,%%eax;"
			"or $0x400, %%eax;"
			"mov %%eax,%0;"
			"fldcw %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}

void fpu_round_tonearest() { // Set rounding mode to nearest.
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"xor %%eax,%%eax;"
			"fstcw %0;"
			"mov %0,%%eax;"
			"and $0xf3ff,%%eax;"
			"mov %%eax,%0;"
			"fldcw %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}

void fpu_round_truncate() { // Set rounding mode toward 0.
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"xor %%eax,%%eax;"
			"fstcw %0;"
			"mov %0,%%eax;"
			"and $0xf3ff,%%eax;"
			"or $0xc00, %%eax;"
			"mov %%eax,%0;"
			"fldcw %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}
