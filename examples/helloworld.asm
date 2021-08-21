 .target "8080"
 .format "bin"
 .org $100
 
bdos    .equ    $0005   ; BDOS entry point
start:  mvi    c,9      ; BDOS function: output string
        lxi    d,msg    ; address of msg
        call   bdos
        hlt             ; Stops execution
msg    .text   "Hello, world!$"
       .end
