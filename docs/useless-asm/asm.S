	.file	"asm.c"
	.intel_syntax noprefix
	.text
	.globl	@fastcall_1@8
	.def	@fastcall_1@8;	.scl	2;	.type	32;	.endef
@fastcall_1@8:
	push	ebp
	mov	ebp, esp
	sub	esp, 8
	mov	DWORD PTR [ebp-4], ecx
	mov	DWORD PTR [ebp-8], edx
	mov	edx, DWORD PTR [ebp-4]
	mov	eax, DWORD PTR [ebp-8]
	add	eax, edx
	leave
	ret
	.globl	_cdecall_1
	.def	_cdecall_1;	.scl	2;	.type	32;	.endef
_cdecall_1:
	push	ebp
	mov	ebp, esp
	mov	edx, DWORD PTR [ebp+8]
	mov	eax, DWORD PTR [ebp+12]
	add	eax, edx
	pop	ebp
	ret
	.globl	_stdcall_1@8
	.def	_stdcall_1@8;	.scl	2;	.type	32;	.endef
_stdcall_1@8:
	push	ebp
	mov	ebp, esp
	mov	edx, DWORD PTR [ebp+8]
	mov	eax, DWORD PTR [ebp+12]
	add	eax, edx
	pop	ebp
	ret	8
	.def	___main;	.scl	2;	.type	32;	.endef
	.globl	_main
	.def	_main;	.scl	2;	.type	32;	.endef
_main:
	push	ebp
	mov	ebp, esp
	and	esp, -16
	sub	esp, 16
	call	___main
	mov	DWORD PTR [esp+12], 1
	mov	eax, DWORD PTR [esp+12]
	add	eax, 1
	mov	DWORD PTR [esp+8], eax
	mov	eax, 0
	leave
	ret
	.ident	"GCC: (x86_64-win32-seh-rev1, Built by MinGW-Builds project) 13.2.0"
