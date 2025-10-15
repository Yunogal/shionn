**x86-64 Assembly Language Summary**  
Dr. Orion Lawlor, last update 2019-10-14

These are all the normal x86-64 registers accessible from user code:

| Name  | Notes                                                                                                                        | Type        | 64-bit long | 32-bit int | 16-bit short | 8-bit char  |
| :---- | :--------------------------------------------------------------------------------------------------------------------------- | :---------- | :---------- | :--------- | :----------- | :---------- |
| rax   | Values are returned from functions in this register.                                                                         | scratch     | rax         | eax        | ax           | ah and al   |
| rcx   | Typical scratch register. Some instructions also use it as a counter.                                                        | scratch     | rcx         | ecx        | cx           | ch and cl   |
| rdx   | Scratch register.                                                                                                            | scratch     | rdx         | edx        | dx           | dh and dl   |
| _rbx_ | _Preserved register: don't use it without saving it\!_                                                                       | _preserved_ | _rbx_       | _ebx_      | _bx_         | _bh and bl_ |
| _rsp_ | _The stack pointer. Points to the top of the stack._                                                                         | _preserved_ | _rsp_       | _esp_      | _sp_         | _spl_       |
| _rbp_ | _Preserved register. Sometimes used to store the old value of the stack pointer, or the "base"._                             | _preserved_ | _rbp_       | _ebp_      | _bp_         | _bpl_       |
| rsi   | Scratch register. Also used to pass function argument \#2 in 64-bit Linux. String instructions treat it as a source pointer. | scratch     | rsi         | esi        | si           | sil         |
| rdi   | Scratch register. Function argument \#1 in 64-bit Linux. String instructions treat it as a destination pointer.              | scratch     | rdi         | edi        | di           | dil         |
| r8    | Scratch register. These were added in 64-bit mode, so they have numbers, not names.                                          | scratch     | r8          | r8d        | r8w          | r8b         |
| r9    | Scratch register.                                                                                                            | scratch     | r9          | r9d        | r9w          | r9b         |
| r10   | Scratch register.                                                                                                            | scratch     | r10         | r10d       | r10w         | r10b        |
| r11   | Scratch register.                                                                                                            | scratch     | r11         | r11d       | r11w         | r11b        |
| _r12_ | _Preserved register. You can use it, but you need to save and restore it._                                                   | _preserved_ | _r12_       | _r12d_     | _r12w_       | _r12b_      |
| _r13_ | _Preserved register._                                                                                                        | _preserved_ | _r13_       | _r13d_     | _r13w_       | _r13b_      |
| _r14_ | _Preserved register._                                                                                                        | _preserved_ | _r14_       | _r14d_     | _r14w_       | _r14b_      |
| _r15_ | _Preserved register._                                                                                                        | _preserved_ | _r15_       | _r15d_     | _r15w_       | _r15b_      |

Functions return values in rax.  
How you pass parameters into functions varies depending on the platform:

![](./asm.png)

-   A 64 bit x86 Linux machine, like NetRun:

    -   Call nasm like: nasm \-f elf64 yourCode.asm
    -   [Function parameters](https://en.wikipedia.org/wiki/X86_calling_conventions#System_V_AMD64_ABI) go in registers rdi, rsi, rdx, rcx, r8, and r9. Any additional parameters get pushed on the stack. OS X in 64 bit uses the same parameter scheme.
    -   Linux 64 floating-point parameters and return values go in xmm0. All xmm0 registers are scratch.
    -   If the function takes a variable number of arguments, like printf, rax is the number of floating point arguments (often zero).
    -   If the function modifies floating point values, you need to align the stack to a 16-byte boundary before making the call.
    -   Example linux 64-bit function call:

        extern putchar  
        mov rdi,'H' ; function parameter: one char to print  
        call putchar

-   Windows in 64 bit x86 is quite different:

    -   Call nasm like: nasm \-f win64 \-gcv8 yourCode.asm
    -   Win64 [function parameters](https://en.wikipedia.org/wiki/X86_calling_conventions#Microsoft_x64_calling_convention) go in registers rcx, rdx, r8, and r9.
    -   Win64 treats the registers rdi and rsi as preserved.
    -   Win64 passes float arguments in xmm0-3, but the register matches the location in the argument list (Linux passes the first float in xmm0 no matter which argument number it is.)
    -   Win64 floating point registers xmm6-15 are preserved.
    -   Win64 functions assume you've allocated 32 bytes of stack space to store the four parameter registers (called the "Parameter Home Area" or "Shadow Space"), plus another 8 bytes to align the stack to a 16-byte boundary.

        sub rsp,32+8; parameter area, and stack alignment  
        extern putchar  
        mov rcx,'H' ; function parameter: one char to print  
        call putchar  
        add rsp,32+8 ; clean up stack

    -   Some functions such as printf only get linked if they're called from C/C++ code, so to call printf from assembly, you need to include at least one call to printf from the C/C++ too.
    -   If you use the Windows MinGW or Visual Studio C++ compiler, "long" is the same size as "int", only 32 bits / 4 bytes even in 64-bit mode. You need to use "long long" to get a 64 bit / 8 byte integer variable on these systems. (Even on Windows, gcc, g++, or WSL makes "long" 64 bits, just like Linux or Mac or Java.) It's probably safest to [\#include \<stdint.h\>](https://en.cppreference.com/w/c/types/integer) and refer to int64_t.
    -   See [NASM assembly in 64-bit Windows in Visual Studio](https://www.cs.uaf.edu/2017/fall/cs301/reference/nasm_vs/) to make linking work.
    -   If you use the Microsoft MASM assembler, memory accesses must include "PTR", like "DWORD PTR \[rsp\]".
        -   I have some [notes on Windows Visual Studio \+ MASM assembly](https://docs.google.com/document/d/1A70BO5UDw80FdLHSbX4iw4CHTHJYeBCaUVESidAhIbY/edit). (advantage: breakpoints work even in assembly. Disadvantage: Microsoft everything).

-   In 32 bit mode, parameters are passed by pushing them onto the stack in reverse order, so the function's first parameter is on top of the stack before making the call. In 32-bit mode Windows and OS X compilers also seem to add an underscore before the name of a user-defined function, so if you call a function foo from C/C++, you need to define it in assembly as "\_foo".

## **Constants, Registers, Memory**

"12" means decimal 12; "0xF0" is hex. "some_function" is the address of the first instruction of the function. Memory access (use register as pointer): "\[rax\]". Same as C "\*rax".  
Memory access with offset (use register \+ offset as pointer): "\[rax+4\]". Same as C "\*(rax+4)".  
Memory access with scaled index (register \+ another register \* scale): "\[rax+rbx\*4\]". Same as C "\*(rax+rbx\*4)".

Different C++ datatypes get stored in different sized registers, and need to be accessed differently:

| C/C++ datatype | Bits | Bytes | Register | Access memory \*ptr | Access Array ptr\[idx\] | Allocate Static Memory                                    |
| :------------- | :--- | :---- | :------- | :------------------ | :---------------------- | :-------------------------------------------------------- |
| char           | 8    | 1     | al       | BYTE \[ptr\]        | BYTE \[ptr \+ 1\*idx\]  | [db](https://www.nasm.us/doc/nasmdoc3.html#section-3.2.1) |
| short          | 16   | 2     | ax       | WORD \[ptr\]        | WORD \[ptr \+ 2\*idx\]  | dw                                                        |
| int            | 32   | 4     | eax      | DWORD \[ptr\]       | DWORD \[ptr \+ 4\*idx\] | dd                                                        |
| long \[1\]     | 64   | 8     | rax      | QWORD \[ptr\]       | QWORD \[ptr \+ 8\*idx\] | dq                                                        |
| float          | 32   | 4     | xmm0     | DWORD \[ptr\]       | DWORD \[ptr \+ 4\*idx\] | dd                                                        |
| double         | 64   | 8     | xmm0     | QWORD \[ptr\]       | QWORD \[ptr \+ 8\*idx\] | dq                                                        |

\[1\] It's "long long" or "int64_t" on Windows MinGW or Visual Studio; but just "long" everywhere else.

You can convert values between different register sizes using different mov instructions:

|            | Source Size |                                                                                                                                                                                                                                                                                                           |                                                                                                                                                                                                                                                                                                                                           |                                                                                                                                                                                                                                                                                                                                           |                                           |
| :--------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------- |
|            | 64 bit rcx  | 32 bit ecx                                                                                                                                                                                                                                                                                                | 16 bit cx                                                                                                                                                                                                                                                                                                                                 | 8 bit cl                                                                                                                                                                                                                                                                                                                                  | **Notes**                                 |
| 64 bit rax | mov rax,rcx | [movsxd](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rcx%2C0xaabbccddeeff0011%0D%0Amovsxd%20rax%2Cecx%0D%0Aret%0D%0A&lang=Assembly-NASM&mach=x64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Disassemble&orun=Grade&ocompile=Optimize&ocompile=Warnings) rax,ecx | [movsx](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rcx%2C0xaaeeccddeeffaabb%0D%0Amovsx%20rax%2Ccx%0D%0Aret%0D%0A&lang=Assembly-NASM&mach=x64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Disassemble&orun=Grade&ocompile=Optimize&ocompile=Warnings) rax,cx                                     | [movsx](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rcx%2C0xaaeeccddeeffaabb%0D%0Amovsx%20rax%2Ccl%0D%0Aret%0D%0A&lang=Assembly-NASM&mach=x64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Disassemble&orun=Grade&ocompile=Optimize&ocompile=Warnings) rax,cl                                     | Writes to whole register                  |
| 32 bit eax | mov eax,ecx | mov eax,ecx                                                                                                                                                                                                                                                                                               | [movsx](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rax%2C0xeeeeeeeeeeeeeeee%0D%0Amov%20rcx%2C0xaaeeccddeeffaabb%0D%0Amovsx%20eax%2Ccx%0D%0Aret%0D%0A&lang=Assembly-NASM&mach=x64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Disassemble&orun=Grade&ocompile=Optimize&ocompile=Warnings) eax,cx | [movsx](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rax%2C0xeeeeeeeeeeeeeeee%0D%0Amov%20rcx%2C0xaaeeccddeeffaabb%0D%0Amovsx%20eax%2Ccl%0D%0Aret%0D%0A&lang=Assembly-NASM&mach=x64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Disassemble&orun=Grade&ocompile=Optimize&ocompile=Warnings) eax,cl | Top half of destination gets zeroed       |
| 16 bit ax  | mov ax,cx   | mov ax,cx                                                                                                                                                                                                                                                                                                 | mov ax,cx                                                                                                                                                                                                                                                                                                                                 | [movsx](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rax%2C0xeeeeeeeeeeeeeeee%0D%0Amov%20rcx%2C0xaaeeccddeeffaabb%0D%0Amovsx%20ax%2Ccl%0D%0Aret%0D%0A&lang=Assembly-NASM&mach=x64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Disassemble&orun=Grade&ocompile=Optimize&ocompile=Warnings) ax,cl   | Only affects low 16 bits, rest unchanged. |
| 8 bit al   | mov al,cl   | mov al,cl                                                                                                                                                                                                                                                                                                 | mov al,cl                                                                                                                                                                                                                                                                                                                                 | mov al,cl                                                                                                                                                                                                                                                                                                                                 | Only affects low 8 bits, rest unchanged.  |

Registers can store either signed or unsigned values.

| Signed      | Unsigned      | Description                                                                                                            |
| :---------- | :------------ | :--------------------------------------------------------------------------------------------------------------------- |
| int         | unsigned int  | In C/C++, int is signed by default.                                                                                    |
| signed char | unsigned char | In C/C++, char may be signed (default on gcc) or unsigned (default on Windows compilers) by default.                   |
| movsxd      | movzxd        | Assembly, **s**ign e**x**tend or **z**ero e**x**tend to change register sizes.                                         |
| jo          | jc            | Assembly, **o**verflow is for signed values, **c**arry for unsigned values.                                            |
| jg          | ja            | Assembly, jump **g**reater is signed, jump **a**bove is unsigned.                                                      |
| jl          | jb            | Assembly, jump **l**ess signed, jump **b**elow unsigned.                                                               |
| imul        | mul           | Assembly, imul is signed (and more modern), mul is for unsigned (and ancient and horrible\!). idiv/div work similarly. |

Normally, your assembly code lives in the code section, which can be read but not modified. When you declare static data, you need to put it in section .data for it to be writeable.

| Name            | Use       | Discussion                                                                                                                                               |
| :-------------- | :-------- | :------------------------------------------------------------------------------------------------------------------------------------------------------- |
| section .data   | r/w data  | This data is initialized, but can be modified.                                                                                                           |
| section .rodata | r/o data  | This data can't be modified, which lets it be shared across copies of the program. In C/C++, global "const" or "const static" data is stored in .rodata. |
| section .bss    | r/w space | This is automatically initialized to zero, meaning the contents don't need to be stored explicitly. This saves space in the executable.                  |
| section .text   | r/o code  | This is the program's executable machine code (it's binary data, not plain text--the Microsoft assembler calls this section ".code", a better name).     |

Before you can call some existing function, you need to declare that the function is "extern":  
 extern puts  
 call puts

If you want to define a function that can be called from outside, you need to declare your function "global":  
 global myGreatFunction  
 myGreatFunction:  
 ret

When linking a program that calls functions directly like this, you may need gcc's "-no-pie" option, to disable the position-independent executable support.

## **Instructions**

For gory instruction set details, read this [per-instruction reference](http://www.felixcloutier.com/x86/), or the [uselessly huge Intel PDF](https://software.intel.com/sites/default/files/managed/39/c5/325462-sdm-vol-1-2abcd-3abcd.pdf) (4000 pages\!).

| Instruction     | Purpose                                                                                                                                                                                                                                                                      | Examples                                                                                                                              |
| :-------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------ |
| mov _dest,src_  | Move data between registers, load immediate data into registers, move data between registers and memory.                                                                                                                                                                     | mov rax,4 ; Load constant into rax mov rdx,rax ; Copy rax into rdx mov \[rdi\],rdx ; Copy rdx into the memory that rdi is pointing to |
| push _src_      | Insert a value onto the stack. Useful for passing arguments, saving registers, etc.                                                                                                                                                                                          | push rbx                                                                                                                              |
| pop _dest_      | Remove topmost value from the stack. Equivalent to "mov _dest_, \[rsp\]; add 8,rsp"                                                                                                                                                                                          | pop rbx                                                                                                                               |
| call _func_     | Push the address of the next instruction and start executing func.                                                                                                                                                                                                           | call puts                                                                                                                             |
| ret             | Pop the return program counter, and jump there. Ends a function.                                                                                                                                                                                                             | ret                                                                                                                                   |
| add _dest,src_  | _dest=dest+src_                                                                                                                                                                                                                                                              | add rax,rdx ; Add rdx to rax                                                                                                          |
| imul _dest,src_ | _dest=dest\*src_ This is the signed multiply.                                                                                                                                                                                                                                | imul rcx,4 ; multiply rcx by 4                                                                                                        |
| mul _src_       | Multiply rax and _src_ as unsigned integers, and put the result in rax. High 64 bits of product (usually zero) go into rdx.                                                                                                                                                  | mul rdx ; Multiply rax by rdx ; rax=low bits, rdx overflow                                                                            |
| jmp _label_     | Goto the instruction _label_:. Skips anything else in the way.                                                                                                                                                                                                               | jmp post_mem mov \[0\],rax ; Write to NULL\! post_mem: ; OK here...                                                                   |
| cmp _a,b_       | Compare two values. Sets flags that are used by the conditional jumps (below).                                                                                                                                                                                               | cmp rax,10                                                                                                                            |
| jl _label_      | Goto _label_ if previous comparison came out as less-than. Other conditionals available are: jle (\<=), je (==), jge (\>=), jg (\>), jne (\!=) Also available in unsigned comparisons: jb (\<), jbe (\<=), ja (\>), jae (\>=) And checking for overflow (jo) and carry (jc). | jl loop_start ; Jump if rax\<10                                                                                                       |

## **Standard Idioms**

Looping over an array of 64-bit long integers, including the first-time test at startup:

**; rdi: pointer to array. rsi: number of elements**  
**mov rcx,0 ; i, loop counter**  
**jmp testFirst ; because rsi might be zero**  
**startLoop:**  
 **… work on QWORD\[rdi+8\*rcx\], which is array\[i\] ...**  
 **add rcx,1 ; i++**  
 **testFirst:**  
 **cmp rcx,rsi ; keep looping while i\<n**  
 **jl startLoop**

[(Try this in NetRun now\!)](https://lawlor.cs.uaf.edu/netrun/run?name=Testing&code=mov%20rdi%2CarrayPtr%0D%0Amov%20rsi%2C3%0D%0A%3B%20rdi%3A%20pointer%20to%20array.%20%20rsi%3A%20number%20of%20elements%0D%0Amov%20rax%2C0%20%3B%20output%20sum%0D%0Amov%20rcx%2C0%20%3B%20i%2C%20loop%20counter%0D%0Ajmp%20testFirst%20%3B%20because%20rsi%20might%20be%20zero%0D%0AstartLoop%3A%0D%0A%09add%20rax%2C%20QWORD%5Brdi%2B8%2Arcx%5D%20%3B%20sum%20%2B%3D%20array%5Bi%5D%3B%0D%0A%09add%20rcx%2C1%0D%0A%09testFirst%3A%0D%0A%09%09cmp%20rcx%2Crsi%0D%0A%09%09jl%20startLoop%0D%0Aret%0D%0A%0D%0A%0D%0Asection%20.data%0D%0AarrayPtr%3A%0D%0A%09dq%207%0D%0A%09dq%2010%0D%0A%09dq%20100%0D%0A&lang=Assembly-NASM&mach=skylake64&mode=frag&input=&linkwith=&foo_ret=long&foo_arg0=void&orun=Run&orun=Grade&ocompile=Optimize&ocompile=Warnings)

Allocating and deallocating memory:

| Memory type                   | The Stack                                  | The Heap                                                               | Static Data                                          |
| :---------------------------- | :----------------------------------------- | :--------------------------------------------------------------------- | :--------------------------------------------------- |
| Allocate _nBytes_ of memory   | sub rsp,_nBytes_                           | mov rdi,_nBytes_ extern malloc call malloc                             | section .data _stuff_: times _nBytes_ db 0           |
| Pointer to the allocated data | rsp                                        | rax                                                                    | _stuff_ or lea rdx,\[rel _stuff_\]                   |
| Deallocate the memory         | add rsp,_nBytes_                           | mov rdi,rax extern free call free                                      | ; Not needed                                         |
| Properties                    | The stack is only 8 megs on most machines. | Slowest memory allocation: costs at least a half-dozen function calls. | Static data stays allocated until the program exits. |

## **SSE Floating Point Instructions**

There are at least three generations of x86 floating point instructions:

-   fldpi, the original "floating point register stack", mostly limited to 32-bit machines now.
-   addss xmm0,xmm2 the SSE instructions
-   vmovss xmm0,xmm1,xmm2 the VEX-coded instructions

The SSE registers are named "xmm0" through "xmm15". The SSE instructions can be coded as shown below, or with a "v" in front for the VEX-coded AVX version, which allows the use of the 32-byte AVX "ymm" registers, and three-operand (destination, source1, source2) instruction format.

|       | Serial Single- precision (1 float) | Serial Double- precision (1 double) | Parallel Single- precision (4 floats)   | Parallel Double- precision (2 doubles)  | Comments                                                                                                                                                                                                                                          |
| :---- | :--------------------------------- | :---------------------------------- | :-------------------------------------- | :-------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| _add_ | addss                              | addsd                               | addps                                   | addpd                                   | sub, mul, div all work the same way                                                                                                                                                                                                               |
| min   | minss                              | minsd                               | minps                                   | minpd                                   | max works the same way                                                                                                                                                                                                                            |
| sqrt  | sqrtss                             | sqrtsd                              | sqrtps                                  | sqrtpd                                  | Square root (sqrt), reciprocal (rcp), and reciprocal-square-root (rsqrt) all work the same way                                                                                                                                                    |
| _mov_ | movss                              | movsd                               | movaps _(aligned)_ movups _(unaligned)_ | movapd _(aligned)_ movupd _(unaligned)_ | Aligned loads are up to 4x faster, but will crash if given an unaligned address\! The stack is always 16-byte aligned before calling a function, specifically for this instruction, as described below. Use "align 16" directive for static data. |
| _cvt_ | cvtss2sd cvtss2si cvttss2si        | cvtsd2ss cvtsd2si cvttsd2si         | cvtps2pd cvtps2dq cvttps2dq             | cvtpd2ps cvtpd2dq cvttpd2dq             | Convert to ("2", get it?) Single Integer (si, stored in register like eax) or four DWORDs (dq, stored in xmm register). "cvtt" versions do truncation (round down); "cvt" versions round to nearest.                                              |
| com   | ucomiss                            | ucomisd                             | _n/a_                                   | _n/a_                                   | Sets CPU flags like normal x86 "cmp" instruction for unsigned, from SSE registers.                                                                                                                                                                |
| cmp   | cmpeqss                            | cmpeqsd                             | cmpeqps                                 | cmpeqpd                                 | Compare for equality ("lt", "le", "neq", "nlt", "nle" versions work the same way). Sets all bits of float to zero if false (0.0), or all bits to ones if true (a NaN). Result is used as a bitmask for the bitwise AND and OR operations.         |
| and   | _n/a_                              | _n/a_                               | andps andnps                            | andpd andnpd                            | Bitwise AND operation. "andn" versions are bitwise AND-NOT operations (A=(\~A) & B). "or" version works the same way.                                                                                                                             |

The algebra of bitwise operators:

| Instruction | C++ Operator | Useful to                                                                                                                                                                                                                    |
| ----------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AND         | &            | Mask out bits (set other bits to zero) 0=A&0 AND by 0's creates 0's, used to mask out bad stuff A=A&~0 AND by 1's has no effect                                                                                              |
| OR          | \|           | Reassemble bit fields A=A \| 0 OR by 0's has no effect ~0=A \| ~0 OR by 1's creates 1's                                                                                                                                      |
| XOR         | ^            | Invert selected bits A=A^0 XOR by zeros has no effect ~A = A ^ ~0 XOR by 1's inverts all the bits 0=A^A XOR by yourself creates 0's--used for initialization A=A^B^B XOR is its own inverse operation--used for cryptography |
| NOT         | ~            | Invert all the bits in a number ~0 All bits are set to one ~A All the bits of A are inverted A=~~A Inverting twice recovers the bits                                                                                         |

## **Weird Instructions**

x86 is ancient, and it has many weird old instructions. The more useful ones include:

| [div](https://www.felixcloutier.com/x86/DIV.html) _src_                                       | Unsigned divide rax by _src_, and put the ratio into rax, and the remainder into rdx. Bizarrely, on input rdx must be zero (high bits of numerator), or you get a SIGFPE.                                                                              | mov rax, 100 ; numerator mov rdx,0 ; avoid error mov rcx, 3 ; denominator div rcx ; compute rax/rcx |
| :-------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------- |
| [idiv](https://www.felixcloutier.com/x86/IDIV.html) _src_                                     | Signed divide rax by the register _src_. rdx \= rax % src rax \= rax / src Before idiv, rdx must be a sign-extended version of rax, usually using [cqo](https://www.felixcloutier.com/x86/CWD:CDQ:CQO.html) (Convert Quadword rax to Octword rdx:rax). | mov rax, 100 ; numerator cqo ; sign-extend into rdx mov rcx, 3 ; denominator idiv rcx               |
| [shr](https://www.felixcloutier.com/x86/SAL:SAR:SHL:SHR.html) _val,bits_                      | Bitshift a value right by a constant, or the low 8 bits of rcx ("cl"). Shift count MUST go in rcx, no other register will work\!                                                                                                                       | add rcx,4 shr rax,cl ; shift by rcx                                                                 |
| [lea](https://www.felixcloutier.com/x86/LEA.html) _dest_,\[_ptr expression_\]                 | Load Effective Address of the pointer into the destination register--doesn't actually access memory, but uses the memory syntax.                                                                                                                       | lea rcx,\[rax \+ 4\*rdx \+12\]                                                                      |
| [loop](https://www.felixcloutier.com/x86/LOOP:LOOPcc.html) _jumplabel_                        | Decrement rcx, and if it's not zero, jump to the label.                                                                                                                                                                                                | mov rcx,10 start: add rax,7 loop start                                                              |
| [lodsb](https://www.felixcloutier.com/x86/LODS:LODSB:LODSW:LODSD:LODSQ.html)                  | Load one char of a string: al \= BYTE PTR \[rsi++\]                                                                                                                                                                                                    |                                                                                                     |
| [stosb](https://www.felixcloutier.com/x86/STOS:STOSB:STOSW:STOSD:STOSQ.html)                  | Store one char of a string: BYTE PTR \[rdi++\] \= al                                                                                                                                                                                                   |                                                                                                     |
| [movsb](https://www.felixcloutier.com/x86/MOVS:MOVSB:MOVSW:MOVSD:MOVSQ.html)                  | Copy one char of a string: BYTE PTR \[rdi++\] \= BYTE PTR \[rsi++\]                                                                                                                                                                                    |                                                                                                     |
| [scasb](https://www.felixcloutier.com/x86/SCAS:SCASB:SCASW:SCASD.html)                        | Compare the next char from string with register al: cmp BYTE PTR\[rdi++\], al                                                                                                                                                                          |                                                                                                     |
| [cmpsb](https://www.felixcloutier.com/x86/CMPS:CMPSB:CMPSW:CMPSD:CMPSQ.html)                  | Compare the next char from each of two strings: cmp BYTE PTR \[rdi++\], BYTE PTR \[rsi++\]                                                                                                                                                             |                                                                                                     |
| [rep](https://www.felixcloutier.com/x86/REP:REPE:REPZ:REPNE:REPNZ.html) _stringinstruction_   | Repeat the string instruction rcx times. Only works with string instructions (lods, stos, cmps, scas, cmps, ins, outs)                                                                                                                                 | mov al,'x' mov rcx,100 mov rdi,bufferStart rep stosb                                                |
| [repne](https://www.felixcloutier.com/x86/REP:REPE:REPZ:REPNE:REPNZ.html) _stringinstruction_ | Repeat the string instruction until the instruction sets the zero flag, or rcx gets decremented down to zero.                                                                                                                                          | mov al,0 mov rcx,-1 mov rsi,stringStart repne lodsb                                                 |

## **Debugging Assembly**

Disassembly using objdump:  
 objdump \-drC \-M intel code.obj  
The command line flags there are:

-   \-d: disassemble
-   \-r: include linker relocations
-   \-C: demangle C++ linker names
-   \-M intel: use the Intel syntax (mov rax,1), instead of the gnu syntax (movl $1,%rax)

error: parser: instruction expected  
error: label or instruction expected at start of line

-   This means you spelled the instruction name wrong.

error: invalid combination of opcode and operands

-   Another way to say this: "there is no instruction taking those arguments". For example, there's a 64-bit "mov rax,rcx", and a 32-bit "mov eax,ecx", but there is no "mov rax,ecx".

It compiles but won't link: "undefined reference to foo**()**" \<- note parenthesis\!

-   The C++ side needs to use "extern **"C"** long foo(void);" because you get this C++-vs-C link error if you leave out the extern "C".

It compiles but won't link: "undefined reference to \_foo" \<- note underscore\!

-   The assembly side may need to add underscores to match the compiler's linker names. This seems common on 32-bit machines.

It compiles but won't link: "undefined reference to foo"

-   The assembly side needs to say "global foo" to get the linker to see it.

It compiles but then crashes with a SIGSEGV

-   This means your code accessed a bad memory location
-   Do you access \[memory\] one too many times? Accessing memory like "mov rax,\[rdx\]" when rdx is a number instead of a pointer will crash, accessing a low address like 0x0.
-   Do you have write access to your \[memory\]? You may need to put your values into section .data
-   Is your stack manipulation (push/pop) OK? It's common to leave extra garbage on the stack, which causes ret to pop the garbage and jump there; or to accidentally remove the return address with an extra pop. Every push must have exactly one pop.
-   Is something trashing your pointer or counter registers? For example, if you call a function to print your output, it trashes all scratch registers. The fix is to push/pop scratch registers around function calls, or use preserved registers.
-   Are you using 64-bit pointers? Some folks doing 32-bit accesses want to write "mov eax, DWORD \[ecx\]" which can crash--even if the value you're accessing is 32 bit, all your address arithmetic needs to be 64 bit, so write "mov eax, DWORD \[rcx\]".

It runs but gives the wrong output

-   Do you need to access \[thingy\] instead of bare thingy? I've had programs that return the value \*of\* the pointer "mov rax,rdx" instead of the value the pointer is pointing to "mov rax,\[rdx\]"
-   Do your loops run the right number of times? It's always tricky getting the last iteration correct.
-   If you access arrays, are you multiplying by the size correctly? It's common to write to DWORD \[rdi+1\] instead of DWORD\[rdi+4\], which results in byte slices of the value you're after.

Using a debugger, like gdb, is very handy both for writing new code, and analysing existing programs even if you just have a compiled binary without source code. Here's my [GDB reverse engineering cheat sheet](https://docs.google.com/document/d/1ggjB8IYmdGDjAD1JMv7ys9SGemlDrWk3zZyWZNheLlM/).
