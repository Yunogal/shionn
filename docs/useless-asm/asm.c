int __fastcall fastcall_1(int a, int b)
{
    return a + b;
}
int _cdecl cdecall_1(int a, int b)
{
    return a + b;
}
int __stdcall stdcall_1(int a, int b)
{
    return a + b;
}

// gcc -m32 -masm=intel -S docs/useless-asm/asm.c -o docs/useless-asm/asm.S
int main()
{
    
    int a = 1;
    int b = a + 1;
}