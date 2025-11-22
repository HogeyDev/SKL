section .text
	mov rdx, 0
	push rdx
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [GLO1+0], rax ; assigned `heap_pointer`
global _malloc
_malloc:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `top`
	mov qword [rsp+0], 0
	mov rax, qword [GLO1+0]
	push rax ; recalled `heap_pointer`
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `top`
	mov rax, qword [GLO1+0]
	push rax ; recalled `heap_pointer`
	mov rax, qword [rbp+16]
	push rax ; recalled `bytes`
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [GLO1+0]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rax, qword [rbp-8]
	push rax ; recalled `top`
	pop rcx
	lea rdx, qword [GLO0+0+rcx*1] ; rcx is multiplied to adjust for type sizing
	mov rax, rdx
	push rax
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
global _free
_free:
	push rbp
	mov rbp, rsp
	mov rsp, rbp
	pop rbp
	ret
global _strlen
_strlen:
	push rbp
	mov rbp, rsp
	mov rsi, qword [rbp+16]
	mov rax, -1
.looper:
	inc rax
	cmp byte [rsi+rax], 0x00
	jne .looper
	mov rsp, rbp
	pop rbp
	ret
global _format
_format:
	push rbp
	mov rbp, rsp
	mov rsp, rbp
	pop rbp
	ret
global _print_char
_print_char:
	push rbp
	mov rbp, rsp
   movzx rax, byte [rbp+16]
    push rax
	mov rax, 1
	mov rdi, 1
	mov rdx, 1
	mov rsi, rsp
	syscall
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _print
_print:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `string_length`
	mov qword [rsp+0], 0
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	call _strlen
	add rsp, 8
	push rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `string_length`
	sub rsp, 8 ; stack reserved for `i`
	mov qword [rsp+0], 0
	mov rdx, 0
	push rdx
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-16], rax ; assigned `i`
lbl2:
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rax, qword [rbp-8]
	push rax ; recalled `string_length`
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl3
	sub rsp, 1 ; stack reserved for `c`
	mov byte [rsp+0], 0
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	pop rbx
	mov rax, qword [rbp+16]
	imul rbx, 1
	add rax, rbx
	sub rsp, 1 ; allocated space to push into
	mov al, byte [rax+0]
	mov byte [rsp+0], al
	mov al, byte [rsp]
	add rsp, 1
	mov byte [rbp-17], al ; assigned `c`
	mov al, byte [rbp-17]
	movzx rax, al
	push rax ; recalled `c`
	call _print_char
	add rsp, 8
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	jmp lbl2
lbl3:
	mov rsp, rbp
	pop rbp
	ret
global _println
_println:
	push rbp
	mov rbp, rsp
	mov rax, qword [rbp+16]
	push rax ; recalled `str`
	call _print
	add rsp, 8
	mov rdx, 10
	push rdx
	call _print_char
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _print_int
_print_int:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `a`
	mov qword [rsp+0], 0
	mov rax, qword [rbp+16]
	push rax ; recalled `num`
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `a`
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 0
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl4
	mov rdx, 45
	push rdx
	call _int_to_char
	add rsp, 8
	movzx rax, al
	push rax
	call _print_char
	add rsp, 8
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	pop rax
	neg rax
	push rax
	lea rdx, qword [rbp-8]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
lbl4:
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 9
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setg al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl5
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 10
	push rdx
	pop rbx
	pop rax
	mov rdx, 0
	div rbx
	push rax
	call _print_int
	add rsp, 8
lbl5:
	mov rdx, 48
	push rdx
	mov rax, qword [rbp-8]
	push rax ; recalled `a`
	mov rdx, 10
	push rdx
	pop rbx
	pop rax
	mov rdx, 0
	div rbx
	mov rax, rdx
	push rax
	pop rbx
	pop rax
	add rax, rbx
	push rax
	call _int_to_char
	add rsp, 8
	movzx rax, al
	push rax
	call _print_char
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _print_int_pointer
_print_int_pointer:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `as_int`
	mov qword [rsp+0], 0
	mov rdx, 0
	push rdx
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `as_int`
	mov rax, qword [rbp+16]
	mov qword [rbp-8], rax
	mov rax, qword [rbp-8]
	push rax ; recalled `as_int`
	call _print_int
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _exit
_exit:
	push rbp
	mov rbp, rsp
	mov rax, 60
	mov rdi, qword [rbp+16]
	syscall
	mov rsp, rbp
	pop rbp
	ret
global _int_to_usize
_int_to_usize:
	push rbp
	mov rbp, rsp
    mov rax, qword [rbp+16]
	mov rsp, rbp
	pop rbp
	ret
global _int_to_char
_int_to_char:
	push rbp
	mov rbp, rsp
    mov rax, qword [rbp+16]
	mov rsp, rbp
	pop rbp
	ret
global _read
_read:
	push rbp
	mov rbp, rsp
	mov rdx, STR0
	push rdx
	call _println
	add rsp, 8
	mov rdx, 1
	push rdx
	call _exit
	add rsp, 8
	mov rsp, rbp
	pop rbp
	ret
global _LinkedList_new
_LinkedList_new:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `list`
	mov qword [rsp+0], 0
	mov rdx, 16
	push rdx
	call _malloc
	add rsp, 8
	push rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `list`
	mov rdx, 0
	push rdx
	lea rdx, qword [rbp-8]
	mov rdx, qword [rdx]
	add rdx, 8 ; `first`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rdx, 0
	push rdx
	lea rdx, qword [rbp-8]
	mov rdx, qword [rdx]
	add rdx, 0 ; `size`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rax, qword [rbp-8]
	push rax ; recalled `list`
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
global _LinkedList_push
_LinkedList_push:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `node`
	mov qword [rsp+0], 0
	mov rdx, 16
	push rdx
	call _malloc
	add rsp, 8
	push rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `node`
	mov rax, qword [rbp+16]
	push rax ; recalled `value`
	lea rdx, qword [rbp-8]
	mov rdx, qword [rdx]
	add rdx, 8 ; `value`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rdx, 0
	push rdx
	lea rdx, qword [rbp-8]
	mov rdx, qword [rdx]
	add rdx, 0 ; `next`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 0 ; `size`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	mov rdx, 0
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setg al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl6
	sub rsp, 8 ; stack reserved for `end`
	mov qword [rsp+0], 0
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 8 ; `first`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-16], rax ; assigned `end`
lbl7:
	lea rdx, qword [rbp-16]
	mov rdx, qword [rdx]
	add rdx, 0 ; `next`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	mov rdx, 0
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setne al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl8
	lea rdx, qword [rbp-16]
	mov rdx, qword [rdx]
	add rdx, 0 ; `next`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	jmp lbl7
lbl8:
	mov rax, qword [rbp-8]
	push rax ; recalled `node`
	lea rdx, qword [rbp-16]
	mov rdx, qword [rdx]
	add rdx, 0 ; `next`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
lbl6:
	mov rax, qword [rbp-8]
	push rax ; recalled `node`
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 8 ; `first`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 0 ; `size`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 0 ; `size`
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rsp, rbp
	pop rbp
	ret
global _LinkedList_get
_LinkedList_get:
	push rbp
	mov rbp, rsp
	mov rax, qword [rbp+16]
	push rax ; recalled `index`
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 0 ; `size`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	pop rbx
	pop rax
	cmp rax, rbx
	setge al
	movzx rax, al
	push rax
	mov rax, qword [rbp+16]
	push rax ; recalled `index`
	mov rdx, 0
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rbx
	pop rax
	or rax, rbx
	push rax
	pop rax
	cmp rax, 0
	je lbl9
	mov rdx, STR1
	push rdx
	call _print
	add rsp, 8
	mov rax, qword [rbp+16]
	push rax ; recalled `index`
	call _print_int
	add rsp, 8
	mov rdx, STR2
	push rdx
	call _print
	add rsp, 8
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 0 ; `size`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	call _print_int
	add rsp, 8
	mov rdx, STR3
	push rdx
	call _println
	add rsp, 8
	mov rdx, 1
	push rdx
	call _exit
	add rsp, 8
lbl9:
	sub rsp, 8 ; stack reserved for `current`
	mov qword [rsp+0], 0
	lea rdx, qword [rbp+24]
	mov rdx, qword [rdx]
	add rdx, 8 ; `first`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `current`
	sub rsp, 8 ; stack reserved for `i`
	mov qword [rsp+0], 0
	mov rdx, 0
	push rdx
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-16], rax ; assigned `i`
lbl10:
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rax, qword [rbp+16]
	push rax ; recalled `index`
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl11
	lea rdx, qword [rbp-8]
	mov rdx, qword [rdx]
	add rdx, 0 ; `next`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	lea rdx, qword [rbp-8]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	jmp lbl10
lbl11:
	mov rax, qword [rbp-8]
	push rax ; recalled `current`
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret
global _main
_main:
	push rbp
	mov rbp, rsp
	sub rsp, 8 ; stack reserved for `list`
	mov qword [rsp+0], 0
	call _LinkedList_new
	add rsp, 0
	push rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-8], rax ; assigned `list`
	sub rsp, 8 ; stack reserved for `i`
	mov qword [rsp+0], 0
	mov rdx, 0
	push rdx
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-16], rax ; assigned `i`
lbl12:
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 32
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl13
	mov rax, qword [rbp-8]
	push rax ; recalled `list`
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	call _LinkedList_push
	add rsp, 16
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	jmp lbl12
lbl13:
	mov rdx, 0
	push rdx
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
lbl14:
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 32
	push rdx
	pop rbx
	pop rax
	cmp rax, rbx
	setl al
	movzx rax, al
	push rax
	pop rax
	cmp rax, 0
	je lbl15
	sub rsp, 8 ; stack reserved for `node`
	mov qword [rsp+0], 0
	mov rax, qword [rbp-8]
	push rax ; recalled `list`
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	call _LinkedList_get
	add rsp, 16
	push rax
	mov rax, qword [rsp]
	add rsp, 8
	mov qword [rbp-24], rax ; assigned `node`
	lea rdx, qword [rbp-24]
	mov rdx, qword [rdx]
	add rdx, 8 ; `value`
	sub rsp, 8 ; allocated space to push into
	mov rax, qword [rdx+0]
	mov qword [rsp+0], rax
	call _print_int
	add rsp, 8
	mov rdx, 10
	push rdx
	call _print_char
	add rsp, 8
	mov rax, qword [rbp-16]
	push rax ; recalled `i`
	mov rdx, 1
	push rdx
	pop rbx
	pop rax
	add rax, rbx
	push rax
	lea rdx, qword [rbp-16]
	mov rax, qword [rsp+0]
	mov qword [rdx+0], rax
	add rsp, 8 ; cleaned up stack
	jmp lbl14
lbl15:
	mov rdx, 0
	push rdx
	pop rax
	mov rsp, rbp
	pop rbp
	ret
	mov rsp, rbp
	pop rbp
	ret

global _start
_start:
	push rbp
	mov rbp, rsp
	call _main
	mov rdi, rax
	mov rax, 60
	syscall
section .data
	STR0 db "Bro thought I was new", 0
	STR1 db "Attempted to access index ", 0
	STR2 db " in LinkedList of size ", 0
	STR3 db ".", 0
	GLO0: resb 1048576
	GLO1: resb 8
