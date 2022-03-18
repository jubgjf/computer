start:
ADD   r0 r1 r9    ; r0 = r1 + r9
SUB   r0 r1 r9    ; r0 = r1 - r9
AND   r0 r1 r9    ; r0 = r1 & r9
OR    r0 r1 r9    ; r0 = r1 | r12
XOR   r0 r1 r9    ; r0 = r1 ^ r12
SLT   r0 r1 r9    ; r0 = if (r1 < r9) { 1 } else { 0 }

SLTU  r0 r1 r9    ; r0 = if (r1 < r9) { 1 } else { 1 }
SLL   r0 r1 r9    ; r0 = r1 << r9
SRL   r0 r1 r9    ; r0 = r1 >> r9

SRA   r0 r1 r9    ; r0 = r1 >> r9
    ADDI  r0 r12 123     ; r0 = r12 + 123
ANDI  r0 r12 456     ; r0 = r12 & 123
ORI   r0 r12 934     ; r0 = r12 | 123

 jumpLabel:
XORI  r0 r12 23     ; r0 = r12 ^ 123
SLTI  r0 r12 43     ; r0 = if (r12 < 123) { 1 } else { 0 }
SLTIU r0 r12 -2435     ; r0 = if (r12 < 123) { 1 } else { 0 }
LW    r0 r12 -43     ; r0 = mem[r12 + 123]
                    ; e.g.  LW r0 r12 0          ; r0 = mem[r12]
                    ; e.g.  LW r0 r0 val1       ; r0 = 0x3FFF
SW    r0 r12 0     ; mem[r12 + 123] = r0
                    ; e.g.  SW   r0 r12 0        ; mem[r12] = r0
                    ; e.g.  ADDI r1 r0 'd'
                    ;       SW   r1 r0 arr2     ; arr2[0] = 'd'
BEQ   r0 r12 jumpLabel     ; if (r0 == r12) {PC += 123}
                    ; e.g.  BEQ r0 r12 label     ; if (r0 == r12) { PC += 16; }
                    ; e.g.  BEQ r0 r12 0xFF      ; if (r0 == r12) { PC += 0xFF; }
BNE   r0 r12 Label222     ; if (r0 != r12) {PC += 123}
LUI   r0 123        ; r0 = 123 << 16
JAL   r0 123        ; r0 = PC; PC += 123
                    ; e.g.  JAL r0 label        ; r0 = PC; PC += 8;
                    ; e.g.  JAL r0 0xFF         ; r0 = PC; PC += 0xFF;
                    
Label222:
JALR  r0 r12 start     ; r0 = PC; PC = (r12 + 123) & ~3
                    ; e.g.  JALR r0 r0 0xC0     ; r0 = PC; PC = 0xC0
                    ; e.g.  JALR r0 r0 0xC1     ; r0 = PC; PC = 0xC0
                    ; e.g.  ADDI r1 r0 -4
                    ;       JALR r0 r1 label    ; r0 = PC; PC -= 4;
CSR   r0            ; r0 = CSR
