
# 如何在 Visual Studio 2022 中创建纯 ASM 项目

本指南将详细介绍如何将一个 Visual Studio C++ 空项目完全配置为用于编译、链接和运行纯汇编代码（MASM）的环境。所有配置都将在项目级别进行，适用于整个项目。

## 前提条件

确保你的 Visual Studio 2022 安装了 **“使用 C++ 的桌面开发(Desktop development with C++)”** 工作负载。在安装过程中，请确保勾选了右侧“安装详细信息”中的 **“适用于 x86 和 x64 的 C++ MASM(C++ MASM for v143 build tools)”** 选项。

---

## 步骤 1: 创建一个新的 C++ 空项目

1.  打开 Visual Studio 2022。
2.  选择 **“创建新项目(Create a new project)”**。
3.  选择 **“空项目(Empty Project)”** (C++)，点击“下一步”。
4.  为你的项目命名（例如 `PureAsmProject`），然后点击 **“创建”**。

## 步骤 2: 启用 MASM 生成支持

这一步会告诉 Visual Studio 你的项目需要 MASM 编译器。

1.  在 **“解决方案资源管理器(Solution Explorer)”** 中，右键单击你的 **项目名称** (例如 `PureAsmProject`)。
2.  选择 **“生成依赖项(Build Dependencies)”** -> **“生成自定义(Build Customizations...)”**。
3.  在弹出的窗口中，勾选 **`masm(.targets, .props)`**。
4.  点击 **“确定”**。

## 步骤 3: 添加汇编代码文件 (.asm)

1.  在 **“解决方案资源管理器(Solution Explorer)”** 中，右键单击 **“源文件(Source Files)”** 文件夹。
2.  选择 **“添加(Add)”** -> **“新建项(New Item...)”**。
3.  选择 **“C++ 文件(.cpp)”**，但将文件名后缀更改为 `.asm`。例如，命名为 `main.asm`。
4.  点击 **“添加”**。此时，Visual Studio 会自动将此文件的“项类型”识别为“Microsoft Macro Assembler”。
5.  在 `main.asm` 文件中，粘贴以下示例代码：

    ```x86asm
    ; 纯 32 位汇编程序
    .386
    .model flat, stdcall
    .stack 4096

    ExitProcess PROTO, dwExitCode:DWORD

    .code
    main PROC
        mov eax, 123      ; 设置退出代码为 123
        invoke ExitProcess, eax
    main ENDP
    END main
    ```

## 步骤 4: 配置项目级的 MASM 和链接器属性

这是核心步骤，我们将为整个项目设置汇编和链接选项。

1.  在 **“解决方案资源管理器(Solution Explorer)”** 中，再次右键单击你的 **项目名称**，选择 **“属性(Properties)”**。
2.  在属性页顶部，将 **“配置(Configuration)”** 设置为 **“所有配置(All Configurations)”**，将 **“平台(Platform)”** 设置为 **“所有平台(All Platforms)”**，以确保设置对所有构建模式都生效。

3.  **设置生成列表文件:**
    *   在左侧导航栏，展开 **“配置属性(Configuration Properties)”** -> **“Microsoft Macro Assembler”**。
    *   点击 **“列表文件(Listing File)”**。
    *   在右侧，将 **“列表所有可用符号(List All Available Symbols)”** 设置为 **“是 (/Sa)”**。
    *   将 **“列表文件名(Listing File)”** 设置为 `$(ProjectDir)$(Platform)\$(Configuration)\$(ProjectName).lst`。这个路径会将列表文件生成在与 `.exe` 文件相同的目录中。

4.  **设置链接器:**
    *   在左侧导航栏，展开 **“配置属性(Configuration Properties)”** -> **“链接器(Linker)”** -> **“系统(System)”**。
    *   在右侧，将 **“子系统(SubSystem)”** 设置为 **“控制台 (/SUBSYSTEM:CONSOLE)”**。
    *   接着，在左侧点击 **“链接器(Linker)”** -> **“高级(Advanced)”**。
    *   在右侧，找到 **“入口点(Entry Point)”** 字段，并输入我们代码中 `END` 指令指定的入口点名称：`main`。

5.  完成所有设置后，点击 **“应用(Apply)”**，然后点击 **“确定(OK)”**。

## 步骤 5: 生成并验证

1.  **生成项目**: 在顶部菜单栏，选择 **“生成(Build)”** -> **“生成解决方案(Build Solution)”** (或按 `F7`)。

2.  **运行程序**: 选择 **“调试(Debug)”** -> **“开始执行(不调试)(Start Without Debugging)”** (或按 `Ctrl+F5`)。控制台窗口会一闪而过。

3.  **验证结果**: 
    *   打开命令提示符 (cmd) 或终端。
    *   导航到你的项目输出目录。例如：`cd C:\Users\ginka\source\repos\PureAsmProject\x64\Debug` (具体路径取决于你的平台和配置)。
    *   运行程序: `PureAsmProject.exe`。
    *   检查退出代码: `echo %ERRORLEVEL%`。
    *   你应该会看到输出 `123`。

4.  **查看列表文件**: 
    *   在同一个输出目录中 (`Debug` 或 `Release` 文件夹)，你会找到 `PureAsmProject.lst` 文件。
    *   用文本编辑器打开它，即可查看详细的汇编列表信息。

---
指南结束.
