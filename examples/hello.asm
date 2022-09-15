BITS 64
DEFAULT REL

GLOBAL main

EXTERN WriteFile
EXTERN GetStdHandle

STD_OUTPUT_HANDLE   EQU -11

SECTION .data

 l1  db "Hello world!", 13, 10, 0
 l2  dd $-l1

SECTION .text

main:
 sub rsp, 30h

 mov ecx, STD_OUTPUT_HANDLE
 call GetStdHandle
 
 mov ecx, eax
 lea rdx, [l1]
 mov r8d, DWORD [l2]
 xor r9, r9
 mov QWORD [rsp+20h], r9
 call WriteFile

 add rsp, 30h
 ret  