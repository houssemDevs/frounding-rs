


unsigned int sse_cw(); // Get the content of the MXCSR register.

void sse_set_cw(unsigned int cw); // Set the content of the MXCSR register.

void sse_round_upward(); // Set rounding mode toward +infinity.

void sse_round_downward(); // Set rounding mode toward -infinity.

void sse_round_tonearest(); // Set rounding mode to nearest.

void sse_round_truncate(); // Set rounding mode to truncate mode.
