00000000003cc620 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>>:
  3cc620:	55                   	push   %rbp
  3cc621:	41 57                	push   %r15
  3cc623:	41 56                	push   %r14
  3cc625:	41 55                	push   %r13
  3cc627:	41 54                	push   %r12
  3cc629:	53                   	push   %rbx
  3cc62a:	48 83 ec 38          	sub    $0x38,%rsp
  3cc62e:	48 89 fb             	mov    %rdi,%rbx
  3cc631:	48 8b 56 10          	mov    0x10(%rsi),%rdx
  3cc635:	b0 01                	mov    $0x1,%al
  3cc637:	48 83 fa f7          	cmp    $0xfffffffffffffff7,%rdx
  3cc63b:	0f 87 0b 02 00 00    	ja     3cc84c <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x22c>
  3cc641:	49 89 f6             	mov    %rsi,%r14
  3cc644:	4c 8d 6a 08          	lea    0x8(%rdx),%r13
  3cc648:	4c 89 6e 10          	mov    %r13,0x10(%rsi)
  3cc64c:	48 81 fa f8 ff ff 1f 	cmp    $0x1ffffff8,%rdx
  3cc653:	77 14                	ja     3cc669 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x49>
  3cc655:	49 8b 6e 08          	mov    0x8(%r14),%rbp
  3cc659:	48 83 fd 07          	cmp    $0x7,%rbp
  3cc65d:	77 22                	ja     3cc681 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x61>
  3cc65f:	b9 08 00 00 00       	mov    $0x8,%ecx
  3cc664:	48 29 e9             	sub    %rbp,%rcx
  3cc667:	31 c0                	xor    %eax,%eax
  3cc669:	88 03                	mov    %al,(%rbx)
  3cc66b:	48 89 4b 08          	mov    %rcx,0x8(%rbx)
  3cc66f:	48 89 d8             	mov    %rbx,%rax
  3cc672:	48 83 c4 38          	add    $0x38,%rsp
  3cc676:	5b                   	pop    %rbx
  3cc677:	41 5c                	pop    %r12
  3cc679:	41 5d                	pop    %r13
  3cc67b:	41 5e                	pop    %r14
  3cc67d:	41 5f                	pop    %r15
  3cc67f:	5d                   	pop    %rbp
  3cc680:	c3                   	ret
  3cc681:	49 8b 0e             	mov    (%r14),%rcx
  3cc684:	48 8d 41 08          	lea    0x8(%rcx),%rax
  3cc688:	4c 8d 65 f8          	lea    -0x8(%rbp),%r12
  3cc68c:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
  3cc691:	48 8b 09             	mov    (%rcx),%rcx
  3cc694:	49 89 06             	mov    %rax,(%r14)
  3cc697:	4d 89 66 08          	mov    %r12,0x8(%r14)
  3cc69b:	48 89 c8             	mov    %rcx,%rax
  3cc69e:	48 c1 e8 3d          	shr    $0x3d,%rax
  3cc6a2:	75 1a                	jne    3cc6be <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x9e>
  3cc6a4:	4c 8d 3c cd 00 00 00 	lea    0x0(,%rcx,8),%r15
  3cc6ab:	00 
  3cc6ac:	4d 01 fd             	add    %r15,%r13
  3cc6af:	72 0d                	jb     3cc6be <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x9e>
  3cc6b1:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc6b5:	49 81 fd 01 00 00 20 	cmp    $0x20000001,%r13
  3cc6bc:	72 05                	jb     3cc6c3 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0xa3>
  3cc6be:	c6 03 01             	movb   $0x1,(%rbx)
  3cc6c1:	eb ac                	jmp    3cc66f <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4f>
  3cc6c3:	48 89 c8             	mov    %rcx,%rax
  3cc6c6:	48 c1 e8 3c          	shr    $0x3c,%rax
  3cc6ca:	74 0b                	je     3cc6d7 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0xb7>
  3cc6cc:	31 ff                	xor    %edi,%edi
  3cc6ce:	4c 89 fe             	mov    %r15,%rsi
  3cc6d1:	ff 15 89 60 26 00    	call   *0x266089(%rip)        # 632760 <_DYNAMIC+0x248>
  3cc6d7:	48 85 c9             	test   %rcx,%rcx
  3cc6da:	74 7b                	je     3cc757 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x137>
  3cc6dc:	48 89 4c 24 28       	mov    %rcx,0x28(%rsp)
  3cc6e1:	ff 15 89 60 26 00    	call   *0x266089(%rip)        # 632770 <_DYNAMIC+0x258>
  3cc6e7:	be 08 00 00 00       	mov    $0x8,%esi
  3cc6ec:	4c 89 ff             	mov    %r15,%rdi
  3cc6ef:	ff 15 83 60 26 00    	call   *0x266083(%rip)        # 632778 <_DYNAMIC+0x260>
  3cc6f5:	bf 08 00 00 00       	mov    $0x8,%edi
  3cc6fa:	48 85 c0             	test   %rax,%rax
  3cc6fd:	74 cf                	je     3cc6ce <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0xae>
  3cc6ff:	48 8b 74 24 28       	mov    0x28(%rsp),%rsi
  3cc704:	48 89 74 24 08       	mov    %rsi,0x8(%rsp)
  3cc709:	48 89 44 24 10       	mov    %rax,0x10(%rsp)
  3cc70e:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
  3cc715:	00 00 
  3cc717:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc71b:	49 83 fc 08          	cmp    $0x8,%r12
  3cc71f:	73 53                	jae    3cc774 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x154>
  3cc721:	b8 08 00 00 00       	mov    $0x8,%eax
  3cc726:	4c 29 e0             	sub    %r12,%rax
  3cc729:	c6 03 00             	movb   $0x0,(%rbx)
  3cc72c:	48 89 43 08          	mov    %rax,0x8(%rbx)
  3cc730:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
  3cc735:	48 85 f6             	test   %rsi,%rsi
  3cc738:	0f 84 31 ff ff ff    	je     3cc66f <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4f>
  3cc73e:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
  3cc743:	48 c1 e6 03          	shl    $0x3,%rsi
  3cc747:	ba 08 00 00 00       	mov    $0x8,%edx
  3cc74c:	ff 15 fe 5f 26 00    	call   *0x265ffe(%rip)        # 632750 <_DYNAMIC+0x238>
  3cc752:	e9 18 ff ff ff       	jmp    3cc66f <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4f>
  3cc757:	48 c7 44 24 08 00 00 	movq   $0x0,0x8(%rsp)
  3cc75e:	00 00 
  3cc760:	48 c7 44 24 10 08 00 	movq   $0x8,0x10(%rsp)
  3cc767:	00 00 
  3cc769:	48 c7 44 24 18 00 00 	movq   $0x0,0x18(%rsp)
  3cc770:	00 00 
  3cc772:	eb 2a                	jmp    3cc79e <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x17e>
  3cc774:	48 8b 54 24 20       	mov    0x20(%rsp),%rdx
  3cc779:	48 8d 4a 10          	lea    0x10(%rdx),%rcx
  3cc77d:	4c 8d 65 f0          	lea    -0x10(%rbp),%r12
  3cc781:	48 8b 52 08          	mov    0x8(%rdx),%rdx
  3cc785:	49 89 0e             	mov    %rcx,(%r14)
  3cc788:	4d 89 66 08          	mov    %r12,0x8(%r14)
  3cc78c:	48 89 10             	mov    %rdx,(%rax)
  3cc78f:	48 c7 44 24 18 01 00 	movq   $0x1,0x18(%rsp)
  3cc796:	00 00 
  3cc798:	48 83 fe 01          	cmp    $0x1,%rsi
  3cc79c:	75 23                	jne    3cc7c1 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x1a1>
  3cc79e:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  3cc7a3:	48 89 43 18          	mov    %rax,0x18(%rbx)
  3cc7a7:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  3cc7ac:	48 89 43 08          	mov    %rax,0x8(%rbx)
  3cc7b0:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  3cc7b5:	48 89 43 10          	mov    %rax,0x10(%rbx)
  3cc7b9:	c6 03 ff             	movb   $0xff,(%rbx)
  3cc7bc:	e9 ae fe ff ff       	jmp    3cc66f <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x4f>
  3cc7c1:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc7c5:	49 83 fc 08          	cmp    $0x8,%r12
  3cc7c9:	0f 82 52 ff ff ff    	jb     3cc721 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x101>
  3cc7cf:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  3cc7d4:	48 83 c0 18          	add    $0x18,%rax
  3cc7d8:	48 83 c5 e8          	add    $0xffffffffffffffe8,%rbp
  3cc7dc:	41 bf 02 00 00 00    	mov    $0x2,%r15d
  3cc7e2:	48 89 c2             	mov    %rax,%rdx
  3cc7e5:	49 8d 47 ff          	lea    -0x1(%r15),%rax
  3cc7e9:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
  3cc7ee:	4e 8b 24 f9          	mov    (%rcx,%r15,8),%r12
  3cc7f2:	48 89 54 24 30       	mov    %rdx,0x30(%rsp)
  3cc7f7:	49 89 16             	mov    %rdx,(%r14)
  3cc7fa:	49 89 6e 08          	mov    %rbp,0x8(%r14)
  3cc7fe:	48 3b 44 24 08       	cmp    0x8(%rsp),%rax
  3cc803:	75 0b                	jne    3cc810 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x1f0>
  3cc805:	48 8d 7c 24 08       	lea    0x8(%rsp),%rdi
  3cc80a:	ff 15 c8 74 26 00    	call   *0x2674c8(%rip)        # 633cd8 <_DYNAMIC+0x17c0>
  3cc810:	48 8b 44 24 10       	mov    0x10(%rsp),%rax
  3cc815:	4e 89 64 f8 f8       	mov    %r12,-0x8(%rax,%r15,8)
  3cc81a:	4c 89 7c 24 18       	mov    %r15,0x18(%rsp)
  3cc81f:	4c 39 7c 24 28       	cmp    %r15,0x28(%rsp)
  3cc824:	0f 84 74 ff ff ff    	je     3cc79e <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x17e>
  3cc82a:	49 ff c7             	inc    %r15
  3cc82d:	4d 89 6e 10          	mov    %r13,0x10(%r14)
  3cc831:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
  3cc836:	48 83 c0 08          	add    $0x8,%rax
  3cc83a:	48 83 c5 f8          	add    $0xfffffffffffffff8,%rbp
  3cc83e:	49 89 ec             	mov    %rbp,%r12
  3cc841:	49 83 c4 08          	add    $0x8,%r12
  3cc845:	73 9b                	jae    3cc7e2 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x1c2>
  3cc847:	e9 d5 fe ff ff       	jmp    3cc721 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x101>
  3cc84c:	e9 18 fe ff ff       	jmp    3cc669 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x49>
  3cc851:	48 89 c3             	mov    %rax,%rbx
  3cc854:	48 8b 74 24 08       	mov    0x8(%rsp),%rsi
  3cc859:	48 85 f6             	test   %rsi,%rsi
  3cc85c:	74 14                	je     3cc872 <<alloc::vec::Vec<u64> as bincode::de::Decode<()>>::decode::<bincode::de::decoder::DecoderImpl<bincode::de::read::SliceReader, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>, ()>>+0x252>
  3cc85e:	48 8b 7c 24 10       	mov    0x10(%rsp),%rdi
  3cc863:	48 c1 e6 03          	shl    $0x3,%rsi
  3cc867:	ba 08 00 00 00       	mov    $0x8,%edx
  3cc86c:	ff 15 de 5e 26 00    	call   *0x265ede(%rip)        # 632750 <_DYNAMIC+0x238>
  3cc872:	48 89 df             	mov    %rbx,%rdi
  3cc875:	e8 96 88 1f 00       	call   5c5110 <_Unwind_Resume@plt>
  3cc87a:	cc                   	int3
  3cc87b:	cc                   	int3
  3cc87c:	cc                   	int3
  3cc87d:	cc                   	int3
  3cc87e:	cc                   	int3
  3cc87f:	cc                   	int3
