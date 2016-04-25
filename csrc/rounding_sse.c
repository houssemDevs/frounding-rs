#include "rounding_sse.h"

unsigned int sse_cw() {
	unsigned int cw = 0;
	__asm__ __volatile__(
			"stmxcsr %0;"
			:"=m"(cw)
			:
			:
	);
	return (cw);
}

void sse_set_cw(unsigned int cw) {
	__asm__ __volatile__ (
			"ldmxcsr %0;"
			:
			:"m"(cw)
			:
	);
}

void sse_round_upward() {
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"xor %%eax,%%eax;"
			"stmxcsr %0;"
			"mov %0, %%eax;"
			"and $0x9fff,%%eax;"
			"or $0x4000,%%eax;"
			"mov %%eax,%0;"
			"ldmxcsr %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}

void sse_round_downward() {
	unsigned int cw = 0;
	__asm__ __volatile__(
			"xor %%eax,%%eax;"
			"stmxcsr %0;"
			"mov %0, %%eax;"
			"and $0x9fff,%%eax;"
			"or $0x2000,%%eax;"
			"mov %%eax,%0;"
			"ldmxcsr %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}

void sse_round_tonearest() {
	unsigned int cw = 0;
	__asm__ __volatile__(
			"xor %%eax,%%eax;"
			"stmxcsr %0;"
			"mov %0, %%eax;"
			"and $0x9fff,%%eax;"
			"mov %%eax,%0;"
			"ldmxcsr %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}

void sse_round_truncate() {
	unsigned int cw = 0;
	__asm__ __volatile__ (
			"xor %%eax,%%eax;"
			"stmxcsr %0;"
			"mov %0, %%eax;"
			"and $0x9fff,%%eax;"
			"or $0x6000,%%eax;"
			"mov %%eax,%0;"
			"ldmxcsr %0;"
			:"=m"(cw)
			:"m"(cw)
			:"%eax"
	);
}
