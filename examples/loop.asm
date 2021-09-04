.target "8080"
.format "bin"
.org $100

        mvi     a,$0f
@loop   dcr     a
        jnz     @loop
        ret
