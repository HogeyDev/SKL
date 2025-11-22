    mov rax, 512
    mov rbx, 0
lbl0:
    sub rax, 1
    add rbx, 1
    cmp rax, 0
    jne lbl0
    hlt
