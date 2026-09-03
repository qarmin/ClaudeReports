0000000000027890 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>>:
   27890:	55                   	push   %rbp
   27891:	48 89 e5             	mov    %rsp,%rbp
   27894:	41 57                	push   %r15
   27896:	41 56                	push   %r14
   27898:	41 55                	push   %r13
   2789a:	41 54                	push   %r12
   2789c:	53                   	push   %rbx
   2789d:	48 83 e4 e0          	and    $0xffffffffffffffe0,%rsp
   278a1:	48 81 ec a0 01 00 00 	sub    $0x1a0,%rsp
   278a8:	49 89 fa             	mov    %rdi,%r10
   278ab:	48 85 f6             	test   %rsi,%rsi
   278ae:	4c 89 4c 24 18       	mov    %r9,0x18(%rsp)
   278b3:	48 89 4c 24 20       	mov    %rcx,0x20(%rsp)
   278b8:	4c 89 44 24 30       	mov    %r8,0x30(%rsp)
   278bd:	48 89 54 24 28       	mov    %rdx,0x28(%rsp)
   278c2:	48 89 7c 24 78       	mov    %rdi,0x78(%rsp)
   278c7:	0f 84 69 03 00 00    	je     27c36 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x3a6>
   278cd:	49 8d 52 ff          	lea    -0x1(%r10),%rdx
   278d1:	49 8d 4a fe          	lea    -0x2(%r10),%rcx
   278d5:	48 d1 e9             	shr    $1,%rcx
   278d8:	41 83 e2 01          	and    $0x1,%r10d
   278dc:	48 89 4c 24 38       	mov    %rcx,0x38(%rsp)
   278e1:	49 01 ca             	add    %rcx,%r10
   278e4:	45 31 f6             	xor    %r14d,%r14d
   278e7:	48 89 54 24 40       	mov    %rdx,0x40(%rsp)
   278ec:	48 83 fa 02          	cmp    $0x2,%rdx
   278f0:	4d 0f 43 f2          	cmovae %r10,%r14
   278f4:	4c 89 74 24 48       	mov    %r14,0x48(%rsp)
   278f9:	49 c1 ee 03          	shr    $0x3,%r14
   278fd:	0f 84 7a 01 00 00    	je     27a7d <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x1ed>
   27903:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   27908:	48 8d 50 08          	lea    0x8(%rax),%rdx
   2790c:	48 8b 4c 24 20       	mov    0x20(%rsp),%rcx
   27911:	48 8d 71 08          	lea    0x8(%rcx),%rsi
   27915:	48 83 e1 f8          	and    $0xfffffffffffffff8,%rcx
   27919:	48 89 4c 24 58       	mov    %rcx,0x58(%rsp)
   2791e:	48 83 e6 f8          	and    $0xfffffffffffffff8,%rsi
   27922:	48 89 74 24 60       	mov    %rsi,0x60(%rsp)
   27927:	48 83 e0 f8          	and    $0xfffffffffffffff8,%rax
   2792b:	48 89 44 24 68       	mov    %rax,0x68(%rsp)
   27930:	48 83 e2 f8          	and    $0xfffffffffffffff8,%rdx
   27934:	48 89 54 24 70       	mov    %rdx,0x70(%rsp)
   27939:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
   2793e:	48 83 c0 04          	add    $0x4,%rax
   27942:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
   27947:	49 c7 c5 ff ff ff ff 	mov    $0xffffffffffffffff,%r13
   2794e:	45 31 e4             	xor    %r12d,%r12d
   27951:	66 66 66 66 66 66 2e 	data16 data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   27958:	0f 1f 84 00 00 00 00 
   2795f:	00 
   27960:	4c 39 64 24 70       	cmp    %r12,0x70(%rsp)
   27965:	0f 84 d9 06 00 00    	je     28044 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7b4>
   2796b:	4c 39 64 24 68       	cmp    %r12,0x68(%rsp)
   27970:	0f 84 e6 06 00 00    	je     2805c <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7cc>
   27976:	4c 39 64 24 60       	cmp    %r12,0x60(%rsp)
   2797b:	0f 84 e5 06 00 00    	je     28066 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7d6>
   27981:	4c 39 64 24 58       	cmp    %r12,0x58(%rsp)
   27986:	0f 84 f2 06 00 00    	je     2807e <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7ee>
   2798c:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   27991:	4a 8d 14 28          	lea    (%rax,%r13,1),%rdx
   27995:	48 83 fa 07          	cmp    $0x7,%rdx
   27999:	0f 86 fb 06 00 00    	jbe    2809a <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x80a>
   2799f:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   279a4:	4a 8d 1c a0          	lea    (%rax,%r12,4),%rbx
   279a8:	48 8b 44 24 50       	mov    0x50(%rsp),%rax
   279ad:	42 0f 10 04 a0       	movups (%rax,%r12,4),%xmm0
   279b2:	42 0f 10 4c a0 10    	movups 0x10(%rax,%r12,4),%xmm1
   279b8:	48 8d 94 24 40 01 00 	lea    0x140(%rsp),%rdx
   279bf:	00 
   279c0:	0f 11 4a 10          	movups %xmm1,0x10(%rdx)
   279c4:	0f 11 02             	movups %xmm0,(%rdx)
   279c7:	0f 10 03             	movups (%rbx),%xmm0
   279ca:	0f 10 4b 10          	movups 0x10(%rbx),%xmm1
   279ce:	0f 29 8c 24 10 01 00 	movaps %xmm1,0x110(%rsp)
   279d5:	00 
   279d6:	0f 29 84 24 00 01 00 	movaps %xmm0,0x100(%rsp)
   279dd:	00 
   279de:	42 0f 10 44 a0 fc    	movups -0x4(%rax,%r12,4),%xmm0
   279e4:	42 0f 10 4c a0 0c    	movups 0xc(%rax,%r12,4),%xmm1
   279ea:	48 8d b4 24 20 01 00 	lea    0x120(%rsp),%rsi
   279f1:	00 
   279f2:	0f 11 4e 10          	movups %xmm1,0x10(%rsi)
   279f6:	0f 11 06             	movups %xmm0,(%rsi)
   279f9:	48 8d bc 24 e0 00 00 	lea    0xe0(%rsp),%rdi
   27a00:	00 
   27a01:	e8 0a be 01 00       	call   43810 <<fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::add_f32x8::kernel::__fearless_simd_kernel>
   27a06:	4c 8d bc 24 60 01 00 	lea    0x160(%rsp),%r15
   27a0d:	00 
   27a0e:	4c 89 ff             	mov    %r15,%rdi
   27a11:	f3 0f 10 05 67 8f ff 	movss  -0x7099(%rip),%xmm0        # 20980 <.LCPI0_0>
   27a18:	ff 
   27a19:	e8 c2 bd 01 00       	call   437e0 <<fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::splat_f32x8::kernel::__fearless_simd_kernel>
   27a1e:	0f 10 03             	movups (%rbx),%xmm0
   27a21:	0f 10 4b 10          	movups 0x10(%rbx),%xmm1
   27a25:	0f 29 8c 24 d0 00 00 	movaps %xmm1,0xd0(%rsp)
   27a2c:	00 
   27a2d:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   27a34:	00 
   27a35:	48 8d bc 24 a0 00 00 	lea    0xa0(%rsp),%rdi
   27a3c:	00 
   27a3d:	48 8d b4 24 e0 00 00 	lea    0xe0(%rsp),%rsi
   27a44:	00 
   27a45:	4c 89 fa             	mov    %r15,%rdx
   27a48:	48 8d 8c 24 c0 00 00 	lea    0xc0(%rsp),%rcx
   27a4f:	00 
   27a50:	e8 9b bd 01 00       	call   437f0 <<fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::mul_add_f32x8::kernel::__fearless_simd_kernel>
   27a55:	0f 28 84 24 a0 00 00 	movaps 0xa0(%rsp),%xmm0
   27a5c:	00 
   27a5d:	0f 28 8c 24 b0 00 00 	movaps 0xb0(%rsp),%xmm1
   27a64:	00 
   27a65:	0f 11 4b 10          	movups %xmm1,0x10(%rbx)
   27a69:	0f 11 03             	movups %xmm0,(%rbx)
   27a6c:	49 83 c4 08          	add    $0x8,%r12
   27a70:	49 83 c5 f8          	add    $0xfffffffffffffff8,%r13
   27a74:	49 ff ce             	dec    %r14
   27a77:	0f 85 e3 fe ff ff    	jne    27960 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0xd0>
   27a7d:	4c 8b 74 24 48       	mov    0x48(%rsp),%r14
   27a82:	4c 89 f7             	mov    %r14,%rdi
   27a85:	48 83 e7 f8          	and    $0xfffffffffffffff8,%rdi
   27a89:	4c 39 f7             	cmp    %r14,%rdi
   27a8c:	48 8b 74 24 18       	mov    0x18(%rsp),%rsi
   27a91:	4c 8b 4c 24 20       	mov    0x20(%rsp),%r9
   27a96:	4c 8b 54 24 30       	mov    0x30(%rsp),%r10
   27a9b:	4c 8b 5c 24 28       	mov    0x28(%rsp),%r11
   27aa0:	48 8b 5c 24 78       	mov    0x78(%rsp),%rbx
   27aa5:	0f 84 28 01 00 00    	je     27bd3 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x343>
   27aab:	48 89 f8             	mov    %rdi,%rax
   27aae:	48 83 c8 01          	or     $0x1,%rax
   27ab2:	49 39 c1             	cmp    %rax,%r9
   27ab5:	48 89 c1             	mov    %rax,%rcx
   27ab8:	49 0f 47 c9          	cmova  %r9,%rcx
   27abc:	48 89 fa             	mov    %rdi,%rdx
   27abf:	48 f7 d2             	not    %rdx
   27ac2:	48 01 d1             	add    %rdx,%rcx
   27ac5:	49 39 c6             	cmp    %rax,%r14
   27ac8:	49 0f 47 c6          	cmova  %r14,%rax
   27acc:	48 01 d0             	add    %rdx,%rax
   27acf:	48 39 c1             	cmp    %rax,%rcx
   27ad2:	48 0f 42 c1          	cmovb  %rcx,%rax
   27ad6:	31 c9                	xor    %ecx,%ecx
   27ad8:	4c 89 ca             	mov    %r9,%rdx
   27adb:	48 29 fa             	sub    %rdi,%rdx
   27ade:	48 0f 42 d1          	cmovb  %rcx,%rdx
   27ae2:	48 39 d0             	cmp    %rdx,%rax
   27ae5:	48 0f 42 d0          	cmovb  %rax,%rdx
   27ae9:	48 89 f0             	mov    %rsi,%rax
   27aec:	48 29 f8             	sub    %rdi,%rax
   27aef:	48 0f 42 c1          	cmovb  %rcx,%rax
   27af3:	48 39 c2             	cmp    %rax,%rdx
   27af6:	48 0f 42 c2          	cmovb  %rdx,%rax
   27afa:	48 83 f8 07          	cmp    $0x7,%rax
   27afe:	0f 86 84 00 00 00    	jbe    27b88 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x2f8>
   27b04:	48 ff c0             	inc    %rax
   27b07:	89 c1                	mov    %eax,%ecx
   27b09:	83 e1 07             	and    $0x7,%ecx
   27b0c:	ba 08 00 00 00       	mov    $0x8,%edx
   27b11:	48 0f 45 d1          	cmovne %rcx,%rdx
   27b15:	48 29 d0             	sub    %rdx,%rax
   27b18:	48 01 c7             	add    %rax,%rdi
   27b1b:	4a 8d 14 b5 00 00 00 	lea    0x0(,%r14,4),%rdx
   27b22:	00 
   27b23:	48 83 e2 e0          	and    $0xffffffffffffffe0,%rdx
   27b27:	4a 8d 0c 1a          	lea    (%rdx,%r11,1),%rcx
   27b2b:	48 83 c1 14          	add    $0x14,%rcx
   27b2f:	4c 01 d2             	add    %r10,%rdx
   27b32:	48 83 c2 10          	add    $0x10,%rdx
   27b36:	45 31 c0             	xor    %r8d,%r8d
   27b39:	0f 28 05 b0 8b ff ff 	movaps -0x7450(%rip),%xmm0        # 206f0 <.LCPI0_1>
   27b40:	42 0f 10 4c 82 f0    	movups -0x10(%rdx,%r8,4),%xmm1
   27b46:	42 0f 10 14 82       	movups (%rdx,%r8,4),%xmm2
   27b4b:	42 0f 10 5c 81 ec    	movups -0x14(%rcx,%r8,4),%xmm3
   27b51:	42 0f 10 64 81 f0    	movups -0x10(%rcx,%r8,4),%xmm4
   27b57:	0f 58 e3             	addps  %xmm3,%xmm4
   27b5a:	42 0f 10 5c 81 fc    	movups -0x4(%rcx,%r8,4),%xmm3
   27b60:	42 0f 10 2c 81       	movups (%rcx,%r8,4),%xmm5
   27b65:	0f 58 eb             	addps  %xmm3,%xmm5
   27b68:	0f 59 e0             	mulps  %xmm0,%xmm4
   27b6b:	0f 5c cc             	subps  %xmm4,%xmm1
   27b6e:	0f 59 e8             	mulps  %xmm0,%xmm5
   27b71:	0f 5c d5             	subps  %xmm5,%xmm2
   27b74:	42 0f 11 4c 82 f0    	movups %xmm1,-0x10(%rdx,%r8,4)
   27b7a:	42 0f 11 14 82       	movups %xmm2,(%rdx,%r8,4)
   27b7f:	49 83 c0 08          	add    $0x8,%r8
   27b83:	4c 39 c0             	cmp    %r8,%rax
   27b86:	75 b8                	jne    27b40 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x2b0>
   27b88:	f3 0f 10 05 f0 8d ff 	movss  -0x7210(%rip),%xmm0        # 20980 <.LCPI0_0>
   27b8f:	ff 
   27b90:	48 39 f7             	cmp    %rsi,%rdi
   27b93:	0f 83 c5 05 00 00    	jae    2815e <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x8ce>
   27b99:	4c 39 cf             	cmp    %r9,%rdi
   27b9c:	0f 83 ac 05 00 00    	jae    2814e <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x8be>
   27ba2:	48 8d 47 01          	lea    0x1(%rdi),%rax
   27ba6:	4c 39 c8             	cmp    %r9,%rax
   27ba9:	0f 83 8c 05 00 00    	jae    2813b <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x8ab>
   27baf:	f3 41 0f 10 0c bb    	movss  (%r11,%rdi,4),%xmm1
   27bb5:	f3 41 0f 58 0c 83    	addss  (%r11,%rax,4),%xmm1
   27bbb:	f3 0f 59 c8          	mulss  %xmm0,%xmm1
   27bbf:	f3 41 0f 58 0c ba    	addss  (%r10,%rdi,4),%xmm1
   27bc5:	f3 41 0f 11 0c ba    	movss  %xmm1,(%r10,%rdi,4)
   27bcb:	48 89 c7             	mov    %rax,%rdi
   27bce:	4c 39 f0             	cmp    %r14,%rax
   27bd1:	72 bd                	jb     27b90 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x300>
   27bd3:	48 83 fb 02          	cmp    $0x2,%rbx
   27bd7:	0f 93 c0             	setae  %al
   27bda:	48 8b 4c 24 40       	mov    0x40(%rsp),%rcx
   27bdf:	84 c8                	test   %cl,%al
   27be1:	0f 84 4e 04 00 00    	je     28035 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7a5>
   27be7:	31 ff                	xor    %edi,%edi
   27be9:	48 83 f9 02          	cmp    $0x2,%rcx
   27bed:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
   27bf2:	48 0f 43 f8          	cmovae %rax,%rdi
   27bf6:	48 39 f0             	cmp    %rsi,%rax
   27bf9:	0f 83 cd 05 00 00    	jae    281cc <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x93c>
   27bff:	4c 39 cf             	cmp    %r9,%rdi
   27c02:	0f 83 d4 05 00 00    	jae    281dc <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x94c>
   27c08:	4c 39 c8             	cmp    %r9,%rax
   27c0b:	0f 83 db 05 00 00    	jae    281ec <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x95c>
   27c11:	f3 41 0f 10 04 bb    	movss  (%r11,%rdi,4),%xmm0
   27c17:	f3 41 0f 58 04 83    	addss  (%r11,%rax,4),%xmm0
   27c1d:	f3 0f 59 05 5b 8d ff 	mulss  -0x72a5(%rip),%xmm0        # 20980 <.LCPI0_0>
   27c24:	ff 
   27c25:	f3 41 0f 58 04 82    	addss  (%r10,%rax,4),%xmm0
   27c2b:	f3 41 0f 11 04 82    	movss  %xmm0,(%r10,%rax,4)
   27c31:	e9 ff 03 00 00       	jmp    28035 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7a5>
   27c36:	48 b8 fe ff ff ff ff 	movabs $0x7ffffffffffffffe,%rax
   27c3d:	ff ff 7f 
   27c40:	4c 01 d0             	add    %r10,%rax
   27c43:	31 ff                	xor    %edi,%edi
   27c45:	49 83 fa 02          	cmp    $0x2,%r10
   27c49:	48 0f 42 f8          	cmovb  %rax,%rdi
   27c4d:	48 85 c9             	test   %rcx,%rcx
   27c50:	0f 84 42 05 00 00    	je     28198 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x908>
   27c56:	4d 85 c9             	test   %r9,%r9
   27c59:	0f 84 4a 05 00 00    	je     281a9 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x919>
   27c5f:	4c 39 cf             	cmp    %r9,%rdi
   27c62:	0f 83 52 05 00 00    	jae    281ba <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x92a>
   27c68:	f3 41 0f 10 00       	movss  (%r8),%xmm0
   27c6d:	f3 0f 58 c0          	addss  %xmm0,%xmm0
   27c71:	f3 0f 59 05 07 8d ff 	mulss  -0x72f9(%rip),%xmm0        # 20980 <.LCPI0_0>
   27c78:	ff 
   27c79:	f3 0f 58 02          	addss  (%rdx),%xmm0
   27c7d:	f3 0f 11 02          	movss  %xmm0,(%rdx)
   27c81:	49 8d 52 ff          	lea    -0x1(%r10),%rdx
   27c85:	49 8d 42 fd          	lea    -0x3(%r10),%rax
   27c89:	48 89 c1             	mov    %rax,%rcx
   27c8c:	48 d1 e9             	shr    $1,%rcx
   27c8f:	48 89 8c 24 88 00 00 	mov    %rcx,0x88(%rsp)
   27c96:	00 
   27c97:	48 29 c8             	sub    %rcx,%rax
   27c9a:	31 c9                	xor    %ecx,%ecx
   27c9c:	48 89 94 24 98 00 00 	mov    %rdx,0x98(%rsp)
   27ca3:	00 
   27ca4:	48 83 fa 03          	cmp    $0x3,%rdx
   27ca8:	48 0f 43 c8          	cmovae %rax,%rcx
   27cac:	48 89 8c 24 90 00 00 	mov    %rcx,0x90(%rsp)
   27cb3:	00 
   27cb4:	48 c1 e9 03          	shr    $0x3,%rcx
   27cb8:	48 89 4c 24 70       	mov    %rcx,0x70(%rsp)
   27cbd:	0f 84 91 01 00 00    	je     27e54 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x5c4>
   27cc3:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
   27cc8:	48 8d 48 07          	lea    0x7(%rax),%rcx
   27ccc:	48 ff c8             	dec    %rax
   27ccf:	48 89 84 24 80 00 00 	mov    %rax,0x80(%rsp)
   27cd6:	00 
   27cd7:	48 c1 e8 03          	shr    $0x3,%rax
   27cdb:	48 89 44 24 60       	mov    %rax,0x60(%rsp)
   27ce0:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
   27ce5:	48 8d 50 08          	lea    0x8(%rax),%rdx
   27ce9:	48 8d 70 ff          	lea    -0x1(%rax),%rsi
   27ced:	48 c1 ee 03          	shr    $0x3,%rsi
   27cf1:	48 89 74 24 48       	mov    %rsi,0x48(%rsp)
   27cf6:	48 83 e0 f8          	and    $0xfffffffffffffff8,%rax
   27cfa:	48 89 44 24 50       	mov    %rax,0x50(%rsp)
   27cff:	48 83 e2 f8          	and    $0xfffffffffffffff8,%rdx
   27d03:	48 89 54 24 58       	mov    %rdx,0x58(%rsp)
   27d08:	48 83 e1 f8          	and    $0xfffffffffffffff8,%rcx
   27d0c:	48 89 4c 24 68       	mov    %rcx,0x68(%rsp)
   27d11:	48 8b 44 24 30       	mov    0x30(%rsp),%rax
   27d16:	48 83 c0 04          	add    $0x4,%rax
   27d1a:	48 89 44 24 38       	mov    %rax,0x38(%rsp)
   27d1f:	48 8b 44 24 28       	mov    0x28(%rsp),%rax
   27d24:	48 83 c0 04          	add    $0x4,%rax
   27d28:	48 89 44 24 40       	mov    %rax,0x40(%rsp)
   27d2d:	45 31 ed             	xor    %r13d,%r13d
   27d30:	45 31 ff             	xor    %r15d,%r15d
   27d33:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   27d3a:	84 00 00 00 00 00 
   27d40:	4c 39 6c 24 68       	cmp    %r13,0x68(%rsp)
   27d45:	0f 84 63 03 00 00    	je     280ae <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x81e>
   27d4b:	4c 39 7c 24 60       	cmp    %r15,0x60(%rsp)
   27d50:	0f 84 73 03 00 00    	je     280c9 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x839>
   27d56:	4c 39 6c 24 58       	cmp    %r13,0x58(%rsp)
   27d5b:	0f 84 87 03 00 00    	je     280e8 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x858>
   27d61:	4c 39 6c 24 50       	cmp    %r13,0x50(%rsp)
   27d66:	0f 84 94 03 00 00    	je     28100 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x870>
   27d6c:	4c 39 7c 24 48       	cmp    %r15,0x48(%rsp)
   27d71:	0f 84 a5 03 00 00    	je     2811c <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x88c>
   27d77:	49 ff c7             	inc    %r15
   27d7a:	48 8b 44 24 40       	mov    0x40(%rsp),%rax
   27d7f:	4a 8d 1c a8          	lea    (%rax,%r13,4),%rbx
   27d83:	48 8b 44 24 38       	mov    0x38(%rsp),%rax
   27d88:	42 0f 10 04 a8       	movups (%rax,%r13,4),%xmm0
   27d8d:	42 0f 10 4c a8 10    	movups 0x10(%rax,%r13,4),%xmm1
   27d93:	48 8d 94 24 40 01 00 	lea    0x140(%rsp),%rdx
   27d9a:	00 
   27d9b:	0f 11 4a 10          	movups %xmm1,0x10(%rdx)
   27d9f:	0f 11 02             	movups %xmm0,(%rdx)
   27da2:	0f 10 03             	movups (%rbx),%xmm0
   27da5:	0f 10 4b 10          	movups 0x10(%rbx),%xmm1
   27da9:	0f 29 8c 24 10 01 00 	movaps %xmm1,0x110(%rsp)
   27db0:	00 
   27db1:	0f 29 84 24 00 01 00 	movaps %xmm0,0x100(%rsp)
   27db8:	00 
   27db9:	42 0f 10 44 a8 fc    	movups -0x4(%rax,%r13,4),%xmm0
   27dbf:	42 0f 10 4c a8 0c    	movups 0xc(%rax,%r13,4),%xmm1
   27dc5:	48 8d b4 24 20 01 00 	lea    0x120(%rsp),%rsi
   27dcc:	00 
   27dcd:	0f 11 4e 10          	movups %xmm1,0x10(%rsi)
   27dd1:	0f 11 06             	movups %xmm0,(%rsi)
   27dd4:	4c 8d a4 24 e0 00 00 	lea    0xe0(%rsp),%r12
   27ddb:	00 
   27ddc:	4c 89 e7             	mov    %r12,%rdi
   27ddf:	e8 2c ba 01 00       	call   43810 <<fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::add_f32x8::kernel::__fearless_simd_kernel>
   27de4:	4c 8d b4 24 60 01 00 	lea    0x160(%rsp),%r14
   27deb:	00 
   27dec:	4c 89 f7             	mov    %r14,%rdi
   27def:	f3 0f 10 05 89 8b ff 	movss  -0x7477(%rip),%xmm0        # 20980 <.LCPI0_0>
   27df6:	ff 
   27df7:	e8 e4 b9 01 00       	call   437e0 <<fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::splat_f32x8::kernel::__fearless_simd_kernel>
   27dfc:	0f 10 03             	movups (%rbx),%xmm0
   27dff:	0f 10 4b 10          	movups 0x10(%rbx),%xmm1
   27e03:	0f 29 8c 24 d0 00 00 	movaps %xmm1,0xd0(%rsp)
   27e0a:	00 
   27e0b:	0f 29 84 24 c0 00 00 	movaps %xmm0,0xc0(%rsp)
   27e12:	00 
   27e13:	48 8d bc 24 a0 00 00 	lea    0xa0(%rsp),%rdi
   27e1a:	00 
   27e1b:	4c 89 e6             	mov    %r12,%rsi
   27e1e:	4c 89 f2             	mov    %r14,%rdx
   27e21:	48 8d 8c 24 c0 00 00 	lea    0xc0(%rsp),%rcx
   27e28:	00 
   27e29:	e8 c2 b9 01 00       	call   437f0 <<fearless_simd::generated::avx512::Avx512 as fearless_simd::generated::simd_trait::Simd>::mul_add_f32x8::kernel::__fearless_simd_kernel>
   27e2e:	0f 28 84 24 a0 00 00 	movaps 0xa0(%rsp),%xmm0
   27e35:	00 
   27e36:	0f 28 8c 24 b0 00 00 	movaps 0xb0(%rsp),%xmm1
   27e3d:	00 
   27e3e:	0f 11 4b 10          	movups %xmm1,0x10(%rbx)
   27e42:	0f 11 03             	movups %xmm0,(%rbx)
   27e45:	49 83 c5 08          	add    $0x8,%r13
   27e49:	4c 39 7c 24 70       	cmp    %r15,0x70(%rsp)
   27e4e:	0f 85 ec fe ff ff    	jne    27d40 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x4b0>
   27e54:	4c 8b bc 24 90 00 00 	mov    0x90(%rsp),%r15
   27e5b:	00 
   27e5c:	4c 89 ff             	mov    %r15,%rdi
   27e5f:	48 83 e7 f8          	and    $0xfffffffffffffff8,%rdi
   27e63:	4c 39 ff             	cmp    %r15,%rdi
   27e66:	48 8b 74 24 18       	mov    0x18(%rsp),%rsi
   27e6b:	4c 8b 4c 24 20       	mov    0x20(%rsp),%r9
   27e70:	4c 8b 54 24 30       	mov    0x30(%rsp),%r10
   27e75:	4c 8b 5c 24 28       	mov    0x28(%rsp),%r11
   27e7a:	48 8b 5c 24 78       	mov    0x78(%rsp),%rbx
   27e7f:	4c 8b b4 24 98 00 00 	mov    0x98(%rsp),%r14
   27e86:	00 
   27e87:	0f 84 38 01 00 00    	je     27fc5 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x735>
   27e8d:	48 89 f9             	mov    %rdi,%rcx
   27e90:	48 83 c9 01          	or     $0x1,%rcx
   27e94:	49 39 c9             	cmp    %rcx,%r9
   27e97:	48 89 c8             	mov    %rcx,%rax
   27e9a:	49 0f 47 c1          	cmova  %r9,%rax
   27e9e:	48 89 fa             	mov    %rdi,%rdx
   27ea1:	48 f7 d2             	not    %rdx
   27ea4:	48 01 d0             	add    %rdx,%rax
   27ea7:	48 39 ce             	cmp    %rcx,%rsi
   27eaa:	49 89 c8             	mov    %rcx,%r8
   27ead:	4c 0f 47 c6          	cmova  %rsi,%r8
   27eb1:	49 01 d0             	add    %rdx,%r8
   27eb4:	4c 39 c0             	cmp    %r8,%rax
   27eb7:	4c 0f 42 c0          	cmovb  %rax,%r8
   27ebb:	49 39 cf             	cmp    %rcx,%r15
   27ebe:	49 0f 47 cf          	cmova  %r15,%rcx
   27ec2:	48 01 d1             	add    %rdx,%rcx
   27ec5:	49 39 c8             	cmp    %rcx,%r8
   27ec8:	49 0f 42 c8          	cmovb  %r8,%rcx
   27ecc:	31 c0                	xor    %eax,%eax
   27ece:	48 89 f2             	mov    %rsi,%rdx
   27ed1:	48 29 fa             	sub    %rdi,%rdx
   27ed4:	48 0f 43 c2          	cmovae %rdx,%rax
   27ed8:	48 39 c1             	cmp    %rax,%rcx
   27edb:	48 0f 42 c1          	cmovb  %rcx,%rax
   27edf:	48 83 f8 07          	cmp    $0x7,%rax
   27ee3:	0f 86 7e 00 00 00    	jbe    27f67 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x6d7>
   27ee9:	48 ff c0             	inc    %rax
   27eec:	89 c1                	mov    %eax,%ecx
   27eee:	83 e1 07             	and    $0x7,%ecx
   27ef1:	ba 08 00 00 00       	mov    $0x8,%edx
   27ef6:	48 0f 45 d1          	cmovne %rcx,%rdx
   27efa:	48 29 d0             	sub    %rdx,%rax
   27efd:	48 8d 0c 07          	lea    (%rdi,%rax,1),%rcx
   27f01:	49 8d 14 bb          	lea    (%r11,%rdi,4),%rdx
   27f05:	48 83 c2 14          	add    $0x14,%rdx
   27f09:	4d 8d 04 ba          	lea    (%r10,%rdi,4),%r8
   27f0d:	49 83 c0 14          	add    $0x14,%r8
   27f11:	31 ff                	xor    %edi,%edi
   27f13:	0f 28 05 d6 87 ff ff 	movaps -0x782a(%rip),%xmm0        # 206f0 <.LCPI0_1>
   27f1a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
   27f20:	0f 10 4c ba f0       	movups -0x10(%rdx,%rdi,4),%xmm1
   27f25:	0f 10 14 ba          	movups (%rdx,%rdi,4),%xmm2
   27f29:	41 0f 10 5c b8 ec    	movups -0x14(%r8,%rdi,4),%xmm3
   27f2f:	41 0f 10 64 b8 f0    	movups -0x10(%r8,%rdi,4),%xmm4
   27f35:	0f 58 e3             	addps  %xmm3,%xmm4
   27f38:	41 0f 10 5c b8 fc    	movups -0x4(%r8,%rdi,4),%xmm3
   27f3e:	41 0f 10 2c b8       	movups (%r8,%rdi,4),%xmm5
   27f43:	0f 58 eb             	addps  %xmm3,%xmm5
   27f46:	0f 59 e0             	mulps  %xmm0,%xmm4
   27f49:	0f 5c cc             	subps  %xmm4,%xmm1
   27f4c:	0f 59 e8             	mulps  %xmm0,%xmm5
   27f4f:	0f 5c d5             	subps  %xmm5,%xmm2
   27f52:	0f 11 4c ba f0       	movups %xmm1,-0x10(%rdx,%rdi,4)
   27f57:	0f 11 14 ba          	movups %xmm2,(%rdx,%rdi,4)
   27f5b:	48 83 c7 08          	add    $0x8,%rdi
   27f5f:	48 39 f8             	cmp    %rdi,%rax
   27f62:	75 bc                	jne    27f20 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x690>
   27f64:	48 89 cf             	mov    %rcx,%rdi
   27f67:	48 ff c7             	inc    %rdi
   27f6a:	f3 0f 10 05 0e 8a ff 	movss  -0x75f2(%rip),%xmm0        # 20980 <.LCPI0_0>
   27f71:	ff 
   27f72:	66 66 66 66 66 2e 0f 	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   27f79:	1f 84 00 00 00 00 00 
   27f80:	4c 39 cf             	cmp    %r9,%rdi
   27f83:	0f 83 ff 01 00 00    	jae    28188 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x8f8>
   27f89:	48 8d 47 ff          	lea    -0x1(%rdi),%rax
   27f8d:	48 39 f0             	cmp    %rsi,%rax
   27f90:	0f 83 e2 01 00 00    	jae    28178 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x8e8>
   27f96:	48 39 f7             	cmp    %rsi,%rdi
   27f99:	0f 83 cc 01 00 00    	jae    2816b <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x8db>
   27f9f:	f3 41 0f 10 4c ba fc 	movss  -0x4(%r10,%rdi,4),%xmm1
   27fa6:	f3 41 0f 58 0c ba    	addss  (%r10,%rdi,4),%xmm1
   27fac:	f3 0f 59 c8          	mulss  %xmm0,%xmm1
   27fb0:	f3 41 0f 58 0c bb    	addss  (%r11,%rdi,4),%xmm1
   27fb6:	f3 41 0f 11 0c bb    	movss  %xmm1,(%r11,%rdi,4)
   27fbc:	4c 39 ff             	cmp    %r15,%rdi
   27fbf:	48 8d 7f 01          	lea    0x1(%rdi),%rdi
   27fc3:	72 bb                	jb     27f80 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x6f0>
   27fc5:	48 83 fb 02          	cmp    $0x2,%rbx
   27fc9:	0f 93 c0             	setae  %al
   27fcc:	41 f6 c6 01          	test   $0x1,%r14b
   27fd0:	0f 94 c1             	sete   %cl
   27fd3:	84 c8                	test   %cl,%al
   27fd5:	48 8b 8c 24 88 00 00 	mov    0x88(%rsp),%rcx
   27fdc:	00 
   27fdd:	74 56                	je     28035 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x7a5>
   27fdf:	4c 89 f7             	mov    %r14,%rdi
   27fe2:	48 d1 ef             	shr    $1,%rdi
   27fe5:	48 b8 fe ff ff ff ff 	movabs $0x7ffffffffffffffe,%rax
   27fec:	ff ff 7f 
   27fef:	48 ff c0             	inc    %rax
   27ff2:	49 83 fe 02          	cmp    $0x2,%r14
   27ff6:	48 0f 43 c1          	cmovae %rcx,%rax
   27ffa:	4c 39 cf             	cmp    %r9,%rdi
   27ffd:	0f 83 fc 01 00 00    	jae    281ff <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x96f>
   28003:	48 39 f0             	cmp    %rsi,%rax
   28006:	0f 83 03 02 00 00    	jae    2820f <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x97f>
   2800c:	48 39 f1             	cmp    %rsi,%rcx
   2800f:	0f 83 0a 02 00 00    	jae    2821f <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x98f>
   28015:	f3 41 0f 10 04 82    	movss  (%r10,%rax,4),%xmm0
   2801b:	f3 41 0f 58 04 8a    	addss  (%r10,%rcx,4),%xmm0
   28021:	f3 0f 59 05 57 89 ff 	mulss  -0x76a9(%rip),%xmm0        # 20980 <.LCPI0_0>
   28028:	ff 
   28029:	f3 41 0f 58 04 bb    	addss  (%r11,%rdi,4),%xmm0
   2802f:	f3 41 0f 11 04 bb    	movss  %xmm0,(%r11,%rdi,4)
   28035:	48 8d 65 d8          	lea    -0x28(%rbp),%rsp
   28039:	5b                   	pop    %rbx
   2803a:	41 5c                	pop    %r12
   2803c:	41 5d                	pop    %r13
   2803e:	41 5e                	pop    %r14
   28040:	41 5f                	pop    %r15
   28042:	5d                   	pop    %rbp
   28043:	c3                   	ret
   28044:	48 8d 0d 05 dd 09 00 	lea    0x9dd05(%rip),%rcx        # c5d50 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x458>
   2804b:	4c 89 e7             	mov    %r12,%rdi
   2804e:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   28053:	48 89 d6             	mov    %rdx,%rsi
   28056:	ff 15 8c 24 0a 00    	call   *0xa248c(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   2805c:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   28061:	4c 29 e2             	sub    %r12,%rdx
   28064:	eb 6e                	jmp    280d4 <hayro_jpeg2000::j2c::idwt::irreversible_step_3::<fearless_simd::generated::avx512::Avx512>+0x844>
   28066:	48 8d 0d cb dc 09 00 	lea    0x9dccb(%rip),%rcx        # c5d38 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x440>
   2806d:	4c 89 e7             	mov    %r12,%rdi
   28070:	48 8b 54 24 20       	mov    0x20(%rsp),%rdx
   28075:	48 89 d6             	mov    %rdx,%rsi
   28078:	ff 15 6a 24 0a 00    	call   *0xa246a(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   2807e:	48 8b 54 24 20       	mov    0x20(%rsp),%rdx
   28083:	4c 29 e2             	sub    %r12,%rdx
   28086:	48 8d 0d 7b dc 09 00 	lea    0x9dc7b(%rip),%rcx        # c5d08 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x410>
   2808d:	be 08 00 00 00       	mov    $0x8,%esi
   28092:	31 ff                	xor    %edi,%edi
   28094:	ff 15 4e 24 0a 00    	call   *0xa244e(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   2809a:	48 8d 0d 7f dc 09 00 	lea    0x9dc7f(%rip),%rcx        # c5d20 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x428>
   280a1:	be 08 00 00 00       	mov    $0x8,%esi
   280a6:	31 ff                	xor    %edi,%edi
   280a8:	ff 15 3a 24 0a 00    	call   *0xa243a(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   280ae:	49 ff c5             	inc    %r13
   280b1:	48 8d 0d 98 dc 09 00 	lea    0x9dc98(%rip),%rcx        # c5d50 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x458>
   280b8:	4c 89 ef             	mov    %r13,%rdi
   280bb:	48 8b 54 24 20       	mov    0x20(%rsp),%rdx
   280c0:	48 89 d6             	mov    %rdx,%rsi
   280c3:	ff 15 1f 24 0a 00    	call   *0xa241f(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   280c9:	48 8b 94 24 80 00 00 	mov    0x80(%rsp),%rdx
   280d0:	00 
   280d1:	4c 29 ea             	sub    %r13,%rdx
   280d4:	48 8d 0d 15 dc 09 00 	lea    0x9dc15(%rip),%rcx        # c5cf0 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3f8>
   280db:	be 08 00 00 00       	mov    $0x8,%esi
   280e0:	31 ff                	xor    %edi,%edi
   280e2:	ff 15 00 24 0a 00    	call   *0xa2400(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   280e8:	48 8d 0d 49 dc 09 00 	lea    0x9dc49(%rip),%rcx        # c5d38 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x440>
   280ef:	4c 89 ef             	mov    %r13,%rdi
   280f2:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   280f7:	48 89 d6             	mov    %rdx,%rsi
   280fa:	ff 15 e8 23 0a 00    	call   *0xa23e8(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   28100:	48 8b 54 24 18       	mov    0x18(%rsp),%rdx
   28105:	4c 29 ea             	sub    %r13,%rdx
   28108:	48 8d 0d f9 db 09 00 	lea    0x9dbf9(%rip),%rcx        # c5d08 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x410>
   2810f:	be 08 00 00 00       	mov    $0x8,%esi
   28114:	31 ff                	xor    %edi,%edi
   28116:	ff 15 cc 23 0a 00    	call   *0xa23cc(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   2811c:	49 f7 d5             	not    %r13
   2811f:	4c 03 6c 24 18       	add    0x18(%rsp),%r13
   28124:	48 8d 0d f5 db 09 00 	lea    0x9dbf5(%rip),%rcx        # c5d20 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x428>
   2812b:	be 08 00 00 00       	mov    $0x8,%esi
   28130:	31 ff                	xor    %edi,%edi
   28132:	4c 89 ea             	mov    %r13,%rdx
   28135:	ff 15 ad 23 0a 00    	call   *0xa23ad(%rip)        # ca4e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
   2813b:	48 ff c7             	inc    %rdi
   2813e:	48 8d 15 93 db 09 00 	lea    0x9db93(%rip),%rdx        # c5cd8 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3e0>
   28145:	4c 89 ce             	mov    %r9,%rsi
   28148:	ff 15 d2 23 0a 00    	call   *0xa23d2(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   2814e:	48 8d 15 6b db 09 00 	lea    0x9db6b(%rip),%rdx        # c5cc0 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3c8>
   28155:	4c 89 ce             	mov    %r9,%rsi
   28158:	ff 15 c2 23 0a 00    	call   *0xa23c2(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   2815e:	48 8d 15 43 db 09 00 	lea    0x9db43(%rip),%rdx        # c5ca8 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3b0>
   28165:	ff 15 b5 23 0a 00    	call   *0xa23b5(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   2816b:	48 8d 15 66 db 09 00 	lea    0x9db66(%rip),%rdx        # c5cd8 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3e0>
   28172:	ff 15 a8 23 0a 00    	call   *0xa23a8(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   28178:	48 8d 15 41 db 09 00 	lea    0x9db41(%rip),%rdx        # c5cc0 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3c8>
   2817f:	48 89 c7             	mov    %rax,%rdi
   28182:	ff 15 98 23 0a 00    	call   *0xa2398(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   28188:	48 8d 15 19 db 09 00 	lea    0x9db19(%rip),%rdx        # c5ca8 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x3b0>
   2818f:	4c 89 ce             	mov    %r9,%rsi
   28192:	ff 15 88 23 0a 00    	call   *0xa2388(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   28198:	48 8d 15 79 da 09 00 	lea    0x9da79(%rip),%rdx        # c5c18 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x320>
   2819f:	31 ff                	xor    %edi,%edi
   281a1:	31 f6                	xor    %esi,%esi
   281a3:	ff 15 77 23 0a 00    	call   *0xa2377(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   281a9:	48 8d 15 80 da 09 00 	lea    0x9da80(%rip),%rdx        # c5c30 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x338>
   281b0:	31 ff                	xor    %edi,%edi
   281b2:	31 f6                	xor    %esi,%esi
   281b4:	ff 15 66 23 0a 00    	call   *0xa2366(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   281ba:	48 8d 15 87 da 09 00 	lea    0x9da87(%rip),%rdx        # c5c48 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x350>
   281c1:	48 8b 74 24 18       	mov    0x18(%rsp),%rsi
   281c6:	ff 15 54 23 0a 00    	call   *0xa2354(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   281cc:	48 8d 15 8d da 09 00 	lea    0x9da8d(%rip),%rdx        # c5c60 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x368>
   281d3:	48 89 c7             	mov    %rax,%rdi
   281d6:	ff 15 44 23 0a 00    	call   *0xa2344(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   281dc:	48 8d 15 95 da 09 00 	lea    0x9da95(%rip),%rdx        # c5c78 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x380>
   281e3:	4c 89 ce             	mov    %r9,%rsi
   281e6:	ff 15 34 23 0a 00    	call   *0xa2334(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   281ec:	48 8d 15 9d da 09 00 	lea    0x9da9d(%rip),%rdx        # c5c90 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x398>
   281f3:	48 89 c7             	mov    %rax,%rdi
   281f6:	4c 89 ce             	mov    %r9,%rsi
   281f9:	ff 15 21 23 0a 00    	call   *0xa2321(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   281ff:	48 8d 15 5a da 09 00 	lea    0x9da5a(%rip),%rdx        # c5c60 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x368>
   28206:	4c 89 ce             	mov    %r9,%rsi
   28209:	ff 15 11 23 0a 00    	call   *0xa2311(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   2820f:	48 8d 15 62 da 09 00 	lea    0x9da62(%rip),%rdx        # c5c78 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x380>
   28216:	48 89 c7             	mov    %rax,%rdi
   28219:	ff 15 01 23 0a 00    	call   *0xa2301(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   2821f:	48 8d 15 6a da 09 00 	lea    0x9da6a(%rip),%rdx        # c5c90 <anon.fa69a01994f97486c68f456eb0b1ada3.5.llvm.9314608568884554383+0x398>
   28226:	48 89 cf             	mov    %rcx,%rdi
   28229:	ff 15 f1 22 0a 00    	call   *0xa22f1(%rip)        # ca520 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
   2822f:	cc                   	int3

