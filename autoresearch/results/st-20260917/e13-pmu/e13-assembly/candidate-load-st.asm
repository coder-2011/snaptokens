
/home/namanchetwani/st-campaign-20260917/e13-candidate-st-eval:     file format elf64-x86-64


Disassembly of section .text:

0000000000430e90 <snaptokens::st::load_st>:
  430e90:	push   %rbp
  430e91:	push   %r15
  430e93:	push   %r14
  430e95:	push   %r13
  430e97:	push   %r12
  430e99:	push   %rbx
  430e9a:	sub    $0x1000,%rsp
  430ea1:	movq   $0x0,(%rsp)
  430ea9:	sub    $0xa98,%rsp
  430eb0:	mov    %rcx,%r14
  430eb3:	mov    %rdx,%r15
  430eb6:	mov    %rsi,%r12
  430eb9:	mov    %rdi,%rbx
  430ebc:	lea    0xe90(%rsp),%rdi
  430ec4:	call   *0x205766(%rip)        # 636630 <_DYNAMIC+0x1a38>
  430eca:	cmpl   $0x2,0xe90(%rsp)
  430ed2:	jne    430ef4 <snaptokens::st::load_st+0x64>
  430ed4:	mov    0xe98(%rsp),%rax
  430edc:	movq   $0x0,0x8(%rbx)
  430ee4:	mov    %rax,0x10(%rbx)
  430ee8:	movq   $0x2,(%rbx)
  430eef:	jmp    4311bf <snaptokens::st::load_st+0x32f>
  430ef4:	cmpq   $0x20000000,0xee0(%rsp)
  430f00:	jbe    430f60 <snaptokens::st::load_st+0xd0>
  430f02:	call   *0x203f48(%rip)        # 634e50 <_DYNAMIC+0x258>
  430f08:	mov    $0x1e,%edi
  430f0d:	mov    $0x1,%esi
  430f12:	call   *0x203f40(%rip)        # 634e58 <_DYNAMIC+0x260>
  430f18:	test   %rax,%rax
  430f1b:	je     431ce2 <snaptokens::st::load_st+0xe52>
  430f21:	movups -0x2d9dfa(%rip),%xmm0        # 15712e <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x28e>
  430f28:	movups %xmm0,0xe(%rax)
  430f2c:	movdqu -0x2d9e14(%rip),%xmm0        # 157120 <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x280>
  430f34:	movdqu %xmm0,(%rax)
  430f38:	movq   $0x3,0x8(%rbx)
  430f40:	movq   $0x1e,0x10(%rbx)
  430f48:	mov    %rax,0x18(%rbx)
  430f4c:	movq   $0x1e,0x20(%rbx)
  430f54:	movq   $0x2,(%rbx)
  430f5b:	jmp    4311bf <snaptokens::st::load_st+0x32f>
  430f60:	lea    0xe90(%rsp),%rdi
  430f68:	mov    %r12,%rsi
  430f6b:	mov    %r15,%rdx
  430f6e:	call   *0x204824(%rip)        # 635798 <_DYNAMIC+0xba0>
  430f74:	mov    0xe90(%rsp),%r15
  430f7c:	mov    0xe98(%rsp),%r12
  430f84:	cmp    $0xffffffffffffffff,%r15
  430f88:	je     430fe7 <snaptokens::st::load_st+0x157>
  430f8a:	mov    0xea0(%rsp),%rax
  430f92:	cmp    $0x53,%rax
  430f96:	ja     430fff <snaptokens::st::load_st+0x16f>
  430f98:	call   *0x203eb2(%rip)        # 634e50 <_DYNAMIC+0x258>
  430f9e:	mov    $0x1f,%r14d
  430fa4:	mov    $0x1f,%edi
  430fa9:	mov    $0x1,%esi
  430fae:	call   *0x203ea4(%rip)        # 634e58 <_DYNAMIC+0x260>
  430fb4:	test   %rax,%rax
  430fb7:	je     431cf2 <snaptokens::st::load_st+0xe62>
  430fbd:	movups -0x2d9eb9(%rip),%xmm0        # 15710b <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x26b>
  430fc4:	movups %xmm0,0xf(%rax)
  430fc8:	movups -0x2d9ed3(%rip),%xmm0        # 1570fc <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x25c>
  430fcf:	movups %xmm0,(%rax)
  430fd2:	mov    $0x3,%ecx
  430fd7:	mov    $0x1f,%r14d
  430fdd:	mov    $0x1f,%edx
  430fe2:	jmp    43117e <snaptokens::st::load_st+0x2ee>
  430fe7:	movq   $0x0,0x8(%rbx)
  430fef:	mov    %r12,0x10(%rbx)
  430ff3:	movq   $0x2,(%rbx)
  430ffa:	jmp    4311bf <snaptokens::st::load_st+0x32f>
  430fff:	movabs $0x545350414e53,%rcx
  431009:	cmp    %rcx,(%r12)
  43100d:	jne    4310e5 <snaptokens::st::load_st+0x255>
  431013:	mov    0x8(%r12),%r13d
  431018:	lea    -0x1(%r13),%ecx
  43101c:	cmp    $0x2,%ecx
  43101f:	jae    431134 <snaptokens::st::load_st+0x2a4>
  431025:	mov    0xc(%r12),%rbp
  43102a:	add    $0xffffffffffffffac,%rax
  43102e:	cmp    %rax,%rbp
  431031:	jne    4311d4 <snaptokens::st::load_st+0x344>
  431037:	cmpb   $0x0,(%r14)
  43103b:	je     431212 <snaptokens::st::load_st+0x382>
  431041:	movdqu 0x1(%r14),%xmm0
  431047:	movdqu 0x11(%r14),%xmm1
  43104d:	movdqa %xmm1,0xea0(%rsp)
  431056:	movdqa %xmm0,0xe90(%rsp)
  43105f:	movdqu 0x14(%r12),%xmm2
  431066:	movdqu 0x24(%r12),%xmm3
  43106d:	pcmpeqb %xmm1,%xmm3
  431071:	pcmpeqb %xmm0,%xmm2
  431075:	pand   %xmm3,%xmm2
  431079:	pmovmskb %xmm2,%eax
  43107d:	cmp    $0xffff,%eax
  431082:	je     431212 <snaptokens::st::load_st+0x382>
  431088:	call   *0x203dc2(%rip)        # 634e50 <_DYNAMIC+0x258>
  43108e:	mov    $0x26,%r14d
  431094:	mov    $0x26,%edi
  431099:	mov    $0x1,%esi
  43109e:	call   *0x203db4(%rip)        # 634e58 <_DYNAMIC+0x260>
  4310a4:	test   %rax,%rax
  4310a7:	je     431cf2 <snaptokens::st::load_st+0xe62>
  4310ad:	movups -0x2da016(%rip),%xmm0        # 15709e <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x1fe>
  4310b4:	movups %xmm0,0x10(%rax)
  4310b8:	movups -0x2da031(%rip),%xmm0        # 15708e <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x1ee>
  4310bf:	movups %xmm0,(%rax)
  4310c2:	movabs $0x4e4f534a20656372,%rcx
  4310cc:	mov    %rcx,0x1e(%rax)
  4310d0:	mov    $0x3,%ecx
  4310d5:	mov    $0x26,%r14d
  4310db:	mov    $0x26,%edx
  4310e0:	jmp    43117e <snaptokens::st::load_st+0x2ee>
  4310e5:	call   *0x203d65(%rip)        # 634e50 <_DYNAMIC+0x258>
  4310eb:	mov    $0x17,%r14d
  4310f1:	mov    $0x17,%edi
  4310f6:	mov    $0x1,%esi
  4310fb:	call   *0x203d57(%rip)        # 634e58 <_DYNAMIC+0x260>
  431101:	test   %rax,%rax
  431104:	je     431cf2 <snaptokens::st::load_st+0xe62>
  43110a:	movups -0x2da02c(%rip),%xmm0        # 1570e5 <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x245>
  431111:	movups %xmm0,(%rax)
  431114:	movabs $0x636967616d20656c,%rcx
  43111e:	mov    %rcx,0xf(%rax)
  431122:	mov    $0x3,%ecx
  431127:	mov    $0x17,%r14d
  43112d:	mov    $0x17,%edx
  431132:	jmp    43117e <snaptokens::st::load_st+0x2ee>
  431134:	call   *0x203d16(%rip)        # 634e50 <_DYNAMIC+0x258>
  43113a:	mov    $0x1a,%r14d
  431140:	mov    $0x1a,%edi
  431145:	mov    $0x1,%esi
  43114a:	call   *0x203d08(%rip)        # 634e58 <_DYNAMIC+0x260>
  431150:	test   %rax,%rax
  431153:	je     431cf2 <snaptokens::st::load_st+0xe62>
  431159:	movups -0x2da08b(%rip),%xmm0        # 1570d5 <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x235>
  431160:	movups %xmm0,0xa(%rax)
  431164:	movups -0x2da0a0(%rip),%xmm0        # 1570cb <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x22b>
  43116b:	movups %xmm0,(%rax)
  43116e:	mov    $0x3,%ecx
  431173:	mov    $0x1a,%r14d
  431179:	mov    $0x1a,%edx
  43117e:	movdqa 0x20(%rsp),%xmm0
  431184:	movdqa %xmm0,0x80(%rsp)
  43118d:	movdqu %xmm0,0x28(%rbx)
  431192:	mov    %rcx,0x8(%rbx)
  431196:	mov    %rdx,0x10(%rbx)
  43119a:	mov    %rax,0x18(%rbx)
  43119e:	mov    %r14,0x20(%rbx)
  4311a2:	movq   $0x2,(%rbx)
  4311a9:	test   %r15,%r15
  4311ac:	je     4311bf <snaptokens::st::load_st+0x32f>
  4311ae:	mov    $0x1,%edx
  4311b3:	mov    %r12,%rdi
  4311b6:	mov    %r15,%rsi
  4311b9:	call   *0x203c71(%rip)        # 634e30 <_DYNAMIC+0x238>
  4311bf:	mov    %rbx,%rax
  4311c2:	add    $0x1a98,%rsp
  4311c9:	pop    %rbx
  4311ca:	pop    %r12
  4311cc:	pop    %r13
  4311ce:	pop    %r14
  4311d0:	pop    %r15
  4311d2:	pop    %rbp
  4311d3:	ret
  4311d4:	call   *0x203c76(%rip)        # 634e50 <_DYNAMIC+0x258>
  4311da:	mov    $0x17,%r14d
  4311e0:	mov    $0x17,%edi
  4311e5:	mov    $0x1,%esi
  4311ea:	call   *0x203c68(%rip)        # 634e58 <_DYNAMIC+0x260>
  4311f0:	test   %rax,%rax
  4311f3:	je     431cf2 <snaptokens::st::load_st+0xe62>
  4311f9:	movups -0x2da14c(%rip),%xmm0        # 1570b4 <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x214>
  431200:	movups %xmm0,(%rax)
  431203:	movabs $0x686374616d73696d,%rcx
  43120d:	jmp    43111e <snaptokens::st::load_st+0x28e>
  431212:	lea    0x54(%r12),%r14
  431217:	lea    0xe90(%rsp),%rdi
  43121f:	mov    %r14,%rsi
  431222:	mov    %rbp,%rdx
  431225:	call   *0x2045b5(%rip)        # 6357e0 <_DYNAMIC+0xbe8>
  43122b:	movdqu 0xea0(%rsp),%xmm0
  431234:	movdqu 0x34(%r12),%xmm1
  43123b:	movdqu 0x44(%r12),%xmm2
  431242:	pcmpeqb 0xe90(%rsp),%xmm1
  43124b:	pcmpeqb %xmm0,%xmm2
  43124f:	pand   %xmm2,%xmm1
  431253:	pmovmskb %xmm1,%eax
  431257:	cmp    $0xffff,%eax
  43125c:	je     4312ad <snaptokens::st::load_st+0x41d>
  43125e:	call   *0x203bec(%rip)        # 634e50 <_DYNAMIC+0x258>
  431264:	mov    $0x19,%r14d
  43126a:	mov    $0x19,%edi
  43126f:	mov    $0x1,%esi
  431274:	call   *0x203bde(%rip)        # 634e58 <_DYNAMIC+0x260>
  43127a:	test   %rax,%rax
  43127d:	je     431cf2 <snaptokens::st::load_st+0xe62>
  431283:	movups -0x2da20c(%rip),%xmm0        # 15707e <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x1de>
  43128a:	movups %xmm0,0x9(%rax)
  43128e:	movups -0x2da220(%rip),%xmm0        # 157075 <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x1d5>
  431295:	movups %xmm0,(%rax)
  431298:	mov    $0x3,%ecx
  43129d:	mov    $0x19,%r14d
  4312a3:	mov    $0x19,%edx
  4312a8:	jmp    43117e <snaptokens::st::load_st+0x2ee>
  4312ad:	cmp    $0x1,%r13d
  4312b1:	jne    4312fe <snaptokens::st::load_st+0x46e>
  4312b3:	lea    0xe90(%rsp),%rdi
  4312bb:	mov    %r14,%rsi
  4312be:	mov    %rbp,%rdx
  4312c1:	call   417510 <bincode::decode_from_slice_with_context::<(), alloc::boxed::Box<snaptokens::st::PayloadV1>, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>>>
  4312c6:	cmpb   $0xff,0xe90(%rsp)
  4312ce:	je     4313e1 <snaptokens::st::load_st+0x551>
  4312d4:	mov    0xe90(%rsp),%rdx
  4312dc:	movups 0xe98(%rsp),%xmm0
  4312e4:	movaps %xmm0,0xb0(%rsp)
  4312ec:	mov    0xea8(%rsp),%rax
  4312f4:	mov    %rax,0xc0(%rsp)
  4312fc:	jmp    43136b <snaptokens::st::load_st+0x4db>
  4312fe:	lea    0xe90(%rsp),%rdi
  431306:	mov    %r14,%rsi
  431309:	mov    %rbp,%rdx
  43130c:	call   417590 <bincode::decode_from_slice_with_context::<(), snaptokens::st::PayloadV2, bincode::config::Configuration<bincode::config::LittleEndian, bincode::config::Fixint, bincode::config::Limit<536870912>>>>
  431311:	mov    0xe90(%rsp),%r13
  431319:	mov    0xe98(%rsp),%rdx
  431321:	movups 0xea0(%rsp),%xmm0
  431329:	movaps %xmm0,0xb0(%rsp)
  431331:	mov    0xeb0(%rsp),%rax
  431339:	mov    %rax,0xc0(%rsp)
  431341:	cmp    $0xffffffffffffffff,%r13
  431345:	je     43136b <snaptokens::st::load_st+0x4db>
  431347:	movdqu 0xeb8(%rsp),%xmm0
  431350:	movdqa %xmm0,0x40(%rsp)
  431356:	mov    0xec8(%rsp),%r14
  43135e:	mov    0xed0(%rsp),%rax
  431366:	jmp    4313f8 <snaptokens::st::load_st+0x568>
  43136b:	mov    0xc0(%rsp),%rax
  431373:	mov    %rax,0x2a8(%rsp)
  43137b:	movdqa 0xb0(%rsp),%xmm0
  431384:	movdqu %xmm0,0x298(%rsp)
  43138d:	mov    %rdx,0x290(%rsp)
  431395:	lea    0xe90(%rsp),%rdi
  43139d:	lea    0x290(%rsp),%rsi
  4313a5:	call   42e880 <snaptokens::st::decode_file::{closure#3}>
  4313aa:	mov    0xe90(%rsp),%rcx
  4313b2:	mov    0xe98(%rsp),%rdx
  4313ba:	mov    0xea0(%rsp),%rax
  4313c2:	mov    0xea8(%rsp),%r14
  4313ca:	movups 0xeb0(%rsp),%xmm0
  4313d2:	movaps %xmm0,0x40(%rsp)
  4313d7:	movaps %xmm0,0x20(%rsp)
  4313dc:	jmp    43117e <snaptokens::st::load_st+0x2ee>
  4313e1:	mov    0xe98(%rsp),%rdx
  4313e9:	mov    0xea0(%rsp),%rax
  4313f1:	mov    $0xffffffffffffffff,%r13
  4313f8:	mov    0xc0(%rsp),%rcx
  431400:	mov    %rcx,0xeb0(%rsp)
  431408:	movaps 0xb0(%rsp),%xmm0
  431410:	movups %xmm0,0xea0(%rsp)
  431418:	movdqa 0x40(%rsp),%xmm0
  43141e:	movdqu %xmm0,0xeb8(%rsp)
  431427:	mov    %r13,0xe90(%rsp)
  43142f:	mov    %rdx,0xe98(%rsp)
  431437:	mov    %r14,0xec8(%rsp)
  43143f:	cmp    %rbp,%rax
  431442:	jne    4317f0 <snaptokens::st::load_st+0x960>
  431448:	mov    0xea0(%rsp),%rbp
  431450:	mov    0xea8(%rsp),%rax
  431458:	mov    0xeb0(%rsp),%rcx
  431460:	mov    %rcx,0x18(%rsp)
  431465:	movaps 0x40(%rsp),%xmm0
  43146a:	movaps %xmm0,0xa0(%rsp)
  431472:	test   %r15,%r15
  431475:	mov    %rdx,0x8(%rsp)
  43147a:	je     431498 <snaptokens::st::load_st+0x608>
  43147c:	mov    $0x1,%edx
  431481:	mov    %r12,%rdi
  431484:	mov    %r15,%rsi
  431487:	mov    %rax,%r15
  43148a:	call   *0x2039a0(%rip)        # 634e30 <_DYNAMIC+0x238>
  431490:	mov    %r15,%rax
  431493:	mov    0x8(%rsp),%rdx
  431498:	cmp    $0xffffffffffffffff,%r13
  43149c:	je     43184f <snaptokens::st::load_st+0x9bf>
  4314a2:	mov    %r13,0x40(%rsp)
  4314a7:	mov    %rdx,0x48(%rsp)
  4314ac:	mov    %rbp,0x50(%rsp)
  4314b1:	mov    %rax,0x10(%rsp)
  4314b6:	mov    %rax,0x58(%rsp)
  4314bb:	mov    0x18(%rsp),%rax
  4314c0:	mov    %rax,0x60(%rsp)
  4314c5:	movdqa 0xa0(%rsp),%xmm0
  4314ce:	movdqu %xmm0,0x68(%rsp)
  4314d4:	mov    %r14,0x78(%rsp)
  4314d9:	mov    %rdx,0x290(%rsp)
  4314e1:	mov    %rbp,0x298(%rsp)
  4314e9:	movq   $0x0,0x2a0(%rsp)
  4314f5:	mov    $0x1,%r15b
  4314f8:	lea    0xe90(%rsp),%rdi
  431500:	lea    0x290(%rsp),%rsi
  431508:	call   3b2bb0 <serde_json::de::from_trait::<serde_json::read::SliceRead, snaptokens::st::TokenizerParts>>
  43150d:	lea    0x58(%rsp),%r15
  431512:	mov    0xe90(%rsp),%rbp
  43151a:	mov    0xe98(%rsp),%r12
  431522:	cmp    $0xffffffffffffffff,%rbp
  431526:	je     431b76 <snaptokens::st::load_st+0xce6>
  43152c:	lea    0xea0(%rsp),%rsi
  431534:	lea    0xc0(%rsp),%rdi
  43153c:	mov    $0x1d0,%edx
  431541:	call   *0x203919(%rip)        # 634e60 <memcpy@GLIBC_2.14>
  431547:	mov    %rbp,0xb0(%rsp)
  43154f:	mov    %r12,0xb8(%rsp)
  431557:	mov    0x10(%r15),%rax
  43155b:	mov    %rax,0xea0(%rsp)
  431563:	movdqu (%r15),%xmm0
  431568:	movdqa %xmm0,0xe90(%rsp)
  431571:	mov    0x70(%rsp),%edx
  431575:	mov    0x74(%rsp),%ecx
  431579:	and    $0x1,%r14d
  43157d:	lea    0x290(%rsp),%rdi
  431585:	lea    0xe90(%rsp),%rsi
  43158d:	mov    %r14d,%r8d
  431590:	call   *0x20525a(%rip)        # 6367f0 <_DYNAMIC+0x1bf8>
  431596:	mov    0x290(%rsp),%r14
  43159e:	cmp    $0xffffffffffffffff,%r14
  4315a2:	je     431bca <snaptokens::st::load_st+0xd3a>
  4315a8:	mov    0x298(%rsp),%r12
  4315b0:	movups 0x2a0(%rsp),%xmm0
  4315b8:	movaps %xmm0,0x20(%rsp)
  4315bd:	mov    0x2b0(%rsp),%rax
  4315c5:	mov    %rax,0x30(%rsp)
  4315ca:	movups 0x2b8(%rsp),%xmm0
  4315d2:	movups %xmm0,0xeb8(%rsp)
  4315da:	lea    0x2c8(%rsp),%rsi
  4315e2:	lea    0xec8(%rsp),%rdi
  4315ea:	mov    0x20386f(%rip),%r15        # 634e60 <memcpy@GLIBC_2.14>
  4315f1:	mov    $0x870,%edx
  4315f6:	call   *%r15
  4315f9:	mov    0x30(%rsp),%rax
  4315fe:	mov    %rax,0x90(%rsp)
  431606:	movdqa 0x20(%rsp),%xmm0
  43160c:	mov    %r14,0xe90(%rsp)
  431614:	mov    %r12,0xe98(%rsp)
  43161c:	movdqu %xmm0,0xea0(%rsp)
  431625:	mov    %rax,0xeb0(%rsp)
  43162d:	lea    0x290(%rsp),%rdi
  431635:	lea    0xb0(%rsp),%rsi
  43163d:	mov    $0x1e0,%edx
  431642:	call   *%r15
  431645:	call   *0x203805(%rip)        # 634e50 <_DYNAMIC+0x258>
  43164b:	mov    $0x8a8,%edi
  431650:	mov    $0x8,%esi
  431655:	call   *0x2037fd(%rip)        # 634e58 <_DYNAMIC+0x260>
  43165b:	test   %rax,%rax
  43165e:	je     431d02 <snaptokens::st::load_st+0xe72>
  431664:	mov    %rax,%r14
  431667:	lea    0xe90(%rsp),%rsi
  43166f:	mov    $0x8a8,%edx
  431674:	mov    %rax,%rdi
  431677:	call   *%r15
  43167a:	movups 0x3f8(%rsp),%xmm0
  431682:	movups %xmm0,0x178(%rbx)
  431689:	mov    0x408(%rsp),%rax
  431691:	mov    %rax,0x188(%rbx)
  431698:	movups 0x370(%rsp),%xmm0
  4316a0:	movups %xmm0,0xf0(%rbx)
  4316a7:	mov    0x380(%rsp),%rax
  4316af:	mov    %rax,0x100(%rbx)
  4316b6:	movups 0x388(%rsp),%xmm0
  4316be:	movups 0x398(%rsp),%xmm1
  4316c6:	movups 0x3a8(%rsp),%xmm2
  4316ce:	movups %xmm0,0x108(%rbx)
  4316d5:	movups %xmm1,0x118(%rbx)
  4316dc:	movups %xmm2,0x128(%rbx)
  4316e3:	mov    0x3b8(%rsp),%rax
  4316eb:	mov    %rax,0x138(%rbx)
  4316f2:	lea    0x2d8(%rsp),%rsi
  4316fa:	lea    0x58(%rbx),%rdi
  4316fe:	mov    $0x98,%edx
  431703:	call   *%r15
  431706:	movups 0x450(%rsp),%xmm0
  43170e:	movups %xmm0,0x1d0(%rbx)
  431715:	movups 0x410(%rsp),%xmm0
  43171d:	movups 0x420(%rsp),%xmm1
  431725:	movups 0x430(%rsp),%xmm2
  43172d:	movups 0x440(%rsp),%xmm3
  431735:	movups %xmm3,0x1c0(%rbx)
  43173c:	movups %xmm2,0x1b0(%rbx)
  431743:	movups %xmm1,0x1a0(%rbx)
  43174a:	movups %xmm0,0x190(%rbx)
  431751:	movups 0x460(%rsp),%xmm0
  431759:	movups %xmm0,0x1e0(%rbx)
  431760:	movups 0x3c0(%rsp),%xmm0
  431768:	movups %xmm0,0x140(%rbx)
  43176f:	mov    0x3f0(%rsp),%rax
  431777:	mov    %rax,0x170(%rbx)
  43177e:	movups 0x3d0(%rsp),%xmm0
  431786:	movups 0x3e0(%rsp),%xmm1
  43178e:	movups %xmm1,0x160(%rbx)
  431795:	movups %xmm0,0x150(%rbx)
  43179c:	movdqu 0x290(%rsp),%xmm0
  4317a5:	movdqu 0x2a0(%rsp),%xmm1
  4317ae:	movdqu 0x2b0(%rsp),%xmm2
  4317b7:	movdqu 0x2c0(%rsp),%xmm3
  4317c0:	movdqu %xmm0,0x10(%rbx)
  4317c5:	movdqu %xmm1,0x20(%rbx)
  4317ca:	movdqu %xmm2,0x30(%rbx)
  4317cf:	movdqu %xmm3,0x40(%rbx)
  4317d4:	mov    0x2d0(%rsp),%rax
  4317dc:	mov    %rax,0x50(%rbx)
  4317e0:	movq   $0x1,(%rbx)
  4317e7:	mov    %r14,0x8(%rbx)
  4317eb:	jmp    431c13 <snaptokens::st::load_st+0xd83>
  4317f0:	call   *0x20365a(%rip)        # 634e50 <_DYNAMIC+0x258>
  4317f6:	mov    $0x16,%r14d
  4317fc:	mov    $0x16,%edi
  431801:	mov    $0x1,%esi
  431806:	call   *0x20364c(%rip)        # 634e58 <_DYNAMIC+0x260>
  43180c:	test   %rax,%rax
  43180f:	je     431d26 <snaptokens::st::load_st+0xe96>
  431815:	movups -0x2da7bd(%rip),%xmm0        # 15705f <anon.b26b47fbf0e38f5bb26520cbd7c5f5c7.29.llvm.8852020054117370645+0x1bf>
  43181c:	movups %xmm0,(%rax)
  43181f:	movabs $0x7365747962206461,%rcx
  431829:	mov    %rcx,0xe(%rax)
  43182d:	lea    0xe90(%rsp),%rdi
  431835:	mov    %rax,%r13
  431838:	call   42aec0 <core::ptr::drop_glue::<snaptokens::st::Payload>>
  43183d:	mov    %r13,%rax
  431840:	mov    $0x3,%ecx
  431845:	mov    $0x16,%edx
  43184a:	jmp    43117e <snaptokens::st::load_st+0x2ee>
  43184f:	movdqu 0x8(%rdx),%xmm0
  431854:	movdqu %xmm0,0x290(%rsp)
  43185d:	movq   $0x0,0x2a0(%rsp)
  431869:	mov    $0x1,%bpl
  43186c:	lea    0xe90(%rsp),%rdi
  431874:	lea    0x290(%rsp),%rsi
  43187c:	call   3b2bb0 <serde_json::de::from_trait::<serde_json::read::SliceRead, snaptokens::st::TokenizerParts>>
  431881:	mov    0xe90(%rsp),%r12
  431889:	mov    0xe98(%rsp),%r14
  431891:	cmp    $0xffffffffffffffff,%r12
  431895:	je     431c2e <snaptokens::st::load_st+0xd9e>
  43189b:	lea    0xea0(%rsp),%rsi
  4318a3:	lea    0xc0(%rsp),%rdi
  4318ab:	mov    0x2035ae(%rip),%r15        # 634e60 <memcpy@GLIBC_2.14>
  4318b2:	mov    $0x1d0,%edx
  4318b7:	call   *%r15
  4318ba:	mov    %r12,0xb0(%rsp)
  4318c2:	mov    %r14,0xb8(%rsp)
  4318ca:	mov    0x8(%rsp),%rax
  4318cf:	lea    0x18(%rax),%rsi
  4318d3:	lea    0xe90(%rsp),%r14
  4318db:	mov    $0x1d8,%edx
  4318e0:	mov    %r14,%rdi
  4318e3:	call   *%r15
  4318e6:	lea    0x290(%rsp),%rdi
  4318ee:	mov    %r14,%rsi
  4318f1:	call   34c920 <<snaptokens::models::bpe::Bpe>::from_native_tables>
  4318f6:	mov    0x290(%rsp),%r14
  4318fe:	cmp    $0xffffffffffffffff,%r14
  431902:	je     431c72 <snaptokens::st::load_st+0xde2>
  431908:	mov    0x298(%rsp),%r12
  431910:	movups 0x2a0(%rsp),%xmm0
  431918:	movaps %xmm0,0x40(%rsp)
  43191d:	mov    0x2b0(%rsp),%rax
  431925:	mov    %rax,0x50(%rsp)
  43192a:	movups 0x2b8(%rsp),%xmm0
  431932:	movups %xmm0,0xeb8(%rsp)
  43193a:	lea    0x2c8(%rsp),%rsi
  431942:	lea    0xec8(%rsp),%rdi
  43194a:	mov    $0xbc8,%edx
  43194f:	call   *%r15
  431952:	mov    0x50(%rsp),%rax
  431957:	mov    %rax,0x30(%rsp)
  43195c:	movdqa 0x40(%rsp),%xmm0
  431962:	mov    %r14,0xe90(%rsp)
  43196a:	mov    %r12,0xe98(%rsp)
  431972:	movdqu %xmm0,0xea0(%rsp)
  43197b:	mov    %rax,0xeb0(%rsp)
  431983:	lea    0x290(%rsp),%rdi
  43198b:	lea    0xb0(%rsp),%rsi
  431993:	mov    $0x1e0,%edx
  431998:	call   *%r15
  43199b:	call   *0x2034af(%rip)        # 634e50 <_DYNAMIC+0x258>
  4319a1:	mov    $0xc00,%edi
  4319a6:	mov    $0x8,%esi
  4319ab:	call   *0x2034a7(%rip)        # 634e58 <_DYNAMIC+0x260>
  4319b1:	test   %rax,%rax
  4319b4:	je     431d14 <snaptokens::st::load_st+0xe84>
  4319ba:	mov    %rax,%r14
  4319bd:	lea    0xe90(%rsp),%rsi
  4319c5:	mov    $0xc00,%edx
  4319ca:	mov    %rax,%rdi
  4319cd:	call   *%r15
  4319d0:	movups 0x3f8(%rsp),%xmm0
  4319d8:	movups %xmm0,0x178(%rbx)
  4319df:	mov    0x408(%rsp),%rax
  4319e7:	mov    %rax,0x188(%rbx)
  4319ee:	movups 0x370(%rsp),%xmm0
  4319f6:	movups %xmm0,0xf0(%rbx)
  4319fd:	mov    0x380(%rsp),%rax
  431a05:	mov    %rax,0x100(%rbx)
  431a0c:	movups 0x388(%rsp),%xmm0
  431a14:	movups 0x398(%rsp),%xmm1
  431a1c:	movups 0x3a8(%rsp),%xmm2
  431a24:	movups %xmm0,0x108(%rbx)
  431a2b:	movups %xmm1,0x118(%rbx)
  431a32:	movups %xmm2,0x128(%rbx)
  431a39:	mov    0x3b8(%rsp),%rax
  431a41:	mov    %rax,0x138(%rbx)
  431a48:	lea    0x2d8(%rsp),%rsi
  431a50:	lea    0x58(%rbx),%rdi
  431a54:	mov    $0x98,%edx
  431a59:	call   *%r15
  431a5c:	movups 0x450(%rsp),%xmm0
  431a64:	movups %xmm0,0x1d0(%rbx)
  431a6b:	movups 0x410(%rsp),%xmm0
  431a73:	movups 0x420(%rsp),%xmm1
  431a7b:	movups 0x430(%rsp),%xmm2
  431a83:	movups 0x440(%rsp),%xmm3
  431a8b:	movups %xmm3,0x1c0(%rbx)
  431a92:	movups %xmm2,0x1b0(%rbx)
  431a99:	movups %xmm1,0x1a0(%rbx)
  431aa0:	movups %xmm0,0x190(%rbx)
  431aa7:	movups 0x460(%rsp),%xmm0
  431aaf:	movups %xmm0,0x1e0(%rbx)
  431ab6:	movups 0x3c0(%rsp),%xmm0
  431abe:	movups %xmm0,0x140(%rbx)
  431ac5:	mov    0x3f0(%rsp),%rax
  431acd:	mov    %rax,0x170(%rbx)
  431ad4:	movups 0x3d0(%rsp),%xmm0
  431adc:	movups 0x3e0(%rsp),%xmm1
  431ae4:	movups %xmm1,0x160(%rbx)
  431aeb:	movups %xmm0,0x150(%rbx)
  431af2:	movdqu 0x290(%rsp),%xmm0
  431afb:	movdqu 0x2a0(%rsp),%xmm1
  431b04:	movdqu 0x2b0(%rsp),%xmm2
  431b0d:	movdqu 0x2c0(%rsp),%xmm3
  431b16:	movdqu %xmm0,0x10(%rbx)
  431b1b:	movdqu %xmm1,0x20(%rbx)
  431b20:	movdqu %xmm2,0x30(%rbx)
  431b25:	movdqu %xmm3,0x40(%rbx)
  431b2a:	mov    0x2d0(%rsp),%rax
  431b32:	mov    %rax,0x50(%rbx)
  431b36:	movq   $0x0,(%rbx)
  431b3d:	mov    %r14,0x8(%rbx)
  431b41:	mov    0x8(%rsp),%rdi
  431b46:	mov    (%rdi),%rsi
  431b49:	test   %rsi,%rsi
  431b4c:	je     431b67 <snaptokens::st::load_st+0xcd7>
  431b4e:	mov    0x8(%rsp),%rax
  431b53:	mov    0x8(%rax),%rdi
  431b57:	mov    $0x1,%edx
  431b5c:	call   *0x2032ce(%rip)        # 634e30 <_DYNAMIC+0x238>
  431b62:	mov    0x8(%rsp),%rdi
  431b67:	mov    $0x1f0,%esi
  431b6c:	mov    $0x8,%edx
  431b71:	jmp    4311b9 <snaptokens::st::load_st+0x329>
  431b76:	movq   $0x1,0x8(%rbx)
  431b7e:	mov    %r12,0x10(%rbx)
  431b82:	movq   $0x2,(%rbx)
  431b89:	test   %r13,%r13
  431b8c:	je     431ba1 <snaptokens::st::load_st+0xd11>
  431b8e:	mov    $0x1,%edx
  431b93:	mov    0x8(%rsp),%rdi
  431b98:	mov    %r13,%rsi
  431b9b:	call   *0x20328f(%rip)        # 634e30 <_DYNAMIC+0x238>
  431ba1:	mov    %r15,%rdi
  431ba4:	call   3cb6f0 <<alloc::vec::Vec<(alloc::string::String, f64)> as core::ops::drop::Drop>::drop>
  431ba9:	mov    0x10(%rsp),%rsi
  431bae:	test   %rsi,%rsi
  431bb1:	je     4311bf <snaptokens::st::load_st+0x32f>
  431bb7:	shl    $0x5,%rsi
  431bbb:	mov    $0x8,%edx
  431bc0:	mov    0x18(%rsp),%rdi
  431bc5:	jmp    4311b9 <snaptokens::st::load_st+0x329>
  431bca:	lea    0x298(%rsp),%rax
  431bd2:	mov    0x10(%rax),%rcx
  431bd6:	movdqu (%rax),%xmm0
  431bda:	movdqa %xmm0,0x80(%rsp)
  431be3:	mov    %rcx,0x90(%rsp)
  431beb:	mov    %rcx,0x20(%rbx)
  431bef:	movdqu %xmm0,0x10(%rbx)
  431bf4:	movq   $0x9,0x8(%rbx)
  431bfc:	movq   $0x2,(%rbx)
  431c03:	xor    %r15d,%r15d
  431c06:	lea    0xb0(%rsp),%rdi
  431c0e:	call   42adb0 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  431c13:	test   %r13,%r13
  431c16:	je     4311bf <snaptokens::st::load_st+0x32f>
  431c1c:	mov    $0x1,%edx
  431c21:	mov    0x8(%rsp),%rdi
  431c26:	mov    %r13,%rsi
  431c29:	jmp    4311b9 <snaptokens::st::load_st+0x329>
  431c2e:	movq   $0x1,0x8(%rbx)
  431c36:	mov    %r14,0x10(%rbx)
  431c3a:	movq   $0x2,(%rbx)
  431c41:	mov    0x8(%rsp),%r14
  431c46:	mov    (%r14),%rsi
  431c49:	test   %rsi,%rsi
  431c4c:	je     431c67 <snaptokens::st::load_st+0xdd7>
  431c4e:	mov    0x8(%rsp),%rax
  431c53:	mov    0x8(%rax),%rdi
  431c57:	mov    $0x1,%edx
  431c5c:	call   *0x2031ce(%rip)        # 634e30 <_DYNAMIC+0x238>
  431c62:	mov    0x8(%rsp),%r14
  431c67:	lea    0x18(%r14),%rdi
  431c6b:	call   42b4f0 <core::ptr::drop_glue::<snaptokens::models::bpe::NativeBpeTables>>
  431c70:	jmp    431cd0 <snaptokens::st::load_st+0xe40>
  431c72:	lea    0x298(%rsp),%rax
  431c7a:	mov    0x10(%rax),%rcx
  431c7e:	movdqu (%rax),%xmm0
  431c82:	movdqa %xmm0,0x20(%rsp)
  431c88:	mov    %rcx,0x30(%rsp)
  431c8d:	mov    %rcx,0x20(%rbx)
  431c91:	movdqu %xmm0,0x10(%rbx)
  431c96:	movq   $0x9,0x8(%rbx)
  431c9e:	movq   $0x2,(%rbx)
  431ca5:	xor    %ebp,%ebp
  431ca7:	lea    0xb0(%rsp),%rdi
  431caf:	call   42adb0 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  431cb4:	mov    0x8(%rsp),%r14
  431cb9:	mov    (%r14),%rsi
  431cbc:	test   %rsi,%rsi
  431cbf:	je     431cd0 <snaptokens::st::load_st+0xe40>
  431cc1:	mov    0x8(%r14),%rdi
  431cc5:	mov    $0x1,%edx
  431cca:	call   *0x203160(%rip)        # 634e30 <_DYNAMIC+0x238>
  431cd0:	mov    $0x1f0,%esi
  431cd5:	mov    $0x8,%edx
  431cda:	mov    %r14,%rdi
  431cdd:	jmp    4311b9 <snaptokens::st::load_st+0x329>
  431ce2:	mov    $0x1,%edi
  431ce7:	mov    $0x1e,%esi
  431cec:	call   *0x20314e(%rip)        # 634e40 <_DYNAMIC+0x248>
  431cf2:	mov    $0x1,%edi
  431cf7:	mov    %r14,%rsi
  431cfa:	call   *0x203140(%rip)        # 634e40 <_DYNAMIC+0x248>
  431d00:	jmp    431d36 <snaptokens::st::load_st+0xea6>
  431d02:	mov    $0x8,%edi
  431d07:	mov    $0x8a8,%esi
  431d0c:	call   *0x20329e(%rip)        # 634fb0 <_DYNAMIC+0x3b8>
  431d12:	jmp    431d36 <snaptokens::st::load_st+0xea6>
  431d14:	mov    $0x8,%edi
  431d19:	mov    $0xc00,%esi
  431d1e:	call   *0x20328c(%rip)        # 634fb0 <_DYNAMIC+0x3b8>
  431d24:	jmp    431d36 <snaptokens::st::load_st+0xea6>
  431d26:	mov    $0x1,%edi
  431d2b:	mov    $0x16,%esi
  431d30:	call   *0x20310a(%rip)        # 634e40 <_DYNAMIC+0x248>
  431d36:	ud2
  431d38:	mov    %rax,%rbx
  431d3b:	cmpq   $0x0,0x10(%rsp)
  431d41:	je     431e80 <snaptokens::st::load_st+0xff0>
  431d47:	mov    0x10(%rsp),%rsi
  431d4c:	shl    $0x5,%rsi
  431d50:	mov    $0x8,%edx
  431d55:	mov    0x18(%rsp),%rdi
  431d5a:	jmp    431e7a <snaptokens::st::load_st+0xfea>
  431d5f:	mov    %rax,%rbx
  431d62:	lea    0xb0(%rsp),%rdi
  431d6a:	call   42adb0 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  431d6f:	jmp    431dc5 <snaptokens::st::load_st+0xf35>
  431d71:	mov    %rax,%rbx
  431d74:	lea    0xb0(%rsp),%rdi
  431d7c:	call   42adb0 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  431d81:	jmp    431e2f <snaptokens::st::load_st+0xf9f>
  431d86:	mov    %rax,%rbx
  431d89:	jmp    431dc7 <snaptokens::st::load_st+0xf37>
  431d8b:	mov    %rax,%rbx
  431d8e:	jmp    431e32 <snaptokens::st::load_st+0xfa2>
  431d93:	mov    %rax,%rbx
  431d96:	lea    0xe90(%rsp),%rdi
  431d9e:	call   42aec0 <core::ptr::drop_glue::<snaptokens::st::Payload>>
  431da3:	jmp    431e6a <snaptokens::st::load_st+0xfda>
  431da8:	mov    %rax,%rbx
  431dab:	lea    0xe90(%rsp),%rdi
  431db3:	call   42b760 <core::ptr::drop_glue::<snaptokens::models::bpe::Bpe>>
  431db8:	lea    0x290(%rsp),%rdi
  431dc0:	call   42adb0 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  431dc5:	xor    %ebp,%ebp
  431dc7:	mov    0x8(%rsp),%rax
  431dcc:	mov    (%rax),%rsi
  431dcf:	test   %rsi,%rsi
  431dd2:	je     431de8 <snaptokens::st::load_st+0xf58>
  431dd4:	mov    0x8(%rsp),%rax
  431dd9:	mov    0x8(%rax),%rdi
  431ddd:	mov    $0x1,%edx
  431de2:	call   *0x203048(%rip)        # 634e30 <_DYNAMIC+0x238>
  431de8:	test   %bpl,%bpl
  431deb:	je     431dfb <snaptokens::st::load_st+0xf6b>
  431ded:	mov    0x8(%rsp),%rax
  431df2:	lea    0x18(%rax),%rdi
  431df6:	call   42b4f0 <core::ptr::drop_glue::<snaptokens::models::bpe::NativeBpeTables>>
  431dfb:	mov    $0x1f0,%esi
  431e00:	mov    $0x8,%edx
  431e05:	mov    0x8(%rsp),%rdi
  431e0a:	jmp    431e7a <snaptokens::st::load_st+0xfea>
  431e0c:	call   *0x203016(%rip)        # 634e28 <_DYNAMIC+0x230>
  431e12:	mov    %rax,%rbx
  431e15:	lea    0xe90(%rsp),%rdi
  431e1d:	call   42bcd0 <core::ptr::drop_glue::<snaptokens::models::unigram::Unigram>>
  431e22:	lea    0x290(%rsp),%rdi
  431e2a:	call   42adb0 <core::ptr::drop_glue::<snaptokens::st::TokenizerParts>>
  431e2f:	xor    %r15d,%r15d
  431e32:	test   %r13,%r13
  431e35:	je     431e4a <snaptokens::st::load_st+0xfba>
  431e37:	mov    $0x1,%edx
  431e3c:	mov    0x8(%rsp),%rdi
  431e41:	mov    %r13,%rsi
  431e44:	call   *0x202fe6(%rip)        # 634e30 <_DYNAMIC+0x238>
  431e4a:	test   %r15b,%r15b
  431e4d:	je     431e80 <snaptokens::st::load_st+0xff0>
  431e4f:	lea    0x58(%rsp),%rdi
  431e54:	call   42bc50 <core::ptr::drop_glue::<snaptokens::models::unigram::UnigramSnapshot>>
  431e59:	mov    %rbx,%rdi
  431e5c:	call   5c7750 <_Unwind_Resume@plt>
  431e61:	call   *0x202fc1(%rip)        # 634e28 <_DYNAMIC+0x230>
  431e67:	mov    %rax,%rbx
  431e6a:	test   %r15,%r15
  431e6d:	je     431e80 <snaptokens::st::load_st+0xff0>
  431e6f:	mov    $0x1,%edx
  431e74:	mov    %r12,%rdi
  431e77:	mov    %r15,%rsi
  431e7a:	call   *0x202fb0(%rip)        # 634e30 <_DYNAMIC+0x238>
  431e80:	mov    %rbx,%rdi
  431e83:	call   5c7750 <_Unwind_Resume@plt>
