.target "8080"
.format "bin"

        .org 0
        lxi     sp,$ff00
        mvi     a,$0f
        jmp     start

int     .org    $8
        sub     a
        ret

start   .org $9700
        ei
@loop   cmp     a
        jnz     @loop
        ret
        .end
        