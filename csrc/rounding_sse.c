#include "rounding_sse.h"

b32 sse_cw() {
	b32 cw = 0;
	__asm__ __volatile__(
			"stmxcsr %0;"
			:"=m"(cw)
			:
			:
	);
	return (cw);
}

set_sse_cw(b32 cw) {
	__asm__ __volatile__ (
			"ldmxcsr %0;"
			:
			:"m"(cw)
			:
	);
}

void round_upward() {
	b32 cw = 0;
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

void round_downward() {
	b32 cw = 0;
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

void round_tonearest() {
	b32 cw = 0;
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

void round_truncate() {
	b32 cw = 0;
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
