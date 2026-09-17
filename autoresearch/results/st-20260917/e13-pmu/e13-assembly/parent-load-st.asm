
/home/namanchetwani/st-campaign-20260917/combined-profile-st-eval:     file format elf64-x86-64


Disassembly of section .text:

000000000043ae30 <snaptokens::st::load_st>:
  43ae30:	push   %rbp
  43ae31:	push   %r15
  43ae33:	push   %r14
  43ae35:	push   %r13
  43ae37:	push   %r12
  43ae39:	push   %rbx
  43ae3a:	sub    $0x1000,%rsp
  43ae41:	movq   $0x0,(%rsp)
  43ae49:	sub    $0xa98,%rsp
  43ae50:	mov    %rcx,%r12
  43ae53:	mov    %rdx,%rbx
  43ae56:	mov    %rsi,%r14
  43ae59:	mov    %rdi,%r15
  43ae5c:	lea    0xe90(%rsp),%rdi
  43ae64:	call   *0x1fb7a6(%rip)        # 636610 <_DYNAMIC+0x1a38>
  43ae6a:	cmpl   $0x2,0xe90(%rsp)
  43ae72:	jne    43ae94 <snaptokens::st::load_st+0x64>
  43ae74:	mov    0xe98(%rsp),%rax
  43ae7c:	movq   $0x0,0x8(%r15)
  43ae84:	mov    %rax,0x10(%r15)
  43ae88:	movq   $0x2,(%r15)
  43ae8f:	jmp    43b15e <snaptokens::st::load_st+0x32e>
  43ae94:	cmpq   $0x20000000,0xee0(%rsp)
  43aea0:	jbe    43af00 <snaptokens::st::load_st+0xd0>
  43aea2:	call   *0x1f9f88(%rip)        # 634e30 <_DYNAMIC+0x258>
  43aea8:	mov    $0x1e,%edi
  43aead:	mov    $0x1,%esi
  43aeb2:	call   *0x1f9f80(%rip)        # 634e38 <_DYNAMIC+0x260>
  43aeb8:	test   %rax,%rax
  43aebb:	je     43bc7e <snaptokens::st::load_st+0xe4e>
  43aec1:	movups -0x2e3a42(%rip),%xmm0        # 157486 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x28e>
  43aec8:	movups %xmm0,0xe(%rax)
  43aecc:	movdqu -0x2e3a5c(%rip),%xmm0        # 157478 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x280>
  43aed4:	movdqu %xmm0,(%rax)
  43aed8:	movq   $0x3,0x8(%r15)
  43aee0:	movq   $0x1e,0x10(%r15)
  43aee8:	mov    %rax,0x18(%r15)
  43aeec:	movq   $0x1e,0x20(%r15)
  43aef4:	movq   $0x2,(%r15)
  43aefb:	jmp    43b15e <snaptokens::st::load_st+0x32e>
  43af00:	lea    0xe90(%rsp),%rdi
  43af08:	mov    %r14,%rsi
  43af0b:	mov    %rbx,%rdx
  43af0e:	call   *0x1fa87c(%rip)        # 635790 <_DYNAMIC+0xbb8>
  43af14:	mov    0xe90(%rsp),%rbx
  43af1c:	mov    0xe98(%rsp),%r14
  43af24:	cmp    $0xffffffffffffffff,%rbx
  43af28:	je     43af87 <snaptokens::st::load_st+0x157>
  43af2a:	mov    0xea0(%rsp),%rax
  43af32:	cmp    $0x53,%rax
  43af36:	ja     43af9f <snaptokens::st::load_st+0x16f>
  43af38:	call   *0x1f9ef2(%rip)        # 634e30 <_DYNAMIC+0x258>
  43af3e:	mov    $0x1f,%r12d
  43af44:	mov    $0x1f,%edi
  43af49:	mov    $0x1,%esi
  43af4e:	call   *0x1f9ee4(%rip)        # 634e38 <_DYNAMIC+0x260>
  43af54:	test   %rax,%rax
  43af57:	je     43bc8e <snaptokens::st::load_st+0xe5e>
  43af5d:	movups -0x2e3b01(%rip),%xmm0        # 157463 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x26b>
  43af64:	movups %xmm0,0xf(%rax)
  43af68:	movups -0x2e3b1b(%rip),%xmm0        # 157454 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x25c>
  43af6f:	movups %xmm0,(%rax)
  43af72:	mov    $0x3,%ecx
  43af77:	mov    $0x1f,%r12d
  43af7d:	mov    $0x1f,%edx
  43af82:	jmp    43b11c <snaptokens::st::load_st+0x2ec>
  43af87:	movq   $0x0,0x8(%r15)
  43af8f:	mov    %r14,0x10(%r15)
  43af93:	movq   $0x2,(%r15)
  43af9a:	jmp    43b15e <snaptokens::st::load_st+0x32e>
  43af9f:	movabs $0x545350414e53,%rcx
  43afa9:	cmp    %rcx,(%r14)
  43afac:	jne    43b083 <snaptokens::st::load_st+0x253>
  43afb2:	mov    0x8(%r14),%r13d
  43afb6:	lea    -0x1(%r13),%ecx
  43afba:	cmp    $0x2,%ecx
  43afbd:	jae    43b0d2 <snaptokens::st::load_st+0x2a2>
  43afc3:	mov    0xc(%r14),%rbp
  43afc7:	add    $0xffffffffffffffac,%rax
  43afcb:	cmp    %rax,%rbp
  43afce:	jne    43b173 <snaptokens::st::load_st+0x343>
  43afd4:	cmpb   $0x0,(%r12)
  43afd9:	je     43b1b1 <snaptokens::st::load_st+0x381>
  43afdf:	movdqu 0x1(%r12),%xmm0
  43afe6:	movdqu 0x11(%r12),%xmm1
  43afed:	movdqa %xmm1,0xea0(%rsp)
  43aff6:	movdqa %xmm0,0xe90(%rsp)
  43afff:	movdqu 0x14(%r14),%xmm2
  43b005:	movdqu 0x24(%r14),%xmm3
  43b00b:	pcmpeqb %xmm1,%xmm3
  43b00f:	pcmpeqb %xmm0,%xmm2
  43b013:	pand   %xmm3,%xmm2
  43b017:	pmovmskb %xmm2,%eax
  43b01b:	cmp    $0xffff,%eax
  43b020:	je     43b1b1 <snaptokens::st::load_st+0x381>
  43b026:	call   *0x1f9e04(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b02c:	mov    $0x26,%r12d
  43b032:	mov    $0x26,%edi
  43b037:	mov    $0x1,%esi
  43b03c:	call   *0x1f9df6(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b042:	test   %rax,%rax
  43b045:	je     43bc8e <snaptokens::st::load_st+0xe5e>
  43b04b:	movups -0x2e3c5c(%rip),%xmm0        # 1573f6 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x1fe>
  43b052:	movups %xmm0,0x10(%rax)
  43b056:	movups -0x2e3c77(%rip),%xmm0        # 1573e6 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x1ee>
  43b05d:	movups %xmm0,(%rax)
  43b060:	movabs $0x4e4f534a20656372,%rcx
  43b06a:	mov    %rcx,0x1e(%rax)
  43b06e:	mov    $0x3,%ecx
  43b073:	mov    $0x26,%r12d
  43b079:	mov    $0x26,%edx
  43b07e:	jmp    43b11c <snaptokens::st::load_st+0x2ec>
  43b083:	call   *0x1f9da7(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b089:	mov    $0x17,%r12d
  43b08f:	mov    $0x17,%edi
  43b094:	mov    $0x1,%esi
  43b099:	call   *0x1f9d99(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b09f:	test   %rax,%rax
  43b0a2:	je     43bc8e <snaptokens::st::load_st+0xe5e>
  43b0a8:	movups -0x2e3c72(%rip),%xmm0        # 15743d <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x245>
  43b0af:	movups %xmm0,(%rax)
  43b0b2:	movabs $0x636967616d20656c,%rcx
  43b0bc:	mov    %rcx,0xf(%rax)
  43b0c0:	mov    $0x3,%ecx
  43b0c5:	mov    $0x17,%r12d
  43b0cb:	mov    $0x17,%edx
  43b0d0:	jmp    43b11c <snaptokens::st::load_st+0x2ec>
  43b0d2:	call   *0x1f9d58(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b0d8:	mov    $0x1a,%r12d
  43b0de:	mov    $0x1a,%edi
  43b0e3:	mov    $0x1,%esi
  43b0e8:	call   *0x1f9d4a(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b0ee:	test   %rax,%rax
  43b0f1:	je     43bc8e <snaptokens::st::load_st+0xe5e>
  43b0f7:	movups -0x2e3cd1(%rip),%xmm0        # 15742d <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x235>
  43b0fe:	movups %xmm0,0xa(%rax)
  43b102:	movups -0x2e3ce6(%rip),%xmm0        # 157423 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x22b>
  43b109:	movups %xmm0,(%rax)
  43b10c:	mov    $0x3,%ecx
  43b111:	mov    $0x1a,%r12d
  43b117:	mov    $0x1a,%edx
  43b11c:	movdqa 0x10(%rsp),%xmm0
  43b122:	movdqa %xmm0,0x80(%rsp)
  43b12b:	movdqu %xmm0,0x28(%r15)
  43b131:	mov    %rcx,0x8(%r15)
  43b135:	mov    %rdx,0x10(%r15)
  43b139:	mov    %rax,0x18(%r15)
  43b13d:	mov    %r12,0x20(%r15)
  43b141:	movq   $0x2,(%r15)
  43b148:	test   %rbx,%rbx
  43b14b:	je     43b15e <snaptokens::st::load_st+0x32e>
  43b14d:	mov    $0x1,%edx
  43b152:	mov    %r14,%rdi
  43b155:	mov    %rbx,%rsi
  43b158:	call   *0x1f9cb2(%rip)        # 634e10 <_DYNAMIC+0x238>
  43b15e:	mov    %r15,%rax
  43b161:	add    $0x1a98,%rsp
  43b168:	pop    %rbx
  43b169:	pop    %r12
  43b16b:	pop    %r13
  43b16d:	pop    %r14
  43b16f:	pop    %r15
  43b171:	pop    %rbp
  43b172:	ret
  43b173:	call   *0x1f9cb7(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b179:	mov    $0x17,%r12d
  43b17f:	mov    $0x17,%edi
  43b184:	mov    $0x1,%esi
  43b189:	call   *0x1f9ca9(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b18f:	test   %rax,%rax
  43b192:	je     43bc8e <snaptokens::st::load_st+0xe5e>
  43b198:	movups -0x2e3d93(%rip),%xmm0        # 15740c <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x214>
  43b19f:	movups %xmm0,(%rax)
  43b1a2:	movabs $0x686374616d73696d,%rcx
  43b1ac:	jmp    43b0bc <snaptokens::st::load_st+0x28c>
  43b1b1:	lea    0x54(%r14),%r12
  43b1b5:	lea    0xe90(%rsp),%rdi
  43b1bd:	mov    %r12,%rsi
  43b1c0:	mov    %rbp,%rdx
  43b1c3:	call   *0x1fa607(%rip)        # 6357d0 <_DYNAMIC+0xbf8>
  43b1c9:	movdqu 0xea0(%rsp),%xmm0
  43b1d2:	movdqu 0x34(%r14),%xmm1
  43b1d8:	movdqu 0x44(%r14),%xmm2
  43b1de:	pcmpeqb 0xe90(%rsp),%xmm1
  43b1e7:	pcmpeqb %xmm0,%xmm2
  43b1eb:	pand   %xmm2,%xmm1
  43b1ef:	pmovmskb %xmm1,%eax
  43b1f3:	cmp    $0xffff,%eax
  43b1f8:	je     43b249 <snaptokens::st::load_st+0x419>
  43b1fa:	call   *0x1f9c30(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b200:	mov    $0x19,%r12d
  43b206:	mov    $0x19,%edi
  43b20b:	mov    $0x1,%esi
  43b210:	call   *0x1f9c22(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b216:	test   %rax,%rax
  43b219:	je     43bc8e <snaptokens::st::load_st+0xe5e>
  43b21f:	movups -0x2e3e50(%rip),%xmm0        # 1573d6 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x1de>
  43b226:	movups %xmm0,0x9(%rax)
  43b22a:	movups -0x2e3e64(%rip),%xmm0        # 1573cd <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x1d5>
  43b231:	movups %xmm0,(%rax)
  43b234:	mov    $0x3,%ecx
  43b239:	mov    $0x19,%r12d
  43b23f:	mov    $0x19,%edx
  43b244:	jmp    43b11c <snaptokens::st::load_st+0x2ec>
  43b249:	cmp    $0x1,%r13d
  43b24d:	jne    43b29a <snaptokens::st::load_st+0x46a>
  43b24f:	lea    0xe90(%rsp),%rdi
  43b257:	mov    %r12,%rsi
  43b25a:	mov    %rbp,%rdx
  43b25d:	call   432420 <bincode::decode_from_slice_with_context::<(), alloc::boxed::Box<snaptokens::st::PayloadV1>, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>>>
  43b262:	cmpb   $0xff,0xe90(%rsp)
  43b26a:	je     43b37d <snaptokens::st::load_st+0x54d>
  43b270:	mov    0xe90(%rsp),%r13
  43b278:	movups 0xe98(%rsp),%xmm0
  43b280:	movaps %xmm0,0xb0(%rsp)
  43b288:	mov    0xea8(%rsp),%rax
  43b290:	mov    %rax,0xc0(%rsp)
  43b298:	jmp    43b307 <snaptokens::st::load_st+0x4d7>
  43b29a:	lea    0xe90(%rsp),%rdi
  43b2a2:	mov    %r12,%rsi
  43b2a5:	mov    %rbp,%rdx
  43b2a8:	call   432490 <bincode::decode_from_slice_with_context::<(), snaptokens::st::PayloadV2, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>>>
  43b2ad:	mov    0xe90(%rsp),%rdx
  43b2b5:	mov    0xe98(%rsp),%r13
  43b2bd:	movups 0xea0(%rsp),%xmm0
  43b2c5:	movaps %xmm0,0xb0(%rsp)
  43b2cd:	mov    0xeb0(%rsp),%rax
  43b2d5:	mov    %rax,0xc0(%rsp)
  43b2dd:	cmp    $0xffffffffffffffff,%rdx
  43b2e1:	je     43b307 <snaptokens::st::load_st+0x4d7>
  43b2e3:	movdqu 0xeb8(%rsp),%xmm0
  43b2ec:	movdqa %xmm0,0x40(%rsp)
  43b2f2:	mov    0xec8(%rsp),%r12
  43b2fa:	mov    0xed0(%rsp),%rax
  43b302:	jmp    43b394 <snaptokens::st::load_st+0x564>
  43b307:	mov    0xc0(%rsp),%rax
  43b30f:	mov    %rax,0x2a8(%rsp)
  43b317:	movdqa 0xb0(%rsp),%xmm0
  43b320:	movdqu %xmm0,0x298(%rsp)
  43b329:	mov    %r13,0x290(%rsp)
  43b331:	lea    0xe90(%rsp),%rdi
  43b339:	lea    0x290(%rsp),%rsi
  43b341:	call   438820 <snaptokens::st::decode_file::{closure#3}>
  43b346:	mov    0xe90(%rsp),%rcx
  43b34e:	mov    0xe98(%rsp),%rdx
  43b356:	mov    0xea0(%rsp),%rax
  43b35e:	mov    0xea8(%rsp),%r12
  43b366:	movups 0xeb0(%rsp),%xmm0
  43b36e:	movaps %xmm0,0x40(%rsp)
  43b373:	movaps %xmm0,0x10(%rsp)
  43b378:	jmp    43b11c <snaptokens::st::load_st+0x2ec>
  43b37d:	mov    0xe98(%rsp),%r13
  43b385:	mov    0xea0(%rsp),%rax
  43b38d:	mov    $0xffffffffffffffff,%rdx
  43b394:	mov    0xc0(%rsp),%rcx
  43b39c:	mov    %rcx,0xeb0(%rsp)
  43b3a4:	movaps 0xb0(%rsp),%xmm0
  43b3ac:	movups %xmm0,0xea0(%rsp)
  43b3b4:	movdqa 0x40(%rsp),%xmm0
  43b3ba:	movdqu %xmm0,0xeb8(%rsp)
  43b3c3:	mov    %rdx,0xe90(%rsp)
  43b3cb:	mov    %r13,0xe98(%rsp)
  43b3d3:	mov    %r12,0xec8(%rsp)
  43b3db:	cmp    %rbp,%rax
  43b3de:	jne    43b77c <snaptokens::st::load_st+0x94c>
  43b3e4:	mov    0xea0(%rsp),%rax
  43b3ec:	mov    0xea8(%rsp),%rcx
  43b3f4:	mov    0xeb0(%rsp),%rsi
  43b3fc:	movaps 0x40(%rsp),%xmm0
  43b401:	movaps %xmm0,0x80(%rsp)
  43b409:	cmp    $0xffffffffffffffff,%rdx
  43b40d:	je     43b7db <snaptokens::st::load_st+0x9ab>
  43b413:	mov    %rdx,0x8(%rsp)
  43b418:	mov    %rdx,0x40(%rsp)
  43b41d:	mov    %r13,0x48(%rsp)
  43b422:	mov    %rax,0x50(%rsp)
  43b427:	mov    %rcx,0x38(%rsp)
  43b42c:	mov    %rcx,0x58(%rsp)
  43b431:	mov    %rsi,0x30(%rsp)
  43b436:	mov    %rsi,0x60(%rsp)
  43b43b:	movdqa 0x80(%rsp),%xmm0
  43b444:	movdqu %xmm0,0x68(%rsp)
  43b44a:	mov    %r12,0x78(%rsp)
  43b44f:	mov    %r13,(%rsp)
  43b453:	mov    %r13,0x290(%rsp)
  43b45b:	mov    %rax,0x298(%rsp)
  43b463:	movq   $0x0,0x2a0(%rsp)
  43b46f:	mov    $0x1,%bpl
  43b472:	lea    0xe90(%rsp),%rdi
  43b47a:	lea    0x290(%rsp),%rsi
  43b482:	call   3b2bf0 <serde_json::de::from_trait::<serde_json::read::SliceRead, snaptokens::st::TokenizerParts>>
  43b487:	lea    0x58(%rsp),%rax
  43b48c:	mov    0xe90(%rsp),%r13
  43b494:	mov    0xe98(%rsp),%rbp
  43b49c:	cmp    $0xffffffffffffffff,%r13
  43b4a0:	je     43bb11 <snaptokens::st::load_st+0xce1>
  43b4a6:	lea    0xea0(%rsp),%rsi
  43b4ae:	lea    0xc0(%rsp),%rdi
  43b4b6:	mov    $0x1d0,%edx
  43b4bb:	call   *0x1f997f(%rip)        # 634e40 <memcpy@GLIBC_2.14>
  43b4c1:	mov    %r13,0xb0(%rsp)
  43b4c9:	mov    %rbp,0xb8(%rsp)
  43b4d1:	lea    0x58(%rsp),%rcx
  43b4d6:	mov    0x10(%rcx),%rax
  43b4da:	mov    %rax,0xea0(%rsp)
  43b4e2:	movdqu (%rcx),%xmm0
  43b4e6:	movdqa %xmm0,0xe90(%rsp)
  43b4ef:	mov    0x70(%rsp),%edx
  43b4f3:	mov    0x74(%rsp),%ecx
  43b4f7:	and    $0x1,%r12d
  43b4fb:	lea    0x290(%rsp),%rdi
  43b503:	lea    0xe90(%rsp),%rsi
  43b50b:	mov    %r12d,%r8d
  43b50e:	call   *0x1fb2bc(%rip)        # 6367d0 <_DYNAMIC+0x1bf8>
  43b514:	mov    0x290(%rsp),%r12
  43b51c:	cmp    $0xffffffffffffffff,%r12
  43b520:	je     43bb6c <snaptokens::st::load_st+0xd3c>
  43b526:	mov    0x298(%rsp),%r13
  43b52e:	movups 0x2a0(%rsp),%xmm0
  43b536:	movaps %xmm0,0x10(%rsp)
  43b53b:	mov    0x2b0(%rsp),%rax
  43b543:	mov    %rax,0x20(%rsp)
  43b548:	movups 0x2b8(%rsp),%xmm0
  43b550:	movups %xmm0,0xeb8(%rsp)
  43b558:	lea    0x2c8(%rsp),%rsi
  43b560:	lea    0xec8(%rsp),%rdi
  43b568:	mov    0x1f98d1(%rip),%rbp        # 634e40 <memcpy@GLIBC_2.14>
  43b56f:	mov    $0x870,%edx
  43b574:	call   *%rbp
  43b576:	mov    0x20(%rsp),%rax
  43b57b:	mov    %rax,0xa0(%rsp)
  43b583:	movdqa 0x10(%rsp),%xmm0
  43b589:	mov    %r12,0xe90(%rsp)
  43b591:	mov    %r13,0xe98(%rsp)
  43b599:	movdqu %xmm0,0xea0(%rsp)
  43b5a2:	mov    %rax,0xeb0(%rsp)
  43b5aa:	lea    0x290(%rsp),%rdi
  43b5b2:	lea    0xb0(%rsp),%rsi
  43b5ba:	mov    $0x1e0,%edx
  43b5bf:	call   *%rbp
  43b5c1:	call   *0x1f9869(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b5c7:	mov    $0x8a8,%edi
  43b5cc:	mov    $0x8,%esi
  43b5d1:	call   *0x1f9861(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b5d7:	test   %rax,%rax
  43b5da:	je     43bc9e <snaptokens::st::load_st+0xe6e>
  43b5e0:	mov    %rax,%r12
  43b5e3:	lea    0xe90(%rsp),%rsi
  43b5eb:	mov    $0x8a8,%edx
  43b5f0:	mov    %rax,%rdi
  43b5f3:	call   *%rbp
  43b5f5:	movups 0x3f8(%rsp),%xmm0
  43b5fd:	movups %xmm0,0x178(%r15)
  43b605:	mov    0x408(%rsp),%rax
  43b60d:	mov    %rax,0x188(%r15)
  43b614:	movups 0x370(%rsp),%xmm0
  43b61c:	movups %xmm0,0xf0(%r15)
  43b624:	mov    0x380(%rsp),%rax
  43b62c:	mov    %rax,0x100(%r15)
  43b633:	movups 0x388(%rsp),%xmm0
  43b63b:	movups 0x398(%rsp),%xmm1
  43b643:	movups 0x3a8(%rsp),%xmm2
  43b64b:	movups %xmm0,0x108(%r15)
  43b653:	movups %xmm1,0x118(%r15)
  43b65b:	movups %xmm2,0x128(%r15)
  43b663:	mov    0x3b8(%rsp),%rax
  43b66b:	mov    %rax,0x138(%r15)
  43b672:	lea    0x2d8(%rsp),%rsi
  43b67a:	lea    0x58(%r15),%rdi
  43b67e:	mov    $0x98,%edx
  43b683:	call   *%rbp
  43b685:	movups 0x450(%rsp),%xmm0
  43b68d:	movups %xmm0,0x1d0(%r15)
  43b695:	movups 0x410(%rsp),%xmm0
  43b69d:	movups 0x420(%rsp),%xmm1
  43b6a5:	movups 0x430(%rsp),%xmm2
  43b6ad:	movups 0x440(%rsp),%xmm3
  43b6b5:	movups %xmm3,0x1c0(%r15)
  43b6bd:	movups %xmm2,0x1b0(%r15)
  43b6c5:	movups %xmm1,0x1a0(%r15)
  43b6cd:	movups %xmm0,0x190(%r15)
  43b6d5:	movups 0x460(%rsp),%xmm0
  43b6dd:	movups %xmm0,0x1e0(%r15)
  43b6e5:	movups 0x3c0(%rsp),%xmm0
  43b6ed:	movups %xmm0,0x140(%r15)
  43b6f5:	mov    0x3f0(%rsp),%rax
  43b6fd:	mov    %rax,0x170(%r15)
  43b704:	movups 0x3d0(%rsp),%xmm0
  43b70c:	movups 0x3e0(%rsp),%xmm1
  43b714:	movups %xmm1,0x160(%r15)
  43b71c:	movups %xmm0,0x150(%r15)
  43b724:	movdqu 0x290(%rsp),%xmm0
  43b72d:	movdqu 0x2a0(%rsp),%xmm1
  43b736:	movdqu 0x2b0(%rsp),%xmm2
  43b73f:	movdqu 0x2c0(%rsp),%xmm3
  43b748:	movdqu %xmm0,0x10(%r15)
  43b74e:	movdqu %xmm1,0x20(%r15)
  43b754:	movdqu %xmm2,0x30(%r15)
  43b75a:	movdqu %xmm3,0x40(%r15)
  43b760:	mov    0x2d0(%rsp),%rax
  43b768:	mov    %rax,0x50(%r15)
  43b76c:	movq   $0x1,(%r15)
  43b773:	mov    %r12,0x8(%r15)
  43b777:	jmp    43bbb5 <snaptokens::st::load_st+0xd85>
  43b77c:	call   *0x1f96ae(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b782:	mov    $0x16,%r12d
  43b788:	mov    $0x16,%edi
  43b78d:	mov    $0x1,%esi
  43b792:	call   *0x1f96a0(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b798:	test   %rax,%rax
  43b79b:	je     43bcc2 <snaptokens::st::load_st+0xe92>
  43b7a1:	movups -0x2e43f1(%rip),%xmm0        # 1573b7 <anon.f04fc52d993cfde3656f6f70ea80afbb.29.llvm.13445530607911787492+0x1bf>
  43b7a8:	movups %xmm0,(%rax)
  43b7ab:	movabs $0x7365747962206461,%rcx
  43b7b5:	mov    %rcx,0xe(%rax)
  43b7b9:	lea    0xe90(%rsp),%rdi
  43b7c1:	mov    %rax,%r13
  43b7c4:	call   434f80 <core::ptr::drop_glue::<snaptokens::st::Payload>>
  43b7c9:	mov    %r13,%rax
  43b7cc:	mov    $0x3,%ecx
  43b7d1:	mov    $0x16,%edx
  43b7d6:	jmp    43b11c <snaptokens::st::load_st+0x2ec>
  43b7db:	movdqu 0x8(%r13),%xmm0
  43b7e1:	movdqu %xmm0,0x290(%rsp)
  43b7ea:	movq   $0x0,0x2a0(%rsp)
  43b7f6:	mov    $0x1,%bpl
  43b7f9:	lea    0xe90(%rsp),%rdi
  43b801:	lea    0x290(%rsp),%rsi
  43b809:	call   3b2bf0 <serde_json::de::from_trait::<serde_json::read::SliceRead, snaptokens::st::TokenizerParts>>
  43b80e:	mov    0xe90(%rsp),%rbp
  43b816:	mov    0xe98(%rsp),%r12
  43b81e:	cmp    $0xffffffffffffffff,%rbp
  43b822:	je     43bbd1 <snaptokens::st::load_st+0xda1>
  43b828:	lea    0xea0(%rsp),%rsi
  43b830:	lea    0xc0(%rsp),%rdi
  43b838:	mov    $0x1d0,%edx
  43b83d:	call   *0x1f95fd(%rip)        # 634e40 <memcpy@GLIBC_2.14>
  43b843:	mov    %rbp,0xb0(%rsp)
  43b84b:	mov    %r12,0xb8(%rsp)
  43b853:	mov    %r13,(%rsp)
  43b857:	lea    0x18(%r13),%rsi
  43b85b:	mov    0x1f95de(%rip),%r13        # 634e40 <memcpy@GLIBC_2.14>
  43b862:	lea    0xe90(%rsp),%r12
  43b86a:	mov    $0x1d8,%edx
  43b86f:	mov    %r12,%rdi
  43b872:	call   *%r13
  43b875:	lea    0x290(%rsp),%rdi
  43b87d:	mov    %r12,%rsi
  43b880:	call   34c960 <<snaptokens::models::bpe::Bpe>::from_native_tables>
  43b885:	mov    0x290(%rsp),%r12
  43b88d:	cmp    $0xffffffffffffffff,%r12
  43b891:	je     43bc07 <snaptokens::st::load_st+0xdd7>
  43b897:	mov    0x298(%rsp),%rbp
  43b89f:	movups 0x2a0(%rsp),%xmm0
  43b8a7:	movaps %xmm0,0x40(%rsp)
  43b8ac:	mov    0x2b0(%rsp),%rax
  43b8b4:	mov    %rax,0x50(%rsp)
  43b8b9:	movups 0x2b8(%rsp),%xmm0
  43b8c1:	movups %xmm0,0xeb8(%rsp)
  43b8c9:	lea    0x2c8(%rsp),%rsi
  43b8d1:	lea    0xec8(%rsp),%rdi
  43b8d9:	mov    $0xbc8,%edx
  43b8de:	call   *%r13
  43b8e1:	mov    0x50(%rsp),%rax
  43b8e6:	mov    %rax,0x20(%rsp)
  43b8eb:	movdqa 0x40(%rsp),%xmm0
  43b8f1:	mov    %r12,0xe90(%rsp)
  43b8f9:	mov    %rbp,0xe98(%rsp)
  43b901:	movdqu %xmm0,0xea0(%rsp)
  43b90a:	mov    %rax,0xeb0(%rsp)
  43b912:	lea    0x290(%rsp),%rdi
  43b91a:	lea    0xb0(%rsp),%rsi
  43b922:	mov    $0x1e0,%edx
  43b927:	call   *%r13
  43b92a:	call   *0x1f9500(%rip)        # 634e30 <_DYNAMIC+0x258>
  43b930:	mov    $0xc00,%edi
  43b935:	mov    $0x8,%esi
  43b93a:	call   *0x1f94f8(%rip)        # 634e38 <_DYNAMIC+0x260>
  43b940:	test   %rax,%rax
  43b943:	je     43bcb0 <snaptokens::st::load_st+0xe80>
  43b949:	mov    %rax,%r12
  43b94c:	lea    0xe90(%rsp),%rsi
  43b954:	mov    $0xc00,%edx
  43b959:	mov    %rax,%rdi
  43b95c:	call   *%r13
  43b95f:	movups 0x3f8(%rsp),%xmm0
  43b967:	movups %xmm0,0x178(%r15)
  43b96f:	mov    0x408(%rsp),%rax
  43b977:	mov    %rax,0x188(%r15)
  43b97e:	movups 0x370(%rsp),%xmm0
  43b986:	movups %xmm0,0xf0(%r15)
  43b98e:	mov    0x380(%rsp),%rax
  43b996:	mov    %rax,0x100(%r15)
  43b99d:	movups 0x388(%rsp),%xmm0
  43b9a5:	movups 0x398(%rsp),%xmm1
  43b9ad:	movups 0x3a8(%rsp),%xmm2
  43b9b5:	movups %xmm0,0x108(%r15)
  43b9bd:	movups %xmm1,0x118(%r15)
  43b9c5:	movups %xmm2,0x128(%r15)
  43b9cd:	mov    0x3b8(%rsp),%rax
  43b9d5:	mov    %rax,0x138(%r15)
  43b9dc:	lea    0x2d8(%rsp),%rsi
  43b9e4:	lea    0x58(%r15),%rdi
  43b9e8:	mov    $0x98,%edx
  43b9ed:	call   *%r13
  43b9f0:	movups 0x450(%rsp),%xmm0
  43b9f8:	movups %xmm0,0x1d0(%r15)
  43ba00:	movups 0x410(%rsp),%xmm0
  43ba08:	movups 0x420(%rsp),%xmm1
  43ba10:	movups 0x430(%rsp),%xmm2
  43ba18:	movups 0x440(%rsp),%xmm3
  43ba20:	movups %xmm3,0x1c0(%r15)
  43ba28:	movups %xmm2,0x1b0(%r15)
  43ba30:	movups %xmm1,0x1a0(%r15)
  43ba38:	movups %xmm0,0x190(%r15)
  43ba40:	movups 0x460(%rsp),%xmm0
  43ba48:	movups %xmm0,0x1e0(%r15)
  43ba50:	movups 0x3c0(%rsp),%xmm0
  43ba58:	movups %xmm0,0x140(%r15)
  43ba60:	mov    0x3f0(%rsp),%rax
  43ba68:	mov    %rax,0x170(%r15)
  43ba6f:	movups 0x3d0(%rsp),%xmm0
  43ba77:	movups 0x3e0(%rsp),%xmm1
  43ba7f:	movups %xmm1,0x160(%r15)
  43ba87:	movups %xmm0,0x150(%r15)
  43ba8f:	movdqu 0x290(%rsp),%xmm0
  43ba98:	movdqu 0x2a0(%rsp),%xmm1
  43baa1:	movdqu 0x2b0(%rsp),%xmm2
  43baaa:	movdqu 0x2c0(%rsp),%xmm3
  43bab3:	movdqu %xmm0,0x10(%r15)
  43bab9:	movdqu %xmm1,0x20(%r15)
  43babf:	movdqu %xmm2,0x30(%r15)
  43bac5:	movdqu %xmm3,0x40(%r15)
  43bacb:	mov    0x2d0(%rsp),%rax
  43bad3:	mov    %rax,0x50(%r15)
  43bad7:	movq   $0x0,(%r15)
  43bade:	mov    %r12,0x8(%r15)
  43bae2:	mov    (%rsp),%r12
  43bae6:	mov    (%r12),%rsi
  43baea:	test   %rsi,%rsi
  43baed:	je     43baff <snaptokens::st::load_st+0xccf>
  43baef:	mov    0x8(%r12),%rdi
  43baf4:	mov    $0x1,%edx
  43baf9:	call   *0x1f9311(%rip)        # 634e10 <_DYNAMIC+0x238>
  43baff:	mov    $0x1f0,%esi
  43bb04:	mov    $0x8,%edx
  43bb09:	mov    %r12,%rdi
  43bb0c:	jmp    43bc73 <snaptokens::st::load_st+0xe43>
  43bb11:	mov    %rax,%r12
  43bb14:	movq   $0x1,0x8(%r15)
  43bb1c:	mov    %rbp,0x10(%r15)
  43bb20:	movq   $0x2,(%r15)
  43bb27:	mov    0x8(%rsp),%rsi
  43bb2c:	test   %rsi,%rsi
  43bb2f:	je     43bb40 <snaptokens::st::load_st+0xd10>
  43bb31:	mov    $0x1,%edx
  43bb36:	mov    (%rsp),%rdi
  43bb3a:	call   *0x1f92d0(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bb40:	mov    %r12,%rdi
  43bb43:	mov    0x38(%rsp),%r12
  43bb48:	call   3cb730 <<alloc::vec::Vec<(alloc::string::String, f64)> as core::ops::drop::Drop>::drop>
  43bb4d:	test   %r12,%r12
  43bb50:	je     43b148 <snaptokens::st::load_st+0x318>
  43bb56:	shl    $0x5,%r12
  43bb5a:	mov    $0x8,%edx
  43bb5f:	mov    0x30(%rsp),%rdi
  43bb64:	mov    %r12,%rsi
  43bb67:	jmp    43bc73 <snaptokens::st::load_st+0xe43>
  43bb6c:	lea    0x298(%rsp),%rax
  43bb74:	mov    0x10(%rax),%rcx
  43bb78:	movdqu (%rax),%xmm0
  43bb7c:	movdqa %xmm0,0x90(%rsp)
  43bb85:	mov    %rcx,0xa0(%rsp)
  43bb8d:	mov    %rcx,0x20(%r15)
  43bb91:	movdqu %xmm0,0x10(%r15)
  43bb97:	movq   $0x9,0x8(%r15)
  43bb9f:	movq   $0x2,(%r15)
  43bba6:	xor    %ebp,%ebp
  43bba8:	lea    0xb0(%rsp),%rdi
  43bbb0:	call   434e70 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  43bbb5:	mov    0x8(%rsp),%rsi
  43bbba:	test   %rsi,%rsi
  43bbbd:	je     43b148 <snaptokens::st::load_st+0x318>
  43bbc3:	mov    $0x1,%edx
  43bbc8:	mov    (%rsp),%rdi
  43bbcc:	jmp    43bc73 <snaptokens::st::load_st+0xe43>
  43bbd1:	movq   $0x1,0x8(%r15)
  43bbd9:	mov    %r12,0x10(%r15)
  43bbdd:	movq   $0x2,(%r15)
  43bbe4:	mov    0x0(%r13),%rsi
  43bbe8:	test   %rsi,%rsi
  43bbeb:	je     43bbfc <snaptokens::st::load_st+0xdcc>
  43bbed:	mov    0x8(%r13),%rdi
  43bbf1:	mov    $0x1,%edx
  43bbf6:	call   *0x1f9214(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bbfc:	lea    0x18(%r13),%rdi
  43bc00:	call   4355b0 <core::ptr::drop_glue::<snaptokens::models::bpe::NativeBpeTables>>
  43bc05:	jmp    43bc66 <snaptokens::st::load_st+0xe36>
  43bc07:	lea    0x298(%rsp),%rax
  43bc0f:	mov    0x10(%rax),%rcx
  43bc13:	movdqu (%rax),%xmm0
  43bc17:	movdqa %xmm0,0x10(%rsp)
  43bc1d:	mov    %rcx,0x20(%rsp)
  43bc22:	mov    %rcx,0x20(%r15)
  43bc26:	movdqu %xmm0,0x10(%r15)
  43bc2c:	movq   $0x9,0x8(%r15)
  43bc34:	movq   $0x2,(%r15)
  43bc3b:	xor    %ebp,%ebp
  43bc3d:	lea    0xb0(%rsp),%rdi
  43bc45:	mov    (%rsp),%r13
  43bc49:	call   434e70 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  43bc4e:	mov    0x0(%r13),%rsi
  43bc52:	test   %rsi,%rsi
  43bc55:	je     43bc66 <snaptokens::st::load_st+0xe36>
  43bc57:	mov    0x8(%r13),%rdi
  43bc5b:	mov    $0x1,%edx
  43bc60:	call   *0x1f91aa(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bc66:	mov    $0x1f0,%esi
  43bc6b:	mov    $0x8,%edx
  43bc70:	mov    %r13,%rdi
  43bc73:	call   *0x1f9197(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bc79:	jmp    43b148 <snaptokens::st::load_st+0x318>
  43bc7e:	mov    $0x1,%edi
  43bc83:	mov    $0x1e,%esi
  43bc88:	call   *0x1f9192(%rip)        # 634e20 <_DYNAMIC+0x248>
  43bc8e:	mov    $0x1,%edi
  43bc93:	mov    %r12,%rsi
  43bc96:	call   *0x1f9184(%rip)        # 634e20 <_DYNAMIC+0x248>
  43bc9c:	jmp    43bcd2 <snaptokens::st::load_st+0xea2>
  43bc9e:	mov    $0x8,%edi
  43bca3:	mov    $0x8a8,%esi
  43bca8:	call   *0x1f92e2(%rip)        # 634f90 <_DYNAMIC+0x3b8>
  43bcae:	jmp    43bcd2 <snaptokens::st::load_st+0xea2>
  43bcb0:	mov    $0x8,%edi
  43bcb5:	mov    $0xc00,%esi
  43bcba:	call   *0x1f92d0(%rip)        # 634f90 <_DYNAMIC+0x3b8>
  43bcc0:	jmp    43bcd2 <snaptokens::st::load_st+0xea2>
  43bcc2:	mov    $0x1,%edi
  43bcc7:	mov    $0x16,%esi
  43bccc:	call   *0x1f914e(%rip)        # 634e20 <_DYNAMIC+0x248>
  43bcd2:	ud2
  43bcd4:	mov    %rax,%r15
  43bcd7:	test   %r12,%r12
  43bcda:	je     43be0c <snaptokens::st::load_st+0xfdc>
  43bce0:	mov    0x38(%rsp),%rsi
  43bce5:	shl    $0x5,%rsi
  43bce9:	mov    $0x8,%edx
  43bcee:	mov    0x30(%rsp),%rdi
  43bcf3:	call   *0x1f9117(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bcf9:	jmp    43be0c <snaptokens::st::load_st+0xfdc>
  43bcfe:	mov    %rax,%r15
  43bd01:	lea    0xb0(%rsp),%rdi
  43bd09:	call   434e70 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  43bd0e:	jmp    43bd68 <snaptokens::st::load_st+0xf38>
  43bd10:	mov    %rax,%r15
  43bd13:	lea    0xb0(%rsp),%rdi
  43bd1b:	call   434e70 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  43bd20:	jmp    43bdd4 <snaptokens::st::load_st+0xfa4>
  43bd25:	mov    %r13,(%rsp)
  43bd29:	mov    %rax,%r15
  43bd2c:	jmp    43bd6a <snaptokens::st::load_st+0xf3a>
  43bd2e:	mov    %rax,%r15
  43bd31:	jmp    43bdd6 <snaptokens::st::load_st+0xfa6>
  43bd36:	mov    %rax,%r15
  43bd39:	lea    0xe90(%rsp),%rdi
  43bd41:	call   434f80 <core::ptr::drop_glue::<snaptokens::st::Payload>>
  43bd46:	jmp    43be0c <snaptokens::st::load_st+0xfdc>
  43bd4b:	mov    %rax,%r15
  43bd4e:	lea    0xe90(%rsp),%rdi
  43bd56:	call   435820 <core::ptr::drop_glue::<snaptokens::models::bpe::Bpe>>
  43bd5b:	lea    0x290(%rsp),%rdi
  43bd63:	call   434e70 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  43bd68:	xor    %ebp,%ebp
  43bd6a:	mov    (%rsp),%rax
  43bd6e:	mov    (%rax),%rsi
  43bd71:	test   %rsi,%rsi
  43bd74:	je     43bd89 <snaptokens::st::load_st+0xf59>
  43bd76:	mov    (%rsp),%rax
  43bd7a:	mov    0x8(%rax),%rdi
  43bd7e:	mov    $0x1,%edx
  43bd83:	call   *0x1f9087(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bd89:	test   %bpl,%bpl
  43bd8c:	je     43bd9b <snaptokens::st::load_st+0xf6b>
  43bd8e:	mov    (%rsp),%rax
  43bd92:	lea    0x18(%rax),%rdi
  43bd96:	call   4355b0 <core::ptr::drop_glue::<snaptokens::models::bpe::NativeBpeTables>>
  43bd9b:	mov    $0x1f0,%esi
  43bda0:	mov    $0x8,%edx
  43bda5:	mov    (%rsp),%rdi
  43bda9:	call   *0x1f9061(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bdaf:	jmp    43be0c <snaptokens::st::load_st+0xfdc>
  43bdb1:	call   *0x1f9051(%rip)        # 634e08 <_DYNAMIC+0x230>
  43bdb7:	mov    %rax,%r15
  43bdba:	lea    0xe90(%rsp),%rdi
  43bdc2:	call   435d90 <core::ptr::drop_glue::<snaptokens::models::unigram::Unigram>>
  43bdc7:	lea    0x290(%rsp),%rdi
  43bdcf:	call   434e70 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  43bdd4:	xor    %ebp,%ebp
  43bdd6:	cmpq   $0x0,0x8(%rsp)
  43bddc:	je     43bdf2 <snaptokens::st::load_st+0xfc2>
  43bdde:	mov    $0x1,%edx
  43bde3:	mov    (%rsp),%rdi
  43bde7:	mov    0x8(%rsp),%rsi
  43bdec:	call   *0x1f901e(%rip)        # 634e10 <_DYNAMIC+0x238>
  43bdf2:	test   %bpl,%bpl
  43bdf5:	je     43be0c <snaptokens::st::load_st+0xfdc>
  43bdf7:	lea    0x58(%rsp),%rdi
  43bdfc:	call   435d10 <core::ptr::drop_glue::<snaptokens::models::unigram::UnigramSnapshot>>
  43be01:	jmp    43be0c <snaptokens::st::load_st+0xfdc>
  43be03:	call   *0x1f8fff(%rip)        # 634e08 <_DYNAMIC+0x230>
  43be09:	mov    %rax,%r15
  43be0c:	test   %rbx,%rbx
  43be0f:	je     43be22 <snaptokens::st::load_st+0xff2>
  43be11:	mov    $0x1,%edx
  43be16:	mov    %r14,%rdi
  43be19:	mov    %rbx,%rsi
  43be1c:	call   *0x1f8fee(%rip)        # 634e10 <_DYNAMIC+0x238>
  43be22:	mov    %r15,%rdi
  43be25:	call   5c7750 <_Unwind_Resume@plt>
