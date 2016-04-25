
unsigned int fpu_cw();

void fpu_set_cw(unsigned int cw);

void fpu_round_upward(); // Set rounding mode toward +infinity.

void fpu_round_downward(); // Set rounding mode toward -infinity.

void fpu_round_tonearest(); // Set rounding mode to nearest.

void fpu_round_truncate(); // Set rounding mode toward 0.

