BITS 64
DEFAULT REL

GLOBAL main

EXTERN WriteFile
EXTERN GetStdHandle

SECTION .data

 l1  db "Hello world!", 13, 10, 0

SECTION .text

main:
 sub rsp, 30h
 lea rdx, [l1]
 call f1
 add rsp, 30h
 ret

f1: 
 mov ecx, -11
 call GetStdHandle
 mov ecx, eax
 mov r8d, DWORD 15
 xor r9, r9
 mov QWORD [rsp+20h], r9
 call WriteFile
 ret
