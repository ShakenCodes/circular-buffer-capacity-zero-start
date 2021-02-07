This is an attempt at the CircularBuffer kata using the ZOMBIES approach.
* Exercise starting at page 19,here: https://www.wingman-sw.com/slides/EmbeddedTdd-Grenning-v3r1.key.pdf
* James Grenning describes ZOMBIES at: http://blog.wingman-sw.com/tdd-guided-by-zombies

Our team had a discussion of the "zero" step of the ZOMBIES process and identified two different approaches.
One idea is that "zero" starts with the a buffer of capacity zero and elaborate the entire interface with this simple case.
The other idea is that "zero" starts with a buffer of capacity one and empty.

I am trying both approaches for comparison.
* Capacity zero start is at: https://github.com/ShakenCodes/circular-buffer-capacity-zero-start
* Capacity one start is at: https://github.com/ShakenCodes/circular-buffer-capacity-one-start
