 .target "8080"
 .format "bin"
 .org $100              ; Useless : "bin" format has no header
 
bdos    .equ    $0005   ; BDOS entry point
start:  mvi    c,9      ; BDOS function: output string
        lxi    d,msg    ; address of msg
        call   bdos
        ret             ; return to CCP
msg    .text   "Hello, world!$"
       .end
