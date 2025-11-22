    mov rax, 48930
    mov rbx, 56980
    mov rcx, rbx
    sub rcx, rax
tmplbl:
    mov rdx, 1
    jmp tmplbl
    hlt
