# frounding-rs

A rust crate that help changing the rounding mode of floating point operations. Actually only
 SSE floating point operations are affected (make changes to the MXCSR register), x87 FPU support is planned.