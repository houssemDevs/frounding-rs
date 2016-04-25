

#define b32 unsigned int


b32 sse_cw(); // Get the content of the MXCSR register.

void set_sse_cw(b32 cw); // Set the content of the MXCSR register.

void round_upward(); // Set rounding mode toward +infinity.

void round_downward(); // Set rounding mode toward -infinity.

void round_tonearest(); // Set rounding mode to nearest.

void round_truncate(); // Set rounding mode to truncate mode.
