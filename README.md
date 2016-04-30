# frounding-rs

Build status ![travis build status](https://travis-ci.org/houssemDevs/frounding-rs.svg?branch=master)

A rust crate that help changing the rounding mode of floating point operations. Actually only
 SSE floating point operations are affected (make changes to the MXCSR register), x87 FPU support is planned.