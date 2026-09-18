
/home/namanchetwani/st-campaign-20260917/e16-candidate-st-eval:     file format elf64-x86-64


Disassembly of section .text:

000000000034c0a0 <<snaptokens::models::bpe::Bpe>::from_native_tables>:
  34c0a0:	push   %rbp
  34c0a1:	push   %r15
  34c0a3:	push   %r14
  34c0a5:	push   %r13
  34c0a7:	push   %r12
  34c0a9:	push   %rbx
  34c0aa:	sub    $0x1000,%rsp
  34c0b1:	movq   $0x0,(%rsp)
  34c0b9:	sub    $0x1000,%rsp
  34c0c0:	movq   $0x0,(%rsp)
  34c0c8:	sub    $0xdb8,%rsp
  34c0cf:	mov    %rdi,0x18(%rsp)
  34c0d4:	mov    (%rsi),%rax
  34c0d7:	mov    %rax,0x28(%rsp)
  34c0dc:	mov    0x8(%rsi),%rax
  34c0e0:	mov    %rax,0x20(%rsp)
  34c0e5:	mov    0x10(%rsi),%r15
  34c0e9:	mov    0x18(%rsi),%r12
  34c0ed:	mov    0x20(%rsi),%rcx
  34c0f1:	mov    0x28(%rsi),%r14
  34c0f5:	mov    0x30(%rsi),%rax
  34c0f9:	mov    %rax,0xe0(%rsp)
  34c101:	mov    0x38(%rsi),%rax
  34c105:	mov    %rax,0x98(%rsp)
  34c10d:	mov    0x40(%rsi),%rdx
  34c111:	mov    0x48(%rsi),%rax
  34c115:	mov    %rax,0x50(%rsp)
  34c11a:	mov    0x50(%rsi),%rax
  34c11e:	mov    %rax,0x48(%rsp)
  34c123:	mov    0x58(%rsi),%rax
  34c127:	mov    %rax,0x68(%rsp)
  34c12c:	movzbl 0x1d0(%rsi),%eax
  34c133:	mov    %al,0x17(%rsp)
  34c137:	movzbl 0x1d1(%rsi),%edi
  34c13e:	movdqu 0x198(%rsi),%xmm0
  34c146:	movdqu 0x1a8(%rsi),%xmm1
  34c14e:	movdqu 0x1b8(%rsi),%xmm2
  34c156:	movdqa %xmm2,0x360(%rsp)
  34c15f:	movdqa %xmm1,0x350(%rsp)
  34c168:	movdqa %xmm0,0x340(%rsp)
  34c171:	movzbl 0x1d2(%rsi),%r8d
  34c179:	mov    0x60(%rsi),%r13
  34c17d:	mov    0x68(%rsi),%rax
  34c181:	mov    %rax,0xf8(%rsp)
  34c189:	mov    0x70(%rsi),%rax
  34c18d:	mov    %rax,0xc0(%rsp)
  34c195:	mov    0x78(%rsi),%rax
  34c199:	mov    %rax,0x128(%rsp)
  34c1a1:	mov    0x80(%rsi),%rax
  34c1a8:	mov    %rax,0x100(%rsp)
  34c1b0:	mov    0x88(%rsi),%rax
  34c1b7:	mov    %rax,0x88(%rsp)
  34c1bf:	mov    0x1c8(%rsi),%eax
  34c1c5:	mov    %rax,0xb8(%rsp)
  34c1cd:	mov    0x90(%rsi),%rax
  34c1d4:	mov    %rax,0x120(%rsp)
  34c1dc:	mov    0x98(%rsi),%rax
  34c1e3:	mov    %rax,0x1b0(%rsp)
  34c1eb:	mov    0xa0(%rsi),%rbx
  34c1f2:	mov    0xa8(%rsi),%rax
  34c1f9:	mov    %rax,0x118(%rsp)
  34c201:	mov    0xb0(%rsi),%rax
  34c208:	mov    %rax,0xa8(%rsp)
  34c210:	mov    0xb8(%rsi),%rax
  34c217:	mov    %rax,0x2c0(%rsp)
  34c21f:	mov    0xc0(%rsi),%rax
  34c226:	mov    %rax,0x110(%rsp)
  34c22e:	mov    0xc8(%rsi),%rax
  34c235:	mov    %rax,0xa0(%rsp)
  34c23d:	mov    0xd0(%rsi),%rax
  34c244:	mov    %rax,0xf0(%rsp)
  34c24c:	mov    0xd8(%rsi),%rax
  34c253:	mov    %rax,0xd8(%rsp)
  34c25b:	mov    0xe0(%rsi),%rax
  34c262:	mov    %rax,0x90(%rsp)
  34c26a:	mov    0xe8(%rsi),%r9
  34c271:	mov    0xf0(%rsi),%rax
  34c278:	mov    %rax,0x1c8(%rsp)
  34c280:	mov    0xf8(%rsi),%rax
  34c287:	mov    %rax,0xd0(%rsp)
  34c28f:	mov    0x100(%rsi),%r11
  34c296:	mov    0x108(%rsi),%rax
  34c29d:	mov    %rax,0x1c0(%rsp)
  34c2a5:	mov    0x110(%rsi),%rax
  34c2ac:	mov    %rax,0x1e8(%rsp)
  34c2b4:	mov    0x118(%rsi),%r10
  34c2bb:	mov    0x120(%rsi),%rax
  34c2c2:	mov    %rax,0x1b8(%rsp)
  34c2ca:	mov    0x128(%rsi),%rax
  34c2d1:	mov    %rax,0xc8(%rsp)
  34c2d9:	mov    0x130(%rsi),%rbp
  34c2e0:	mov    0x138(%rsi),%rax
  34c2e7:	mov    %rax,0x108(%rsp)
  34c2ef:	mov    0x140(%rsi),%rax
  34c2f6:	mov    %rax,0x190(%rsp)
  34c2fe:	mov    0x148(%rsi),%rax
  34c305:	mov    %rax,0x80(%rsp)
  34c30d:	mov    0x150(%rsi),%rax
  34c314:	mov    %rax,0x60(%rsp)
  34c319:	mov    0x158(%rsi),%rax
  34c320:	mov    %rax,0x1a8(%rsp)
  34c328:	mov    0x168(%rsi),%rax
  34c32f:	mov    %rax,0x78(%rsp)
  34c334:	mov    0x170(%rsi),%rax
  34c33b:	mov    %rax,0x1a0(%rsp)
  34c343:	mov    0x180(%rsi),%rax
  34c34a:	mov    %rax,0x70(%rsp)
  34c34f:	mov    0x188(%rsi),%rax
  34c356:	mov    %rax,0x198(%rsp)
  34c35e:	cmp    $0x2,%r14
  34c362:	mov    %rcx,0x58(%rsp)
  34c367:	jb     34c372 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2d2>
  34c369:	cmpl   $0x0,(%rcx)
  34c36c:	je     34c688 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5e8>
  34c372:	call   *0x2e7f18(%rip)        # 634290 <_DYNAMIC+0x258>
  34c378:	mov    $0x1e,%ebp
  34c37d:	mov    $0x1e,%edi
  34c382:	mov    $0x1,%esi
  34c387:	call   *0x2e7f0b(%rip)        # 634298 <_DYNAMIC+0x260>
  34c38d:	test   %rax,%rax
  34c390:	je     34cf30 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe90>
  34c396:	mov    %rax,%r15
  34c399:	movups -0x2213cf(%rip),%xmm0        # 12afd1 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x371>
  34c3a0:	movups %xmm0,0xe(%rax)
  34c3a4:	movdqu -0x2213e9(%rip),%xmm0        # 12afc3 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x363>
  34c3ac:	movdqu %xmm0,(%rax)
  34c3b0:	mov    $0x1e,%r14d
  34c3b6:	mov    0x28(%rsp),%rbx
  34c3bb:	test   %r12,%r12
  34c3be:	mov    0x18(%rsp),%rbp
  34c3c3:	mov    0x58(%rsp),%rdi
  34c3c8:	je     34c3dc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x33c>
  34c3ca:	shl    $0x2,%r12
  34c3ce:	mov    $0x4,%edx
  34c3d3:	mov    %r12,%rsi
  34c3d6:	call   *0x2e7e94(%rip)        # 634270 <_DYNAMIC+0x238>
  34c3dc:	test   %rbx,%rbx
  34c3df:	je     34c3f4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x354>
  34c3e1:	mov    $0x1,%edx
  34c3e6:	mov    0x20(%rsp),%rdi
  34c3eb:	mov    %rbx,%rsi
  34c3ee:	call   *0x2e7e7c(%rip)        # 634270 <_DYNAMIC+0x238>
  34c3f4:	mov    %r14,%r12
  34c3f7:	mov    0x70(%rsp),%rsi
  34c3fc:	mov    %r14,0x8(%rbp)
  34c400:	mov    %r15,0x10(%rbp)
  34c404:	mov    %r12,0x18(%rbp)
  34c408:	movq   $0xffffffffffffffff,0x0(%rbp)
  34c410:	mov    $0x1,%r12b
  34c413:	xor    %ebx,%ebx
  34c415:	mov    0x60(%rsp),%r14
  34c41a:	mov    0x78(%rsp),%r15
  34c41f:	test   %rsi,%rsi
  34c422:	je     34c43b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x39b>
  34c424:	shl    $0x2,%rsi
  34c428:	mov    $0x4,%edx
  34c42d:	mov    0x198(%rsp),%rdi
  34c435:	call   *0x2e7e35(%rip)        # 634270 <_DYNAMIC+0x238>
  34c43b:	test   %r15,%r15
  34c43e:	je     34c45a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x3ba>
  34c440:	shl    $0x3,%r15
  34c444:	mov    $0x8,%edx
  34c449:	mov    0x1a0(%rsp),%rdi
  34c451:	mov    %r15,%rsi
  34c454:	call   *0x2e7e16(%rip)        # 634270 <_DYNAMIC+0x238>
  34c45a:	test   %r14,%r14
  34c45d:	je     34c479 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x3d9>
  34c45f:	shl    $0x2,%r14
  34c463:	mov    $0x4,%edx
  34c468:	mov    0x1a8(%rsp),%rdi
  34c470:	mov    %r14,%rsi
  34c473:	call   *0x2e7df7(%rip)        # 634270 <_DYNAMIC+0x238>
  34c479:	mov    0x340(%rsp),%rax
  34c481:	cmp    $0xffffffffffffffff,%rax
  34c485:	setne  %cl
  34c488:	test   %r12b,%cl
  34c48b:	je     34c4cd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x42d>
  34c48d:	test   %rax,%rax
  34c490:	je     34c4ad <<snaptokens::models::bpe::Bpe>::from_native_tables+0x40d>
  34c492:	mov    0x348(%rsp),%rdi
  34c49a:	shl    $0x2,%rax
  34c49e:	lea    (%rax,%rax,2),%rsi
  34c4a2:	mov    $0x4,%edx
  34c4a7:	call   *0x2e7dc3(%rip)        # 634270 <_DYNAMIC+0x238>
  34c4ad:	mov    0x358(%rsp),%rsi
  34c4b5:	test   %rsi,%rsi
  34c4b8:	je     34c4cd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x42d>
  34c4ba:	mov    0x360(%rsp),%rdi
  34c4c2:	mov    $0x1,%edx
  34c4c7:	call   *0x2e7da3(%rip)        # 634270 <_DYNAMIC+0x238>
  34c4cd:	mov    0x108(%rsp),%rsi
  34c4d5:	test   %rsi,%rsi
  34c4d8:	je     34c4f1 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x451>
  34c4da:	shl    $0x3,%rsi
  34c4de:	mov    $0x8,%edx
  34c4e3:	mov    0x190(%rsp),%rdi
  34c4eb:	call   *0x2e7d7f(%rip)        # 634270 <_DYNAMIC+0x238>
  34c4f1:	mov    0x1b8(%rsp),%rsi
  34c4f9:	test   %rsi,%rsi
  34c4fc:	je     34c515 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x475>
  34c4fe:	shl    $0x2,%rsi
  34c502:	mov    $0x4,%edx
  34c507:	mov    0xc8(%rsp),%rdi
  34c50f:	call   *0x2e7d5b(%rip)        # 634270 <_DYNAMIC+0x238>
  34c515:	mov    0x1c0(%rsp),%rsi
  34c51d:	test   %rsi,%rsi
  34c520:	je     34c539 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x499>
  34c522:	shl    $0x3,%rsi
  34c526:	mov    $0x8,%edx
  34c52b:	mov    0x1e8(%rsp),%rdi
  34c533:	call   *0x2e7d37(%rip)        # 634270 <_DYNAMIC+0x238>
  34c539:	mov    0x1c8(%rsp),%rsi
  34c541:	test   %rsi,%rsi
  34c544:	je     34c55d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x4bd>
  34c546:	shl    $0x2,%rsi
  34c54a:	mov    $0x4,%edx
  34c54f:	mov    0xd0(%rsp),%rdi
  34c557:	call   *0x2e7d13(%rip)        # 634270 <_DYNAMIC+0x238>
  34c55d:	mov    0xd8(%rsp),%rsi
  34c565:	test   %rsi,%rsi
  34c568:	je     34c581 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x4e1>
  34c56a:	shl    $0x5,%rsi
  34c56e:	mov    $0x10,%edx
  34c573:	mov    0x90(%rsp),%rdi
  34c57b:	call   *0x2e7cef(%rip)        # 634270 <_DYNAMIC+0x238>
  34c581:	mov    0x110(%rsp),%rsi
  34c589:	test   %rsi,%rsi
  34c58c:	je     34c5a5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x505>
  34c58e:	shl    $0x3,%rsi
  34c592:	mov    $0x8,%edx
  34c597:	mov    0xa0(%rsp),%rdi
  34c59f:	call   *0x2e7ccb(%rip)        # 634270 <_DYNAMIC+0x238>
  34c5a5:	mov    0x118(%rsp),%rsi
  34c5ad:	test   %rsi,%rsi
  34c5b0:	je     34c5c9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x529>
  34c5b2:	shl    $0x3,%rsi
  34c5b6:	mov    $0x8,%edx
  34c5bb:	mov    0xa8(%rsp),%rdi
  34c5c3:	call   *0x2e7ca7(%rip)        # 634270 <_DYNAMIC+0x238>
  34c5c9:	mov    0x120(%rsp),%rsi
  34c5d1:	test   %rsi,%rsi
  34c5d4:	je     34c5ed <<snaptokens::models::bpe::Bpe>::from_native_tables+0x54d>
  34c5d6:	shl    $0x2,%rsi
  34c5da:	mov    $0x4,%edx
  34c5df:	mov    0x1b0(%rsp),%rdi
  34c5e7:	call   *0x2e7c83(%rip)        # 634270 <_DYNAMIC+0x238>
  34c5ed:	mov    0x128(%rsp),%rsi
  34c5f5:	test   %rsi,%rsi
  34c5f8:	je     34c611 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x571>
  34c5fa:	shl    $0x2,%rsi
  34c5fe:	mov    $0x4,%edx
  34c603:	mov    0x100(%rsp),%rdi
  34c60b:	call   *0x2e7c5f(%rip)        # 634270 <_DYNAMIC+0x238>
  34c611:	test   %r13,%r13
  34c614:	je     34c630 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x590>
  34c616:	shl    $0x2,%r13
  34c61a:	mov    $0x4,%edx
  34c61f:	mov    0xf8(%rsp),%rdi
  34c627:	mov    %r13,%rsi
  34c62a:	call   *0x2e7c40(%rip)        # 634270 <_DYNAMIC+0x238>
  34c630:	mov    0x50(%rsp),%rsi
  34c635:	test   %rsi,%rsi
  34c638:	sete   %al
  34c63b:	or     %al,%bl
  34c63d:	jne    34c64f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5af>
  34c63f:	mov    $0x1,%edx
  34c644:	mov    0x48(%rsp),%rdi
  34c649:	call   *0x2e7c21(%rip)        # 634270 <_DYNAMIC+0x238>
  34c64f:	mov    0xe0(%rsp),%rsi
  34c657:	test   %rsi,%rsi
  34c65a:	je     34c673 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5d3>
  34c65c:	shl    $0x3,%rsi
  34c660:	mov    $0x4,%edx
  34c665:	mov    0x98(%rsp),%rdi
  34c66d:	call   *0x2e7bfd(%rip)        # 634270 <_DYNAMIC+0x238>
  34c673:	mov    %rbp,%rax
  34c676:	add    $0x2db8,%rsp
  34c67d:	pop    %rbx
  34c67e:	pop    %r12
  34c680:	pop    %r13
  34c682:	pop    %r14
  34c684:	pop    %r15
  34c686:	pop    %rbp
  34c687:	ret
  34c688:	mov    %r13,0x38(%rsp)
  34c68d:	mov    -0x4(%rcx,%r14,4),%r13d
  34c692:	cmp    %r13,%r15
  34c695:	jne    34c82c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x78c>
  34c69b:	mov    %rbp,0x1e0(%rsp)
  34c6a3:	mov    %rbx,0x40(%rsp)
  34c6a8:	mov    %r11,0x188(%rsp)
  34c6b0:	mov    %r10,0xb0(%rsp)
  34c6b8:	mov    %r9,0x178(%rsp)
  34c6c0:	mov    %r8b,0x16(%rsp)
  34c6c5:	mov    %dil,0x37(%rsp)
  34c6ca:	mov    %rdx,0x8(%rsp)
  34c6cf:	lea    0x198(%rsi),%rax
  34c6d6:	mov    %rax,0x2d0(%rsp)
  34c6de:	mov    0x1cc(%rsi),%eax
  34c6e4:	mov    %eax,0x1d0(%rsp)
  34c6eb:	mov    0x160(%rsi),%rax
  34c6f2:	mov    %rax,0xe8(%rsp)
  34c6fa:	mov    0x178(%rsi),%rax
  34c701:	mov    %rax,0x1d8(%rsp)
  34c709:	mov    0x190(%rsi),%rax
  34c710:	mov    %rax,0x180(%rsp)
  34c718:	lea    0x4(%rcx),%rbx
  34c71c:	lea    0x1(%r14),%rbp
  34c720:	mov    -0x4(%rbx),%esi
  34c723:	mov    (%rbx),%edx
  34c725:	cmp    %esi,%edx
  34c727:	jb     34c881 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x7e1>
  34c72d:	cmp    %edx,%r13d
  34c730:	jb     34c881 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x7e1>
  34c736:	sub    %esi,%edx
  34c738:	add    0x20(%rsp),%rsi
  34c73d:	lea    0x130(%rsp),%rdi
  34c745:	call   *0x2e7ba5(%rip)        # 6342f0 <_DYNAMIC+0x2b8>
  34c74b:	cmpl   $0x1,0x130(%rsp)
  34c753:	je     34c8cf <<snaptokens::models::bpe::Bpe>::from_native_tables+0x82f>
  34c759:	add    $0x4,%rbx
  34c75d:	dec    %rbp
  34c760:	cmp    $0x3,%rbp
  34c764:	jae    34c720 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x680>
  34c766:	mov    0x28(%rsp),%rax
  34c76b:	cmp    $0xffffffffffffffff,%rax
  34c76f:	je     34c91d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x87d>
  34c775:	mov    0x58(%rsp),%rcx
  34c77a:	mov    %rcx,0x280(%rsp)
  34c782:	mov    %r14,0x288(%rsp)
  34c78a:	mov    %rax,0x260(%rsp)
  34c792:	mov    0x20(%rsp),%rax
  34c797:	mov    %rax,0x268(%rsp)
  34c79f:	mov    %r15,0x270(%rsp)
  34c7a7:	mov    %r12,0x278(%rsp)
  34c7af:	cmp    $0x1,%r14
  34c7b3:	adc    $0xffffffffffffffff,%r14
  34c7b7:	mov    %r14,%rax
  34c7ba:	shr    $0x20,%rax
  34c7be:	mov    0x38(%rsp),%r13
  34c7c3:	mov    0x18(%rsp),%rbp
  34c7c8:	je     34c936 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x896>
  34c7ce:	call   *0x2e7abc(%rip)        # 634290 <_DYNAMIC+0x258>
  34c7d4:	mov    $0x1b,%ebx
  34c7d9:	mov    $0x1b,%edi
  34c7de:	mov    $0x1,%esi
  34c7e3:	call   *0x2e7aaf(%rip)        # 634298 <_DYNAMIC+0x260>
  34c7e9:	test   %rax,%rax
  34c7ec:	mov    0x60(%rsp),%r14
  34c7f1:	mov    0x78(%rsp),%r15
  34c7f6:	je     34d290 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11f0>
  34c7fc:	movups -0x2211ac(%rip),%xmm0        # 12b657 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x9f7>
  34c803:	movups %xmm0,0xb(%rax)
  34c807:	movdqu -0x2211c3(%rip),%xmm0        # 12b64c <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x9ec>
  34c80f:	movdqu %xmm0,(%rax)
  34c813:	movq   $0x1b,0x8(%rbp)
  34c81b:	mov    %rax,0x10(%rbp)
  34c81f:	movq   $0x1b,0x18(%rbp)
  34c827:	jmp    34ca12 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34c82c:	call   *0x2e7a5e(%rip)        # 634290 <_DYNAMIC+0x258>
  34c832:	mov    $0x23,%ebp
  34c837:	mov    $0x23,%edi
  34c83c:	mov    $0x1,%esi
  34c841:	call   *0x2e7a51(%rip)        # 634298 <_DYNAMIC+0x260>
  34c847:	test   %rax,%rax
  34c84a:	mov    0x38(%rsp),%r13
  34c84f:	je     34cf30 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe90>
  34c855:	mov    %rax,%r15
  34c858:	movups -0x2218af(%rip),%xmm0        # 12afb0 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x350>
  34c85f:	movups %xmm0,0x10(%rax)
  34c863:	movdqu -0x2218cb(%rip),%xmm0        # 12afa0 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x340>
  34c86b:	movdqu %xmm0,(%rax)
  34c86f:	movl   $0x6874676e,0x1f(%rax)
  34c876:	mov    $0x23,%r14d
  34c87c:	jmp    34c3b6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x316>
  34c881:	call   *0x2e7a09(%rip)        # 634290 <_DYNAMIC+0x258>
  34c887:	mov    $0x1b,%ebp
  34c88c:	mov    $0x1b,%edi
  34c891:	mov    $0x1,%esi
  34c896:	call   *0x2e79fc(%rip)        # 634298 <_DYNAMIC+0x260>
  34c89c:	test   %rax,%rax
  34c89f:	mov    0x38(%rsp),%r13
  34c8a4:	je     34cf30 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe90>
  34c8aa:	mov    %rax,%r15
  34c8ad:	movups -0x221924(%rip),%xmm0        # 12af90 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x330>
  34c8b4:	movups %xmm0,0xb(%rax)
  34c8b8:	movdqu -0x22193b(%rip),%xmm0        # 12af85 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x325>
  34c8c0:	movdqu %xmm0,(%rax)
  34c8c4:	mov    $0x1b,%r14d
  34c8ca:	jmp    34c3b6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x316>
  34c8cf:	call   *0x2e79bb(%rip)        # 634290 <_DYNAMIC+0x258>
  34c8d5:	mov    $0x1c,%ebp
  34c8da:	mov    $0x1c,%edi
  34c8df:	mov    $0x1,%esi
  34c8e4:	call   *0x2e79ae(%rip)        # 634298 <_DYNAMIC+0x260>
  34c8ea:	test   %rax,%rax
  34c8ed:	mov    0x38(%rsp),%r13
  34c8f2:	je     34cf30 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe90>
  34c8f8:	mov    %rax,%r15
  34c8fb:	movups -0x22198d(%rip),%xmm0        # 12af75 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x315>
  34c902:	movups %xmm0,0xc(%rax)
  34c906:	movdqu -0x2219a5(%rip),%xmm0        # 12af69 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x309>
  34c90e:	movdqu %xmm0,(%rax)
  34c912:	mov    $0x1c,%r14d
  34c918:	jmp    34c3b6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x316>
  34c91d:	mov    0x20(%rsp),%r14
  34c922:	mov    0x38(%rsp),%r13
  34c927:	mov    0x70(%rsp),%rsi
  34c92c:	mov    0x18(%rsp),%rbp
  34c931:	jmp    34c3fc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x35c>
  34c936:	mov    0x8(%rsp),%rax
  34c93b:	cmp    %r14,%rax
  34c93e:	mov    0x78(%rsp),%r15
  34c943:	jne    34c9b8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x918>
  34c945:	cmp    %rax,0x68(%rsp)
  34c94a:	jne    34c9b8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x918>
  34c94c:	cmpq   $0x100,0xc0(%rsp)
  34c958:	mov    0x60(%rsp),%r14
  34c95d:	jne    34ca3f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x99f>
  34c963:	cmpq   $0x100,0x88(%rsp)
  34c96f:	jne    34ca3f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x99f>
  34c975:	cmpq   $0x400,0x80(%rsp)
  34c981:	jne    34ca90 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x9f0>
  34c987:	mov    0x2c0(%rsp),%rdx
  34c98f:	test   %rdx,%rdx
  34c992:	je     34cae4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xa44>
  34c998:	lea    -0x1(,%rdx,2),%rax
  34c9a0:	bsr    %rax,%rcx
  34c9a4:	not    %ecx
  34c9a6:	mov    $0xffffffffffffffff,%rbx
  34c9ad:	shr    %cl,%rbx
  34c9b0:	inc    %rbx
  34c9b3:	jmp    34cae6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xa46>
  34c9b8:	call   *0x2e78d2(%rip)        # 634290 <_DYNAMIC+0x258>
  34c9be:	mov    $0x22,%ebx
  34c9c3:	mov    $0x22,%edi
  34c9c8:	mov    $0x1,%esi
  34c9cd:	call   *0x2e78c5(%rip)        # 634298 <_DYNAMIC+0x260>
  34c9d3:	test   %rax,%rax
  34c9d6:	mov    0x60(%rsp),%r14
  34c9db:	je     34d290 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11f0>
  34c9e1:	movups -0x2213ae(%rip),%xmm0        # 12b63a <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x9da>
  34c9e8:	movups %xmm0,0x10(%rax)
  34c9ec:	movdqu -0x2213ca(%rip),%xmm0        # 12b62a <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x9ca>
  34c9f4:	movdqu %xmm0,(%rax)
  34c9f8:	movw   $0x6874,0x20(%rax)
  34c9fe:	movq   $0x22,0x8(%rbp)
  34ca06:	mov    %rax,0x10(%rbp)
  34ca0a:	movq   $0x22,0x18(%rbp)
  34ca12:	movq   $0xffffffffffffffff,0x0(%rbp)
  34ca1a:	mov    $0x1,%r12b
  34ca1d:	xor    %ebx,%ebx
  34ca1f:	lea    0x260(%rsp),%rdi
  34ca27:	call   33e1a0 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabArena>>
  34ca2c:	mov    0x70(%rsp),%rsi
  34ca31:	test   %rsi,%rsi
  34ca34:	jne    34c424 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x384>
  34ca3a:	jmp    34c43b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x39b>
  34ca3f:	call   *0x2e784b(%rip)        # 634290 <_DYNAMIC+0x258>
  34ca45:	mov    $0x1d,%ebx
  34ca4a:	mov    $0x1d,%edi
  34ca4f:	mov    $0x1,%esi
  34ca54:	call   *0x2e783e(%rip)        # 634298 <_DYNAMIC+0x260>
  34ca5a:	test   %rax,%rax
  34ca5d:	je     34d290 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11f0>
  34ca63:	movups -0x221450(%rip),%xmm0        # 12b61a <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x9ba>
  34ca6a:	movups %xmm0,0xd(%rax)
  34ca6e:	movdqu -0x221469(%rip),%xmm0        # 12b60d <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x9ad>
  34ca76:	movdqu %xmm0,(%rax)
  34ca7a:	movq   $0x1d,0x8(%rbp)
  34ca82:	mov    %rax,0x10(%rbp)
  34ca86:	movq   $0x1d,0x18(%rbp)
  34ca8e:	jmp    34ca12 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34ca90:	call   *0x2e77fa(%rip)        # 634290 <_DYNAMIC+0x258>
  34ca96:	mov    $0x1f,%ebx
  34ca9b:	mov    $0x1f,%edi
  34caa0:	mov    $0x1,%esi
  34caa5:	call   *0x2e77ed(%rip)        # 634298 <_DYNAMIC+0x260>
  34caab:	test   %rax,%rax
  34caae:	je     34d290 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11f0>
  34cab4:	movups -0x2214be(%rip),%xmm0        # 12b5fd <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x99d>
  34cabb:	movups %xmm0,0xf(%rax)
  34cabf:	movdqu -0x2214d9(%rip),%xmm0        # 12b5ee <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x98e>
  34cac7:	movdqu %xmm0,(%rax)
  34cacb:	movq   $0x1f,0x8(%rbp)
  34cad3:	mov    %rax,0x10(%rbp)
  34cad7:	movq   $0x1f,0x18(%rbp)
  34cadf:	jmp    34ca12 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34cae4:	xor    %ebx,%ebx
  34cae6:	cmp    %rdx,0x40(%rsp)
  34caeb:	jne    34cbc9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb29>
  34caf1:	mov    0xf0(%rsp),%rax
  34caf9:	cmp    %rax,0x40(%rsp)
  34cafe:	jne    34cbc9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb29>
  34cb04:	cmp    0xb8(%rsp),%rbx
  34cb0c:	jne    34cbc9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb29>
  34cb12:	lea    0x230(%rsp),%rdi
  34cb1a:	mov    $0xffffffffffffffff,%rsi
  34cb21:	mov    %rbx,%rdx
  34cb24:	call   341560 <<u64 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>>
  34cb29:	lea    0x248(%rsp),%rdi
  34cb31:	xor    %esi,%esi
  34cb33:	mov    %rbx,%rdx
  34cb36:	call   341560 <<u64 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>>
  34cb3b:	cmpq   $0x0,0x40(%rsp)
  34cb41:	je     34cc76 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xbd6>
  34cb47:	mov    0x238(%rsp),%rbp
  34cb4f:	mov    0x240(%rsp),%rbx
  34cb57:	mov    0x250(%rsp),%rax
  34cb5f:	mov    0x258(%rsp),%rsi
  34cb67:	xor    %ecx,%ecx
  34cb69:	mov    0x1b0(%rsp),%r9
  34cb71:	mov    0xa8(%rsp),%r10
  34cb79:	mov    0xa0(%rsp),%r11
  34cb81:	mov    (%r9,%rcx,4),%edi
  34cb85:	cmp    %rdi,%rbx
  34cb88:	jbe    34cc1d <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb7d>
  34cb8e:	mov    (%r10,%rcx,8),%rdx
  34cb92:	cmp    $0xffffffffffffffff,%rdx
  34cb96:	je     34cc1d <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb7d>
  34cb9c:	cmpq   $0xffffffffffffffff,0x0(%rbp,%rdi,8)
  34cba2:	jne    34cc1d <<snaptokens::models::bpe::Bpe>::from_native_tables+0xb7d>
  34cba4:	mov    (%r11,%rcx,8),%r8
  34cba8:	mov    %rdx,0x0(%rbp,%rdi,8)
  34cbad:	cmp    %rdi,%rsi
  34cbb0:	jbe    34d2a3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1203>
  34cbb6:	inc    %rcx
  34cbb9:	mov    %r8,(%rax,%rdi,8)
  34cbbd:	cmp    %rcx,0x40(%rsp)
  34cbc2:	jne    34cb81 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xae1>
  34cbc4:	jmp    34cc86 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xbe6>
  34cbc9:	call   *0x2e76c1(%rip)        # 634290 <_DYNAMIC+0x258>
  34cbcf:	mov    $0x1e,%ebx
  34cbd4:	mov    $0x1e,%edi
  34cbd9:	mov    $0x1,%esi
  34cbde:	call   *0x2e76b4(%rip)        # 634298 <_DYNAMIC+0x260>
  34cbe4:	test   %rax,%rax
  34cbe7:	je     34d290 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x11f0>
  34cbed:	movups -0x2216e0(%rip),%xmm0        # 12b514 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8b4>
  34cbf4:	movups %xmm0,0xe(%rax)
  34cbf8:	movdqu -0x2216fa(%rip),%xmm0        # 12b506 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8a6>
  34cc00:	movdqu %xmm0,(%rax)
  34cc04:	movq   $0x1e,0x8(%rbp)
  34cc0c:	mov    %rax,0x10(%rbp)
  34cc10:	movq   $0x1e,0x18(%rbp)
  34cc18:	jmp    34ca12 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x972>
  34cc1d:	call   *0x2e766d(%rip)        # 634290 <_DYNAMIC+0x258>
  34cc23:	mov    $0x1d,%ebx
  34cc28:	mov    $0x1d,%edi
  34cc2d:	mov    $0x1,%esi
  34cc32:	call   *0x2e7660(%rip)        # 634298 <_DYNAMIC+0x260>
  34cc38:	test   %rax,%rax
  34cc3b:	je     34d2b5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1215>
  34cc41:	movups -0x22166a(%rip),%xmm0        # 12b5de <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x97e>
  34cc48:	movups %xmm0,0xd(%rax)
  34cc4c:	movdqu -0x221683(%rip),%xmm0        # 12b5d1 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x971>
  34cc54:	movdqu %xmm0,(%rax)
  34cc58:	mov    0x18(%rsp),%rcx
  34cc5d:	movq   $0x1d,0x8(%rcx)
  34cc65:	mov    %rax,0x10(%rcx)
  34cc69:	movq   $0x1d,0x18(%rcx)
  34cc71:	jmp    34cec8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe28>
  34cc76:	mov    0x238(%rsp),%rbp
  34cc7e:	mov    0x240(%rsp),%rbx
  34cc86:	cmpq   $0x0,0xb8(%rsp)
  34cc8f:	je     34cca4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc04>
  34cc91:	mov    %rbp,%rdi
  34cc94:	mov    %rbx,%rsi
  34cc97:	call   3546a0 <<u64 as core::slice::cmp::SliceContains>::slice_contains>
  34cc9c:	test   %al,%al
  34cc9e:	je     34cdb7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd17>
  34cca4:	test   %rbx,%rbx
  34cca7:	je     34ccca <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc2a>
  34cca9:	mov    %rbp,%rdx
  34ccac:	lea    0x0(,%rbx,8),%rcx
  34ccb4:	xor    %eax,%eax
  34ccb6:	cmpq   $0xffffffffffffffff,(%rdx,%rax,8)
  34ccbb:	je     34ce10 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd70>
  34ccc1:	inc    %rax
  34ccc4:	add    $0xfffffffffffffff8,%rcx
  34ccc8:	jne    34ccb6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc16>
  34ccca:	mov    0x8(%rsp),%rax
  34cccf:	inc    %rax
  34ccd2:	cmp    %rax,0x188(%rsp)
  34ccda:	jne    34cd5a <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcba>
  34ccdc:	mov    0xd0(%rsp),%rax
  34cce4:	cmpl   $0x0,(%rax)
  34cce7:	jne    34cd5a <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcba>
  34cce9:	mov    0xd0(%rsp),%rax
  34ccf1:	mov    0xb0(%rsp),%rcx
  34ccf9:	mov    0x188(%rsp),%rdx
  34cd01:	cmp    %ecx,-0x4(%rax,%rdx,4)
  34cd05:	jne    34cd5a <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcba>
  34cd07:	mov    0x1e0(%rsp),%rax
  34cd0f:	cmp    %rax,0xb0(%rsp)
  34cd17:	jne    34cd5a <<snaptokens::models::bpe::Bpe>::from_native_tables+0xcba>
  34cd19:	mov    0xd0(%rsp),%rax
  34cd21:	mov    %rax,0x130(%rsp)
  34cd29:	mov    0x188(%rsp),%rax
  34cd31:	mov    %rax,0x138(%rsp)
  34cd39:	movq   $0x2,0x140(%rsp)
  34cd45:	lea    0x130(%rsp),%rdi
  34cd4d:	call   341700 <<core::slice::iter::Windows<u32> as core::iter::traits::iterator::Iterator>::try_fold::<(), core::iter::traits::iterator::Iterator::any::check<&[u32], <snaptokens::models::bpe::Bpe>::from_native_tables::{closure#1}>::{closure#0}, core::ops::control_flow::ControlFlow<()>>>
  34cd52:	test   %al,%al
  34cd54:	je     34cf48 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xea8>
  34cd5a:	call   *0x2e7530(%rip)        # 634290 <_DYNAMIC+0x258>
  34cd60:	mov    $0x21,%ebx
  34cd65:	mov    $0x21,%edi
  34cd6a:	mov    $0x1,%esi
  34cd6f:	call   *0x2e7523(%rip)        # 634298 <_DYNAMIC+0x260>
  34cd75:	test   %rax,%rax
  34cd78:	je     34d2b5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1215>
  34cd7e:	movups -0x2217c5(%rip),%xmm0        # 12b5c0 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x960>
  34cd85:	movups %xmm0,0x10(%rax)
  34cd89:	movdqu -0x2217e1(%rip),%xmm0        # 12b5b0 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x950>
  34cd91:	movdqu %xmm0,(%rax)
  34cd95:	movb   $0x65,0x20(%rax)
  34cd99:	mov    0x18(%rsp),%rcx
  34cd9e:	movq   $0x21,0x8(%rcx)
  34cda6:	mov    %rax,0x10(%rcx)
  34cdaa:	movq   $0x21,0x18(%rcx)
  34cdb2:	jmp    34cec8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe28>
  34cdb7:	call   *0x2e74d3(%rip)        # 634290 <_DYNAMIC+0x258>
  34cdbd:	mov    $0x1e,%ebx
  34cdc2:	mov    $0x1e,%edi
  34cdc7:	mov    $0x1,%esi
  34cdcc:	call   *0x2e74c6(%rip)        # 634298 <_DYNAMIC+0x260>
  34cdd2:	test   %rax,%rax
  34cdd5:	je     34d2b5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1215>
  34cddb:	movups -0x2218ce(%rip),%xmm0        # 12b514 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8b4>
  34cde2:	movups %xmm0,0xe(%rax)
  34cde6:	movdqu -0x2218e8(%rip),%xmm0        # 12b506 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8a6>
  34cdee:	movdqu %xmm0,(%rax)
  34cdf2:	mov    0x18(%rsp),%rcx
  34cdf7:	movq   $0x1e,0x8(%rcx)
  34cdff:	mov    %rax,0x10(%rcx)
  34ce03:	movq   $0x1e,0x18(%rcx)
  34ce0b:	jmp    34cec8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe28>
  34ce10:	lea    -0x1(%rbx),%rcx
  34ce14:	mov    %rbx,%rdx
  34ce17:	sub    %rax,%rdx
  34ce1a:	mov    $0x1,%r8d
  34ce20:	movabs $0x517cc1b727220a95,%rsi
  34ce2a:	mov    %r8,%rdi
  34ce2d:	mov    %rbp,%r10
  34ce30:	cmp    %rbx,%rdi
  34ce33:	jae    34ccca <<snaptokens::models::bpe::Bpe>::from_native_tables+0xc2a>
  34ce39:	lea    (%rax,%rdi,1),%r9
  34ce3d:	and    %rcx,%r9
  34ce40:	mov    (%r10,%r9,8),%r9
  34ce44:	cmp    $0xffffffffffffffff,%r9
  34ce48:	je     34ce67 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xdc7>
  34ce4a:	imul   %rsi,%r9
  34ce4e:	and    %rcx,%r9
  34ce51:	add    %rdx,%r9
  34ce54:	and    %rcx,%r9
  34ce57:	cmp    %r8,%r9
  34ce5a:	jb     34ce6f <<snaptokens::models::bpe::Bpe>::from_native_tables+0xdcf>
  34ce5c:	cmp    %rdi,%r9
  34ce5f:	lea    0x1(%rdi),%rdi
  34ce63:	jbe    34ce30 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd90>
  34ce65:	jmp    34ce6f <<snaptokens::models::bpe::Bpe>::from_native_tables+0xdcf>
  34ce67:	inc    %rdi
  34ce6a:	mov    %rdi,%r8
  34ce6d:	jmp    34ce2d <<snaptokens::models::bpe::Bpe>::from_native_tables+0xd8d>
  34ce6f:	call   *0x2e741b(%rip)        # 634290 <_DYNAMIC+0x258>
  34ce75:	mov    $0x24,%ebx
  34ce7a:	mov    $0x24,%edi
  34ce7f:	mov    $0x1,%esi
  34ce84:	call   *0x2e740e(%rip)        # 634298 <_DYNAMIC+0x260>
  34ce8a:	test   %rax,%rax
  34ce8d:	je     34d2b5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1215>
  34ce93:	movups -0x221966(%rip),%xmm0        # 12b534 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8d4>
  34ce9a:	movups %xmm0,0x10(%rax)
  34ce9e:	movups -0x221981(%rip),%xmm0        # 12b524 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8c4>
  34cea5:	movups %xmm0,(%rax)
  34cea8:	movl   $0x6e696168,0x20(%rax)
  34ceaf:	mov    0x18(%rsp),%rcx
  34ceb4:	movq   $0x24,0x8(%rcx)
  34cebc:	mov    %rax,0x10(%rcx)
  34cec0:	movq   $0x24,0x18(%rcx)
  34cec8:	movq   $0xffffffffffffffff,(%rcx)
  34cecf:	mov    $0x1,%r12b
  34ced2:	xor    %ebx,%ebx
  34ced4:	mov    0x248(%rsp),%rsi
  34cedc:	test   %rsi,%rsi
  34cedf:	je     34cef8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe58>
  34cee1:	mov    0x250(%rsp),%rdi
  34cee9:	shl    $0x3,%rsi
  34ceed:	mov    $0x8,%edx
  34cef2:	call   *0x2e7378(%rip)        # 634270 <_DYNAMIC+0x238>
  34cef8:	mov    0x230(%rsp),%rsi
  34cf00:	test   %rsi,%rsi
  34cf03:	je     34cf17 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe77>
  34cf05:	shl    $0x3,%rsi
  34cf09:	mov    $0x8,%edx
  34cf0e:	mov    %rbp,%rdi
  34cf11:	call   *0x2e7359(%rip)        # 634270 <_DYNAMIC+0x238>
  34cf17:	mov    0x38(%rsp),%r13
  34cf1c:	mov    0x60(%rsp),%r14
  34cf21:	mov    0x78(%rsp),%r15
  34cf26:	mov    0x18(%rsp),%rbp
  34cf2b:	jmp    34ca1f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x97f>
  34cf30:	mov    %r13,0x38(%rsp)
  34cf35:	mov    $0x1,%edi
  34cf3a:	mov    %rbp,%rsi
  34cf3d:	call   *0x2e733d(%rip)        # 634280 <_DYNAMIC+0x248>
  34cf43:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34cf48:	sub    $0x8,%rsp
  34cf4c:	lea    0x138(%rsp),%rdi
  34cf54:	lea    0x268(%rsp),%rsi
  34cf5c:	mov    0x1d8(%rsp),%edx
  34cf63:	mov    0x1b0(%rsp),%rcx
  34cf6b:	mov    0xf0(%rsp),%r8
  34cf73:	mov    0x1a8(%rsp),%r9
  34cf7b:	push   0x188(%rsp)
  34cf82:	push   0x1a8(%rsp)
  34cf89:	push   0x1f0(%rsp)
  34cf90:	call   341770 <<snaptokens::models::bpe::VocabLookup>::from_cached_slots>
  34cf95:	add    $0x20,%rsp
  34cf99:	mov    0x130(%rsp),%rax
  34cfa1:	movups 0x138(%rsp),%xmm0
  34cfa9:	movaps %xmm0,0x1f0(%rsp)
  34cfb1:	mov    0x148(%rsp),%rcx
  34cfb9:	mov    %rcx,0x200(%rsp)
  34cfc1:	cmp    $0xffffffffffffffff,%rax
  34cfc5:	je     34d020 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xf80>
  34cfc7:	mov    0x160(%rsp),%rcx
  34cfcf:	mov    %rcx,0x338(%rsp)
  34cfd7:	movups 0x150(%rsp),%xmm0
  34cfdf:	movups %xmm0,0x328(%rsp)
  34cfe7:	movdqa 0x1f0(%rsp),%xmm0
  34cff0:	movdqu %xmm0,0x310(%rsp)
  34cff9:	mov    0x200(%rsp),%rcx
  34d001:	mov    %rcx,0x320(%rsp)
  34d009:	mov    %rax,0x308(%rsp)
  34d011:	cmpq   $0x4,0x8(%rsp)
  34d017:	jae    34d044 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xfa4>
  34d019:	xor    %eax,%eax
  34d01b:	jmp    34d0f5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1055>
  34d020:	mov    0x200(%rsp),%rax
  34d028:	mov    0x18(%rsp),%rcx
  34d02d:	mov    %rax,0x18(%rcx)
  34d031:	movdqa 0x1f0(%rsp),%xmm0
  34d03a:	movdqu %xmm0,0x8(%rcx)
  34d03f:	jmp    34cec8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe28>
  34d044:	movabs $0x1000000000000000,%rcx
  34d04e:	cmpq   $0x20,0x8(%rsp)
  34d054:	jae    34d05a <<snaptokens::models::bpe::Bpe>::from_native_tables+0xfba>
  34d056:	xor    %eax,%eax
  34d058:	jmp    34d0b3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1013>
  34d05a:	lea    -0x20(%rcx),%rax
  34d05e:	and    0x8(%rsp),%rax
  34d063:	xor    %edx,%edx
  34d065:	pxor   %xmm0,%xmm0
  34d069:	movdqa -0x299331(%rip),%xmm1        # b3d40 <anon.a797828b882385d973cc882f19fee84c.17.llvm.17644316995513683882+0x150>
  34d071:	mov    0x48(%rsp),%rsi
  34d076:	movdqu (%rsi,%rdx,1),%xmm2
  34d07b:	movdqu 0x10(%rsi,%rdx,1),%xmm3
  34d081:	pcmpeqb %xmm0,%xmm2
  34d085:	pandn  %xmm1,%xmm2
  34d089:	pcmpeqb %xmm0,%xmm3
  34d08d:	pandn  %xmm1,%xmm3
  34d091:	movdqu %xmm2,(%rsi,%rdx,1)
  34d096:	movdqu %xmm3,0x10(%rsi,%rdx,1)
  34d09c:	add    $0x20,%rdx
  34d0a0:	cmp    %rdx,%rax
  34d0a3:	jne    34d076 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xfd6>
  34d0a5:	cmp    %rax,0x8(%rsp)
  34d0aa:	je     34d10c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x106c>
  34d0ac:	testb  $0x1c,0x8(%rsp)
  34d0b1:	je     34d0f5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1055>
  34d0b3:	mov    %rax,%rdx
  34d0b6:	add    $0xfffffffffffffffc,%rcx
  34d0ba:	mov    %rcx,%rax
  34d0bd:	and    0x8(%rsp),%rax
  34d0c2:	pxor   %xmm0,%xmm0
  34d0c6:	movdqa -0x298d2e(%rip),%xmm1        # b43a0 <anon.2e19753996ac26bdd79597722b35bd89.102.llvm.7838387873399110284+0x650>
  34d0ce:	mov    0x48(%rsp),%rcx
  34d0d3:	movd   (%rcx,%rdx,1),%xmm2
  34d0d8:	pcmpeqb %xmm0,%xmm2
  34d0dc:	pandn  %xmm1,%xmm2
  34d0e0:	movd   %xmm2,(%rcx,%rdx,1)
  34d0e5:	add    $0x4,%rdx
  34d0e9:	cmp    %rdx,%rax
  34d0ec:	jne    34d0d3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1033>
  34d0ee:	cmp    %rax,0x8(%rsp)
  34d0f3:	je     34d10c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x106c>
  34d0f5:	mov    0x48(%rsp),%rcx
  34d0fa:	cmpb   $0x0,(%rcx,%rax,1)
  34d0fe:	setne  (%rcx,%rax,1)
  34d102:	inc    %rax
  34d105:	cmp    %rax,0x8(%rsp)
  34d10a:	jne    34d0fa <<snaptokens::models::bpe::Bpe>::from_native_tables+0x105a>
  34d10c:	mov    0x98(%rsp),%rax
  34d114:	mov    (%rax),%edx
  34d116:	mov    0x4(%rax),%eax
  34d119:	cmp    %edx,%eax
  34d11b:	cmova  %eax,%edx
  34d11e:	cmpq   $0x1,0x8(%rsp)
  34d124:	je     34d6ee <<snaptokens::models::bpe::Bpe>::from_native_tables+0x164e>
  34d12a:	movabs $0x2000000000000000,%rsi
  34d134:	lea    -0x1(%rsi),%rax
  34d138:	mov    0x8(%rsp),%rcx
  34d13d:	add    %rsi,%rcx
  34d140:	dec    %rcx
  34d143:	and    %rcx,%rax
  34d146:	cmp    $0x8,%rax
  34d14a:	jae    34d153 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10b3>
  34d14c:	xor    %ecx,%ecx
  34d14e:	jmp    34d6ca <<snaptokens::models::bpe::Bpe>::from_native_tables+0x162a>
  34d153:	add    $0xfffffffffffffff8,%rsi
  34d157:	and    %rsi,%rcx
  34d15a:	movd   %edx,%xmm0
  34d15e:	pshufd $0x0,%xmm0,%xmm1
  34d163:	xor    %edx,%edx
  34d165:	movaps -0x29719c(%rip),%xmm0        # b5fd0 <anon.2e19753996ac26bdd79597722b35bd89.32.llvm.7838387873399110284+0x12c0>
  34d16c:	movdqa %xmm1,%xmm2
  34d170:	mov    0x98(%rsp),%rsi
  34d178:	movups 0x8(%rsi,%rdx,8),%xmm3
  34d17d:	movups 0x18(%rsi,%rdx,8),%xmm4
  34d182:	movups 0x28(%rsi,%rdx,8),%xmm5
  34d187:	movups 0x38(%rsi,%rdx,8),%xmm6
  34d18c:	movaps %xmm3,%xmm7
  34d18f:	shufps $0x88,%xmm4,%xmm7
  34d193:	shufps $0xdd,%xmm4,%xmm3
  34d197:	movaps %xmm5,%xmm8
  34d19b:	shufps $0x88,%xmm6,%xmm8
  34d1a0:	shufps $0xdd,%xmm6,%xmm5
  34d1a4:	movaps %xmm7,%xmm6
  34d1a7:	xorps  %xmm0,%xmm6
  34d1aa:	movaps %xmm3,%xmm4
  34d1ad:	xorps  %xmm0,%xmm4
  34d1b0:	pcmpgtd %xmm6,%xmm4
  34d1b4:	andps  %xmm4,%xmm3
  34d1b7:	andnps %xmm7,%xmm4
  34d1ba:	orps   %xmm3,%xmm4
  34d1bd:	movaps %xmm8,%xmm6
  34d1c1:	xorps  %xmm0,%xmm6
  34d1c4:	movaps %xmm5,%xmm3
  34d1c7:	xorps  %xmm0,%xmm3
  34d1ca:	pcmpgtd %xmm6,%xmm3
  34d1ce:	andps  %xmm3,%xmm5
  34d1d1:	andnps %xmm8,%xmm3
  34d1d5:	orps   %xmm5,%xmm3
  34d1d8:	movaps %xmm4,%xmm5
  34d1db:	xorps  %xmm0,%xmm5
  34d1de:	movdqa %xmm1,%xmm6
  34d1e2:	pxor   %xmm0,%xmm1
  34d1e6:	pcmpgtd %xmm5,%xmm1
  34d1ea:	pand   %xmm1,%xmm6
  34d1ee:	pandn  %xmm4,%xmm1
  34d1f2:	por    %xmm6,%xmm1
  34d1f6:	movaps %xmm3,%xmm4
  34d1f9:	xorps  %xmm0,%xmm4
  34d1fc:	movdqa %xmm2,%xmm5
  34d200:	pxor   %xmm0,%xmm2
  34d204:	pcmpgtd %xmm4,%xmm2
  34d208:	pand   %xmm2,%xmm5
  34d20c:	pandn  %xmm3,%xmm2
  34d210:	por    %xmm5,%xmm2
  34d214:	add    $0x8,%rdx
  34d218:	cmp    %rdx,%rcx
  34d21b:	jne    34d178 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x10d8>
  34d221:	movdqa %xmm2,%xmm3
  34d225:	pxor   %xmm0,%xmm3
  34d229:	movdqa %xmm1,%xmm4
  34d22d:	pxor   %xmm0,%xmm4
  34d231:	pcmpgtd %xmm3,%xmm4
  34d235:	pand   %xmm4,%xmm1
  34d239:	pandn  %xmm2,%xmm4
  34d23d:	por    %xmm1,%xmm4
  34d241:	pshufd $0xee,%xmm4,%xmm1
  34d246:	movdqa %xmm4,%xmm2
  34d24a:	pxor   %xmm0,%xmm2
  34d24e:	movdqa %xmm1,%xmm3
  34d252:	pxor   %xmm0,%xmm3
  34d256:	pcmpgtd %xmm3,%xmm2
  34d25a:	pand   %xmm2,%xmm4
  34d25e:	pandn  %xmm1,%xmm2
  34d262:	por    %xmm4,%xmm2
  34d266:	pshufd $0x55,%xmm2,%xmm1
  34d26b:	movdqa %xmm2,%xmm3
  34d26f:	pxor   %xmm0,%xmm3
  34d273:	pxor   %xmm1,%xmm0
  34d277:	pcmpgtd %xmm0,%xmm3
  34d27b:	pand   %xmm3,%xmm2
  34d27f:	pandn  %xmm1,%xmm3
  34d283:	por    %xmm2,%xmm3
  34d287:	movd   %xmm3,%edx
  34d28b:	jmp    34d6e9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1649>
  34d290:	mov    $0x1,%edi
  34d295:	mov    %rbx,%rsi
  34d298:	call   *0x2e6fe2(%rip)        # 634280 <_DYNAMIC+0x248>
  34d29e:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34d2a3:	lea    0x2a5ede(%rip),%rdx        # 5f3188 <anon.2751bc418103d8dead16706da22f1d23.109.llvm.1329807758949723497+0x2b8>
  34d2aa:	call   *0x2e7470(%rip)        # 634720 <_DYNAMIC+0x6e8>
  34d2b0:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34d2b5:	mov    $0x1,%edi
  34d2ba:	mov    %rbx,%rsi
  34d2bd:	call   *0x2e6fbd(%rip)        # 634280 <_DYNAMIC+0x248>
  34d2c3:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34d2c8:	mov    %rax,%rbp
  34d2cb:	mov    $0x1,%r12b
  34d2ce:	mov    $0x1,%r15b
  34d2d1:	mov    $0x1,%r14b
  34d2d4:	mov    $0x1,%bl
  34d2d6:	jmp    34d9cc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x192c>
  34d2db:	mov    %rax,%rbp
  34d2de:	mov    $0x1,%r12b
  34d2e1:	mov    $0x1,%r15b
  34d2e4:	mov    $0x1,%r14b
  34d2e7:	mov    $0x1,%bl
  34d2e9:	jmp    34d9a8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1908>
  34d2ee:	mov    %rax,%rbp
  34d2f1:	mov    $0x1,%r12b
  34d2f4:	mov    $0x1,%r15b
  34d2f7:	mov    $0x1,%r14b
  34d2fa:	mov    $0x1,%bl
  34d2fc:	lea    0x260(%rsp),%rdi
  34d304:	call   33e1a0 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabArena>>
  34d309:	xor    $0x1,%r14b
  34d30d:	mov    $0x1,%r13b
  34d310:	cmpq   $0x0,0x70(%rsp)
  34d316:	jne    34d36e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ce>
  34d318:	jmp    34d38a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ea>
  34d31a:	jmp    34d31c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x127c>
  34d31c:	mov    %rax,%rbp
  34d31f:	test   %r12,%r12
  34d322:	je     34d33b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x129b>
  34d324:	shl    $0x2,%r12
  34d328:	mov    $0x4,%edx
  34d32d:	mov    0x58(%rsp),%rdi
  34d332:	mov    %r12,%rsi
  34d335:	call   *0x2e6f35(%rip)        # 634270 <_DYNAMIC+0x238>
  34d33b:	cmpq   $0x0,0x28(%rsp)
  34d341:	je     34d358 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12b8>
  34d343:	mov    $0x1,%edx
  34d348:	mov    0x20(%rsp),%rdi
  34d34d:	mov    0x28(%rsp),%rsi
  34d352:	call   *0x2e6f18(%rip)        # 634270 <_DYNAMIC+0x238>
  34d358:	mov    $0x1,%r12b
  34d35b:	xor    %r14d,%r14d
  34d35e:	mov    $0x1,%r15b
  34d361:	mov    $0x1,%r13b
  34d364:	mov    $0x1,%bl
  34d366:	cmpq   $0x0,0x70(%rsp)
  34d36c:	je     34d38a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ea>
  34d36e:	mov    0x70(%rsp),%rsi
  34d373:	shl    $0x2,%rsi
  34d377:	mov    $0x4,%edx
  34d37c:	mov    0x198(%rsp),%rdi
  34d384:	call   *0x2e6ee6(%rip)        # 634270 <_DYNAMIC+0x238>
  34d38a:	cmpq   $0x0,0x78(%rsp)
  34d390:	jne    34d3a7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1307>
  34d392:	cmpq   $0x0,0x60(%rsp)
  34d398:	jne    34d3cb <<snaptokens::models::bpe::Bpe>::from_native_tables+0x132b>
  34d39a:	cmpq   $0xffffffffffffffff,0x340(%rsp)
  34d3a3:	jne    34d3f2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1352>
  34d3a5:	jmp    34d404 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1364>
  34d3a7:	mov    0x78(%rsp),%rsi
  34d3ac:	shl    $0x3,%rsi
  34d3b0:	mov    $0x8,%edx
  34d3b5:	mov    0x1a0(%rsp),%rdi
  34d3bd:	call   *0x2e6ead(%rip)        # 634270 <_DYNAMIC+0x238>
  34d3c3:	cmpq   $0x0,0x60(%rsp)
  34d3c9:	je     34d39a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12fa>
  34d3cb:	mov    0x60(%rsp),%rsi
  34d3d0:	shl    $0x2,%rsi
  34d3d4:	mov    $0x4,%edx
  34d3d9:	mov    0x1a8(%rsp),%rdi
  34d3e1:	call   *0x2e6e89(%rip)        # 634270 <_DYNAMIC+0x238>
  34d3e7:	cmpq   $0xffffffffffffffff,0x340(%rsp)
  34d3f0:	je     34d404 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1364>
  34d3f2:	test   %r12b,%r12b
  34d3f5:	je     34d404 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1364>
  34d3f7:	lea    0x340(%rsp),%rdi
  34d3ff:	call   33e2f0 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenTrie>>
  34d404:	cmpq   $0x0,0x108(%rsp)
  34d40d:	setne  %al
  34d410:	test   %al,%r15b
  34d413:	je     34d434 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1394>
  34d415:	mov    0x108(%rsp),%rsi
  34d41d:	shl    $0x3,%rsi
  34d421:	mov    $0x8,%edx
  34d426:	mov    0x190(%rsp),%rdi
  34d42e:	call   *0x2e6e3c(%rip)        # 634270 <_DYNAMIC+0x238>
  34d434:	test   %r13b,%r13b
  34d437:	je     34d490 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13f0>
  34d439:	cmpq   $0x0,0x1b8(%rsp)
  34d442:	jne    34d639 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1599>
  34d448:	cmpq   $0x0,0x1c0(%rsp)
  34d451:	jne    34d667 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x15c7>
  34d457:	cmpq   $0x0,0x1c8(%rsp)
  34d460:	jne    34d695 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x15f5>
  34d466:	cmpq   $0x0,0xd8(%rsp)
  34d46f:	je     34d490 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13f0>
  34d471:	mov    0xd8(%rsp),%rsi
  34d479:	shl    $0x5,%rsi
  34d47d:	mov    $0x10,%edx
  34d482:	mov    0x90(%rsp),%rdi
  34d48a:	call   *0x2e6de0(%rip)        # 634270 <_DYNAMIC+0x238>
  34d490:	cmpq   $0x0,0x110(%rsp)
  34d499:	jne    34d525 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1485>
  34d49f:	cmpq   $0x0,0x118(%rsp)
  34d4a8:	jne    34d553 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x14b3>
  34d4ae:	cmpq   $0x0,0x120(%rsp)
  34d4b7:	jne    34d581 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x14e1>
  34d4bd:	cmpq   $0x0,0x128(%rsp)
  34d4c6:	jne    34d5af <<snaptokens::models::bpe::Bpe>::from_native_tables+0x150f>
  34d4cc:	cmpq   $0x0,0x38(%rsp)
  34d4d2:	jne    34d5da <<snaptokens::models::bpe::Bpe>::from_native_tables+0x153a>
  34d4d8:	cmpq   $0x0,0x50(%rsp)
  34d4de:	sete   %al
  34d4e1:	or     %al,%r14b
  34d4e4:	je     34d608 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1568>
  34d4ea:	cmpq   $0x0,0xe0(%rsp)
  34d4f3:	setne  %al
  34d4f6:	test   %al,%bl
  34d4f8:	je     34d631 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1591>
  34d4fe:	mov    0xe0(%rsp),%rsi
  34d506:	shl    $0x3,%rsi
  34d50a:	mov    $0x4,%edx
  34d50f:	mov    0x98(%rsp),%rdi
  34d517:	call   *0x2e6d53(%rip)        # 634270 <_DYNAMIC+0x238>
  34d51d:	mov    %rbp,%rdi
  34d520:	call   5c6c50 <_Unwind_Resume@plt>
  34d525:	mov    0x110(%rsp),%rsi
  34d52d:	shl    $0x3,%rsi
  34d531:	mov    $0x8,%edx
  34d536:	mov    0xa0(%rsp),%rdi
  34d53e:	call   *0x2e6d2c(%rip)        # 634270 <_DYNAMIC+0x238>
  34d544:	cmpq   $0x0,0x118(%rsp)
  34d54d:	je     34d4ae <<snaptokens::models::bpe::Bpe>::from_native_tables+0x140e>
  34d553:	mov    0x118(%rsp),%rsi
  34d55b:	shl    $0x3,%rsi
  34d55f:	mov    $0x8,%edx
  34d564:	mov    0xa8(%rsp),%rdi
  34d56c:	call   *0x2e6cfe(%rip)        # 634270 <_DYNAMIC+0x238>
  34d572:	cmpq   $0x0,0x120(%rsp)
  34d57b:	je     34d4bd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x141d>
  34d581:	mov    0x120(%rsp),%rsi
  34d589:	shl    $0x2,%rsi
  34d58d:	mov    $0x4,%edx
  34d592:	mov    0x1b0(%rsp),%rdi
  34d59a:	call   *0x2e6cd0(%rip)        # 634270 <_DYNAMIC+0x238>
  34d5a0:	cmpq   $0x0,0x128(%rsp)
  34d5a9:	je     34d4cc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x142c>
  34d5af:	mov    0x128(%rsp),%rsi
  34d5b7:	shl    $0x2,%rsi
  34d5bb:	mov    $0x4,%edx
  34d5c0:	mov    0x100(%rsp),%rdi
  34d5c8:	call   *0x2e6ca2(%rip)        # 634270 <_DYNAMIC+0x238>
  34d5ce:	cmpq   $0x0,0x38(%rsp)
  34d5d4:	je     34d4d8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1438>
  34d5da:	mov    0x38(%rsp),%rsi
  34d5df:	shl    $0x2,%rsi
  34d5e3:	mov    $0x4,%edx
  34d5e8:	mov    0xf8(%rsp),%rdi
  34d5f0:	call   *0x2e6c7a(%rip)        # 634270 <_DYNAMIC+0x238>
  34d5f6:	cmpq   $0x0,0x50(%rsp)
  34d5fc:	sete   %al
  34d5ff:	or     %al,%r14b
  34d602:	jne    34d4ea <<snaptokens::models::bpe::Bpe>::from_native_tables+0x144a>
  34d608:	mov    $0x1,%edx
  34d60d:	mov    0x48(%rsp),%rdi
  34d612:	mov    0x50(%rsp),%rsi
  34d617:	call   *0x2e6c53(%rip)        # 634270 <_DYNAMIC+0x238>
  34d61d:	cmpq   $0x0,0xe0(%rsp)
  34d626:	setne  %al
  34d629:	test   %al,%bl
  34d62b:	jne    34d4fe <<snaptokens::models::bpe::Bpe>::from_native_tables+0x145e>
  34d631:	mov    %rbp,%rdi
  34d634:	call   5c6c50 <_Unwind_Resume@plt>
  34d639:	mov    0x1b8(%rsp),%rsi
  34d641:	shl    $0x2,%rsi
  34d645:	mov    $0x4,%edx
  34d64a:	mov    0xc8(%rsp),%rdi
  34d652:	call   *0x2e6c18(%rip)        # 634270 <_DYNAMIC+0x238>
  34d658:	cmpq   $0x0,0x1c0(%rsp)
  34d661:	je     34d457 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13b7>
  34d667:	mov    0x1c0(%rsp),%rsi
  34d66f:	shl    $0x3,%rsi
  34d673:	mov    $0x8,%edx
  34d678:	mov    0x1e8(%rsp),%rdi
  34d680:	call   *0x2e6bea(%rip)        # 634270 <_DYNAMIC+0x238>
  34d686:	cmpq   $0x0,0x1c8(%rsp)
  34d68f:	je     34d466 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13c6>
  34d695:	mov    0x1c8(%rsp),%rsi
  34d69d:	shl    $0x2,%rsi
  34d6a1:	mov    $0x4,%edx
  34d6a6:	mov    0xd0(%rsp),%rdi
  34d6ae:	call   *0x2e6bbc(%rip)        # 634270 <_DYNAMIC+0x238>
  34d6b4:	cmpq   $0x0,0xd8(%rsp)
  34d6bd:	jne    34d471 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13d1>
  34d6c3:	jmp    34d490 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x13f0>
  34d6c8:	ud2
  34d6ca:	mov    %edx,%esi
  34d6cc:	mov    0x98(%rsp),%rdi
  34d6d4:	mov    0x8(%rdi,%rcx,8),%edx
  34d6d8:	mov    0xc(%rdi,%rcx,8),%edi
  34d6dc:	cmp    %edx,%edi
  34d6de:	cmova  %edi,%edx
  34d6e1:	cmp    %edx,%esi
  34d6e3:	cmova  %esi,%edx
  34d6e6:	inc    %rcx
  34d6e9:	cmp    %rcx,%rax
  34d6ec:	jne    34d6ca <<snaptokens::models::bpe::Bpe>::from_native_tables+0x162a>
  34d6ee:	mov    %edx,%eax
  34d6f0:	cmp    %rax,0x8(%rsp)
  34d6f5:	jbe    34d75b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x16bb>
  34d6f7:	mov    0xf8(%rsp),%rsi
  34d6ff:	lea    0x400(%rsi),%rax
  34d706:	xor    %ecx,%ecx
  34d708:	mov    $0xffffffff,%edx
  34d70d:	jmp    34d731 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1691>
  34d70f:	lea    0x4(%rsi),%rdi
  34d713:	mov    (%rsi),%esi
  34d715:	cmp    %rdx,%rsi
  34d718:	setne  %r8b
  34d71c:	cmp    %rsi,0x8(%rsp)
  34d721:	setbe  %r9b
  34d725:	mov    %rdi,%rsi
  34d728:	test   %r9b,%r8b
  34d72b:	jne    34d7b8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1718>
  34d731:	test   %rsi,%rsi
  34d734:	je     34d73b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x169b>
  34d736:	cmp    %rax,%rsi
  34d739:	jne    34d70f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x166f>
  34d73b:	cmp    $0x400,%rcx
  34d742:	je     34d858 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x17b8>
  34d748:	mov    0x100(%rsp),%rsi
  34d750:	add    %rcx,%rsi
  34d753:	add    $0x4,%rcx
  34d757:	xor    %edi,%edi
  34d759:	jmp    34d713 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1673>
  34d75b:	call   *0x2e6b2f(%rip)        # 634290 <_DYNAMIC+0x258>
  34d761:	mov    $0x24,%ebx
  34d766:	mov    $0x24,%edi
  34d76b:	mov    $0x1,%esi
  34d770:	call   *0x2e6b22(%rip)        # 634298 <_DYNAMIC+0x260>
  34d776:	test   %rax,%rax
  34d779:	je     34e830 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2790>
  34d77f:	movups -0x2221ea(%rip),%xmm0        # 12b59c <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x93c>
  34d786:	movups %xmm0,0x10(%rax)
  34d78a:	movdqu -0x222206(%rip),%xmm0        # 12b58c <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x92c>
  34d792:	movdqu %xmm0,(%rax)
  34d796:	movl   $0x6e656b6f,0x20(%rax)
  34d79d:	mov    0x18(%rsp),%rcx
  34d7a2:	movq   $0x24,0x8(%rcx)
  34d7aa:	mov    %rax,0x10(%rcx)
  34d7ae:	movq   $0x24,0x18(%rcx)
  34d7b6:	jmp    34d80c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x176c>
  34d7b8:	call   *0x2e6ad2(%rip)        # 634290 <_DYNAMIC+0x258>
  34d7be:	mov    $0x20,%ebx
  34d7c3:	mov    $0x20,%edi
  34d7c8:	mov    $0x1,%esi
  34d7cd:	call   *0x2e6ac5(%rip)        # 634298 <_DYNAMIC+0x260>
  34d7d3:	test   %rax,%rax
  34d7d6:	je     34e830 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2790>
  34d7dc:	movups -0x28e2f3(%rip),%xmm0        # bf4f0 <anon.bdde665530aa007dd7855882198d96b9.18.llvm.12424819659099699973+0xb0>
  34d7e3:	movups %xmm0,0x10(%rax)
  34d7e7:	movdqu -0x28e30f(%rip),%xmm0        # bf4e0 <anon.bdde665530aa007dd7855882198d96b9.18.llvm.12424819659099699973+0xa0>
  34d7ef:	movdqu %xmm0,(%rax)
  34d7f3:	mov    0x18(%rsp),%rcx
  34d7f8:	movq   $0x20,0x8(%rcx)
  34d800:	mov    %rax,0x10(%rcx)
  34d804:	movq   $0x20,0x18(%rcx)
  34d80c:	movq   $0xffffffffffffffff,(%rcx)
  34d813:	mov    $0x1,%r12b
  34d816:	cmpq   $0x0,0x50(%rsp)
  34d81c:	je     34d833 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1793>
  34d81e:	mov    $0x1,%edx
  34d823:	mov    0x48(%rsp),%rdi
  34d828:	mov    0x50(%rsp),%rsi
  34d82d:	call   *0x2e6a3d(%rip)        # 634270 <_DYNAMIC+0x238>
  34d833:	lea    0x308(%rsp),%rdi
  34d83b:	call   33e2b0 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabLookup>>
  34d840:	mov    $0x1,%bl
  34d842:	mov    0x248(%rsp),%rsi
  34d84a:	test   %rsi,%rsi
  34d84d:	jne    34cee1 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe41>
  34d853:	jmp    34cef8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0xe58>
  34d858:	cmpq   $0x0,0xb0(%rsp)
  34d861:	je     34da76 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19d6>
  34d867:	mov    0xc8(%rsp),%rax
  34d86f:	mov    (%rax),%edx
  34d871:	cmpq   $0x1,0xb0(%rsp)
  34d87a:	je     34da69 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19c9>
  34d880:	movabs $0x3fffffffffffffff,%rsi
  34d88a:	mov    0xb0(%rsp),%rax
  34d892:	lea    (%rax,%rsi,1),%rcx
  34d896:	mov    %rcx,%rax
  34d899:	and    %rsi,%rax
  34d89c:	cmp    $0x8,%rax
  34d8a0:	jae    34d8a9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1809>
  34d8a2:	xor    %ecx,%ecx
  34d8a4:	jmp    34da4e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19ae>
  34d8a9:	add    $0xfffffffffffffff9,%rsi
  34d8ad:	and    %rsi,%rcx
  34d8b0:	movd   %edx,%xmm0
  34d8b4:	pshufd $0x0,%xmm0,%xmm1
  34d8b9:	xor    %edx,%edx
  34d8bb:	movdqa -0x2978f3(%rip),%xmm0        # b5fd0 <anon.2e19753996ac26bdd79597722b35bd89.32.llvm.7838387873399110284+0x12c0>
  34d8c3:	movdqa %xmm1,%xmm2
  34d8c7:	mov    0xc8(%rsp),%rsi
  34d8cf:	movdqu 0x4(%rsi,%rdx,4),%xmm3
  34d8d5:	movdqu 0x14(%rsi,%rdx,4),%xmm4
  34d8db:	movdqa %xmm1,%xmm5
  34d8df:	pxor   %xmm0,%xmm1
  34d8e3:	movdqa %xmm3,%xmm6
  34d8e7:	pxor   %xmm0,%xmm6
  34d8eb:	pcmpgtd %xmm6,%xmm1
  34d8ef:	pand   %xmm1,%xmm5
  34d8f3:	pandn  %xmm3,%xmm1
  34d8f7:	por    %xmm5,%xmm1
  34d8fb:	movdqa %xmm2,%xmm3
  34d8ff:	pxor   %xmm0,%xmm2
  34d903:	movdqa %xmm4,%xmm5
  34d907:	pxor   %xmm0,%xmm5
  34d90b:	pcmpgtd %xmm5,%xmm2
  34d90f:	pand   %xmm2,%xmm3
  34d913:	pandn  %xmm4,%xmm2
  34d917:	por    %xmm3,%xmm2
  34d91b:	add    $0x8,%rdx
  34d91f:	cmp    %rdx,%rcx
  34d922:	jne    34d8cf <<snaptokens::models::bpe::Bpe>::from_native_tables+0x182f>
  34d924:	movdqa %xmm2,%xmm3
  34d928:	pxor   %xmm0,%xmm3
  34d92c:	movdqa %xmm1,%xmm4
  34d930:	pxor   %xmm0,%xmm4
  34d934:	pcmpgtd %xmm3,%xmm4
  34d938:	pand   %xmm4,%xmm1
  34d93c:	pandn  %xmm2,%xmm4
  34d940:	por    %xmm1,%xmm4
  34d944:	pshufd $0xee,%xmm4,%xmm1
  34d949:	movdqa %xmm4,%xmm2
  34d94d:	pxor   %xmm0,%xmm2
  34d951:	movdqa %xmm1,%xmm3
  34d955:	pxor   %xmm0,%xmm3
  34d959:	pcmpgtd %xmm3,%xmm2
  34d95d:	pand   %xmm2,%xmm4
  34d961:	pandn  %xmm1,%xmm2
  34d965:	por    %xmm4,%xmm2
  34d969:	pshufd $0x55,%xmm2,%xmm1
  34d96e:	movdqa %xmm2,%xmm3
  34d972:	pxor   %xmm0,%xmm3
  34d976:	pxor   %xmm1,%xmm0
  34d97a:	pcmpgtd %xmm0,%xmm3
  34d97e:	pand   %xmm3,%xmm2
  34d982:	pandn  %xmm1,%xmm3
  34d986:	por    %xmm2,%xmm3
  34d98a:	movd   %xmm3,%edx
  34d98e:	jmp    34da64 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19c4>
  34d993:	test   %r14b,%r14b
  34d996:	je     34d9f9 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1959>
  34d998:	lea    0x308(%rsp),%rdi
  34d9a0:	call   33e2b0 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabLookup>>
  34d9a5:	xor    %r14d,%r14d
  34d9a8:	mov    0x248(%rsp),%rsi
  34d9b0:	test   %rsi,%rsi
  34d9b3:	je     34d9cc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x192c>
  34d9b5:	mov    0x250(%rsp),%rdi
  34d9bd:	shl    $0x3,%rsi
  34d9c1:	mov    $0x8,%edx
  34d9c6:	call   *0x2e68a4(%rip)        # 634270 <_DYNAMIC+0x238>
  34d9cc:	mov    0x230(%rsp),%rsi
  34d9d4:	test   %rsi,%rsi
  34d9d7:	je     34d2fc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x125c>
  34d9dd:	mov    0x238(%rsp),%rdi
  34d9e5:	shl    $0x3,%rsi
  34d9e9:	mov    $0x8,%edx
  34d9ee:	call   *0x2e687c(%rip)        # 634270 <_DYNAMIC+0x238>
  34d9f4:	jmp    34d2fc <<snaptokens::models::bpe::Bpe>::from_native_tables+0x125c>
  34d9f9:	mov    $0x1,%r14b
  34d9fc:	xor    %r13d,%r13d
  34d9ff:	cmpq   $0x0,0x70(%rsp)
  34da05:	jne    34d36e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ce>
  34da0b:	jmp    34d38a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x12ea>
  34da10:	mov    %rax,%rbp
  34da13:	mov    $0x1,%al
  34da15:	mov    %eax,0x58(%rsp)
  34da19:	mov    $0x1,%r15b
  34da1c:	mov    $0x1,%r14b
  34da1f:	mov    $0x1,%bl
  34da21:	cmpq   $0x0,0x50(%rsp)
  34da27:	setne  %al
  34da2a:	test   %al,0x58(%rsp)
  34da2e:	je     34d993 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x18f3>
  34da34:	mov    $0x1,%edx
  34da39:	mov    0x48(%rsp),%rdi
  34da3e:	mov    0x50(%rsp),%rsi
  34da43:	call   *0x2e6827(%rip)        # 634270 <_DYNAMIC+0x238>
  34da49:	jmp    34d993 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x18f3>
  34da4e:	mov    %edx,%esi
  34da50:	mov    0xc8(%rsp),%rdx
  34da58:	mov    0x4(%rdx,%rcx,4),%edx
  34da5c:	cmp    %edx,%esi
  34da5e:	cmova  %esi,%edx
  34da61:	inc    %rcx
  34da64:	cmp    %rcx,%rax
  34da67:	jne    34da4e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19ae>
  34da69:	mov    %edx,%eax
  34da6b:	cmp    %rax,0x8(%rsp)
  34da70:	jbe    34db7b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1adb>
  34da76:	cmpq   $0x0,0x40(%rsp)
  34da7c:	je     34db2d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1a8d>
  34da82:	xor    %eax,%eax
  34da84:	jmp    34da94 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19f4>
  34da86:	inc    %rax
  34da89:	cmp    %rax,0x40(%rsp)
  34da8e:	je     34db2d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1a8d>
  34da94:	mov    0xa8(%rsp),%rcx
  34da9c:	mov    (%rcx,%rax,8),%rcx
  34daa0:	cmp    $0xffffffffffffffff,%rcx
  34daa4:	je     34da86 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19e6>
  34daa6:	mov    %rcx,%rdx
  34daa9:	shr    $0x20,%rdx
  34daad:	cmp    0x8(%rsp),%rdx
  34dab2:	jae    34dacf <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1a2f>
  34dab4:	mov    %ecx,%ecx
  34dab6:	cmp    0x8(%rsp),%rcx
  34dabb:	jae    34dacf <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1a2f>
  34dabd:	mov    0xa0(%rsp),%rcx
  34dac5:	mov    (%rcx,%rax,8),%ecx
  34dac8:	cmp    0x8(%rsp),%rcx
  34dacd:	jb     34da86 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x19e6>
  34dacf:	call   *0x2e67bb(%rip)        # 634290 <_DYNAMIC+0x258>
  34dad5:	mov    $0x23,%ebx
  34dada:	mov    $0x23,%edi
  34dadf:	mov    $0x1,%esi
  34dae4:	call   *0x2e67ae(%rip)        # 634298 <_DYNAMIC+0x260>
  34daea:	test   %rax,%rax
  34daed:	je     34e830 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2790>
  34daf3:	movups -0x222581(%rip),%xmm0        # 12b579 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x919>
  34dafa:	movups %xmm0,0x10(%rax)
  34dafe:	movups -0x22259c(%rip),%xmm0        # 12b569 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x909>
  34db05:	movups %xmm0,(%rax)
  34db08:	movl   $0x6e656b6f,0x1f(%rax)
  34db0f:	mov    0x18(%rsp),%rcx
  34db14:	movq   $0x23,0x8(%rcx)
  34db1c:	mov    %rax,0x10(%rcx)
  34db20:	movq   $0x23,0x18(%rcx)
  34db28:	jmp    34d80c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x176c>
  34db2d:	cmpq   $0x0,0x178(%rsp)
  34db36:	je     34dd28 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1c88>
  34db3c:	mov    0x90(%rsp),%rax
  34db44:	mov    0x10(%rax),%edx
  34db47:	cmpq   $0x1,0x178(%rsp)
  34db50:	je     34dd1b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1c7b>
  34db56:	movabs $0x7ffffffffffffff,%rax
  34db60:	mov    0x178(%rsp),%rcx
  34db68:	add    %rax,%rcx
  34db6b:	and    %rcx,%rax
  34db6e:	cmp    $0x9,%rax
  34db72:	jae    34dbb7 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1b17>
  34db74:	xor    %ecx,%ecx
  34db76:	jmp    34dcf3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1c53>
  34db7b:	call   *0x2e670f(%rip)        # 634290 <_DYNAMIC+0x258>
  34db81:	mov    $0x20,%ebx
  34db86:	mov    $0x20,%edi
  34db8b:	mov    $0x1,%esi
  34db90:	call   *0x2e6702(%rip)        # 634298 <_DYNAMIC+0x260>
  34db96:	test   %rax,%rax
  34db99:	je     34e830 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2790>
  34db9f:	movups -0x28e1b6(%rip),%xmm0        # bf9f0 <anon.3b3cd0bc6b2536aa46935e5d0aff9f95.56.llvm.6491278627211742442+0x110>
  34dba6:	movups %xmm0,0x10(%rax)
  34dbaa:	movdqu -0x28e1d2(%rip),%xmm0        # bf9e0 <anon.3b3cd0bc6b2536aa46935e5d0aff9f95.56.llvm.6491278627211742442+0x100>
  34dbb2:	jmp    34d7ef <<snaptokens::models::bpe::Bpe>::from_native_tables+0x174f>
  34dbb7:	and    $0x7,%ecx
  34dbba:	mov    $0x8,%esi
  34dbbf:	cmovne %rcx,%rsi
  34dbc3:	mov    %rax,%rcx
  34dbc6:	sub    %rsi,%rcx
  34dbc9:	movd   %edx,%xmm0
  34dbcd:	pshufd $0x0,%xmm0,%xmm1
  34dbd2:	mov    0x90(%rsp),%rdx
  34dbda:	add    $0x110,%rdx
  34dbe1:	movdqa -0x297c19(%rip),%xmm0        # b5fd0 <anon.2e19753996ac26bdd79597722b35bd89.32.llvm.7838387873399110284+0x12c0>
  34dbe9:	mov    %rcx,%rsi
  34dbec:	movdqa %xmm1,%xmm2
  34dbf0:	movd   -0x80(%rdx),%xmm3
  34dbf5:	movd   -0xa0(%rdx),%xmm4
  34dbfd:	punpckldq %xmm3,%xmm4
  34dc01:	movd   -0xc0(%rdx),%xmm3
  34dc09:	movd   -0xe0(%rdx),%xmm5
  34dc11:	punpckldq %xmm3,%xmm5
  34dc15:	punpcklqdq %xmm4,%xmm5
  34dc19:	movd   (%rdx),%xmm3
  34dc1d:	movd   -0x20(%rdx),%xmm4
  34dc22:	punpckldq %xmm3,%xmm4
  34dc26:	movd   -0x40(%rdx),%xmm3
  34dc2b:	movd   -0x60(%rdx),%xmm6
  34dc30:	punpckldq %xmm3,%xmm6
  34dc34:	punpcklqdq %xmm4,%xmm6
  34dc38:	movdqa %xmm5,%xmm3
  34dc3c:	pxor   %xmm0,%xmm3
  34dc40:	movdqa %xmm1,%xmm4
  34dc44:	pxor   %xmm0,%xmm1
  34dc48:	pcmpgtd %xmm3,%xmm1
  34dc4c:	pand   %xmm1,%xmm4
  34dc50:	pandn  %xmm5,%xmm1
  34dc54:	por    %xmm4,%xmm1
  34dc58:	movdqa %xmm6,%xmm3
  34dc5c:	pxor   %xmm0,%xmm3
  34dc60:	movdqa %xmm2,%xmm4
  34dc64:	pxor   %xmm0,%xmm2
  34dc68:	pcmpgtd %xmm3,%xmm2
  34dc6c:	pand   %xmm2,%xmm4
  34dc70:	pandn  %xmm6,%xmm2
  34dc74:	por    %xmm4,%xmm2
  34dc78:	add    $0x100,%rdx
  34dc7f:	add    $0xfffffffffffffff8,%rsi
  34dc83:	jne    34dbf0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1b50>
  34dc89:	movdqa %xmm2,%xmm3
  34dc8d:	pxor   %xmm0,%xmm3
  34dc91:	movdqa %xmm1,%xmm4
  34dc95:	pxor   %xmm0,%xmm4
  34dc99:	pcmpgtd %xmm3,%xmm4
  34dc9d:	pand   %xmm4,%xmm1
  34dca1:	pandn  %xmm2,%xmm4
  34dca5:	por    %xmm1,%xmm4
  34dca9:	pshufd $0xee,%xmm4,%xmm1
  34dcae:	movdqa %xmm4,%xmm2
  34dcb2:	pxor   %xmm0,%xmm2
  34dcb6:	movdqa %xmm1,%xmm3
  34dcba:	pxor   %xmm0,%xmm3
  34dcbe:	pcmpgtd %xmm3,%xmm2
  34dcc2:	pand   %xmm2,%xmm4
  34dcc6:	pandn  %xmm1,%xmm2
  34dcca:	por    %xmm4,%xmm2
  34dcce:	pshufd $0x55,%xmm2,%xmm1
  34dcd3:	movdqa %xmm2,%xmm3
  34dcd7:	pxor   %xmm0,%xmm3
  34dcdb:	pxor   %xmm1,%xmm0
  34dcdf:	pcmpgtd %xmm0,%xmm3
  34dce3:	pand   %xmm3,%xmm2
  34dce7:	pandn  %xmm1,%xmm3
  34dceb:	por    %xmm2,%xmm3
  34dcef:	movd   %xmm3,%edx
  34dcf3:	sub    %rcx,%rax
  34dcf6:	shl    $0x5,%rcx
  34dcfa:	mov    0x90(%rsp),%rsi
  34dd02:	add    %rsi,%rcx
  34dd05:	add    $0x30,%rcx
  34dd09:	mov    %edx,%esi
  34dd0b:	mov    (%rcx),%edx
  34dd0d:	cmp    %edx,%esi
  34dd0f:	cmova  %esi,%edx
  34dd12:	add    $0x20,%rcx
  34dd16:	dec    %rax
  34dd19:	jne    34dd09 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1c69>
  34dd1b:	mov    %edx,%eax
  34dd1d:	cmp    %rax,0x8(%rsp)
  34dd22:	jbe    34ddde <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1d3e>
  34dd28:	cmpq   $0xffffffffffffffff,0x340(%rsp)
  34dd31:	je     34de3b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1d9b>
  34dd37:	xor    %r12d,%r12d
  34dd3a:	lea    0x130(%rsp),%rdi
  34dd42:	lea    0x260(%rsp),%r9
  34dd4a:	mov    0x2d0(%rsp),%rsi
  34dd52:	mov    0x8(%rsp),%rdx
  34dd57:	mov    0x48(%rsp),%rcx
  34dd5c:	mov    %rdx,%r8
  34dd5f:	call   33c260 <<snaptokens::models::bpe::ExactTokenTrie>::validate_with::<<snaptokens::models::bpe::Bpe>::from_native_tables::{closure#5}>>
  34dd64:	mov    0x130(%rsp),%rax
  34dd6c:	movups 0x138(%rsp),%xmm0
  34dd74:	movaps %xmm0,0x1f0(%rsp)
  34dd7c:	mov    0x148(%rsp),%rcx
  34dd84:	mov    %rcx,0x200(%rsp)
  34dd8c:	cmp    $0xffffffffffffffff,%rax
  34dd90:	je     34e7d2 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2732>
  34dd96:	movups 0x150(%rsp),%xmm0
  34dd9e:	movups %xmm0,0x2b0(%rsp)
  34dda6:	movdqa 0x1f0(%rsp),%xmm0
  34ddaf:	movdqu %xmm0,0x298(%rsp)
  34ddb8:	mov    0x200(%rsp),%rcx
  34ddc0:	mov    %rcx,0x2a8(%rsp)
  34ddc8:	mov    %rax,0x290(%rsp)
  34ddd0:	mov    $0x1,%al
  34ddd2:	mov    %eax,0x58(%rsp)
  34ddd6:	xor    %r12d,%r12d
  34ddd9:	jmp    34de79 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1dd9>
  34ddde:	call   *0x2e64ac(%rip)        # 634290 <_DYNAMIC+0x258>
  34dde4:	mov    $0x21,%ebx
  34dde9:	mov    $0x21,%edi
  34ddee:	mov    $0x1,%esi
  34ddf3:	call   *0x2e649f(%rip)        # 634298 <_DYNAMIC+0x260>
  34ddf9:	test   %rax,%rax
  34ddfc:	je     34e830 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2790>
  34de02:	movups -0x2228b1(%rip),%xmm0        # 12b558 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8f8>
  34de09:	movups %xmm0,0x10(%rax)
  34de0d:	movdqu -0x2228cd(%rip),%xmm0        # 12b548 <anon.2751bc418103d8dead16706da22f1d23.40.llvm.1329807758949723497+0x8e8>
  34de15:	movdqu %xmm0,(%rax)
  34de19:	movb   $0x64,0x20(%rax)
  34de1d:	mov    0x18(%rsp),%rcx
  34de22:	movq   $0x21,0x8(%rcx)
  34de2a:	mov    %rax,0x10(%rcx)
  34de2e:	movq   $0x21,0x18(%rcx)
  34de36:	jmp    34d80c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x176c>
  34de3b:	mov    0x50(%rsp),%rax
  34de40:	mov    %rax,0x298(%rsp)
  34de48:	mov    0x48(%rsp),%rax
  34de4d:	mov    %rax,0x2a0(%rsp)
  34de55:	mov    0x8(%rsp),%rax
  34de5a:	mov    %rax,0x2a8(%rsp)
  34de62:	movq   $0xffffffffffffffff,0x290(%rsp)
  34de6e:	mov    $0x1,%r12b
  34de71:	movl   $0x0,0x58(%rsp)
  34de79:	lea    0x3d0(%rsp),%rdi
  34de81:	mov    0x2e6418(%rip),%rbx        # 6342a0 <memcpy@GLIBC_2.14>
  34de88:	mov    $0x400,%edx
  34de8d:	mov    0xf8(%rsp),%rsi
  34de95:	call   *%rbx
  34de97:	lea    0x9d0(%rsp),%rdi
  34de9f:	mov    $0x400,%edx
  34dea4:	mov    0x100(%rsp),%rsi
  34deac:	call   *%rbx
  34deae:	lea    0x130(%rsp),%rdi
  34deb6:	mov    $0x10000,%esi
  34debb:	call   3414c0 <<u32 as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>>
  34dec0:	lea    0x130(%rsp),%rdi
  34dec8:	call   3c3bc0 <<alloc::vec::Vec<u32>>::into_boxed_slice>
  34decd:	mov    %rax,0x28(%rsp)
  34ded2:	mov    %rdx,0x20(%rsp)
  34ded7:	xor    %r14d,%r14d
  34deda:	lea    0x260(%rsp),%rbx
  34dee2:	lea    0x130(%rsp),%r15
  34deea:	jmp    34def6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e56>
  34deec:	inc    %r14
  34deef:	cmp    %r14,0x8(%rsp)
  34def4:	je     34df70 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1ed0>
  34def6:	mov    %rbx,%rdi
  34def9:	mov    %r14d,%esi
  34defc:	call   344c80 <<snaptokens::models::bpe::VocabArena>::get>
  34df01:	test   %rax,%rax
  34df04:	je     34e800 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2760>
  34df0a:	add    %rax,%rdx
  34df0d:	mov    %rax,0x130(%rsp)
  34df15:	mov    %rdx,0x138(%rsp)
  34df1d:	mov    %r15,%rdi
  34df20:	call   33e470 <core::str::validations::next_code_point::<core::slice::iter::Iter<u8>>>
  34df25:	and    $0x1,%al
  34df27:	cmp    $0x1,%al
  34df29:	mov    $0x0,%r13d
  34df2f:	sbb    %r13d,%r13d
  34df32:	or     %edx,%r13d
  34df35:	mov    %r15,%rdi
  34df38:	call   33e470 <core::str::validations::next_code_point::<core::slice::iter::Iter<u8>>>
  34df3d:	cmp    $0xffffffff,%r13d
  34df41:	sete   %cl
  34df44:	or     %al,%cl
  34df46:	test   $0x1,%cl
  34df49:	jne    34deec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e4c>
  34df4b:	cmp    $0xffff,%r13d
  34df52:	ja     34deec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e4c>
  34df54:	mov    %r13d,%edi
  34df57:	cmp    %rdi,0x20(%rsp)
  34df5c:	jbe    34e846 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x27a6>
  34df62:	mov    0x28(%rsp),%rax
  34df67:	mov    %r14d,(%rax,%rdi,4)
  34df6b:	jmp    34deec <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1e4c>
  34df70:	cmpq   $0x7f,0x20(%rsp)
  34df76:	jbe    34e812 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2772>
  34df7c:	lea    0x7d0(%rsp),%rdi
  34df84:	mov    $0x200,%edx
  34df89:	mov    0x28(%rsp),%rsi
  34df8e:	call   *0x2e630c(%rip)        # 6342a0 <memcpy@GLIBC_2.14>
  34df94:	movq   $0x0,0x1f8(%rsp)
  34dfa0:	mov    0x8(%rsp),%rax
  34dfa5:	mov    %rax,0x200(%rsp)
  34dfad:	mov    %rbx,0x1f0(%rsp)
  34dfb5:	lea    0x130(%rsp),%rdi
  34dfbd:	lea    0x1f0(%rsp),%rsi
  34dfc5:	call   40f3e0 <core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::Bpe>::from_native_tables::{closure#6}>, u8, core::result::Result<core::convert::Infallible, alloc::string::String>, <core::result::Result<alloc::vec::Vec<u8>, alloc::string::String> as core::iter::traits::collect::FromIterator<core::result::Result<u8, alloc::string::String>>>::from_iter<core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::Bpe>::from_native_tables::{closure#6}>>::{closure#0}, alloc::vec::Vec<u8>>>
  34dfca:	mov    0x138(%rsp),%rax
  34dfd2:	mov    %rax,0x68(%rsp)
  34dfd7:	mov    0x140(%rsp),%rax
  34dfdf:	mov    %rax,0xc0(%rsp)
  34dfe7:	mov    0x148(%rsp),%r15
  34dfef:	cmpb   $0x0,0x130(%rsp)
  34dff7:	je     34e058 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1fb8>
  34dff9:	mov    0x18(%rsp),%rax
  34dffe:	mov    0x68(%rsp),%rcx
  34e003:	mov    %rcx,0x8(%rax)
  34e007:	mov    0xc0(%rsp),%rcx
  34e00f:	mov    %rcx,0x10(%rax)
  34e013:	mov    %r15,0x18(%rax)
  34e017:	movq   $0xffffffffffffffff,(%rax)
  34e01e:	mov    0x20(%rsp),%rax
  34e023:	lea    0x0(,%rax,4),%rsi
  34e02b:	mov    $0x4,%edx
  34e030:	mov    0x28(%rsp),%rdi
  34e035:	call   *0x2e6235(%rip)        # 634270 <_DYNAMIC+0x238>
  34e03b:	lea    0x290(%rsp),%rdi
  34e043:	call   33e3d0 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenMatcher>>
  34e048:	cmpb   $0x0,0x58(%rsp)
  34e04d:	jne    34d816 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1776>
  34e053:	jmp    34d833 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1793>
  34e058:	mov    0x190(%rsp),%rax
  34e060:	mov    (%rax),%rbx
  34e063:	movdqu 0x8(%rax),%xmm0
  34e068:	movdqa %xmm0,0x2c0(%rsp)
  34e071:	lea    0x18(%rax),%rsi
  34e075:	lea    0xdd0(%rsp),%rdi
  34e07d:	mov    $0x1fe8,%edx
  34e082:	call   *0x2e6218(%rip)        # 6342a0 <memcpy@GLIBC_2.14>
  34e088:	cmpq   $0x0,0x108(%rsp)
  34e091:	je     34e0b6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2016>
  34e093:	mov    0x108(%rsp),%rax
  34e09b:	lea    0x0(,%rax,8),%rsi
  34e0a3:	mov    $0x8,%edx
  34e0a8:	mov    0x190(%rsp),%rdi
  34e0b0:	call   *0x2e61ba(%rip)        # 634270 <_DYNAMIC+0x238>
  34e0b6:	lea    0x2d8(%rsp),%rdi
  34e0be:	lea    0x3d0(%rsp),%rsi
  34e0c6:	mov    0x8(%rsp),%rdx
  34e0cb:	call   353620 <snaptokens::models::bpe::initial_token_byte_map>
  34e0d0:	mov    0x2e0(%rsp),%r9
  34e0d8:	sub    $0x8,%rsp
  34e0dc:	lea    0x2f8(%rsp),%rdi
  34e0e4:	mov    0xb0(%rsp),%rsi
  34e0ec:	mov    0x48(%rsp),%rdx
  34e0f1:	mov    0xa8(%rsp),%rcx
  34e0f9:	mov    %rdx,%r8
  34e0fc:	mov    %r9,0x1d8(%rsp)
  34e104:	push   0x2f0(%rsp)
  34e10b:	call   354180 <snaptokens::models::bpe::byte_pair_initial_from_ranked>
  34e110:	add    $0x10,%rsp
  34e114:	movzbl 0x17(%rsp),%eax
  34e119:	movzbl 0x16(%rsp),%r10d
  34e11f:	lea    0x130(%rsp),%rdi
  34e127:	lea    0x3d0(%rsp),%r9
  34e12f:	mov    0xa8(%rsp),%rsi
  34e137:	mov    0x40(%rsp),%rdx
  34e13c:	mov    0xa0(%rsp),%rcx
  34e144:	mov    %rdx,%r8
  34e147:	push   %rax
  34e148:	push   %r10
  34e14a:	call   353750 <snaptokens::models::bpe::dense_tables_from_ranked>
  34e14f:	add    $0x10,%rsp
  34e153:	mov    0x148(%rsp),%eax
  34e15a:	mov    %eax,0x40(%rsp)
  34e15e:	mov    0x130(%rsp),%rax
  34e166:	mov    %rax,0x88(%rsp)
  34e16e:	mov    0x138(%rsp),%rax
  34e176:	mov    %rax,0xb8(%rsp)
  34e17e:	mov    0x140(%rsp),%rax
  34e186:	mov    %rax,0x1e0(%rsp)
  34e18e:	mov    0x150(%rsp),%rax
  34e196:	mov    %rax,0x80(%rsp)
  34e19e:	mov    0x158(%rsp),%rax
  34e1a6:	mov    %rax,0xf0(%rsp)
  34e1ae:	mov    0x160(%rsp),%rax
  34e1b6:	mov    %rax,0x1d8(%rsp)
  34e1be:	mov    $0x1,%eax
  34e1c3:	mov    $0x1,%r13d
  34e1c9:	lock xadd %r13,0x2eeba6(%rip)        # 63cd78 <snaptokens::models::bpe::BPE_ID_COUNTER>
  34e1d2:	test   %r13,%r13
  34e1d5:	jne    34e1e3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2143>
  34e1d7:	lock xadd %rax,0x2eeb98(%rip)        # 63cd78 <snaptokens::models::bpe::BPE_ID_COUNTER>
  34e1e0:	mov    %rax,%r13
  34e1e3:	lea    0x370(%rsp),%rdi
  34e1eb:	mov    $0x40,%edx
  34e1f0:	xor    %esi,%esi
  34e1f2:	call   3c5ad0 <<alloc::vec::Vec<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>, core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::SharedCache>::new::{closure#0}>>>::from_iter>
  34e1f7:	lea    0x388(%rsp),%rdi
  34e1ff:	mov    $0x40,%edx
  34e204:	xor    %esi,%esi
  34e206:	call   3c5ad0 <<alloc::vec::Vec<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<std::sync::poison::mutex::Mutex<std::collections::hash::map::HashMap<alloc::string::String, alloc::vec::Vec<u32>, core::hash::BuildHasherDefault<snaptokens::models::bpe::FxStrHasher>>>, core::iter::adapters::map::Map<core::ops::range::Range<usize>, <snaptokens::models::bpe::SharedCache>::new::{closure#0}>>>::from_iter>
  34e20b:	movups 0x260(%rsp),%xmm0
  34e213:	movdqu 0x270(%rsp),%xmm1
  34e21c:	movdqu 0x280(%rsp),%xmm2
  34e225:	movdqa %xmm2,0x3c0(%rsp)
  34e22e:	movdqa %xmm1,0x3b0(%rsp)
  34e237:	mov    0x240(%rsp),%rax
  34e23f:	cmp    $0x1,%rax
  34e243:	mov    %rax,0x200(%rsp)
  34e24b:	adc    $0xffffffffffffffff,%rax
  34e24f:	movaps %xmm0,0x3a0(%rsp)
  34e257:	movups 0x230(%rsp),%xmm0
  34e25f:	movaps %xmm0,0x1f0(%rsp)
  34e267:	movdqu 0x248(%rsp),%xmm0
  34e270:	movdqu %xmm0,0x208(%rsp)
  34e279:	mov    0x258(%rsp),%rcx
  34e281:	mov    %rcx,0x218(%rsp)
  34e289:	mov    %rax,0x220(%rsp)
  34e291:	mov    0x2f0(%rsp),%rax
  34e299:	mov    %rax,0xe8(%rsp)
  34e2a1:	mov    0x2f8(%rsp),%rax
  34e2a9:	mov    %rax,0x180(%rsp)
  34e2b1:	mov    0x300(%rsp),%r14
  34e2b9:	mov    0x1c8(%rsp),%rax
  34e2c1:	mov    %rax,0x130(%rsp)
  34e2c9:	mov    0xd0(%rsp),%rax
  34e2d1:	mov    %rax,0x138(%rsp)
  34e2d9:	mov    0x188(%rsp),%rax
  34e2e1:	mov    %rax,0x140(%rsp)
  34e2e9:	mov    0x1c0(%rsp),%rax
  34e2f1:	mov    %rax,0x148(%rsp)
  34e2f9:	mov    0x1e8(%rsp),%rax
  34e301:	mov    %rax,0x150(%rsp)
  34e309:	mov    0xb0(%rsp),%rcx
  34e311:	mov    %rcx,0x158(%rsp)
  34e319:	mov    0x1b8(%rsp),%rax
  34e321:	mov    %rax,0x160(%rsp)
  34e329:	mov    0xc8(%rsp),%rax
  34e331:	mov    %rax,0x168(%rsp)
  34e339:	mov    %rcx,0x170(%rsp)
  34e341:	mov    $0x8,%edi
  34e346:	mov    $0x2000,%esi
  34e34b:	call   353430 <alloc::boxed::box_new_uninit>
  34e350:	mov    %rax,%r12
  34e353:	mov    %rbx,(%rax)
  34e356:	movaps 0x2c0(%rsp),%xmm0
  34e35e:	movups %xmm0,0x8(%rax)
  34e362:	add    $0x18,%rax
  34e366:	lea    0xdd0(%rsp),%rsi
  34e36e:	mov    $0x1fe8,%edx
  34e373:	mov    %rax,%rdi
  34e376:	mov    0x2e5f23(%rip),%rbp        # 6342a0 <memcpy@GLIBC_2.14>
  34e37d:	call   *%rbp
  34e37f:	movups 0x290(%rsp),%xmm0
  34e387:	movups 0x2a0(%rsp),%xmm1
  34e38f:	movups 0x2b0(%rsp),%xmm2
  34e397:	mov    0x18(%rsp),%rbx
  34e39c:	movups %xmm2,0x1c8(%rbx)
  34e3a3:	movups %xmm1,0x1b8(%rbx)
  34e3aa:	movups %xmm0,0x1a8(%rbx)
  34e3b1:	mov    0x380(%rsp),%rax
  34e3b9:	mov    %rax,0x40(%rbx)
  34e3bd:	movups 0x370(%rsp),%xmm0
  34e3c5:	movups %xmm0,0x30(%rbx)
  34e3c9:	mov    0x398(%rsp),%rax
  34e3d1:	mov    %rax,0x58(%rbx)
  34e3d5:	movups 0x388(%rsp),%xmm0
  34e3dd:	movups %xmm0,0x48(%rbx)
  34e3e1:	movaps 0x3a0(%rsp),%xmm0
  34e3e9:	movaps 0x3b0(%rsp),%xmm1
  34e3f1:	movaps 0x3c0(%rsp),%xmm2
  34e3f9:	movups %xmm1,0x70(%rbx)
  34e3fd:	movups %xmm0,0x60(%rbx)
  34e401:	movups %xmm2,0x80(%rbx)
  34e408:	mov    0x338(%rsp),%rax
  34e410:	mov    %rax,0xc0(%rbx)
  34e417:	movups 0x308(%rsp),%xmm0
  34e41f:	movups 0x318(%rsp),%xmm1
  34e427:	movups 0x328(%rsp),%xmm2
  34e42f:	movups %xmm2,0xb0(%rbx)
  34e436:	movups %xmm1,0xa0(%rbx)
  34e43d:	movups %xmm0,0x90(%rbx)
  34e444:	lea    0x1f8(%rbx),%rdi
  34e44b:	lea    0x3d0(%rsp),%rsi
  34e453:	mov    $0x400,%edx
  34e458:	call   *%rbp
  34e45a:	lea    0x5f8(%rbx),%rdi
  34e461:	lea    0x9d0(%rsp),%rsi
  34e469:	mov    $0x400,%edx
  34e46e:	call   *%rbp
  34e470:	lea    0x9f8(%rbx),%rdi
  34e477:	lea    0x7d0(%rsp),%rsi
  34e47f:	mov    $0x200,%edx
  34e484:	call   *%rbp
  34e486:	mov    0x220(%rsp),%rax
  34e48e:	mov    %rax,0xf8(%rbx)
  34e495:	movaps 0x1f0(%rsp),%xmm0
  34e49d:	movaps 0x200(%rsp),%xmm1
  34e4a5:	movaps 0x210(%rsp),%xmm2
  34e4ad:	movups %xmm0,0xc8(%rbx)
  34e4b4:	movups %xmm1,0xd8(%rbx)
  34e4bb:	movups %xmm2,0xe8(%rbx)
  34e4c2:	mov    0x170(%rsp),%rax
  34e4ca:	mov    %rax,0x1a0(%rbx)
  34e4d1:	movdqu 0x130(%rsp),%xmm0
  34e4da:	movdqu 0x140(%rsp),%xmm1
  34e4e3:	movdqu 0x150(%rsp),%xmm2
  34e4ec:	movdqu 0x160(%rsp),%xmm3
  34e4f5:	movdqu %xmm3,0x190(%rbx)
  34e4fd:	movdqu %xmm2,0x180(%rbx)
  34e505:	movdqu %xmm1,0x170(%rbx)
  34e50d:	movdqu %xmm0,0x160(%rbx)
  34e515:	mov    0xe0(%rsp),%rax
  34e51d:	mov    %rax,(%rbx)
  34e520:	mov    0x98(%rsp),%rax
  34e528:	mov    %rax,0x8(%rbx)
  34e52c:	mov    0x8(%rsp),%rax
  34e531:	mov    %rax,0x10(%rbx)
  34e535:	mov    0x68(%rsp),%rax
  34e53a:	mov    %rax,0x18(%rbx)
  34e53e:	mov    0xc0(%rsp),%rax
  34e546:	mov    %rax,0x20(%rbx)
  34e54a:	mov    %r15,0x28(%rbx)
  34e54e:	mov    0xe8(%rsp),%rax
  34e556:	mov    %rax,0x100(%rbx)
  34e55d:	mov    0x180(%rsp),%rax
  34e565:	mov    %rax,0x108(%rbx)
  34e56c:	mov    %r14,0x110(%rbx)
  34e573:	mov    0x80(%rsp),%rax
  34e57b:	mov    %rax,0x118(%rbx)
  34e582:	mov    0xf0(%rsp),%rax
  34e58a:	mov    %rax,0x120(%rbx)
  34e591:	mov    0x1d8(%rsp),%rax
  34e599:	mov    %rax,0x128(%rbx)
  34e5a0:	mov    0x88(%rsp),%rax
  34e5a8:	mov    %rax,0x130(%rbx)
  34e5af:	mov    0xb8(%rsp),%rax
  34e5b7:	mov    %rax,0x138(%rbx)
  34e5be:	mov    0x1e0(%rsp),%rax
  34e5c6:	mov    %rax,0x140(%rbx)
  34e5cd:	mov    0xd8(%rsp),%rax
  34e5d5:	mov    %rax,0x148(%rbx)
  34e5dc:	mov    0x90(%rsp),%rax
  34e5e4:	mov    %rax,0x150(%rbx)
  34e5eb:	mov    0x178(%rsp),%rax
  34e5f3:	mov    %rax,0x158(%rbx)
  34e5fa:	mov    0x28(%rsp),%rax
  34e5ff:	mov    %rax,0x1d8(%rbx)
  34e606:	mov    0x20(%rsp),%rax
  34e60b:	mov    %rax,0x1e0(%rbx)
  34e612:	mov    %r12,0x1e8(%rbx)
  34e619:	mov    %r13,0x1f0(%rbx)
  34e620:	mov    0x40(%rsp),%eax
  34e624:	mov    %eax,0xbf8(%rbx)
  34e62a:	movzbl 0x17(%rsp),%eax
  34e62f:	mov    %al,0xbfc(%rbx)
  34e635:	movzbl 0x37(%rsp),%eax
  34e63a:	mov    %al,0xbfd(%rbx)
  34e640:	movzbl 0x16(%rsp),%eax
  34e645:	mov    %al,0xbfe(%rbx)
  34e64b:	mov    0x2d8(%rsp),%rsi
  34e653:	test   %rsi,%rsi
  34e656:	je     34e66e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x25ce>
  34e658:	mov    0x2e0(%rsp),%rdi
  34e660:	add    %rsi,%rsi
  34e663:	mov    $0x2,%edx
  34e668:	call   *0x2e5c02(%rip)        # 634270 <_DYNAMIC+0x238>
  34e66e:	cmpq   $0x0,0x50(%rsp)
  34e674:	setne  %al
  34e677:	test   %al,0x58(%rsp)
  34e67b:	je     34e692 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x25f2>
  34e67d:	mov    $0x1,%edx
  34e682:	mov    0x48(%rsp),%rdi
  34e687:	mov    0x50(%rsp),%rsi
  34e68c:	call   *0x2e5bde(%rip)        # 634270 <_DYNAMIC+0x238>
  34e692:	cmpq   $0x0,0x70(%rsp)
  34e698:	je     34e6b6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2616>
  34e69a:	mov    0x70(%rsp),%rsi
  34e69f:	shl    $0x2,%rsi
  34e6a3:	mov    $0x4,%edx
  34e6a8:	mov    0x198(%rsp),%rdi
  34e6b0:	call   *0x2e5bba(%rip)        # 634270 <_DYNAMIC+0x238>
  34e6b6:	cmpq   $0x0,0x78(%rsp)
  34e6bc:	je     34e6da <<snaptokens::models::bpe::Bpe>::from_native_tables+0x263a>
  34e6be:	mov    0x78(%rsp),%rsi
  34e6c3:	shl    $0x3,%rsi
  34e6c7:	mov    $0x8,%edx
  34e6cc:	mov    0x1a0(%rsp),%rdi
  34e6d4:	call   *0x2e5b96(%rip)        # 634270 <_DYNAMIC+0x238>
  34e6da:	cmpq   $0x0,0x60(%rsp)
  34e6e0:	je     34e6fe <<snaptokens::models::bpe::Bpe>::from_native_tables+0x265e>
  34e6e2:	mov    0x60(%rsp),%rsi
  34e6e7:	shl    $0x2,%rsi
  34e6eb:	mov    $0x4,%edx
  34e6f0:	mov    0x1a8(%rsp),%rdi
  34e6f8:	call   *0x2e5b72(%rip)        # 634270 <_DYNAMIC+0x238>
  34e6fe:	cmpq   $0x0,0x110(%rsp)
  34e707:	je     34e728 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2688>
  34e709:	mov    0x110(%rsp),%rsi
  34e711:	shl    $0x3,%rsi
  34e715:	mov    $0x8,%edx
  34e71a:	mov    0xa0(%rsp),%rdi
  34e722:	call   *0x2e5b48(%rip)        # 634270 <_DYNAMIC+0x238>
  34e728:	cmpq   $0x0,0x118(%rsp)
  34e731:	je     34e752 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x26b2>
  34e733:	mov    0x118(%rsp),%rsi
  34e73b:	shl    $0x3,%rsi
  34e73f:	mov    $0x8,%edx
  34e744:	mov    0xa8(%rsp),%rdi
  34e74c:	call   *0x2e5b1e(%rip)        # 634270 <_DYNAMIC+0x238>
  34e752:	cmpq   $0x0,0x120(%rsp)
  34e75b:	je     34e77c <<snaptokens::models::bpe::Bpe>::from_native_tables+0x26dc>
  34e75d:	mov    0x120(%rsp),%rsi
  34e765:	shl    $0x2,%rsi
  34e769:	mov    $0x4,%edx
  34e76e:	mov    0x1b0(%rsp),%rdi
  34e776:	call   *0x2e5af4(%rip)        # 634270 <_DYNAMIC+0x238>
  34e77c:	cmpq   $0x0,0x128(%rsp)
  34e785:	je     34e7a6 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2706>
  34e787:	mov    0x128(%rsp),%rsi
  34e78f:	shl    $0x2,%rsi
  34e793:	mov    $0x4,%edx
  34e798:	mov    0x100(%rsp),%rdi
  34e7a0:	call   *0x2e5aca(%rip)        # 634270 <_DYNAMIC+0x238>
  34e7a6:	cmpq   $0x0,0x38(%rsp)
  34e7ac:	mov    0x18(%rsp),%rbp
  34e7b1:	je     34c673 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5d3>
  34e7b7:	mov    0x38(%rsp),%rsi
  34e7bc:	shl    $0x2,%rsi
  34e7c0:	mov    $0x4,%edx
  34e7c5:	mov    0xf8(%rsp),%rdi
  34e7cd:	jmp    34c66d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x5cd>
  34e7d2:	mov    0x200(%rsp),%rax
  34e7da:	mov    0x18(%rsp),%rcx
  34e7df:	mov    %rax,0x18(%rcx)
  34e7e3:	movdqa 0x1f0(%rsp),%xmm0
  34e7ec:	movdqu %xmm0,0x8(%rcx)
  34e7f1:	movq   $0xffffffffffffffff,(%rcx)
  34e7f8:	xor    %r12d,%r12d
  34e7fb:	jmp    34d816 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1776>
  34e800:	lea    0x2a4951(%rip),%rdi        # 5f3158 <anon.2751bc418103d8dead16706da22f1d23.109.llvm.1329807758949723497+0x288>
  34e807:	call   *0x2e5d63(%rip)        # 634570 <_DYNAMIC+0x538>
  34e80d:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34e812:	lea    0x2a4927(%rip),%rcx        # 5f3140 <anon.2751bc418103d8dead16706da22f1d23.109.llvm.1329807758949723497+0x270>
  34e819:	mov    $0x80,%esi
  34e81e:	xor    %edi,%edi
  34e820:	mov    0x20(%rsp),%rdx
  34e825:	call   *0x2e5f55(%rip)        # 634780 <_DYNAMIC+0x748>
  34e82b:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34e830:	mov    $0x1,%r12b
  34e833:	mov    $0x1,%edi
  34e838:	mov    %rbx,%rsi
  34e83b:	call   *0x2e5a3f(%rip)        # 634280 <_DYNAMIC+0x248>
  34e841:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34e846:	lea    0x2a4923(%rip),%rdx        # 5f3170 <anon.2751bc418103d8dead16706da22f1d23.109.llvm.1329807758949723497+0x2a0>
  34e84d:	mov    0x20(%rsp),%rsi
  34e852:	call   *0x2e5ec8(%rip)        # 634720 <_DYNAMIC+0x6e8>
  34e858:	jmp    34d6c8 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1628>
  34e85d:	mov    %rax,%rbp
  34e860:	lea    0x130(%rsp),%rdi
  34e868:	call   33e330 <core::ptr::drop_glue::<snaptokens::models::bpe::MergeAdjacency>>
  34e86d:	cmpq   $0x0,0xd8(%rsp)
  34e876:	jne    34e91d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x287d>
  34e87c:	cmpq   $0x0,0x88(%rsp)
  34e885:	jne    34e94b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x28ab>
  34e88b:	cmpq   $0x0,0x80(%rsp)
  34e894:	jne    34e97d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x28dd>
  34e89a:	cmpq   $0x0,0xe8(%rsp)
  34e8a3:	je     34e8c4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2824>
  34e8a5:	mov    0xe8(%rsp),%rsi
  34e8ad:	shl    $0x3,%rsi
  34e8b1:	mov    $0x4,%edx
  34e8b6:	mov    0x180(%rsp),%rdi
  34e8be:	call   *0x2e59ac(%rip)        # 634270 <_DYNAMIC+0x238>
  34e8c4:	lea    0x1f0(%rsp),%rdi
  34e8cc:	call   33e390 <core::ptr::drop_glue::<snaptokens::models::bpe::RankedMergeMap>>
  34e8d1:	mov    0x20(%rsp),%rax
  34e8d6:	lea    0x0(,%rax,4),%rsi
  34e8de:	mov    $0x4,%edx
  34e8e3:	mov    0x28(%rsp),%rdi
  34e8e8:	call   *0x2e5982(%rip)        # 634270 <_DYNAMIC+0x238>
  34e8ee:	lea    0x308(%rsp),%rdi
  34e8f6:	call   33e2b0 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabLookup>>
  34e8fb:	lea    0x3a0(%rsp),%rdi
  34e903:	call   33e1a0 <core::ptr::drop_glue::<snaptokens::models::bpe::VocabArena>>
  34e908:	lea    0x388(%rsp),%rdi
  34e910:	call   33e1e0 <core::ptr::drop_glue::<snaptokens::models::bpe::SharedCache>>
  34e915:	xor    %r15d,%r15d
  34e918:	jmp    34e9ba <<snaptokens::models::bpe::Bpe>::from_native_tables+0x291a>
  34e91d:	mov    0xd8(%rsp),%rsi
  34e925:	shl    $0x5,%rsi
  34e929:	mov    $0x10,%edx
  34e92e:	mov    0x90(%rsp),%rdi
  34e936:	call   *0x2e5934(%rip)        # 634270 <_DYNAMIC+0x238>
  34e93c:	cmpq   $0x0,0x88(%rsp)
  34e945:	je     34e88b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x27eb>
  34e94b:	mov    0x88(%rsp),%rax
  34e953:	lea    0x0(,%rax,4),%rsi
  34e95b:	mov    $0x4,%edx
  34e960:	mov    0xb8(%rsp),%rdi
  34e968:	call   *0x2e5902(%rip)        # 634270 <_DYNAMIC+0x238>
  34e96e:	cmpq   $0x0,0x80(%rsp)
  34e977:	je     34e89a <<snaptokens::models::bpe::Bpe>::from_native_tables+0x27fa>
  34e97d:	mov    0x80(%rsp),%rax
  34e985:	lea    0x0(,%rax,8),%rsi
  34e98d:	mov    $0x8,%edx
  34e992:	mov    0xf0(%rsp),%rdi
  34e99a:	call   *0x2e58d0(%rip)        # 634270 <_DYNAMIC+0x238>
  34e9a0:	cmpq   $0x0,0xe8(%rsp)
  34e9a9:	jne    34e8a5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2805>
  34e9af:	jmp    34e8c4 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2824>
  34e9b4:	mov    %rax,%rbp
  34e9b7:	mov    $0x1,%r15b
  34e9ba:	lea    0x370(%rsp),%rdi
  34e9c2:	call   33e1e0 <core::ptr::drop_glue::<snaptokens::models::bpe::SharedCache>>
  34e9c7:	jmp    34e9d5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2935>
  34e9c9:	call   *0x2e5899(%rip)        # 634268 <_DYNAMIC+0x230>
  34e9cf:	mov    %rax,%rbp
  34e9d2:	mov    $0x1,%r15b
  34e9d5:	cmpq   $0x0,0x68(%rsp)
  34e9db:	je     34e9f5 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2955>
  34e9dd:	mov    $0x1,%edx
  34e9e2:	mov    0xc0(%rsp),%rdi
  34e9ea:	mov    0x68(%rsp),%rsi
  34e9ef:	call   *0x2e587b(%rip)        # 634270 <_DYNAMIC+0x238>
  34e9f5:	cmpq   $0x0,0xe0(%rsp)
  34e9fe:	je     34ea23 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2983>
  34ea00:	mov    0xe0(%rsp),%rax
  34ea08:	lea    0x0(,%rax,8),%rsi
  34ea10:	mov    $0x4,%edx
  34ea15:	mov    0x98(%rsp),%rdi
  34ea1d:	call   *0x2e584d(%rip)        # 634270 <_DYNAMIC+0x238>
  34ea23:	lea    0x290(%rsp),%rdi
  34ea2b:	call   33e3d0 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenMatcher>>
  34ea30:	xor    %r14d,%r14d
  34ea33:	xor    %ebx,%ebx
  34ea35:	test   %r15b,%r15b
  34ea38:	je     34eacd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2a2d>
  34ea3e:	cmpq   $0x0,0x80(%rsp)
  34ea47:	je     34ea68 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x29c8>
  34ea49:	mov    0x80(%rsp),%rsi
  34ea51:	shl    $0x3,%rsi
  34ea55:	mov    $0x8,%edx
  34ea5a:	mov    0xf0(%rsp),%rdi
  34ea62:	call   *0x2e5808(%rip)        # 634270 <_DYNAMIC+0x238>
  34ea68:	cmpq   $0x0,0x88(%rsp)
  34ea71:	je     34ea92 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x29f2>
  34ea73:	mov    0x88(%rsp),%rsi
  34ea7b:	shl    $0x2,%rsi
  34ea7f:	mov    $0x4,%edx
  34ea84:	mov    0xb8(%rsp),%rdi
  34ea8c:	call   *0x2e57de(%rip)        # 634270 <_DYNAMIC+0x238>
  34ea92:	xor    %r14d,%r14d
  34ea95:	jmp    34ea9d <<snaptokens::models::bpe::Bpe>::from_native_tables+0x29fd>
  34ea97:	mov    %rax,%rbp
  34ea9a:	mov    $0x1,%r14b
  34ea9d:	mov    0x2f0(%rsp),%rsi
  34eaa5:	mov    $0x1,%bl
  34eaa7:	test   %rsi,%rsi
  34eaaa:	je     34eacd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2a2d>
  34eaac:	mov    0x2f8(%rsp),%rdi
  34eab4:	shl    $0x3,%rsi
  34eab8:	mov    $0x4,%edx
  34eabd:	call   *0x2e57ad(%rip)        # 634270 <_DYNAMIC+0x238>
  34eac3:	jmp    34eacd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2a2d>
  34eac5:	mov    %rax,%rbp
  34eac8:	mov    $0x1,%bl
  34eaca:	mov    $0x1,%r14b
  34eacd:	mov    0x2d8(%rsp),%rsi
  34ead5:	test   %rsi,%rsi
  34ead8:	je     34eaf0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2a50>
  34eada:	add    %rsi,%rsi
  34eadd:	mov    $0x2,%edx
  34eae2:	mov    0x1d0(%rsp),%rdi
  34eaea:	call   *0x2e5780(%rip)        # 634270 <_DYNAMIC+0x238>
  34eaf0:	test   %r14b,%r14b
  34eaf3:	jne    34eb35 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2a95>
  34eaf5:	test   %bl,%bl
  34eaf7:	je     34eb23 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2a83>
  34eaf9:	mov    0x20(%rsp),%rax
  34eafe:	lea    0x0(,%rax,4),%rsi
  34eb06:	mov    $0x4,%edx
  34eb0b:	mov    0x28(%rsp),%rdi
  34eb10:	call   *0x2e575a(%rip)        # 634270 <_DYNAMIC+0x238>
  34eb16:	mov    $0x1,%r14b
  34eb19:	xor    %r15d,%r15d
  34eb1c:	xor    %ebx,%ebx
  34eb1e:	jmp    34da21 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1981>
  34eb23:	xor    %r15d,%r15d
  34eb26:	xor    %r14d,%r14d
  34eb29:	xor    %ebx,%ebx
  34eb2b:	jmp    34da21 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x1981>
  34eb30:	mov    %rax,%rbp
  34eb33:	mov    $0x1,%bl
  34eb35:	cmpq   $0x0,0x68(%rsp)
  34eb3b:	je     34eb55 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2ab5>
  34eb3d:	mov    $0x1,%edx
  34eb42:	mov    0xc0(%rsp),%rdi
  34eb4a:	mov    0x68(%rsp),%rsi
  34eb4f:	call   *0x2e571b(%rip)        # 634270 <_DYNAMIC+0x238>
  34eb55:	test   %bl,%bl
  34eb57:	je     34eb7b <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2adb>
  34eb59:	mov    0x20(%rsp),%rax
  34eb5e:	lea    0x0(,%rax,4),%rsi
  34eb66:	mov    $0x4,%edx
  34eb6b:	mov    0x28(%rsp),%rdi
  34eb70:	call   *0x2e56fa(%rip)        # 634270 <_DYNAMIC+0x238>
  34eb76:	mov    $0x1,%r14b
  34eb79:	jmp    34eb7e <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2ade>
  34eb7b:	xor    %r14d,%r14d
  34eb7e:	xor    %r15d,%r15d
  34eb81:	jmp    34ebc3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2b23>
  34eb83:	mov    %rax,%rbp
  34eb86:	jmp    34eba0 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2b00>
  34eb88:	mov    %rax,%rbp
  34eb8b:	jmp    34ebbd <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2b1d>
  34eb8d:	jmp    34eb8f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2aef>
  34eb8f:	mov    %rax,%rbp
  34eb92:	mov    $0x1,%r14b
  34eb95:	mov    $0x1,%r15b
  34eb98:	cmpq   $0x0,0x20(%rsp)
  34eb9e:	je     34ebc3 <<snaptokens::models::bpe::Bpe>::from_native_tables+0x2b23>
  34eba0:	mov    0x20(%rsp),%rax
  34eba5:	lea    0x0(,%rax,4),%rsi
  34ebad:	mov    $0x4,%edx
  34ebb2:	mov    0x28(%rsp),%rdi
  34ebb7:	call   *0x2e56b3(%rip)        # 634270 <_DYNAMIC+0x238>
  34ebbd:	mov    $0x1,%r14b
  34ebc0:	mov    $0x1,%r15b
  34ebc3:	lea    0x290(%rsp),%rdi
  34ebcb:	call   33e3d0 <core::ptr::drop_glue::<snaptokens::models::bpe::ExactTokenMatcher>>
  34ebd0:	jmp    34da1f <<snaptokens::models::bpe::Bpe>::from_native_tables+0x197f>
