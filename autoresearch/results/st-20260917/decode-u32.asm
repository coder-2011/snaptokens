00000000003cc3e0 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>>:
  3cc3e0:	55                   	push   %rbp
  3cc3e1:	41 57                	push   %r15
  3cc3e3:	41 56                	push   %r14
  3cc3e5:	41 55                	push   %r13
  3cc3e7:	41 54                	push   %r12
  3cc3e9:	53                   	push   %rbx
  3cc3ea:	48 83 ec 38          	sub    $0x38,%rsp
  3cc3ee:	48 8b 56 10          	mov    0x10(%rsi),%rdx
  3cc3f2:	b0 01                	mov    $0x1,%al
  3cc3f4:	48 83 fa f7          	cmp    $0xfffffffffffffff7,%rdx
  3cc3f8:	0f 87 e7 01 00 00    	ja     3cc5e5 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x205>
  3cc3fe:	49 89 f6             	mov    %rsi,%r14
  3cc401:	4c 8d 6a 08          	lea    0x8(%rdx),%r13
  3cc405:	4c 89 6e 10          	mov    %r13,0x10(%rsi)
  3cc409:	48 81 fa f8 ff ff 1f 	cmp    $0x1ffffff8,%rdx
  3cc410:	77 14                	ja     3cc426 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x46>
  3cc412:	4d 8b 7e 08          	mov    0x8(%r14),%r15
  3cc416:	49 83 ff 07          	cmp    $0x7,%r15
  3cc41a:	77 22                	ja     3cc43e <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x5e>
  3cc41c:	b9 08 00 00 00       	mov    $0x8,%ecx
  3cc421:	4c 29 f9             	sub    %r15,%rcx
  3cc424:	31 c0                	xor    %eax,%eax
  3cc426:	88 07                	mov    %al,(%rdi)
  3cc428:	48 89 4f 08          	mov    %rcx,0x8(%rdi)
  3cc42c:	48 89 f8             	mov    %rdi,%rax
  3cc42f:	48 83 c4 38          	add    $0x38,%rsp
  3cc433:	5b                   	pop    %rbx
  3cc434:	41 5c                	pop    %r12
  3cc436:	41 5d                	pop    %r13
  3cc438:	41 5e                	pop    %r14
  3cc43a:	41 5f                	pop    %r15
  3cc43c:	5d                   	pop    %rbp
  3cc43d:	c3                   	ret
  3cc43e:	49 8b 0e             	mov    (%r14),%rcx
  3cc441:	48 8d 41 08          	lea    0x8(%rcx),%rax
  3cc445:	49 8d 6f f8          	lea    -0x8(%r15),%rbp
  3cc449:	48 89 4c 24 30       	mov    %rcx,0x30(%rsp)
  3cc44e:	4c 8b 21             	mov    (%rcx),%r12
  3cc451:	49 89 06             	mov    %rax,(%r14)
  3cc454:	49 89 6e 08          	mov    %rbp,0x8(%r14)
  3cc458:	4c 89 e0             	mov    %r12,%rax
  3cc45b:	48 c1 e8 3e          	shr    $0x3e,%rax
  3cc45f:	75 1a                	jne    3cc47b <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x9b>
  3cc461:	4a 8d 1c a5 00 00 00 	lea    0x0(,%r12,4),%rbx
  3cc468:	00 
  3cc469:	49 01 dd             	add    %rbx,%r13
  3cc46c:	72 0d                	jb     3cc47b <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x9b>
  3cc46e:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc472:	49 81 fd 01 00 00 20 	cmp    $0x20000001,%r13
  3cc479:	72 05                	jb     3cc480 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0xa0>
  3cc47b:	c6 07 01             	movb   $0x1,(%rdi)
  3cc47e:	eb ac                	jmp    3cc42c <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4c>
  3cc480:	4c 89 e0             	mov    %r12,%rax
  3cc483:	48 c1 e8 3d          	shr    $0x3d,%rax
  3cc487:	74 0b                	je     3cc494 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0xb4>
  3cc489:	31 ff                	xor    %edi,%edi
  3cc48b:	48 89 de             	mov    %rbx,%rsi
  3cc48e:	ff 15 cc 62 26 00    	call   *0x2662cc(%rip)        # 632760 <_DYNAMIC+0x248>
  3cc494:	4d 85 e4             	test   %r12,%r12
  3cc497:	48 89 7c 24 20       	mov    %rdi,0x20(%rsp)
  3cc49c:	0f 84 00 01 00 00    	je     3cc5a2 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x1c2>
  3cc4a2:	ff 15 c8 62 26 00    	call   *0x2662c8(%rip)        # 632770 <_DYNAMIC+0x258>
  3cc4a8:	be 04 00 00 00       	mov    $0x4,%esi
  3cc4ad:	48 89 df             	mov    %rbx,%rdi
  3cc4b0:	ff 15 c2 62 26 00    	call   *0x2662c2(%rip)        # 632778 <_DYNAMIC+0x260>
  3cc4b6:	bf 04 00 00 00       	mov    $0x4,%edi
  3cc4bb:	48 85 c0             	test   %rax,%rax
  3cc4be:	74 cb                	je     3cc48b <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0xab>
  3cc4c0:	4c 89 64 24 08       	mov    %r12,0x8(%rsp)
  3cc4c5:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  3cc4ca:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
  3cc4d1:	00 00 
  3cc4d3:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc4d7:	48 83 fd 04          	cmp    $0x4,%rbp
  3cc4db:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  3cc4e0:	0f 82 7d 00 00 00    	jb     3cc563 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x183>
  3cc4e6:	4c 89 64 24 28       	mov    %r12,0x28(%rsp)
  3cc4eb:	49 83 c7 f4          	add    $0xfffffffffffffff4,%r15
  3cc4ef:	41 bc 01 00 00 00    	mov    $0x1,%r12d
  3cc4f5:	bb 0c 00 00 00       	mov    $0xc,%ebx
  3cc4fa:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
  3cc500:	49 8d 44 24 ff       	lea    -0x1(%r12),%rax
  3cc505:	48 8b 54 24 30       	mov    0x30(%rsp),%rdx
  3cc50a:	48 8d 0c 1a          	lea    (%rdx,%rbx,1),%rcx
  3cc50e:	42 8b 6c a2 04       	mov    0x4(%rdx,%r12,4),%ebp
  3cc513:	49 89 0e             	mov    %rcx,(%r14)
  3cc516:	4d 89 7e 08          	mov    %r15,0x8(%r14)
  3cc51a:	48 3b 44 24 08       	cmp    0x8(%rsp),%rax
  3cc51f:	75 0b                	jne    3cc52c <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x14c>
  3cc521:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
  3cc526:	ff 15 8c 77 26 00    	call   *0x26778c(%rip)        # 633cb8 <_DYNAMIC+0x17a0>
  3cc52c:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  3cc531:	42 89 6c a0 fc       	mov    %ebp,-0x4(%rax,%r12,4)
  3cc536:	4c 89 64 24 18       	mov    %r12,0x18(%rsp)
  3cc53b:	4c 39 64 24 28       	cmp    %r12,0x28(%rsp)
  3cc540:	74 7b                	je     3cc5bd <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x1dd>
  3cc542:	89 e8                	mov    %ebp,%eax
  3cc544:	49 ff c4             	inc    %r12
  3cc547:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc54b:	48 83 c3 04          	add    $0x4,%rbx
  3cc54f:	49 83 c7 fc          	add    $0xfffffffffffffffc,%r15
  3cc553:	4c 89 fd             	mov    %r15,%rbp
  3cc556:	48 83 c5 04          	add    $0x4,%rbp
  3cc55a:	73 a4                	jae    3cc500 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x120>
  3cc55c:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  3cc561:	89 c1                	mov    %eax,%ecx
  3cc563:	b8 04 00 00 00       	mov    $0x4,%eax
  3cc568:	48 29 e8             	sub    %rbp,%rax
  3cc56b:	48 89 47 08          	mov    %rax,0x8(%rdi)
  3cc56f:	c6 07 00             	movb   $0x0,(%rdi)
  3cc572:	89 4f 04             	mov    %ecx,0x4(%rdi)
  3cc575:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
  3cc57a:	48 85 f6             	test   %rsi,%rsi
  3cc57d:	0f 84 a9 fe ff ff    	je     3cc42c <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4c>
  3cc583:	48 89 fb             	mov    %rdi,%rbx
  3cc586:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
  3cc58b:	48 c1 e6 02          	shl    $0x2,%rsi
  3cc58f:	ba 04 00 00 00       	mov    $0x4,%edx
  3cc594:	ff 15 b6 61 26 00    	call   *0x2661b6(%rip)        # 632750 <_DYNAMIC+0x238>
  3cc59a:	48 89 df             	mov    %rbx,%rdi
  3cc59d:	e9 8a fe ff ff       	jmp    3cc42c <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4c>
  3cc5a2:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
  3cc5a9:	00 00 
  3cc5ab:	48 c7 44 24 10 04 00 	movq   $0x4,0x10(%rsp)
  3cc5b2:	00 00 
  3cc5b4:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
  3cc5bb:	00 00 
  3cc5bd:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  3cc5c2:	48 8b 7c 24 20       	mov    0x20(%rsp),%rdi
  3cc5c7:	48 89 47 18          	mov    %rax,0x18(%rdi)
  3cc5cb:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  3cc5d0:	48 89 47 08          	mov    %rax,0x8(%rdi)
  3cc5d4:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  3cc5d9:	48 89 47 10          	mov    %rax,0x10(%rdi)
  3cc5dd:	c6 07 ff             	movb   $0xff,(%rdi)
  3cc5e0:	e9 47 fe ff ff       	jmp    3cc42c <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4c>
  3cc5e5:	e9 3c fe ff ff       	jmp    3cc426 <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x46>
  3cc5ea:	48 89 c3             	mov    %rax,%rbx
  3cc5ed:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
  3cc5f2:	48 85 f6             	test   %rsi,%rsi
  3cc5f5:	74 14                	je     3cc60b <<alloc::vec::Vec<u32> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x22b>
  3cc5f7:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
  3cc5fc:	48 c1 e6 02          	shl    $0x2,%rsi
  3cc600:	ba 04 00 00 00       	mov    $0x4,%edx
  3cc605:	ff 15 45 61 26 00    	call   *0x266145(%rip)        # 632750 <_DYNAMIC+0x238>
  3cc60b:	48 89 df             	mov    %rbx,%rdi
  3cc60e:	e8 fd 8a 1f 00       	call   5c5110 <_Unwind_Resume@plt>
  3cc613:	cc                   	int3
  3cc614:	cc                   	int3
  3cc615:	cc                   	int3
  3cc616:	cc                   	int3
  3cc617:	cc                   	int3
  3cc618:	cc                   	int3
  3cc619:	cc                   	int3
  3cc61a:	cc                   	int3
  3cc61b:	cc                   	int3
  3cc61c:	cc                   	int3
  3cc61d:	cc                   	int3
  3cc61e:	cc                   	int3
  3cc61f:	cc                   	int3
