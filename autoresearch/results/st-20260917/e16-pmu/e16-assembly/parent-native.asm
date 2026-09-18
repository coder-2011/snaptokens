
/home/namanchetwani/st-campaign-20260917/combined-profile-st-eval:     file format elf64-x86-64


Disassembly of section .text:

000000000034c960 <<snaptokens::models::bpe::Bpe>::from_native_tables>:
  34c960:	push   %rbp
  34c961:	push   %r15
  34c963:	push   %r14
  34c965:	push   %r13
  34c967:	push   %r12
  34c969:	push   %rbx
  34c96a:	sub    $0x1000,%rsp
  34c971:	movq   $0x0,(%rsp)
  34c979:	sub    $0x1000,%rsp
  34c980:	movq   $0x0,(%rsp)
  34c988:	sub    $0xdb8,%rsp
  34c98f:	mov    %rdi,0x18(%rsp)
  34c994:	mov    (%rsi),%rax
  34c997:	mov    %rax,0x28(%rsp)
  34c99c:	mov    0x8(%rsi),%rax
  34c9a0:	mov    %rax,0x20(%rsp)
  34c9a5:	mov    0x10(%rsi),%r15
  34c9a9:	mov    0x18(%rsi),%r12
  34c9ad:	mov    0x20(%rsi),%rcx
  34c9b1:	mov    0x28(%rsi),%r14
  34c9b5:	mov    0x30(%rsi),%rax
  34c9b9:	mov    %rax,0xc0(%rsp)
  34c9c1:	mov    0x38(%rsi),%rax
  34c9c5:	mov    %rax,0xf0(%rsp)
  34c9cd:	mov    0x40(%rsi),%rdx
  34c9d1:	mov    0x48(%rsi),%rax
  34c9d5:	mov    %rax,0x50(%rsp)
  34c9da:	mov    0x50(%rsi),%rax
  34c9de:	mov    %rax,0x48(%rsp)
  34c9e3:	mov    0x58(%rsi),%rax
  34c9e7:	mov    %rax,0x68(%rsp)
  34c9ec:	movzbl 0x1d0(%rsi),%eax
  34c9f3:	mov    %al,0x17(%rsp)
  34c9f7:	movzbl 0x1d1(%rsi),%edi
  34c9fe:	movdqu 0x198(%rsi),%xmm0
  34ca06:	movdqu 0x1a8(%rsi),%xmm1
  34ca0e:	movdqu 0x1b8(%rsi),%xmm2
  34ca16:	movdqa %xmm2,0x360(%rsp)
  34ca1f:	movdqa %xmm1,0x350(%rsp)
  34ca28:	movdqa %xmm0,0x340(%rsp)
  34ca31:	movzbl 0x1d2(%rsi),%r8d
  34ca39:	mov    0x60(%rsi),%r13
  34ca3d:	mov    0x68(%rsi),%rax
  34ca41:	mov    %rax,0xe8(%rsp)
  34ca49:	mov    0x70(%rsi),%rax
  34ca4d:	mov    %rax,0xa8(%rsp)
  34ca55:	mov    0x78(%rsi),%rax
  34ca59:	mov    %rax,0x120(%rsp)
  34ca61:	mov    0x80(%rsi),%rax
  34ca68:	mov    %rax,0xf8(%rsp)
  34ca70:	mov    0x88(%rsi),%rax
  34ca77:	mov    %rax,0x88(%rsp)
  34ca7f:	mov    0x1c8(%rsi),%eax
  34ca85:	mov    %rax,0xa0(%rsp)
  34ca8d:	mov    0x90(%rsi),%rax
  34ca94:	mov    %rax,0x118(%rsp)
  34ca9c:	mov    0x98(%rsi),%rax
  34caa3:	mov    %rax,0x1a8(%rsp)
  34caab:	mov    0xa0(%rsi),%rbx
  34cab2:	mov    0xa8(%rsi),%rax
  34cab9:	mov    %rax,0x110(%rsp)
  34cac1:	mov    0xb0(%rsi),%rax
  34cac8:	mov    %rax,0x98(%rsp)
  34cad0:	mov    0xb8(%rsi),%rax
  34cad7:	mov    %rax,0x2c0(%rsp)
  34cadf:	mov    0xc0(%rsi),%rax
  34cae6:	mov    %rax,0x108(%rsp)
  34caee:	mov    0xc8(%rsi),%rax
  34caf5:	mov    %rax,0x90(%rsp)
  34cafd:	mov    0xd0(%rsi),%rax
  34cb04:	mov    %rax,0xd8(%rsp)
  34cb0c:	mov    0xd8(%rsi),%rax
  34cb13:	mov    %rax,0xb8(%rsp)
  34cb1b:	mov    0xe0(%rsi),%rax
  34cb22:	mov    %rax,0xe0(%rsp)
  34cb2a:	mov    0xe8(%rsi),%r9
  34cb31:	mov    0xf0(%rsi),%rax
  34cb38:	mov    %rax,0x1c0(%rsp)
  34cb40:	mov    0xf8(%rsi),%rax
  34cb47:	mov    %rax,0xb0(%rsp)
  34cb4f:	mov    0x100(%rsi),%r11
  34cb56:	mov    0x108(%rsi),%rax
  34cb5d:	mov    %rax,0x1b8(%rsp)
  34cb65:	mov    0x110(%rsi),%rax
  34cb6c:	mov    %rax,0x1e8(%rsp)
  34cb74:	mov    0x118(%rsi),%r10
  34cb7b:	mov    0x120(%rsi),%rax
  34cb82:	mov    %rax,0x1b0(%rsp)
  34cb8a:	mov    0x128(%rsi),%rax
  34cb91:	mov    %rax,0x188(%rsp)
  34cb99:	mov    0x130(%rsi),%rbp
  34cba0:	mov    0x138(%rsi),%rax
  34cba7:	mov    %rax,0x100(%rsp)
  34cbaf:	mov    0x140(%rsi),%rax
  34cbb6:	mov    %rax,0x180(%rsp)
  34cbbe:	mov    0x148(%rsi),%rax
  34cbc5:	mov    %rax,0x80(%rsp)
  34cbcd:	mov    0x150(%rsi),%rax
  34cbd4:	mov    %rax,0x60(%rsp)
  34cbd9:	mov    0x158(%rsi),%rax
  34cbe0:	mov    %rax,0x1a0(%rsp)
  34cbe8:	mov    0x168(%rsi),%rax
  34cbef:	mov    %rax,0x78(%rsp)
  34cbf4:	mov    0x170(%rsi),%rax
  34cbfb:	mov    %rax,0x198(%rsp)
  34cc03:	mov    0x180(%rsi),%rax
  34cc0a:	mov    %rax,0x70(%rsp)
  34cc0f:	mov    0x188(%rsi),%rax
  34cc16:	mov    %rax,0x190(%rsp)
  34cc1e:	cmp    $0x2,%r14
  34cc22:	mov    %rcx,0x58(%rsp)
  34cc27:	jb     34cc32 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2d2>
  34cc29:	cmpl   $0x0,(%rcx)
  34cc2c:	je     34cf48 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5e8>
  34cc32:	call   *0x2e81f8(%rip)        # 634e30 <_DYNAMIC+0x258>
  34cc38:	mov    $0x1e,%ebp
  34cc3d:	mov    $0x1e,%edi
  34cc42:	mov    $0x1,%esi
  34cc47:	call   *0x2e81eb(%rip)        # 634e38 <_DYNAMIC+0x260>
  34cc4d:	test   %rax,%rax
  34cc50:	je     34d7ed <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe8d>
  34cc56:	mov    %rax,%r15
  34cc59:	movups -0x22198f(%rip),%xmm0        # 12b2d1 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x371>
  34cc60:	movups %xmm0,0xe(%rax)
  34cc64:	movdqu -0x2219a9(%rip),%xmm0        # 12b2c3 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x363>
  34cc6c:	movdqu %xmm0,(%rax)
  34cc70:	mov    $0x1e,%r14d
  34cc76:	mov    0x28(%rsp),%rbx
  34cc7b:	test   %r12,%r12
  34cc7e:	mov    0x18(%rsp),%rbp
  34cc83:	mov    0x58(%rsp),%rdi
  34cc88:	je     34cc9c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x33c>
  34cc8a:	shl    $0x2,%r12
  34cc8e:	mov    $0x4,%edx
  34cc93:	mov    %r12,%rsi
  34cc96:	call   *0x2e8174(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cc9c:	test   %rbx,%rbx
  34cc9f:	je     34ccb4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x354>
  34cca1:	mov    $0x1,%edx
  34cca6:	mov    0x20(%rsp),%rdi
  34ccab:	mov    %rbx,%rsi
  34ccae:	call   *0x2e815c(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ccb4:	mov    %r14,%r12
  34ccb7:	mov    0x70(%rsp),%rsi
  34ccbc:	mov    %r14,0x8(%rbp)
  34ccc0:	mov    %r15,0x10(%rbp)
  34ccc4:	mov    %r12,0x18(%rbp)
  34ccc8:	movq   $0xffffffffffffffff,0x0(%rbp)
  34ccd0:	mov    $0x1,%r12b
  34ccd3:	xor    %ebx,%ebx
  34ccd5:	mov    0x60(%rsp),%r14
  34ccda:	mov    0x78(%rsp),%r15
  34ccdf:	test   %rsi,%rsi
  34cce2:	je     34ccfb <<snaptokens::models::bpe::Bpe>::from_native_tables+0x39b>
  34cce4:	shl    $0x2,%rsi
  34cce8:	mov    $0x4,%edx
  34cced:	mov    0x190(%rsp),%rdi
  34ccf5:	call   *0x2e8115(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ccfb:	test   %r15,%r15
  34ccfe:	je     34cd1a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x3ba>
  34cd00:	shl    $0x3,%r15
  34cd04:	mov    $0x8,%edx
  34cd09:	mov    0x198(%rsp),%rdi
  34cd11:	mov    %r15,%rsi
  34cd14:	call   *0x2e80f6(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cd1a:	test   %r14,%r14
  34cd1d:	je     34cd39 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x3d9>
  34cd1f:	shl    $0x2,%r14
  34cd23:	mov    $0x4,%edx
  34cd28:	mov    0x1a0(%rsp),%rdi
  34cd30:	mov    %r14,%rsi
  34cd33:	call   *0x2e80d7(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cd39:	mov    0x340(%rsp),%rax
  34cd41:	cmp    $0xffffffffffffffff,%rax
  34cd45:	setne  %cl
  34cd48:	test   %r12b,%cl
  34cd4b:	je     34cd8d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x42d>
  34cd4d:	test   %rax,%rax
  34cd50:	je     34cd6d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x40d>
  34cd52:	mov    0x348(%rsp),%rdi
  34cd5a:	shl    $0x2,%rax
  34cd5e:	lea    (%rax,%rax,2),%rsi
  34cd62:	mov    $0x4,%edx
  34cd67:	call   *0x2e80a3(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cd6d:	mov    0x358(%rsp),%rsi
  34cd75:	test   %rsi,%rsi
  34cd78:	je     34cd8d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x42d>
  34cd7a:	mov    0x360(%rsp),%rdi
  34cd82:	mov    $0x1,%edx
  34cd87:	call   *0x2e8083(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cd8d:	mov    0x100(%rsp),%rsi
  34cd95:	test   %rsi,%rsi
  34cd98:	je     34cdb1 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x451>
  34cd9a:	shl    $0x3,%rsi
  34cd9e:	mov    $0x8,%edx
  34cda3:	mov    0x180(%rsp),%rdi
  34cdab:	call   *0x2e805f(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cdb1:	mov    0x1b0(%rsp),%rsi
  34cdb9:	test   %rsi,%rsi
  34cdbc:	je     34cdd5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x475>
  34cdbe:	shl    $0x2,%rsi
  34cdc2:	mov    $0x4,%edx
  34cdc7:	mov    0x188(%rsp),%rdi
  34cdcf:	call   *0x2e803b(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cdd5:	mov    0x1b8(%rsp),%rsi
  34cddd:	test   %rsi,%rsi
  34cde0:	je     34cdf9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x499>
  34cde2:	shl    $0x3,%rsi
  34cde6:	mov    $0x8,%edx
  34cdeb:	mov    0x1e8(%rsp),%rdi
  34cdf3:	call   *0x2e8017(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cdf9:	mov    0x1c0(%rsp),%rsi
  34ce01:	test   %rsi,%rsi
  34ce04:	je     34ce1d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x4bd>
  34ce06:	shl    $0x2,%rsi
  34ce0a:	mov    $0x4,%edx
  34ce0f:	mov    0xb0(%rsp),%rdi
  34ce17:	call   *0x2e7ff3(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ce1d:	mov    0xb8(%rsp),%rsi
  34ce25:	test   %rsi,%rsi
  34ce28:	je     34ce41 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x4e1>
  34ce2a:	shl    $0x5,%rsi
  34ce2e:	mov    $0x10,%edx
  34ce33:	mov    0xe0(%rsp),%rdi
  34ce3b:	call   *0x2e7fcf(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ce41:	mov    0x108(%rsp),%rsi
  34ce49:	test   %rsi,%rsi
  34ce4c:	je     34ce65 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x505>
  34ce4e:	shl    $0x3,%rsi
  34ce52:	mov    $0x8,%edx
  34ce57:	mov    0x90(%rsp),%rdi
  34ce5f:	call   *0x2e7fab(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ce65:	mov    0x110(%rsp),%rsi
  34ce6d:	test   %rsi,%rsi
  34ce70:	je     34ce89 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x529>
  34ce72:	shl    $0x3,%rsi
  34ce76:	mov    $0x8,%edx
  34ce7b:	mov    0x98(%rsp),%rdi
  34ce83:	call   *0x2e7f87(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ce89:	mov    0x118(%rsp),%rsi
  34ce91:	test   %rsi,%rsi
  34ce94:	je     34cead <<snaptokens::models::bpe::Bpe>::from_native_tables+0x54d>
  34ce96:	shl    $0x2,%rsi
  34ce9a:	mov    $0x4,%edx
  34ce9f:	mov    0x1a8(%rsp),%rdi
  34cea7:	call   *0x2e7f63(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cead:	mov    0x120(%rsp),%rsi
  34ceb5:	test   %rsi,%rsi
  34ceb8:	je     34ced1 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x571>
  34ceba:	shl    $0x2,%rsi
  34cebe:	mov    $0x4,%edx
  34cec3:	mov    0xf8(%rsp),%rdi
  34cecb:	call   *0x2e7f3f(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ced1:	test   %r13,%r13
  34ced4:	je     34cef0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x590>
  34ced6:	shl    $0x2,%r13
  34ceda:	mov    $0x4,%edx
  34cedf:	mov    0xe8(%rsp),%rdi
  34cee7:	mov    %r13,%rsi
  34ceea:	call   *0x2e7f20(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cef0:	mov    0x50(%rsp),%rsi
  34cef5:	test   %rsi,%rsi
  34cef8:	sete   %al
  34cefb:	or     %al,%bl
  34cefd:	jne    34cf0f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5af>
  34ceff:	mov    $0x1,%edx
  34cf04:	mov    0x48(%rsp),%rdi
  34cf09:	call   *0x2e7f01(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cf0f:	mov    0xc0(%rsp),%rsi
  34cf17:	test   %rsi,%rsi
  34cf1a:	je     34cf33 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5d3>
  34cf1c:	shl    $0x3,%rsi
  34cf20:	mov    $0x4,%edx
  34cf25:	mov    0xf0(%rsp),%rdi
  34cf2d:	call   *0x2e7edd(%rip)        # 634e10 <_DYNAMIC+0x238>
  34cf33:	mov    %rbp,%rax
  34cf36:	add    $0x2db8,%rsp
  34cf3d:	pop    %rbx
  34cf3e:	pop    %r12
  34cf40:	pop    %r13
  34cf42:	pop    %r14
  34cf44:	pop    %r15
  34cf46:	pop    %rbp
  34cf47:	ret
  34cf48:	mov    %r13,0x38(%rsp)
  34cf4d:	mov    -0x4(%rcx,%r14,4),%r13d
  34cf52:	cmp    %r13,%r15
  34cf55:	jne    34d0ec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x78c>
  34cf5b:	mov    %rbp,0x1e0(%rsp)
  34cf63:	mov    %rbx,0x40(%rsp)
  34cf68:	mov    %r11,0x178(%rsp)
  34cf70:	mov    %r10,0xd0(%rsp)
  34cf78:	mov    %r9,0x1c8(%rsp)
  34cf80:	mov    %r8b,0x16(%rsp)
  34cf85:	mov    %dil,0x37(%rsp)
  34cf8a:	mov    %rdx,0x8(%rsp)
  34cf8f:	lea    0x198(%rsi),%rax
  34cf96:	mov    %rax,0x2d0(%rsp)
  34cf9e:	mov    0x1cc(%rsi),%eax
  34cfa4:	mov    %eax,0x1d0(%rsp)
  34cfab:	mov    0x160(%rsi),%rax
  34cfb2:	mov    %rax,0xc8(%rsp)
  34cfba:	mov    0x178(%rsi),%rax
  34cfc1:	mov    %rax,0x1d8(%rsp)
  34cfc9:	mov    0x190(%rsi),%rax
  34cfd0:	mov    %rax,0x170(%rsp)
  34cfd8:	lea    0x4(%rcx),%rbx
  34cfdc:	lea    0x1(%r14),%rbp
  34cfe0:	mov    -0x4(%rbx),%esi
  34cfe3:	mov    (%rbx),%edx
  34cfe5:	cmp    %esi,%edx
  34cfe7:	jb     34d141 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x7e1>
  34cfed:	cmp    %edx,%r13d
  34cff0:	jb     34d141 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x7e1>
  34cff6:	sub    %esi,%edx
  34cff8:	add    0x20(%rsp),%rsi
  34cffd:	lea    0x128(%rsp),%rdi
  34d005:	call   *0x2e7e85(%rip)        # 634e90 <_DYNAMIC+0x2b8>
  34d00b:	cmpl   $0x1,0x128(%rsp)
  34d013:	je     34d18f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x82f>
  34d019:	add    $0x4,%rbx
  34d01d:	dec    %rbp
  34d020:	cmp    $0x3,%rbp
  34d024:	jae    34cfe0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x680>
  34d026:	mov    0x28(%rsp),%rax
  34d02b:	cmp    $0xffffffffffffffff,%rax
  34d02f:	je     34d1dd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x87d>
  34d035:	mov    0x58(%rsp),%rcx
  34d03a:	mov    %rcx,0x280(%rsp)
  34d042:	mov    %r14,0x288(%rsp)
  34d04a:	mov    %rax,0x260(%rsp)
  34d052:	mov    0x20(%rsp),%rax
  34d057:	mov    %rax,0x268(%rsp)
  34d05f:	mov    %r15,0x270(%rsp)
  34d067:	mov    %r12,0x278(%rsp)
  34d06f:	cmp    $0x1,%r14
  34d073:	adc    $0xffffffffffffffff,%r14
  34d077:	mov    %r14,%rax
  34d07a:	shr    $0x20,%rax
  34d07e:	mov    0x38(%rsp),%r13
  34d083:	mov    0x18(%rsp),%rbp
  34d088:	je     34d1f6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x896>
  34d08e:	call   *0x2e7d9c(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d094:	mov    $0x1b,%ebx
  34d099:	mov    $0x1b,%edi
  34d09e:	mov    $0x1,%esi
  34d0a3:	call   *0x2e7d8f(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d0a9:	test   %rax,%rax
  34d0ac:	mov    0x60(%rsp),%r14
  34d0b1:	mov    0x78(%rsp),%r15
  34d0b6:	je     34dd75 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1415>
  34d0bc:	movups -0x22176c(%rip),%xmm0        # 12b957 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x9f7>
  34d0c3:	movups %xmm0,0xb(%rax)
  34d0c7:	movdqu -0x221783(%rip),%xmm0        # 12b94c <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x9ec>
  34d0cf:	movdqu %xmm0,(%rax)
  34d0d3:	movq   $0x1b,0x8(%rbp)
  34d0db:	mov    %rax,0x10(%rbp)
  34d0df:	movq   $0x1b,0x18(%rbp)
  34d0e7:	jmp    34d2d2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34d0ec:	call   *0x2e7d3e(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d0f2:	mov    $0x23,%ebp
  34d0f7:	mov    $0x23,%edi
  34d0fc:	mov    $0x1,%esi
  34d101:	call   *0x2e7d31(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d107:	test   %rax,%rax
  34d10a:	mov    0x38(%rsp),%r13
  34d10f:	je     34d7ed <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe8d>
  34d115:	mov    %rax,%r15
  34d118:	movups -0x221e6f(%rip),%xmm0        # 12b2b0 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x350>
  34d11f:	movups %xmm0,0x10(%rax)
  34d123:	movdqu -0x221e8b(%rip),%xmm0        # 12b2a0 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x340>
  34d12b:	movdqu %xmm0,(%rax)
  34d12f:	movl   $0x6874676e,0x1f(%rax)
  34d136:	mov    $0x23,%r14d
  34d13c:	jmp    34cc76 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x316>
  34d141:	call   *0x2e7ce9(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d147:	mov    $0x1b,%ebp
  34d14c:	mov    $0x1b,%edi
  34d151:	mov    $0x1,%esi
  34d156:	call   *0x2e7cdc(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d15c:	test   %rax,%rax
  34d15f:	mov    0x38(%rsp),%r13
  34d164:	je     34d7ed <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe8d>
  34d16a:	mov    %rax,%r15
  34d16d:	movups -0x221ee4(%rip),%xmm0        # 12b290 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x330>
  34d174:	movups %xmm0,0xb(%rax)
  34d178:	movdqu -0x221efb(%rip),%xmm0        # 12b285 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x325>
  34d180:	movdqu %xmm0,(%rax)
  34d184:	mov    $0x1b,%r14d
  34d18a:	jmp    34cc76 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x316>
  34d18f:	call   *0x2e7c9b(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d195:	mov    $0x1c,%ebp
  34d19a:	mov    $0x1c,%edi
  34d19f:	mov    $0x1,%esi
  34d1a4:	call   *0x2e7c8e(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d1aa:	test   %rax,%rax
  34d1ad:	mov    0x38(%rsp),%r13
  34d1b2:	je     34d7ed <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe8d>
  34d1b8:	mov    %rax,%r15
  34d1bb:	movups -0x221f4d(%rip),%xmm0        # 12b275 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x315>
  34d1c2:	movups %xmm0,0xc(%rax)
  34d1c6:	movdqu -0x221f65(%rip),%xmm0        # 12b269 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x309>
  34d1ce:	movdqu %xmm0,(%rax)
  34d1d2:	mov    $0x1c,%r14d
  34d1d8:	jmp    34cc76 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x316>
  34d1dd:	mov    0x20(%rsp),%r14
  34d1e2:	mov    0x38(%rsp),%r13
  34d1e7:	mov    0x70(%rsp),%rsi
  34d1ec:	mov    0x18(%rsp),%rbp
  34d1f1:	jmp    34ccbc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x35c>
  34d1f6:	mov    0x8(%rsp),%rax
  34d1fb:	cmp    %r14,%rax
  34d1fe:	mov    0x78(%rsp),%r15
  34d203:	jne    34d278 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x918>
  34d205:	cmp    %rax,0x68(%rsp)
  34d20a:	jne    34d278 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x918>
  34d20c:	cmpq   $0x100,0xa8(%rsp)
  34d218:	mov    0x60(%rsp),%r14
  34d21d:	jne    34d2ff <<snaptokens::models::bpe::Bpe>::from_native_tables+0x99f>
  34d223:	cmpq   $0x100,0x88(%rsp)
  34d22f:	jne    34d2ff <<snaptokens::models::bpe::Bpe>::from_native_tables+0x99f>
  34d235:	cmpq   $0x400,0x80(%rsp)
  34d241:	jne    34d350 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x9f0>
  34d247:	mov    0x2c0(%rsp),%rdx
  34d24f:	test   %rdx,%rdx
  34d252:	je     34d3a4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xa44>
  34d258:	lea    -0x1(,%rdx,2),%rax
  34d260:	bsr    %rax,%rcx
  34d264:	not    %ecx
  34d266:	mov    $0xffffffffffffffff,%rbx
  34d26d:	shr    %cl,%rbx
  34d270:	inc    %rbx
  34d273:	jmp    34d3a6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xa46>
  34d278:	call   *0x2e7bb2(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d27e:	mov    $0x22,%ebx
  34d283:	mov    $0x22,%edi
  34d288:	mov    $0x1,%esi
  34d28d:	call   *0x2e7ba5(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d293:	test   %rax,%rax
  34d296:	mov    0x60(%rsp),%r14
  34d29b:	je     34dd75 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1415>
  34d2a1:	movups -0x22196e(%rip),%xmm0        # 12b93a <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x9da>
  34d2a8:	movups %xmm0,0x10(%rax)
  34d2ac:	movdqu -0x22198a(%rip),%xmm0        # 12b92a <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x9ca>
  34d2b4:	movdqu %xmm0,(%rax)
  34d2b8:	movw   $0x6874,0x20(%rax)
  34d2be:	movq   $0x22,0x8(%rbp)
  34d2c6:	mov    %rax,0x10(%rbp)
  34d2ca:	movq   $0x22,0x18(%rbp)
  34d2d2:	movq   $0xffffffffffffffff,0x0(%rbp)
  34d2da:	mov    $0x1,%r12b
  34d2dd:	xor    %ebx,%ebx
  34d2df:	lea    0x260(%rsp),%rdi
  34d2e7:	call   33ea60 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabArena>>
  34d2ec:	mov    0x70(%rsp),%rsi
  34d2f1:	test   %rsi,%rsi
  34d2f4:	jne    34cce4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x384>
  34d2fa:	jmp    34ccfb <<snaptokens::models::bpe::Bpe>::from_native_tables+0x39b>
  34d2ff:	call   *0x2e7b2b(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d305:	mov    $0x1d,%ebx
  34d30a:	mov    $0x1d,%edi
  34d30f:	mov    $0x1,%esi
  34d314:	call   *0x2e7b1e(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d31a:	test   %rax,%rax
  34d31d:	je     34dd75 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1415>
  34d323:	movups -0x221a10(%rip),%xmm0        # 12b91a <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x9ba>
  34d32a:	movups %xmm0,0xd(%rax)
  34d32e:	movdqu -0x221a29(%rip),%xmm0        # 12b90d <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x9ad>
  34d336:	movdqu %xmm0,(%rax)
  34d33a:	movq   $0x1d,0x8(%rbp)
  34d342:	mov    %rax,0x10(%rbp)
  34d346:	movq   $0x1d,0x18(%rbp)
  34d34e:	jmp    34d2d2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34d350:	call   *0x2e7ada(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d356:	mov    $0x1f,%ebx
  34d35b:	mov    $0x1f,%edi
  34d360:	mov    $0x1,%esi
  34d365:	call   *0x2e7acd(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d36b:	test   %rax,%rax
  34d36e:	je     34dd75 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1415>
  34d374:	movups -0x221a7e(%rip),%xmm0        # 12b8fd <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x99d>
  34d37b:	movups %xmm0,0xf(%rax)
  34d37f:	movdqu -0x221a99(%rip),%xmm0        # 12b8ee <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x98e>
  34d387:	movdqu %xmm0,(%rax)
  34d38b:	movq   $0x1f,0x8(%rbp)
  34d393:	mov    %rax,0x10(%rbp)
  34d397:	movq   $0x1f,0x18(%rbp)
  34d39f:	jmp    34d2d2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34d3a4:	xor    %ebx,%ebx
  34d3a6:	cmp    %rdx,0x40(%rsp)
  34d3ab:	jne    34d486 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb26>
  34d3b1:	mov    0xd8(%rsp),%rax
  34d3b9:	cmp    %rax,0x40(%rsp)
  34d3be:	jne    34d486 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb26>
  34d3c4:	cmp    0xa0(%rsp),%rbx
  34d3cc:	jne    34d486 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb26>
  34d3d2:	lea    0x230(%rsp),%rdi
  34d3da:	mov    $0xffffffffffffffff,%rsi
  34d3e1:	mov    %rbx,%rdx
  34d3e4:	call   341e20 <<u64 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>>
  34d3e9:	lea    0x248(%rsp),%rdi
  34d3f1:	xor    %esi,%esi
  34d3f3:	mov    %rbx,%rdx
  34d3f6:	call   341e20 <<u64 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>>
  34d3fb:	cmpq   $0x0,0x40(%rsp)
  34d401:	je     34d533 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xbd3>
  34d407:	mov    0x238(%rsp),%r15
  34d40f:	mov    0x240(%rsp),%rbx
  34d417:	mov    0x250(%rsp),%rax
  34d41f:	mov    0x258(%rsp),%rsi
  34d427:	xor    %ecx,%ecx
  34d429:	mov    0x1a8(%rsp),%r9
  34d431:	mov    0x98(%rsp),%r10
  34d439:	mov    0x90(%rsp),%r11
  34d441:	mov    0x40(%rsp),%r14
  34d446:	mov    (%r9,%rcx,4),%edi
  34d44a:	cmp    %rdi,%rbx
  34d44d:	jbe    34d4da <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb7a>
  34d453:	mov    (%r10,%rcx,8),%rdx
  34d457:	cmp    $0xffffffffffffffff,%rdx
  34d45b:	je     34d4da <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb7a>
  34d45d:	cmpq   $0xffffffffffffffff,(%r15,%rdi,8)
  34d462:	jne    34d4da <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb7a>
  34d464:	mov    (%r11,%rcx,8),%r8
  34d468:	mov    %rdx,(%r15,%rdi,8)
  34d46c:	cmp    %rdi,%rsi
  34d46f:	jbe    34e7c4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e64>
  34d475:	inc    %rcx
  34d478:	mov    %r8,(%rax,%rdi,8)
  34d47c:	cmp    %rcx,%r14
  34d47f:	jne    34d446 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xae6>
  34d481:	jmp    34d543 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xbe3>
  34d486:	call   *0x2e79a4(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d48c:	mov    $0x1e,%ebx
  34d491:	mov    $0x1e,%edi
  34d496:	mov    $0x1,%esi
  34d49b:	call   *0x2e7997(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d4a1:	test   %rax,%rax
  34d4a4:	je     34dd75 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1415>
  34d4aa:	movups -0x221c9d(%rip),%xmm0        # 12b814 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8b4>
  34d4b1:	movups %xmm0,0xe(%rax)
  34d4b5:	movdqu -0x221cb7(%rip),%xmm0        # 12b806 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8a6>
  34d4bd:	movdqu %xmm0,(%rax)
  34d4c1:	movq   $0x1e,0x8(%rbp)
  34d4c9:	mov    %rax,0x10(%rbp)
  34d4cd:	movq   $0x1e,0x18(%rbp)
  34d4d5:	jmp    34d2d2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34d4da:	call   *0x2e7950(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d4e0:	mov    $0x1d,%ebx
  34d4e5:	mov    $0x1d,%edi
  34d4ea:	mov    $0x1,%esi
  34d4ef:	call   *0x2e7943(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d4f5:	test   %rax,%rax
  34d4f8:	je     34e7e2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e82>
  34d4fe:	movups -0x221c27(%rip),%xmm0        # 12b8de <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x97e>
  34d505:	movups %xmm0,0xd(%rax)
  34d509:	movdqu -0x221c40(%rip),%xmm0        # 12b8d1 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x971>
  34d511:	movdqu %xmm0,(%rax)
  34d515:	mov    0x18(%rsp),%rcx
  34d51a:	movq   $0x1d,0x8(%rcx)
  34d522:	mov    %rax,0x10(%rcx)
  34d526:	movq   $0x1d,0x18(%rcx)
  34d52e:	jmp    34d785 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe25>
  34d533:	mov    0x238(%rsp),%r15
  34d53b:	mov    0x240(%rsp),%rbx
  34d543:	cmpq   $0x0,0xa0(%rsp)
  34d54c:	je     34d561 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc01>
  34d54e:	mov    %r15,%rdi
  34d551:	mov    %rbx,%rsi
  34d554:	call   354af0 <<u64 as core::slice::cmp::SliceContains>::slice_contains>
  34d559:	test   %al,%al
  34d55b:	je     34d674 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd14>
  34d561:	test   %rbx,%rbx
  34d564:	je     34d587 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc27>
  34d566:	mov    %r15,%rdx
  34d569:	lea    0x0(,%rbx,8),%rcx
  34d571:	xor    %eax,%eax
  34d573:	cmpq   $0xffffffffffffffff,(%rdx,%rax,8)
  34d578:	je     34d6cd <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd6d>
  34d57e:	inc    %rax
  34d581:	add    $0xfffffffffffffff8,%rcx
  34d585:	jne    34d573 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc13>
  34d587:	mov    0x8(%rsp),%rax
  34d58c:	inc    %rax
  34d58f:	cmp    %rax,0x178(%rsp)
  34d597:	jne    34d617 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcb7>
  34d599:	mov    0xb0(%rsp),%rax
  34d5a1:	cmpl   $0x0,(%rax)
  34d5a4:	jne    34d617 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcb7>
  34d5a6:	mov    0xb0(%rsp),%rax
  34d5ae:	mov    0xd0(%rsp),%rcx
  34d5b6:	mov    0x178(%rsp),%rdx
  34d5be:	cmp    %ecx,-0x4(%rax,%rdx,4)
  34d5c2:	jne    34d617 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcb7>
  34d5c4:	mov    0x1e0(%rsp),%rax
  34d5cc:	cmp    %rax,0xd0(%rsp)
  34d5d4:	jne    34d617 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcb7>
  34d5d6:	mov    0xb0(%rsp),%rax
  34d5de:	mov    %rax,0x128(%rsp)
  34d5e6:	mov    0x178(%rsp),%rax
  34d5ee:	mov    %rax,0x130(%rsp)
  34d5f6:	movq   $0x2,0x138(%rsp)
  34d602:	lea    0x128(%rsp),%rdi
  34d60a:	call   341fc0 <<core::slice::iter::Windows<u32> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::traits::iterator::Iterator::any::check<&[u32], <snaptokens::models::bpe::Bpe>::from_native_tables::{closure#1}>::{closure#0}, core::ops::control_flow::ControlFlow<()>>>
  34d60f:	test   %al,%al
  34d611:	je     34d805 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xea5>
  34d617:	call   *0x2e7813(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d61d:	mov    $0x21,%ebx
  34d622:	mov    $0x21,%edi
  34d627:	mov    $0x1,%esi
  34d62c:	call   *0x2e7806(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d632:	test   %rax,%rax
  34d635:	je     34e7e2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e82>
  34d63b:	movups -0x221d82(%rip),%xmm0        # 12b8c0 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x960>
  34d642:	movups %xmm0,0x10(%rax)
  34d646:	movdqu -0x221d9e(%rip),%xmm0        # 12b8b0 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x950>
  34d64e:	movdqu %xmm0,(%rax)
  34d652:	movb   $0x65,0x20(%rax)
  34d656:	mov    0x18(%rsp),%rcx
  34d65b:	movq   $0x21,0x8(%rcx)
  34d663:	mov    %rax,0x10(%rcx)
  34d667:	movq   $0x21,0x18(%rcx)
  34d66f:	jmp    34d785 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe25>
  34d674:	call   *0x2e77b6(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d67a:	mov    $0x1e,%ebx
  34d67f:	mov    $0x1e,%edi
  34d684:	mov    $0x1,%esi
  34d689:	call   *0x2e77a9(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d68f:	test   %rax,%rax
  34d692:	je     34e7e2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e82>
  34d698:	movups -0x221e8b(%rip),%xmm0        # 12b814 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8b4>
  34d69f:	movups %xmm0,0xe(%rax)
  34d6a3:	movdqu -0x221ea5(%rip),%xmm0        # 12b806 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8a6>
  34d6ab:	movdqu %xmm0,(%rax)
  34d6af:	mov    0x18(%rsp),%rcx
  34d6b4:	movq   $0x1e,0x8(%rcx)
  34d6bc:	mov    %rax,0x10(%rcx)
  34d6c0:	movq   $0x1e,0x18(%rcx)
  34d6c8:	jmp    34d785 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe25>
  34d6cd:	lea    -0x1(%rbx),%rcx
  34d6d1:	mov    %rbx,%rdx
  34d6d4:	sub    %rax,%rdx
  34d6d7:	mov    $0x1,%r8d
  34d6dd:	movabs $0x517cc1b727220a95,%rsi
  34d6e7:	mov    %r8,%rdi
  34d6ea:	mov    %r15,%r10
  34d6ed:	cmp    %rbx,%rdi
  34d6f0:	jae    34d587 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc27>
  34d6f6:	lea    (%rax,%rdi,1),%r9
  34d6fa:	and    %rcx,%r9
  34d6fd:	mov    (%r10,%r9,8),%r9
  34d701:	cmp    $0xffffffffffffffff,%r9
  34d705:	je     34d724 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xdc4>
  34d707:	imul   %rsi,%r9
  34d70b:	and    %rcx,%r9
  34d70e:	add    %rdx,%r9
  34d711:	and    %rcx,%r9
  34d714:	cmp    %r8,%r9
  34d717:	jb     34d72c <<snaptokens::models::bpe::Bpe>::from_native_tables+0xdcc>
  34d719:	cmp    %rdi,%r9
  34d71c:	lea    0x1(%rdi),%rdi
  34d720:	jbe    34d6ed <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd8d>
  34d722:	jmp    34d72c <<snaptokens::models::bpe::Bpe>::from_native_tables+0xdcc>
  34d724:	inc    %rdi
  34d727:	mov    %rdi,%r8
  34d72a:	jmp    34d6ea <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd8a>
  34d72c:	call   *0x2e76fe(%rip)        # 634e30 <_DYNAMIC+0x258>
  34d732:	mov    $0x24,%ebx
  34d737:	mov    $0x24,%edi
  34d73c:	mov    $0x1,%esi
  34d741:	call   *0x2e76f1(%rip)        # 634e38 <_DYNAMIC+0x260>
  34d747:	test   %rax,%rax
  34d74a:	je     34e7e2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e82>
  34d750:	movups -0x221f23(%rip),%xmm0        # 12b834 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8d4>
  34d757:	movups %xmm0,0x10(%rax)
  34d75b:	movups -0x221f3e(%rip),%xmm0        # 12b824 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8c4>
  34d762:	movups %xmm0,(%rax)
  34d765:	movl   $0x6e696168,0x20(%rax)
  34d76c:	mov    0x18(%rsp),%rcx
  34d771:	movq   $0x24,0x8(%rcx)
  34d779:	mov    %rax,0x10(%rcx)
  34d77d:	movq   $0x24,0x18(%rcx)
  34d785:	movq   $0xffffffffffffffff,(%rcx)
  34d78c:	mov    $0x1,%r12b
  34d78f:	xor    %ebx,%ebx
  34d791:	mov    0x248(%rsp),%rsi
  34d799:	test   %rsi,%rsi
  34d79c:	je     34d7b5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe55>
  34d79e:	mov    0x250(%rsp),%rdi
  34d7a6:	shl    $0x3,%rsi
  34d7aa:	mov    $0x8,%edx
  34d7af:	call   *0x2e765b(%rip)        # 634e10 <_DYNAMIC+0x238>
  34d7b5:	mov    0x230(%rsp),%rsi
  34d7bd:	test   %rsi,%rsi
  34d7c0:	je     34d7d4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe74>
  34d7c2:	shl    $0x3,%rsi
  34d7c6:	mov    $0x8,%edx
  34d7cb:	mov    %r15,%rdi
  34d7ce:	call   *0x2e763c(%rip)        # 634e10 <_DYNAMIC+0x238>
  34d7d4:	mov    0x38(%rsp),%r13
  34d7d9:	mov    0x60(%rsp),%r14
  34d7de:	mov    0x78(%rsp),%r15
  34d7e3:	mov    0x18(%rsp),%rbp
  34d7e8:	jmp    34d2df <<snaptokens::models::bpe::Bpe>::from_native_tables+0x97f>
  34d7ed:	mov    %r13,0x38(%rsp)
  34d7f2:	mov    $0x1,%edi
  34d7f7:	mov    %rbp,%rsi
  34d7fa:	call   *0x2e7620(%rip)        # 634e20 <_DYNAMIC+0x248>
  34d800:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34d805:	sub    $0x8,%rsp
  34d809:	lea    0x130(%rsp),%rdi
  34d811:	lea    0x268(%rsp),%rsi
  34d819:	mov    0x1d8(%rsp),%edx
  34d820:	mov    0x1a8(%rsp),%rcx
  34d828:	mov    0xd0(%rsp),%r8
  34d830:	mov    0x1a0(%rsp),%r9
  34d838:	push   0x178(%rsp)
  34d83f:	push   0x1a0(%rsp)
  34d846:	push   0x1f0(%rsp)
  34d84d:	call   342030 <<snaptokens::models::bpe::VocabLookup>::from_cached_slots>
  34d852:	add    $0x20,%rsp
  34d856:	mov    0x128(%rsp),%rax
  34d85e:	movups 0x130(%rsp),%xmm0
  34d866:	movaps %xmm0,0x1f0(%rsp)
  34d86e:	mov    0x140(%rsp),%rcx
  34d876:	mov    %rcx,0x200(%rsp)
  34d87e:	cmp    $0xffffffffffffffff,%rax
  34d882:	je     34d8dd <<snaptokens::models::bpe::Bpe>::from_native_tables+0xf7d>
  34d884:	mov    0x158(%rsp),%rcx
  34d88c:	mov    %rcx,0x338(%rsp)
  34d894:	movups 0x148(%rsp),%xmm0
  34d89c:	movups %xmm0,0x328(%rsp)
  34d8a4:	movdqa 0x1f0(%rsp),%xmm0
  34d8ad:	movdqu %xmm0,0x310(%rsp)
  34d8b6:	mov    0x200(%rsp),%rcx
  34d8be:	mov    %rcx,0x320(%rsp)
  34d8c6:	mov    %rax,0x308(%rsp)
  34d8ce:	cmpq   $0x4,0x8(%rsp)
  34d8d4:	jae    34d901 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xfa1>
  34d8d6:	xor    %eax,%eax
  34d8d8:	jmp    34d9b2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1052>
  34d8dd:	mov    0x200(%rsp),%rax
  34d8e5:	mov    0x18(%rsp),%rcx
  34d8ea:	mov    %rax,0x18(%rcx)
  34d8ee:	movdqa 0x1f0(%rsp),%xmm0
  34d8f7:	movdqu %xmm0,0x8(%rcx)
  34d8fc:	jmp    34d785 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe25>
  34d901:	movabs $0x1000000000000000,%rcx
  34d90b:	cmpq   $0x20,0x8(%rsp)
  34d911:	jae    34d917 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xfb7>
  34d913:	xor    %eax,%eax
  34d915:	jmp    34d970 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1010>
  34d917:	lea    -0x20(%rcx),%rax
  34d91b:	and    0x8(%rsp),%rax
  34d920:	xor    %edx,%edx
  34d922:	pxor   %xmm0,%xmm0
  34d926:	movdqa -0x2999ae(%rip),%xmm1        # b3f80 <anon.3936869e533d7f4477a48c47cce68eb7.17.llvm.7645234870251011244+0x150>
  34d92e:	mov    0x48(%rsp),%rsi
  34d933:	movdqu (%rsi,%rdx,1),%xmm2
  34d938:	movdqu 0x10(%rsi,%rdx,1),%xmm3
  34d93e:	pcmpeqb %xmm0,%xmm2
  34d942:	pandn  %xmm1,%xmm2
  34d946:	pcmpeqb %xmm0,%xmm3
  34d94a:	pandn  %xmm1,%xmm3
  34d94e:	movdqu %xmm2,(%rsi,%rdx,1)
  34d953:	movdqu %xmm3,0x10(%rsi,%rdx,1)
  34d959:	add    $0x20,%rdx
  34d95d:	cmp    %rdx,%rax
  34d960:	jne    34d933 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xfd3>
  34d962:	cmp    %rax,0x8(%rsp)
  34d967:	je     34d9c9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1069>
  34d969:	testb  $0x1c,0x8(%rsp)
  34d96e:	je     34d9b2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1052>
  34d970:	mov    %rax,%rdx
  34d973:	add    $0xfffffffffffffffc,%rcx
  34d977:	mov    %rcx,%rax
  34d97a:	and    0x8(%rsp),%rax
  34d97f:	pxor   %xmm0,%xmm0
  34d983:	movdqa -0x2993ab(%rip),%xmm1        # b45e0 <anon.27fe456d1cf0abeeb19e136867562327.101.llvm.18345180354296356013+0x650>
  34d98b:	mov    0x48(%rsp),%rcx
  34d990:	movd   (%rcx,%rdx,1),%xmm2
  34d995:	pcmpeqb %xmm0,%xmm2
  34d999:	pandn  %xmm1,%xmm2
  34d99d:	movd   %xmm2,(%rcx,%rdx,1)
  34d9a2:	add    $0x4,%rdx
  34d9a6:	cmp    %rdx,%rax
  34d9a9:	jne    34d990 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1030>
  34d9ab:	cmp    %rax,0x8(%rsp)
  34d9b0:	je     34d9c9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1069>
  34d9b2:	mov    0x48(%rsp),%rcx
  34d9b7:	cmpb   $0x0,(%rcx,%rax,1)
  34d9bb:	setne  (%rcx,%rax,1)
  34d9bf:	inc    %rax
  34d9c2:	cmp    %rax,0x8(%rsp)
  34d9c7:	jne    34d9b7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1057>
  34d9c9:	mov    0xf0(%rsp),%rcx
  34d9d1:	mov    0x8(%rsp),%rax
  34d9d6:	lea    (%rcx,%rax,8),%rax
  34d9da:	mov    (%rcx),%edx
  34d9dc:	cmp    %rdx,0x8(%rsp)
  34d9e1:	jbe    34da5a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10fa>
  34d9e3:	mov    0x4(%rcx),%edx
  34d9e6:	cmp    %rdx,0x8(%rsp)
  34d9eb:	jbe    34da5a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10fa>
  34d9ed:	add    $0x8,%rcx
  34d9f1:	cmp    %rax,%rcx
  34d9f4:	jne    34d9da <<snaptokens::models::bpe::Bpe>::from_native_tables+0x107a>
  34d9f6:	mov    0xe8(%rsp),%rsi
  34d9fe:	lea    0x400(%rsi),%rax
  34da05:	xor    %ecx,%ecx
  34da07:	mov    $0xffffffff,%edx
  34da0c:	jmp    34da30 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10d0>
  34da0e:	lea    0x4(%rsi),%rdi
  34da12:	mov    (%rsi),%esi
  34da14:	cmp    %rdx,%rsi
  34da17:	setne  %r8b
  34da1b:	cmp    %rsi,0x8(%rsp)
  34da20:	setbe  %r9b
  34da24:	mov    %rdi,%rsi
  34da27:	test   %r9b,%r8b
  34da2a:	jne    34dab7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1157>
  34da30:	test   %rsi,%rsi
  34da33:	je     34da3a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10da>
  34da35:	cmp    %rax,%rsi
  34da38:	jne    34da0e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10ae>
  34da3a:	cmp    $0x400,%rcx
  34da41:	je     34db57 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11f7>
  34da47:	mov    0xf8(%rsp),%rsi
  34da4f:	add    %rcx,%rsi
  34da52:	add    $0x4,%rcx
  34da56:	xor    %edi,%edi
  34da58:	jmp    34da12 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10b2>
  34da5a:	call   *0x2e73d0(%rip)        # 634e30 <_DYNAMIC+0x258>
  34da60:	mov    $0x24,%ebx
  34da65:	mov    $0x24,%edi
  34da6a:	mov    $0x1,%esi
  34da6f:	call   *0x2e73c3(%rip)        # 634e38 <_DYNAMIC+0x260>
  34da75:	test   %rax,%rax
  34da78:	je     34e80d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ead>
  34da7e:	movups -0x22222d(%rip),%xmm0        # 12b858 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8f8>
  34da85:	movups %xmm0,0x10(%rax)
  34da89:	movdqu -0x222249(%rip),%xmm0        # 12b848 <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x8e8>
  34da91:	movdqu %xmm0,(%rax)
  34da95:	movl   $0x6e656b6f,0x20(%rax)
  34da9c:	mov    0x18(%rsp),%rcx
  34daa1:	movq   $0x24,0x8(%rcx)
  34daa9:	mov    %rax,0x10(%rcx)
  34daad:	movq   $0x24,0x18(%rcx)
  34dab5:	jmp    34db0b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11ab>
  34dab7:	call   *0x2e7373(%rip)        # 634e30 <_DYNAMIC+0x258>
  34dabd:	mov    $0x20,%ebx
  34dac2:	mov    $0x20,%edi
  34dac7:	mov    $0x1,%esi
  34dacc:	call   *0x2e7366(%rip)        # 634e38 <_DYNAMIC+0x260>
  34dad2:	test   %rax,%rax
  34dad5:	je     34e80d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ead>
  34dadb:	movups -0x28e3b2(%rip),%xmm0        # bf730 <anon.bdde665530aa007dd7855882198d96b9.18.llvm.12424819659099699973+0xb0>
  34dae2:	movups %xmm0,0x10(%rax)
  34dae6:	movdqu -0x28e3ce(%rip),%xmm0        # bf720 <anon.bdde665530aa007dd7855882198d96b9.18.llvm.12424819659099699973+0xa0>
  34daee:	movdqu %xmm0,(%rax)
  34daf2:	mov    0x18(%rsp),%rcx
  34daf7:	movq   $0x20,0x8(%rcx)
  34daff:	mov    %rax,0x10(%rcx)
  34db03:	movq   $0x20,0x18(%rcx)
  34db0b:	movq   $0xffffffffffffffff,(%rcx)
  34db12:	mov    $0x1,%r12b
  34db15:	cmpq   $0x0,0x50(%rsp)
  34db1b:	je     34db32 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11d2>
  34db1d:	mov    $0x1,%edx
  34db22:	mov    0x48(%rsp),%rdi
  34db27:	mov    0x50(%rsp),%rsi
  34db2c:	call   *0x2e72de(%rip)        # 634e10 <_DYNAMIC+0x238>
  34db32:	lea    0x308(%rsp),%rdi
  34db3a:	call   33eb70 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabLookup>>
  34db3f:	mov    $0x1,%bl
  34db41:	mov    0x248(%rsp),%rsi
  34db49:	test   %rsi,%rsi
  34db4c:	jne    34d79e <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe3e>
  34db52:	jmp    34d7b5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe55>
  34db57:	cmpq   $0x0,0xd0(%rsp)
  34db60:	je     34db93 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1233>
  34db62:	mov    0xd0(%rsp),%rax
  34db6a:	lea    0x0(,%rax,4),%rax
  34db72:	xor    %ecx,%ecx
  34db74:	mov    0x188(%rsp),%rdx
  34db7c:	mov    (%rdx,%rcx,1),%edx
  34db7f:	cmp    %rdx,0x8(%rsp)
  34db84:	jbe    34dd39 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13d9>
  34db8a:	add    $0x4,%rcx
  34db8e:	cmp    %rcx,%rax
  34db91:	jne    34db74 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1214>
  34db93:	cmpq   $0x0,0x40(%rsp)
  34db99:	je     34dc4a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ea>
  34db9f:	xor    %eax,%eax
  34dba1:	jmp    34dbb1 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1251>
  34dba3:	inc    %rax
  34dba6:	cmp    %rax,0x40(%rsp)
  34dbab:	je     34dc4a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ea>
  34dbb1:	mov    0x98(%rsp),%rcx
  34dbb9:	mov    (%rcx,%rax,8),%rcx
  34dbbd:	cmp    $0xffffffffffffffff,%rcx
  34dbc1:	je     34dba3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1243>
  34dbc3:	mov    %rcx,%rdx
  34dbc6:	shr    $0x20,%rdx
  34dbca:	cmp    0x8(%rsp),%rdx
  34dbcf:	jae    34dbec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x128c>
  34dbd1:	mov    %ecx,%ecx
  34dbd3:	cmp    0x8(%rsp),%rcx
  34dbd8:	jae    34dbec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x128c>
  34dbda:	mov    0x90(%rsp),%rcx
  34dbe2:	mov    (%rcx,%rax,8),%ecx
  34dbe5:	cmp    0x8(%rsp),%rcx
  34dbea:	jb     34dba3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1243>
  34dbec:	call   *0x2e723e(%rip)        # 634e30 <_DYNAMIC+0x258>
  34dbf2:	mov    $0x23,%ebx
  34dbf7:	mov    $0x23,%edi
  34dbfc:	mov    $0x1,%esi
  34dc01:	call   *0x2e7231(%rip)        # 634e38 <_DYNAMIC+0x260>
  34dc07:	test   %rax,%rax
  34dc0a:	je     34e80d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ead>
  34dc10:	movups -0x22237a(%rip),%xmm0        # 12b89d <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x93d>
  34dc17:	movups %xmm0,0x10(%rax)
  34dc1b:	movups -0x222395(%rip),%xmm0        # 12b88d <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x92d>
  34dc22:	movups %xmm0,(%rax)
  34dc25:	movl   $0x6e656b6f,0x1f(%rax)
  34dc2c:	mov    0x18(%rsp),%rcx
  34dc31:	movq   $0x23,0x8(%rcx)
  34dc39:	mov    %rax,0x10(%rcx)
  34dc3d:	movq   $0x23,0x18(%rcx)
  34dc45:	jmp    34db0b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11ab>
  34dc4a:	cmpq   $0x0,0x1c8(%rsp)
  34dc53:	je     34dc83 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1323>
  34dc55:	mov    0x1c8(%rsp),%rax
  34dc5d:	shl    $0x5,%rax
  34dc61:	xor    %ecx,%ecx
  34dc63:	mov    0xe0(%rsp),%rdx
  34dc6b:	mov    0x10(%rdx,%rcx,1),%edx
  34dc6f:	cmp    %rdx,0x8(%rsp)
  34dc74:	jbe    34dd88 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1428>
  34dc7a:	add    $0x20,%rcx
  34dc7e:	cmp    %rcx,%rax
  34dc81:	jne    34dc63 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1303>
  34dc83:	cmpq   $0xffffffffffffffff,0x340(%rsp)
  34dc8c:	je     34dde3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1483>
  34dc92:	xor    %r12d,%r12d
  34dc95:	lea    0x128(%rsp),%rdi
  34dc9d:	lea    0x260(%rsp),%r9
  34dca5:	mov    0x2d0(%rsp),%rsi
  34dcad:	mov    0x8(%rsp),%rdx
  34dcb2:	mov    0x48(%rsp),%rcx
  34dcb7:	mov    %rdx,%r8
  34dcba:	call   33ca60 <<snaptokens::models::bpe::ExactTokenTrie>::validate_with::<<snaptokens::models::bpe::Bpe>::from_native_tables::{closure#3}>>
  34dcbf:	mov    0x128(%rsp),%rax
  34dcc7:	movups 0x130(%rsp),%xmm0
  34dccf:	movaps %xmm0,0x1f0(%rsp)
  34dcd7:	mov    0x140(%rsp),%rcx
  34dcdf:	mov    %rcx,0x200(%rsp)
  34dce7:	cmp    $0xffffffffffffffff,%rax
  34dceb:	je     34e796 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e36>
  34dcf1:	movups 0x148(%rsp),%xmm0
  34dcf9:	movups %xmm0,0x2b0(%rsp)
  34dd01:	movdqa 0x1f0(%rsp),%xmm0
  34dd0a:	movdqu %xmm0,0x298(%rsp)
  34dd13:	mov    0x200(%rsp),%rcx
  34dd1b:	mov    %rcx,0x2a8(%rsp)
  34dd23:	mov    %rax,0x290(%rsp)
  34dd2b:	mov    $0x1,%al
  34dd2d:	mov    %eax,0x58(%rsp)
  34dd31:	xor    %r12d,%r12d
  34dd34:	jmp    34de21 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x14c1>
  34dd39:	call   *0x2e70f1(%rip)        # 634e30 <_DYNAMIC+0x258>
  34dd3f:	mov    $0x20,%ebx
  34dd44:	mov    $0x20,%edi
  34dd49:	mov    $0x1,%esi
  34dd4e:	call   *0x2e70e4(%rip)        # 634e38 <_DYNAMIC+0x260>
  34dd54:	test   %rax,%rax
  34dd57:	je     34e80d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ead>
  34dd5d:	movups -0x28e134(%rip),%xmm0        # bfc30 <anon.3b3cd0bc6b2536aa46935e5d0aff9f95.56.llvm.6491278627211742442+0x110>
  34dd64:	movups %xmm0,0x10(%rax)
  34dd68:	movdqu -0x28e150(%rip),%xmm0        # bfc20 <anon.3b3cd0bc6b2536aa46935e5d0aff9f95.56.llvm.6491278627211742442+0x100>
  34dd70:	jmp    34daee <<snaptokens::models::bpe::Bpe>::from_native_tables+0x118e>
  34dd75:	mov    $0x1,%edi
  34dd7a:	mov    %rbx,%rsi
  34dd7d:	call   *0x2e709d(%rip)        # 634e20 <_DYNAMIC+0x248>
  34dd83:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34dd88:	call   *0x2e70a2(%rip)        # 634e30 <_DYNAMIC+0x258>
  34dd8e:	mov    $0x21,%ebx
  34dd93:	mov    $0x21,%edi
  34dd98:	mov    $0x1,%esi
  34dd9d:	call   *0x2e7095(%rip)        # 634e38 <_DYNAMIC+0x260>
  34dda3:	test   %rax,%rax
  34dda6:	je     34e80d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ead>
  34ddac:	movups -0x222537(%rip),%xmm0        # 12b87c <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x91c>
  34ddb3:	movups %xmm0,0x10(%rax)
  34ddb7:	movups -0x222552(%rip),%xmm0        # 12b86c <anon.46580aa77014d508262b833402f42857.40.llvm.14282664211216954348+0x90c>
  34ddbe:	movups %xmm0,(%rax)
  34ddc1:	movb   $0x64,0x20(%rax)
  34ddc5:	mov    0x18(%rsp),%rcx
  34ddca:	movq   $0x21,0x8(%rcx)
  34ddd2:	mov    %rax,0x10(%rcx)
  34ddd6:	movq   $0x21,0x18(%rcx)
  34ddde:	jmp    34db0b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11ab>
  34dde3:	mov    0x50(%rsp),%rax
  34dde8:	mov    %rax,0x298(%rsp)
  34ddf0:	mov    0x48(%rsp),%rax
  34ddf5:	mov    %rax,0x2a0(%rsp)
  34ddfd:	mov    0x8(%rsp),%rax
  34de02:	mov    %rax,0x2a8(%rsp)
  34de0a:	movq   $0xffffffffffffffff,0x290(%rsp)
  34de16:	mov    $0x1,%r12b
  34de19:	movl   $0x0,0x58(%rsp)
  34de21:	lea    0x3d0(%rsp),%rdi
  34de29:	mov    0x2e7010(%rip),%rbx        # 634e40 <memcpy@GLIBC_2.14>
  34de30:	mov    $0x400,%edx
  34de35:	mov    0xe8(%rsp),%rsi
  34de3d:	call   *%rbx
  34de3f:	lea    0x9d0(%rsp),%rdi
  34de47:	mov    $0x400,%edx
  34de4c:	mov    0xf8(%rsp),%rsi
  34de54:	call   *%rbx
  34de56:	lea    0x128(%rsp),%rdi
  34de5e:	mov    $0x10000,%esi
  34de63:	call   341d80 <<u32 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>>
  34de68:	lea    0x128(%rsp),%rdi
  34de70:	call   3c83e0 <<alloc::vec::Vec<u32>>::into_boxed_slice>
  34de75:	mov    %rax,0x28(%rsp)
  34de7a:	mov    %rdx,0x20(%rsp)
  34de7f:	mov    0x8(%rsp),%r13
  34de84:	cmp    $0x1,%r13
  34de88:	adc    $0x0,%r13
  34de8c:	xor    %r14d,%r14d
  34de8f:	lea    0x128(%rsp),%rbx
  34de97:	jmp    34deaa <<snaptokens::models::bpe::Bpe>::from_native_tables+0x154a>
  34de99:	mov    0x28(%rsp),%rax
  34de9e:	mov    %r14d,(%rax,%rdi,4)
  34dea2:	inc    %r14
  34dea5:	cmp    %r14,%r13
  34dea8:	je     34df28 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x15c8>
  34deaa:	lea    0x260(%rsp),%rdi
  34deb2:	mov    %r14d,%esi
  34deb5:	call   345540 <<snaptokens::models::bpe::VocabArena>::get>
  34deba:	test   %rax,%rax
  34debd:	je     34e7d3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e73>
  34dec3:	add    %rax,%rdx
  34dec6:	mov    %rax,0x128(%rsp)
  34dece:	mov    %rdx,0x130(%rsp)
  34ded6:	mov    %rbx,%rdi
  34ded9:	call   33ed30 <core::str::validations::next_code_point::<core::slice::iter::Iter<u8>>>
  34dede:	and    $0x1,%al
  34dee0:	cmp    $0x1,%al
  34dee2:	mov    $0x0,%ebp
  34dee7:	sbb    %ebp,%ebp
  34dee9:	or     %edx,%ebp
  34deeb:	mov    %rbx,%rdi
  34deee:	call   33ed30 <core::str::validations::next_code_point::<core::slice::iter::Iter<u8>>>
  34def3:	cmp    $0xffffffff,%ebp
  34def6:	sete   %cl
  34def9:	or     %al,%cl
  34defb:	test   $0x1,%cl
  34defe:	jne    34dea2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1542>
  34df00:	cmp    $0xffff,%ebp
  34df06:	ja     34dea2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1542>
  34df08:	mov    %ebp,%edi
  34df0a:	cmp    %rdi,0x20(%rsp)
  34df0f:	ja     34de99 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1539>
  34df11:	lea    0x2a5d58(%rip),%rdx        # 5f3c70 <anon.46580aa77014d508262b833402f42857.109.llvm.14282664211216954348+0x2a0>
  34df18:	mov    0x20(%rsp),%rsi
  34df1d:	call   *0x2e739d(%rip)        # 6352c0 <_DYNAMIC+0x6e8>
  34df23:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34df28:	cmpq   $0x7f,0x20(%rsp)
  34df2e:	jbe    34e7f2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e92>
  34df34:	lea    0x7d0(%rsp),%rdi
  34df3c:	mov    $0x200,%edx
  34df41:	mov    0x28(%rsp),%rsi
  34df46:	call   *0x2e6ef4(%rip)        # 634e40 <memcpy@GLIBC_2.14>
  34df4c:	movq   $0x0,0x1f8(%rsp)
  34df58:	mov    0x8(%rsp),%rax
  34df5d:	mov    %rax,0x200(%rsp)
  34df65:	lea    0x260(%rsp),%rax
  34df6d:	mov    %rax,0x1f0(%rsp)
  34df75:	lea    0x128(%rsp),%rdi
  34df7d:	lea    0x1f0(%rsp),%rsi
  34df85:	call   390b70 <core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::Bpe>::from_native_tables::{closure#4}>, u8, core::result::Result<core::convert::Infallible, alloc::string::String>, <core::result::Result<alloc::vec::Vec<u8>, alloc::string::String> as core::iter::traits::collect::FromIterator<core::result::Result<u8, alloc::string::String>>>::from_iter<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::Bpe>::from_native_tables::{closure#4}>>::{closure#0}, alloc::vec::Vec<u8>>>
  34df8a:	mov    0x130(%rsp),%rax
  34df92:	mov    %rax,0x68(%rsp)
  34df97:	mov    0x138(%rsp),%rax
  34df9f:	mov    %rax,0xa8(%rsp)
  34dfa7:	mov    0x140(%rsp),%rbp
  34dfaf:	cmpb   $0x0,0x128(%rsp)
  34dfb7:	je     34e018 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x16b8>
  34dfb9:	mov    0x18(%rsp),%rax
  34dfbe:	mov    0x68(%rsp),%rcx
  34dfc3:	mov    %rcx,0x8(%rax)
  34dfc7:	mov    0xa8(%rsp),%rcx
  34dfcf:	mov    %rcx,0x10(%rax)
  34dfd3:	mov    %rbp,0x18(%rax)
  34dfd7:	movq   $0xffffffffffffffff,(%rax)
  34dfde:	mov    0x20(%rsp),%rax
  34dfe3:	lea    0x0(,%rax,4),%rsi
  34dfeb:	mov    $0x4,%edx
  34dff0:	mov    0x28(%rsp),%rdi
  34dff5:	call   *0x2e6e15(%rip)        # 634e10 <_DYNAMIC+0x238>
  34dffb:	lea    0x290(%rsp),%rdi
  34e003:	call   33ec90 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenMatcher>>
  34e008:	cmpb   $0x0,0x58(%rsp)
  34e00d:	jne    34db15 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11b5>
  34e013:	jmp    34db32 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11d2>
  34e018:	mov    0x180(%rsp),%rax
  34e020:	mov    (%rax),%rbx
  34e023:	movdqu 0x8(%rax),%xmm0
  34e028:	movdqa %xmm0,0x2c0(%rsp)
  34e031:	lea    0x18(%rax),%rsi
  34e035:	lea    0xdd0(%rsp),%rdi
  34e03d:	mov    $0x1fe8,%edx
  34e042:	call   *0x2e6df8(%rip)        # 634e40 <memcpy@GLIBC_2.14>
  34e048:	cmpq   $0x0,0x100(%rsp)
  34e051:	je     34e076 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1716>
  34e053:	mov    0x100(%rsp),%rax
  34e05b:	lea    0x0(,%rax,8),%rsi
  34e063:	mov    $0x8,%edx
  34e068:	mov    0x180(%rsp),%rdi
  34e070:	call   *0x2e6d9a(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e076:	lea    0x2d8(%rsp),%rdi
  34e07e:	lea    0x3d0(%rsp),%rsi
  34e086:	mov    0x8(%rsp),%rdx
  34e08b:	call   353a70 <snaptokens::models::bpe::initial_token_byte_map>
  34e090:	mov    0x2e0(%rsp),%r9
  34e098:	sub    $0x8,%rsp
  34e09c:	lea    0x2f8(%rsp),%rdi
  34e0a4:	mov    0xa0(%rsp),%rsi
  34e0ac:	mov    0x48(%rsp),%rdx
  34e0b1:	mov    0x98(%rsp),%rcx
  34e0b9:	mov    %rdx,%r8
  34e0bc:	mov    %r9,0x1d8(%rsp)
  34e0c4:	push   0x2f0(%rsp)
  34e0cb:	call   3545d0 <snaptokens::models::bpe::byte_pair_initial_from_ranked>
  34e0d0:	add    $0x10,%rsp
  34e0d4:	movzbl 0x17(%rsp),%eax
  34e0d9:	movzbl 0x16(%rsp),%r10d
  34e0df:	lea    0x128(%rsp),%rdi
  34e0e7:	lea    0x3d0(%rsp),%r9
  34e0ef:	mov    0x98(%rsp),%rsi
  34e0f7:	mov    0x40(%rsp),%rdx
  34e0fc:	mov    0x90(%rsp),%rcx
  34e104:	mov    %rdx,%r8
  34e107:	push   %rax
  34e108:	push   %r10
  34e10a:	call   353ba0 <snaptokens::models::bpe::dense_tables_from_ranked>
  34e10f:	add    $0x10,%rsp
  34e113:	mov    0x140(%rsp),%eax
  34e11a:	mov    %eax,0x40(%rsp)
  34e11e:	mov    0x128(%rsp),%rax
  34e126:	mov    %rax,0x88(%rsp)
  34e12e:	mov    0x130(%rsp),%rax
  34e136:	mov    %rax,0xa0(%rsp)
  34e13e:	mov    0x138(%rsp),%rax
  34e146:	mov    %rax,0x1e0(%rsp)
  34e14e:	mov    0x148(%rsp),%rax
  34e156:	mov    %rax,0x80(%rsp)
  34e15e:	mov    0x150(%rsp),%rax
  34e166:	mov    %rax,0xd8(%rsp)
  34e16e:	mov    0x158(%rsp),%rax
  34e176:	mov    %rax,0x1d8(%rsp)
  34e17e:	mov    $0x1,%eax
  34e183:	mov    $0x1,%r13d
  34e189:	lock xadd %r13,0x2ef786(%rip)        # 63d918 <snaptokens::models::bpe::BPE_ID_COUNTER>
  34e192:	test   %r13,%r13
  34e195:	jne    34e1a3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1843>
  34e197:	lock xadd %rax,0x2ef778(%rip)        # 63d918 <snaptokens::models::bpe::BPE_ID_COUNTER>
  34e1a0:	mov    %rax,%r13
  34e1a3:	lea    0x370(%rsp),%rdi
  34e1ab:	mov    $0x40,%edx
  34e1b0:	xor    %esi,%esi
  34e1b2:	call   3ca2f0 <<alloc::vec::Vec<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>, core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::SharedCache>::new::{closure#0}>>>::from_iter>
  34e1b7:	lea    0x388(%rsp),%rdi
  34e1bf:	mov    $0x40,%edx
  34e1c4:	xor    %esi,%esi
  34e1c6:	call   3ca2f0 <<alloc::vec::Vec<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>, core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::SharedCache>::new::{closure#0}>>>::from_iter>
  34e1cb:	movups 0x260(%rsp),%xmm0
  34e1d3:	movdqu 0x270(%rsp),%xmm1
  34e1dc:	movdqu 0x280(%rsp),%xmm2
  34e1e5:	movdqa %xmm2,0x3c0(%rsp)
  34e1ee:	movdqa %xmm1,0x3b0(%rsp)
  34e1f7:	mov    0x240(%rsp),%rax
  34e1ff:	cmp    $0x1,%rax
  34e203:	mov    %rax,0x200(%rsp)
  34e20b:	adc    $0xffffffffffffffff,%rax
  34e20f:	movaps %xmm0,0x3a0(%rsp)
  34e217:	movups 0x230(%rsp),%xmm0
  34e21f:	movaps %xmm0,0x1f0(%rsp)
  34e227:	movdqu 0x248(%rsp),%xmm0
  34e230:	movdqu %xmm0,0x208(%rsp)
  34e239:	mov    0x258(%rsp),%rcx
  34e241:	mov    %rcx,0x218(%rsp)
  34e249:	mov    %rax,0x220(%rsp)
  34e251:	mov    0x2f0(%rsp),%rax
  34e259:	mov    %rax,0xc8(%rsp)
  34e261:	mov    0x2f8(%rsp),%rax
  34e269:	mov    %rax,0x170(%rsp)
  34e271:	mov    0x300(%rsp),%r14
  34e279:	mov    0x1c0(%rsp),%rax
  34e281:	mov    %rax,0x128(%rsp)
  34e289:	mov    0xb0(%rsp),%rax
  34e291:	mov    %rax,0x130(%rsp)
  34e299:	mov    0x178(%rsp),%rax
  34e2a1:	mov    %rax,0x138(%rsp)
  34e2a9:	mov    0x1b8(%rsp),%rax
  34e2b1:	mov    %rax,0x140(%rsp)
  34e2b9:	mov    0x1e8(%rsp),%rax
  34e2c1:	mov    %rax,0x148(%rsp)
  34e2c9:	mov    0xd0(%rsp),%rcx
  34e2d1:	mov    %rcx,0x150(%rsp)
  34e2d9:	mov    0x1b0(%rsp),%rax
  34e2e1:	mov    %rax,0x158(%rsp)
  34e2e9:	mov    0x188(%rsp),%rax
  34e2f1:	mov    %rax,0x160(%rsp)
  34e2f9:	mov    %rcx,0x168(%rsp)
  34e301:	mov    $0x8,%edi
  34e306:	mov    $0x2000,%esi
  34e30b:	call   353880 <alloc::boxed::box_new_uninit>
  34e310:	mov    %rax,%r12
  34e313:	mov    %rbx,(%rax)
  34e316:	movaps 0x2c0(%rsp),%xmm0
  34e31e:	movups %xmm0,0x8(%rax)
  34e322:	add    $0x18,%rax
  34e326:	lea    0xdd0(%rsp),%rsi
  34e32e:	mov    $0x1fe8,%edx
  34e333:	mov    %rax,%rdi
  34e336:	mov    0x2e6b03(%rip),%r15        # 634e40 <memcpy@GLIBC_2.14>
  34e33d:	call   *%r15
  34e340:	movups 0x290(%rsp),%xmm0
  34e348:	movups 0x2a0(%rsp),%xmm1
  34e350:	movups 0x2b0(%rsp),%xmm2
  34e358:	mov    0x18(%rsp),%rbx
  34e35d:	movups %xmm2,0x1c8(%rbx)
  34e364:	movups %xmm1,0x1b8(%rbx)
  34e36b:	movups %xmm0,0x1a8(%rbx)
  34e372:	mov    0x380(%rsp),%rax
  34e37a:	mov    %rax,0x40(%rbx)
  34e37e:	movups 0x370(%rsp),%xmm0
  34e386:	movups %xmm0,0x30(%rbx)
  34e38a:	mov    0x398(%rsp),%rax
  34e392:	mov    %rax,0x58(%rbx)
  34e396:	movups 0x388(%rsp),%xmm0
  34e39e:	movups %xmm0,0x48(%rbx)
  34e3a2:	movaps 0x3a0(%rsp),%xmm0
  34e3aa:	movaps 0x3b0(%rsp),%xmm1
  34e3b2:	movaps 0x3c0(%rsp),%xmm2
  34e3ba:	movups %xmm1,0x70(%rbx)
  34e3be:	movups %xmm0,0x60(%rbx)
  34e3c2:	movups %xmm2,0x80(%rbx)
  34e3c9:	mov    0x338(%rsp),%rax
  34e3d1:	mov    %rax,0xc0(%rbx)
  34e3d8:	movups 0x308(%rsp),%xmm0
  34e3e0:	movups 0x318(%rsp),%xmm1
  34e3e8:	movups 0x328(%rsp),%xmm2
  34e3f0:	movups %xmm2,0xb0(%rbx)
  34e3f7:	movups %xmm1,0xa0(%rbx)
  34e3fe:	movups %xmm0,0x90(%rbx)
  34e405:	lea    0x1f8(%rbx),%rdi
  34e40c:	lea    0x3d0(%rsp),%rsi
  34e414:	mov    $0x400,%edx
  34e419:	call   *%r15
  34e41c:	lea    0x5f8(%rbx),%rdi
  34e423:	lea    0x9d0(%rsp),%rsi
  34e42b:	mov    $0x400,%edx
  34e430:	call   *%r15
  34e433:	lea    0x9f8(%rbx),%rdi
  34e43a:	lea    0x7d0(%rsp),%rsi
  34e442:	mov    $0x200,%edx
  34e447:	call   *%r15
  34e44a:	mov    0x220(%rsp),%rax
  34e452:	mov    %rax,0xf8(%rbx)
  34e459:	movaps 0x1f0(%rsp),%xmm0
  34e461:	movaps 0x200(%rsp),%xmm1
  34e469:	movaps 0x210(%rsp),%xmm2
  34e471:	movups %xmm0,0xc8(%rbx)
  34e478:	movups %xmm1,0xd8(%rbx)
  34e47f:	movups %xmm2,0xe8(%rbx)
  34e486:	mov    0x168(%rsp),%rax
  34e48e:	mov    %rax,0x1a0(%rbx)
  34e495:	movdqu 0x128(%rsp),%xmm0
  34e49e:	movdqu 0x138(%rsp),%xmm1
  34e4a7:	movdqu 0x148(%rsp),%xmm2
  34e4b0:	movdqu 0x158(%rsp),%xmm3
  34e4b9:	movdqu %xmm3,0x190(%rbx)
  34e4c1:	movdqu %xmm2,0x180(%rbx)
  34e4c9:	movdqu %xmm1,0x170(%rbx)
  34e4d1:	movdqu %xmm0,0x160(%rbx)
  34e4d9:	mov    0xc0(%rsp),%rax
  34e4e1:	mov    %rax,(%rbx)
  34e4e4:	mov    0xf0(%rsp),%rax
  34e4ec:	mov    %rax,0x8(%rbx)
  34e4f0:	mov    0x8(%rsp),%rax
  34e4f5:	mov    %rax,0x10(%rbx)
  34e4f9:	mov    0x68(%rsp),%rax
  34e4fe:	mov    %rax,0x18(%rbx)
  34e502:	mov    0xa8(%rsp),%rax
  34e50a:	mov    %rax,0x20(%rbx)
  34e50e:	mov    %rbp,0x28(%rbx)
  34e512:	mov    0xc8(%rsp),%rax
  34e51a:	mov    %rax,0x100(%rbx)
  34e521:	mov    0x170(%rsp),%rax
  34e529:	mov    %rax,0x108(%rbx)
  34e530:	mov    %r14,0x110(%rbx)
  34e537:	mov    0x80(%rsp),%rax
  34e53f:	mov    %rax,0x118(%rbx)
  34e546:	mov    0xd8(%rsp),%rax
  34e54e:	mov    %rax,0x120(%rbx)
  34e555:	mov    0x1d8(%rsp),%rax
  34e55d:	mov    %rax,0x128(%rbx)
  34e564:	mov    0x88(%rsp),%rax
  34e56c:	mov    %rax,0x130(%rbx)
  34e573:	mov    0xa0(%rsp),%rax
  34e57b:	mov    %rax,0x138(%rbx)
  34e582:	mov    0x1e0(%rsp),%rax
  34e58a:	mov    %rax,0x140(%rbx)
  34e591:	mov    0xb8(%rsp),%rax
  34e599:	mov    %rax,0x148(%rbx)
  34e5a0:	mov    0xe0(%rsp),%rax
  34e5a8:	mov    %rax,0x150(%rbx)
  34e5af:	mov    0x1c8(%rsp),%rax
  34e5b7:	mov    %rax,0x158(%rbx)
  34e5be:	mov    0x28(%rsp),%rax
  34e5c3:	mov    %rax,0x1d8(%rbx)
  34e5ca:	mov    0x20(%rsp),%rax
  34e5cf:	mov    %rax,0x1e0(%rbx)
  34e5d6:	mov    %r12,0x1e8(%rbx)
  34e5dd:	mov    %r13,0x1f0(%rbx)
  34e5e4:	mov    0x40(%rsp),%eax
  34e5e8:	mov    %eax,0xbf8(%rbx)
  34e5ee:	movzbl 0x17(%rsp),%eax
  34e5f3:	mov    %al,0xbfc(%rbx)
  34e5f9:	movzbl 0x37(%rsp),%eax
  34e5fe:	mov    %al,0xbfd(%rbx)
  34e604:	movzbl 0x16(%rsp),%eax
  34e609:	mov    %al,0xbfe(%rbx)
  34e60f:	mov    0x2d8(%rsp),%rsi
  34e617:	test   %rsi,%rsi
  34e61a:	je     34e632 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1cd2>
  34e61c:	mov    0x2e0(%rsp),%rdi
  34e624:	add    %rsi,%rsi
  34e627:	mov    $0x2,%edx
  34e62c:	call   *0x2e67de(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e632:	cmpq   $0x0,0x50(%rsp)
  34e638:	setne  %al
  34e63b:	test   %al,0x58(%rsp)
  34e63f:	je     34e656 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1cf6>
  34e641:	mov    $0x1,%edx
  34e646:	mov    0x48(%rsp),%rdi
  34e64b:	mov    0x50(%rsp),%rsi
  34e650:	call   *0x2e67ba(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e656:	cmpq   $0x0,0x70(%rsp)
  34e65c:	je     34e67a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1d1a>
  34e65e:	mov    0x70(%rsp),%rsi
  34e663:	shl    $0x2,%rsi
  34e667:	mov    $0x4,%edx
  34e66c:	mov    0x190(%rsp),%rdi
  34e674:	call   *0x2e6796(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e67a:	cmpq   $0x0,0x78(%rsp)
  34e680:	je     34e69e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1d3e>
  34e682:	mov    0x78(%rsp),%rsi
  34e687:	shl    $0x3,%rsi
  34e68b:	mov    $0x8,%edx
  34e690:	mov    0x198(%rsp),%rdi
  34e698:	call   *0x2e6772(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e69e:	cmpq   $0x0,0x60(%rsp)
  34e6a4:	je     34e6c2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1d62>
  34e6a6:	mov    0x60(%rsp),%rsi
  34e6ab:	shl    $0x2,%rsi
  34e6af:	mov    $0x4,%edx
  34e6b4:	mov    0x1a0(%rsp),%rdi
  34e6bc:	call   *0x2e674e(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e6c2:	cmpq   $0x0,0x108(%rsp)
  34e6cb:	je     34e6ec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1d8c>
  34e6cd:	mov    0x108(%rsp),%rsi
  34e6d5:	shl    $0x3,%rsi
  34e6d9:	mov    $0x8,%edx
  34e6de:	mov    0x90(%rsp),%rdi
  34e6e6:	call   *0x2e6724(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e6ec:	cmpq   $0x0,0x110(%rsp)
  34e6f5:	je     34e716 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1db6>
  34e6f7:	mov    0x110(%rsp),%rsi
  34e6ff:	shl    $0x3,%rsi
  34e703:	mov    $0x8,%edx
  34e708:	mov    0x98(%rsp),%rdi
  34e710:	call   *0x2e66fa(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e716:	cmpq   $0x0,0x118(%rsp)
  34e71f:	je     34e740 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1de0>
  34e721:	mov    0x118(%rsp),%rsi
  34e729:	shl    $0x2,%rsi
  34e72d:	mov    $0x4,%edx
  34e732:	mov    0x1a8(%rsp),%rdi
  34e73a:	call   *0x2e66d0(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e740:	cmpq   $0x0,0x120(%rsp)
  34e749:	je     34e76a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e0a>
  34e74b:	mov    0x120(%rsp),%rsi
  34e753:	shl    $0x2,%rsi
  34e757:	mov    $0x4,%edx
  34e75c:	mov    0xf8(%rsp),%rdi
  34e764:	call   *0x2e66a6(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e76a:	cmpq   $0x0,0x38(%rsp)
  34e770:	mov    0x18(%rsp),%rbp
  34e775:	je     34cf33 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5d3>
  34e77b:	mov    0x38(%rsp),%rsi
  34e780:	shl    $0x2,%rsi
  34e784:	mov    $0x4,%edx
  34e789:	mov    0xe8(%rsp),%rdi
  34e791:	jmp    34cf2d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5cd>
  34e796:	mov    0x200(%rsp),%rax
  34e79e:	mov    0x18(%rsp),%rcx
  34e7a3:	mov    %rax,0x18(%rcx)
  34e7a7:	movdqa 0x1f0(%rsp),%xmm0
  34e7b0:	movdqu %xmm0,0x8(%rcx)
  34e7b5:	movq   $0xffffffffffffffff,(%rcx)
  34e7bc:	xor    %r12d,%r12d
  34e7bf:	jmp    34db15 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11b5>
  34e7c4:	lea    0x2a54bd(%rip),%rdx        # 5f3c88 <anon.46580aa77014d508262b833402f42857.109.llvm.14282664211216954348+0x2b8>
  34e7cb:	call   *0x2e6aef(%rip)        # 6352c0 <_DYNAMIC+0x6e8>
  34e7d1:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34e7d3:	lea    0x2a547e(%rip),%rdi        # 5f3c58 <anon.46580aa77014d508262b833402f42857.109.llvm.14282664211216954348+0x288>
  34e7da:	call   *0x2e6930(%rip)        # 635110 <_DYNAMIC+0x538>
  34e7e0:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34e7e2:	mov    $0x1,%edi
  34e7e7:	mov    %rbx,%rsi
  34e7ea:	call   *0x2e6630(%rip)        # 634e20 <_DYNAMIC+0x248>
  34e7f0:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34e7f2:	lea    0x2a5447(%rip),%rcx        # 5f3c40 <anon.46580aa77014d508262b833402f42857.109.llvm.14282664211216954348+0x270>
  34e7f9:	mov    $0x80,%esi
  34e7fe:	xor    %edi,%edi
  34e800:	mov    0x20(%rsp),%rdx
  34e805:	call   *0x2e6b15(%rip)        # 635320 <_DYNAMIC+0x748>
  34e80b:	jmp    34e81e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ebe>
  34e80d:	mov    $0x1,%r12b
  34e810:	mov    $0x1,%edi
  34e815:	mov    %rbx,%rsi
  34e818:	call   *0x2e6602(%rip)        # 634e20 <_DYNAMIC+0x248>
  34e81e:	ud2
  34e820:	mov    %rax,%rbp
  34e823:	lea    0x128(%rsp),%rdi
  34e82b:	call   33ebf0 <core::ptr::drop_glue::<snaptokens::models::bpe::MergeAdjacency>>
  34e830:	cmpq   $0x0,0xb8(%rsp)
  34e839:	jne    34e8e0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1f80>
  34e83f:	cmpq   $0x0,0x88(%rsp)
  34e848:	jne    34e90e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1fae>
  34e84e:	cmpq   $0x0,0x80(%rsp)
  34e857:	jne    34e940 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1fe0>
  34e85d:	cmpq   $0x0,0xc8(%rsp)
  34e866:	je     34e887 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1f27>
  34e868:	mov    0xc8(%rsp),%rsi
  34e870:	shl    $0x3,%rsi
  34e874:	mov    $0x4,%edx
  34e879:	mov    0x170(%rsp),%rdi
  34e881:	call   *0x2e6589(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e887:	lea    0x1f0(%rsp),%rdi
  34e88f:	call   33ec50 <core::ptr::drop_glue::<snaptokens::models::bpe::RankedMergeMap>>
  34e894:	mov    0x20(%rsp),%rax
  34e899:	lea    0x0(,%rax,4),%rsi
  34e8a1:	mov    $0x4,%edx
  34e8a6:	mov    0x28(%rsp),%rdi
  34e8ab:	call   *0x2e655f(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e8b1:	lea    0x308(%rsp),%rdi
  34e8b9:	call   33eb70 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabLookup>>
  34e8be:	lea    0x3a0(%rsp),%rdi
  34e8c6:	call   33ea60 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabArena>>
  34e8cb:	lea    0x388(%rsp),%rdi
  34e8d3:	call   33eaa0 <core::ptr::drop_glue::<snaptokens::models::bpe::SharedCache>>
  34e8d8:	xor    %r13d,%r13d
  34e8db:	jmp    34e97d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x201d>
  34e8e0:	mov    0xb8(%rsp),%rsi
  34e8e8:	shl    $0x5,%rsi
  34e8ec:	mov    $0x10,%edx
  34e8f1:	mov    0xe0(%rsp),%rdi
  34e8f9:	call   *0x2e6511(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e8ff:	cmpq   $0x0,0x88(%rsp)
  34e908:	je     34e84e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1eee>
  34e90e:	mov    0x88(%rsp),%rax
  34e916:	lea    0x0(,%rax,4),%rsi
  34e91e:	mov    $0x4,%edx
  34e923:	mov    0xa0(%rsp),%rdi
  34e92b:	call   *0x2e64df(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e931:	cmpq   $0x0,0x80(%rsp)
  34e93a:	je     34e85d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1efd>
  34e940:	mov    0x80(%rsp),%rax
  34e948:	lea    0x0(,%rax,8),%rsi
  34e950:	mov    $0x8,%edx
  34e955:	mov    0xd8(%rsp),%rdi
  34e95d:	call   *0x2e64ad(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e963:	cmpq   $0x0,0xc8(%rsp)
  34e96c:	jne    34e868 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1f08>
  34e972:	jmp    34e887 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1f27>
  34e977:	mov    %rax,%rbp
  34e97a:	mov    $0x1,%r13b
  34e97d:	lea    0x370(%rsp),%rdi
  34e985:	call   33eaa0 <core::ptr::drop_glue::<snaptokens::models::bpe::SharedCache>>
  34e98a:	jmp    34e998 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2038>
  34e98c:	call   *0x2e6476(%rip)        # 634e08 <_DYNAMIC+0x230>
  34e992:	mov    %rax,%rbp
  34e995:	mov    $0x1,%r13b
  34e998:	cmpq   $0x0,0x68(%rsp)
  34e99e:	je     34e9b8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2058>
  34e9a0:	mov    $0x1,%edx
  34e9a5:	mov    0xa8(%rsp),%rdi
  34e9ad:	mov    0x68(%rsp),%rsi
  34e9b2:	call   *0x2e6458(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e9b8:	cmpq   $0x0,0xc0(%rsp)
  34e9c1:	je     34e9e6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2086>
  34e9c3:	mov    0xc0(%rsp),%rax
  34e9cb:	lea    0x0(,%rax,8),%rsi
  34e9d3:	mov    $0x4,%edx
  34e9d8:	mov    0xf0(%rsp),%rdi
  34e9e0:	call   *0x2e642a(%rip)        # 634e10 <_DYNAMIC+0x238>
  34e9e6:	lea    0x290(%rsp),%rdi
  34e9ee:	call   33ec90 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenMatcher>>
  34e9f3:	xor    %r14d,%r14d
  34e9f6:	xor    %ebx,%ebx
  34e9f8:	test   %r13b,%r13b
  34e9fb:	je     34ea90 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2130>
  34ea01:	cmpq   $0x0,0x80(%rsp)
  34ea0a:	je     34ea2b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x20cb>
  34ea0c:	mov    0x80(%rsp),%rsi
  34ea14:	shl    $0x3,%rsi
  34ea18:	mov    $0x8,%edx
  34ea1d:	mov    0xd8(%rsp),%rdi
  34ea25:	call   *0x2e63e5(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ea2b:	cmpq   $0x0,0x88(%rsp)
  34ea34:	je     34ea55 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x20f5>
  34ea36:	mov    0x88(%rsp),%rsi
  34ea3e:	shl    $0x2,%rsi
  34ea42:	mov    $0x4,%edx
  34ea47:	mov    0xa0(%rsp),%rdi
  34ea4f:	call   *0x2e63bb(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ea55:	xor    %r14d,%r14d
  34ea58:	jmp    34ea60 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2100>
  34ea5a:	mov    %rax,%rbp
  34ea5d:	mov    $0x1,%r14b
  34ea60:	mov    0x2f0(%rsp),%rsi
  34ea68:	mov    $0x1,%bl
  34ea6a:	test   %rsi,%rsi
  34ea6d:	je     34ea90 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2130>
  34ea6f:	mov    0x2f8(%rsp),%rdi
  34ea77:	shl    $0x3,%rsi
  34ea7b:	mov    $0x4,%edx
  34ea80:	call   *0x2e638a(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ea86:	jmp    34ea90 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2130>
  34ea88:	mov    %rax,%rbp
  34ea8b:	mov    $0x1,%bl
  34ea8d:	mov    $0x1,%r14b
  34ea90:	mov    0x2d8(%rsp),%rsi
  34ea98:	test   %rsi,%rsi
  34ea9b:	je     34eab3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2153>
  34ea9d:	add    %rsi,%rsi
  34eaa0:	mov    $0x2,%edx
  34eaa5:	mov    0x1d0(%rsp),%rdi
  34eaad:	call   *0x2e635d(%rip)        # 634e10 <_DYNAMIC+0x238>
  34eab3:	test   %r14b,%r14b
  34eab6:	jne    34eaf3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2193>
  34eab8:	test   %bl,%bl
  34eaba:	je     34eae1 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2181>
  34eabc:	mov    0x20(%rsp),%rax
  34eac1:	lea    0x0(,%rax,4),%rsi
  34eac9:	mov    $0x4,%edx
  34eace:	mov    0x28(%rsp),%rdi
  34ead3:	call   *0x2e6337(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ead9:	mov    $0x1,%r14b
  34eadc:	xor    %r15d,%r15d
  34eadf:	jmp    34eae7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2187>
  34eae1:	xor    %r15d,%r15d
  34eae4:	xor    %r14d,%r14d
  34eae7:	xor    %ebx,%ebx
  34eae9:	jmp    34ebb4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2254>
  34eaee:	mov    %rax,%rbp
  34eaf1:	mov    $0x1,%bl
  34eaf3:	cmpq   $0x0,0x68(%rsp)
  34eaf9:	je     34eb13 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x21b3>
  34eafb:	mov    $0x1,%edx
  34eb00:	mov    0xa8(%rsp),%rdi
  34eb08:	mov    0x68(%rsp),%rsi
  34eb0d:	call   *0x2e62fd(%rip)        # 634e10 <_DYNAMIC+0x238>
  34eb13:	test   %bl,%bl
  34eb15:	je     34eb39 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x21d9>
  34eb17:	mov    0x20(%rsp),%rax
  34eb1c:	lea    0x0(,%rax,4),%rsi
  34eb24:	mov    $0x4,%edx
  34eb29:	mov    0x28(%rsp),%rdi
  34eb2e:	call   *0x2e62dc(%rip)        # 634e10 <_DYNAMIC+0x238>
  34eb34:	mov    $0x1,%r14b
  34eb37:	jmp    34eb3c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x21dc>
  34eb39:	xor    %r14d,%r14d
  34eb3c:	xor    %r15d,%r15d
  34eb3f:	jmp    34eba5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2245>
  34eb41:	mov    %rax,%rbp
  34eb44:	jmp    34eb82 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2222>
  34eb46:	mov    %rax,%rbp
  34eb49:	jmp    34eb9f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x223f>
  34eb4b:	mov    %rax,%rbp
  34eb4e:	mov    $0x1,%al
  34eb50:	mov    %eax,0x58(%rsp)
  34eb54:	mov    $0x1,%r15b
  34eb57:	mov    $0x1,%r14b
  34eb5a:	jmp    34ebb2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2252>
  34eb5c:	jmp    34eb71 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2211>
  34eb5e:	mov    %rax,%rbp
  34eb61:	mov    $0x1,%r12b
  34eb64:	mov    $0x1,%r15b
  34eb67:	mov    $0x1,%r14b
  34eb6a:	mov    $0x1,%bl
  34eb6c:	jmp    34ec2c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x22cc>
  34eb71:	mov    %rax,%rbp
  34eb74:	mov    $0x1,%r14b
  34eb77:	mov    $0x1,%r15b
  34eb7a:	cmpq   $0x0,0x20(%rsp)
  34eb80:	je     34eba5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2245>
  34eb82:	mov    0x20(%rsp),%rax
  34eb87:	lea    0x0(,%rax,4),%rsi
  34eb8f:	mov    $0x4,%edx
  34eb94:	mov    0x28(%rsp),%rdi
  34eb99:	call   *0x2e6271(%rip)        # 634e10 <_DYNAMIC+0x238>
  34eb9f:	mov    $0x1,%r14b
  34eba2:	mov    $0x1,%r15b
  34eba5:	lea    0x290(%rsp),%rdi
  34ebad:	call   33ec90 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenMatcher>>
  34ebb2:	mov    $0x1,%bl
  34ebb4:	cmpq   $0x0,0x50(%rsp)
  34ebba:	setne  %al
  34ebbd:	test   %al,0x58(%rsp)
  34ebc1:	je     34ebd8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2278>
  34ebc3:	mov    $0x1,%edx
  34ebc8:	mov    0x48(%rsp),%rdi
  34ebcd:	mov    0x50(%rsp),%rsi
  34ebd2:	call   *0x2e6238(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ebd8:	test   %r14b,%r14b
  34ebdb:	je     34ebef <<snaptokens::models::bpe::Bpe>::from_native_tables+0x228f>
  34ebdd:	lea    0x308(%rsp),%rdi
  34ebe5:	call   33eb70 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabLookup>>
  34ebea:	xor    %r14d,%r14d
  34ebed:	jmp    34ec08 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x22a8>
  34ebef:	mov    $0x1,%r14b
  34ebf2:	xor    %r13d,%r13d
  34ebf5:	jmp    34ecc2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2362>
  34ebfa:	mov    %rax,%rbp
  34ebfd:	mov    $0x1,%r12b
  34ec00:	mov    $0x1,%r15b
  34ec03:	mov    $0x1,%r14b
  34ec06:	mov    $0x1,%bl
  34ec08:	mov    0x248(%rsp),%rsi
  34ec10:	test   %rsi,%rsi
  34ec13:	je     34ec2c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x22cc>
  34ec15:	mov    0x250(%rsp),%rdi
  34ec1d:	shl    $0x3,%rsi
  34ec21:	mov    $0x8,%edx
  34ec26:	call   *0x2e61e4(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ec2c:	mov    0x230(%rsp),%rsi
  34ec34:	test   %rsi,%rsi
  34ec37:	je     34ec60 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2300>
  34ec39:	mov    0x238(%rsp),%rdi
  34ec41:	shl    $0x3,%rsi
  34ec45:	mov    $0x8,%edx
  34ec4a:	call   *0x2e61c0(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ec50:	jmp    34ec60 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2300>
  34ec52:	mov    %rax,%rbp
  34ec55:	mov    $0x1,%r12b
  34ec58:	mov    $0x1,%r15b
  34ec5b:	mov    $0x1,%r14b
  34ec5e:	mov    $0x1,%bl
  34ec60:	lea    0x260(%rsp),%rdi
  34ec68:	call   33ea60 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabArena>>
  34ec6d:	xor    $0x1,%r14b
  34ec71:	mov    $0x1,%r13b
  34ec74:	jmp    34ecc2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2362>
  34ec76:	jmp    34ec78 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2318>
  34ec78:	mov    %rax,%rbp
  34ec7b:	test   %r12,%r12
  34ec7e:	je     34ec97 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2337>
  34ec80:	shl    $0x2,%r12
  34ec84:	mov    $0x4,%edx
  34ec89:	mov    0x58(%rsp),%rdi
  34ec8e:	mov    %r12,%rsi
  34ec91:	call   *0x2e6179(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ec97:	cmpq   $0x0,0x28(%rsp)
  34ec9d:	je     34ecb4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2354>
  34ec9f:	mov    $0x1,%edx
  34eca4:	mov    0x20(%rsp),%rdi
  34eca9:	mov    0x28(%rsp),%rsi
  34ecae:	call   *0x2e615c(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ecb4:	mov    $0x1,%r12b
  34ecb7:	xor    %r14d,%r14d
  34ecba:	mov    $0x1,%r15b
  34ecbd:	mov    $0x1,%r13b
  34ecc0:	mov    $0x1,%bl
  34ecc2:	cmpq   $0x0,0x70(%rsp)
  34ecc8:	jne    34ecea <<snaptokens::models::bpe::Bpe>::from_native_tables+0x238a>
  34ecca:	cmpq   $0x0,0x78(%rsp)
  34ecd0:	jne    34ed0e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x23ae>
  34ecd2:	cmpq   $0x0,0x60(%rsp)
  34ecd8:	jne    34ed32 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x23d2>
  34ecda:	cmpq   $0xffffffffffffffff,0x340(%rsp)
  34ece3:	jne    34ed59 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x23f9>
  34ece5:	jmp    34ed6b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x240b>
  34ecea:	mov    0x70(%rsp),%rsi
  34ecef:	shl    $0x2,%rsi
  34ecf3:	mov    $0x4,%edx
  34ecf8:	mov    0x190(%rsp),%rdi
  34ed00:	call   *0x2e610a(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ed06:	cmpq   $0x0,0x78(%rsp)
  34ed0c:	je     34ecd2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2372>
  34ed0e:	mov    0x78(%rsp),%rsi
  34ed13:	shl    $0x3,%rsi
  34ed17:	mov    $0x8,%edx
  34ed1c:	mov    0x198(%rsp),%rdi
  34ed24:	call   *0x2e60e6(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ed2a:	cmpq   $0x0,0x60(%rsp)
  34ed30:	je     34ecda <<snaptokens::models::bpe::Bpe>::from_native_tables+0x237a>
  34ed32:	mov    0x60(%rsp),%rsi
  34ed37:	shl    $0x2,%rsi
  34ed3b:	mov    $0x4,%edx
  34ed40:	mov    0x1a0(%rsp),%rdi
  34ed48:	call   *0x2e60c2(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ed4e:	cmpq   $0xffffffffffffffff,0x340(%rsp)
  34ed57:	je     34ed6b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x240b>
  34ed59:	test   %r12b,%r12b
  34ed5c:	je     34ed6b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x240b>
  34ed5e:	lea    0x340(%rsp),%rdi
  34ed66:	call   33ebb0 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenTrie>>
  34ed6b:	cmpq   $0x0,0x100(%rsp)
  34ed74:	setne  %al
  34ed77:	test   %al,%r15b
  34ed7a:	je     34ed9b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x243b>
  34ed7c:	mov    0x100(%rsp),%rsi
  34ed84:	shl    $0x3,%rsi
  34ed88:	mov    $0x8,%edx
  34ed8d:	mov    0x180(%rsp),%rdi
  34ed95:	call   *0x2e6075(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ed9b:	test   %r13b,%r13b
  34ed9e:	je     34edf7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2497>
  34eda0:	cmpq   $0x0,0x1b0(%rsp)
  34eda9:	jne    34efa0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2640>
  34edaf:	cmpq   $0x0,0x1b8(%rsp)
  34edb8:	jne    34efce <<snaptokens::models::bpe::Bpe>::from_native_tables+0x266e>
  34edbe:	cmpq   $0x0,0x1c0(%rsp)
  34edc7:	jne    34effc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x269c>
  34edcd:	cmpq   $0x0,0xb8(%rsp)
  34edd6:	je     34edf7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2497>
  34edd8:	mov    0xb8(%rsp),%rsi
  34ede0:	shl    $0x5,%rsi
  34ede4:	mov    $0x10,%edx
  34ede9:	mov    0xe0(%rsp),%rdi
  34edf1:	call   *0x2e6019(%rip)        # 634e10 <_DYNAMIC+0x238>
  34edf7:	cmpq   $0x0,0x108(%rsp)
  34ee00:	jne    34ee8c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x252c>
  34ee06:	cmpq   $0x0,0x110(%rsp)
  34ee0f:	jne    34eeba <<snaptokens::models::bpe::Bpe>::from_native_tables+0x255a>
  34ee15:	cmpq   $0x0,0x118(%rsp)
  34ee1e:	jne    34eee8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2588>
  34ee24:	cmpq   $0x0,0x120(%rsp)
  34ee2d:	jne    34ef16 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x25b6>
  34ee33:	cmpq   $0x0,0x38(%rsp)
  34ee39:	jne    34ef41 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x25e1>
  34ee3f:	cmpq   $0x0,0x50(%rsp)
  34ee45:	sete   %al
  34ee48:	or     %al,%r14b
  34ee4b:	je     34ef6f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x260f>
  34ee51:	cmpq   $0x0,0xc0(%rsp)
  34ee5a:	setne  %al
  34ee5d:	test   %al,%bl
  34ee5f:	je     34ef98 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2638>
  34ee65:	mov    0xc0(%rsp),%rsi
  34ee6d:	shl    $0x3,%rsi
  34ee71:	mov    $0x4,%edx
  34ee76:	mov    0xf0(%rsp),%rdi
  34ee7e:	call   *0x2e5f8c(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ee84:	mov    %rbp,%rdi
  34ee87:	call   5c7750 <_Unwind_Resume@plt>
  34ee8c:	mov    0x108(%rsp),%rsi
  34ee94:	shl    $0x3,%rsi
  34ee98:	mov    $0x8,%edx
  34ee9d:	mov    0x90(%rsp),%rdi
  34eea5:	call   *0x2e5f65(%rip)        # 634e10 <_DYNAMIC+0x238>
  34eeab:	cmpq   $0x0,0x110(%rsp)
  34eeb4:	je     34ee15 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x24b5>
  34eeba:	mov    0x110(%rsp),%rsi
  34eec2:	shl    $0x3,%rsi
  34eec6:	mov    $0x8,%edx
  34eecb:	mov    0x98(%rsp),%rdi
  34eed3:	call   *0x2e5f37(%rip)        # 634e10 <_DYNAMIC+0x238>
  34eed9:	cmpq   $0x0,0x118(%rsp)
  34eee2:	je     34ee24 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x24c4>
  34eee8:	mov    0x118(%rsp),%rsi
  34eef0:	shl    $0x2,%rsi
  34eef4:	mov    $0x4,%edx
  34eef9:	mov    0x1a8(%rsp),%rdi
  34ef01:	call   *0x2e5f09(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ef07:	cmpq   $0x0,0x120(%rsp)
  34ef10:	je     34ee33 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x24d3>
  34ef16:	mov    0x120(%rsp),%rsi
  34ef1e:	shl    $0x2,%rsi
  34ef22:	mov    $0x4,%edx
  34ef27:	mov    0xf8(%rsp),%rdi
  34ef2f:	call   *0x2e5edb(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ef35:	cmpq   $0x0,0x38(%rsp)
  34ef3b:	je     34ee3f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x24df>
  34ef41:	mov    0x38(%rsp),%rsi
  34ef46:	shl    $0x2,%rsi
  34ef4a:	mov    $0x4,%edx
  34ef4f:	mov    0xe8(%rsp),%rdi
  34ef57:	call   *0x2e5eb3(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ef5d:	cmpq   $0x0,0x50(%rsp)
  34ef63:	sete   %al
  34ef66:	or     %al,%r14b
  34ef69:	jne    34ee51 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x24f1>
  34ef6f:	mov    $0x1,%edx
  34ef74:	mov    0x48(%rsp),%rdi
  34ef79:	mov    0x50(%rsp),%rsi
  34ef7e:	call   *0x2e5e8c(%rip)        # 634e10 <_DYNAMIC+0x238>
  34ef84:	cmpq   $0x0,0xc0(%rsp)
  34ef8d:	setne  %al
  34ef90:	test   %al,%bl
  34ef92:	jne    34ee65 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2505>
  34ef98:	mov    %rbp,%rdi
  34ef9b:	call   5c7750 <_Unwind_Resume@plt>
  34efa0:	mov    0x1b0(%rsp),%rsi
  34efa8:	shl    $0x2,%rsi
  34efac:	mov    $0x4,%edx
  34efb1:	mov    0x188(%rsp),%rdi
  34efb9:	call   *0x2e5e51(%rip)        # 634e10 <_DYNAMIC+0x238>
  34efbf:	cmpq   $0x0,0x1b8(%rsp)
  34efc8:	je     34edbe <<snaptokens::models::bpe::Bpe>::from_native_tables+0x245e>
  34efce:	mov    0x1b8(%rsp),%rsi
  34efd6:	shl    $0x3,%rsi
  34efda:	mov    $0x8,%edx
  34efdf:	mov    0x1e8(%rsp),%rdi
  34efe7:	call   *0x2e5e23(%rip)        # 634e10 <_DYNAMIC+0x238>
  34efed:	cmpq   $0x0,0x1c0(%rsp)
  34eff6:	je     34edcd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x246d>
  34effc:	mov    0x1c0(%rsp),%rsi
  34f004:	shl    $0x2,%rsi
  34f008:	mov    $0x4,%edx
  34f00d:	mov    0xb0(%rsp),%rdi
  34f015:	call   *0x2e5df5(%rip)        # 634e10 <_DYNAMIC+0x238>
  34f01b:	cmpq   $0x0,0xb8(%rsp)
  34f024:	jne    34edd8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2478>
  34f02a:	jmp    34edf7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2497>
