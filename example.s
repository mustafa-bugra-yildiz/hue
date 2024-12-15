msg:
  adr X0, .lit0
  ret

main:
  mov X8, #1
  mov X9, #2
  add X8, X8, X9
  mov X9, #3
  sub X0, X8, X9
  ret

.lit0: .ascii "hello world"

