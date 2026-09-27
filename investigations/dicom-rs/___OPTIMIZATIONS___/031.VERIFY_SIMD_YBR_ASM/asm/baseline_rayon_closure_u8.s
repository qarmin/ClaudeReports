000000000039a780 <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut>:
  39a780:	55                   	push   %rbp
  39a781:	41 57                	push   %r15
  39a783:	41 56                	push   %r14
  39a785:	53                   	push   %rbx
  39a786:	48 83 ec 38          	sub    $0x38,%rsp
  39a78a:	48 85 d2             	test   %rdx,%rdx
  39a78d:	0f 84 57 01 00 00    	je     39a8ea <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut+0x16a>
  39a793:	48 83 fa 01          	cmp    $0x1,%rdx
  39a797:	0f 84 5e 01 00 00    	je     39a8fb <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut+0x17b>
  39a79d:	48 83 fa 02          	cmp    $0x2,%rdx
  39a7a1:	0f 86 6b 01 00 00    	jbe    39a912 <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut+0x192>
  39a7a7:	48 89 f3             	mov    %rsi,%rbx
  39a7aa:	0f b6 06             	movzbl (%rsi),%eax
  39a7ad:	f3 0f 2a c8          	cvtsi2ss %eax,%xmm1
  39a7b1:	0f b7 46 01          	movzwl 0x1(%rsi),%eax
  39a7b5:	66 0f 6e c0          	movd   %eax,%xmm0
  39a7b9:	66 0f ef d2          	pxor   %xmm2,%xmm2
  39a7bd:	66 0f 60 c2          	punpcklbw %xmm2,%xmm0
  39a7c1:	66 0f 61 c2          	punpcklwd %xmm2,%xmm0
  39a7c5:	0f 5b e0             	cvtdq2ps %xmm0,%xmm4
  39a7c8:	0f 58 25 11 e8 df ff 	addps  -0x2017ef(%rip),%xmm4        # 198fe0 <.LCPI42_0>
  39a7cf:	0f 28 c4             	movaps %xmm4,%xmm0
  39a7d2:	0f c6 c4 55          	shufps $0x55,%xmm4,%xmm0
  39a7d6:	f3 0f 59 05 e6 05 e0 	mulss  -0x1ffa1a(%rip),%xmm0        # 19adc4 <.LCPI42_1>
  39a7dd:	ff 
  39a7de:	f3 0f 58 c1          	addss  %xmm1,%xmm0
  39a7e2:	f3 0f 10 15 26 06 e0 	movss  -0x1ff9da(%rip),%xmm2        # 19ae10 <.LCPI10_1>
  39a7e9:	ff 
  39a7ea:	f3 0f 58 c2          	addss  %xmm2,%xmm0
  39a7ee:	f2 0f 10 1d fa ca df 	movsd  -0x203506(%rip),%xmm3        # 1972f0 <.LCPI42_7>
  39a7f5:	ff 
  39a7f6:	0f 59 dc             	mulps  %xmm4,%xmm3
  39a7f9:	0f 28 eb             	movaps %xmm3,%xmm5
  39a7fc:	f3 0f 58 e9          	addss  %xmm1,%xmm5
  39a800:	0f c6 db 55          	shufps $0x55,%xmm3,%xmm3
  39a804:	f3 0f 5c eb          	subss  %xmm3,%xmm5
  39a808:	f3 0f 58 ea          	addss  %xmm2,%xmm5
  39a80c:	0f 29 6c 24 10       	movaps %xmm5,0x10(%rsp)
  39a811:	f3 0f 59 25 93 05 e0 	mulss  -0x1ffa6d(%rip),%xmm4        # 19adac <.LCPI42_4>
  39a818:	ff 
  39a819:	f3 0f 58 e1          	addss  %xmm1,%xmm4
  39a81d:	f3 0f 58 e2          	addss  %xmm2,%xmm4
  39a821:	0f 29 64 24 20       	movaps %xmm4,0x20(%rsp)
  39a826:	4c 8b 35 8b 13 14 00 	mov    0x14138b(%rip),%r14        # 4dbbb8 <floorf$got>
  39a82d:	41 ff d6             	call   *%r14
  39a830:	0f 29 04 24          	movaps %xmm0,(%rsp)
  39a834:	0f 28 44 24 10       	movaps 0x10(%rsp),%xmm0
  39a839:	41 ff d6             	call   *%r14
  39a83c:	0f 14 04 24          	unpcklps (%rsp),%xmm0
  39a840:	0f 57 c9             	xorps  %xmm1,%xmm1
  39a843:	0f 5f c8             	maxps  %xmm0,%xmm1
  39a846:	0f 28 05 f3 db df ff 	movaps -0x20240d(%rip),%xmm0        # 198440 <.LCPI138_0>
  39a84d:	0f c2 c1 01          	cmpltps %xmm1,%xmm0
  39a851:	0f 14 c0             	unpcklps %xmm0,%xmm0
  39a854:	66 0f 50 c0          	movmskpd %xmm0,%eax
  39a858:	89 c1                	mov    %eax,%ecx
  39a85a:	80 e1 01             	and    $0x1,%cl
  39a85d:	0f 28 c1             	movaps %xmm1,%xmm0
  39a860:	0f 28 d1             	movaps %xmm1,%xmm2
  39a863:	0f c6 c1 55          	shufps $0x55,%xmm1,%xmm0
  39a867:	0f 57 c9             	xorps  %xmm1,%xmm1
  39a86a:	f3 0f 5f c8          	maxss  %xmm0,%xmm1
  39a86e:	f3 0f 10 1d aa 05 e0 	movss  -0x1ffa56(%rip),%xmm3        # 19ae20 <.LCPI10_6>
  39a875:	ff 
  39a876:	0f 28 c3             	movaps %xmm3,%xmm0
  39a879:	f3 0f 5d c1          	minss  %xmm1,%xmm0
  39a87d:	f3 0f 2c d0          	cvttss2si %xmm0,%edx
  39a881:	a8 02                	test   $0x2,%al
  39a883:	bd ff 00 00 00       	mov    $0xff,%ebp
  39a888:	0f 44 ea             	cmove  %edx,%ebp
  39a88b:	0f 57 c0             	xorps  %xmm0,%xmm0
  39a88e:	f3 0f 5f c2          	maxss  %xmm2,%xmm0
  39a892:	0f 28 cb             	movaps %xmm3,%xmm1
  39a895:	f3 0f 5d c8          	minss  %xmm0,%xmm1
  39a899:	f3 0f 2c c1          	cvttss2si %xmm1,%eax
  39a89d:	45 31 ff             	xor    %r15d,%r15d
  39a8a0:	f6 d9                	neg    %cl
  39a8a2:	45 19 ff             	sbb    %r15d,%r15d
  39a8a5:	41 08 c7             	or     %al,%r15b
  39a8a8:	0f 28 44 24 20       	movaps 0x20(%rsp),%xmm0
  39a8ad:	41 ff d6             	call   *%r14
  39a8b0:	0f 57 c9             	xorps  %xmm1,%xmm1
  39a8b3:	f3 0f 5f c8          	maxss  %xmm0,%xmm1
  39a8b7:	f3 0f 10 15 61 05 e0 	movss  -0x1ffa9f(%rip),%xmm2        # 19ae20 <.LCPI10_6>
  39a8be:	ff 
  39a8bf:	0f 28 c2             	movaps %xmm2,%xmm0
  39a8c2:	f3 0f 5d c1          	minss  %xmm1,%xmm0
  39a8c6:	0f 57 c9             	xorps  %xmm1,%xmm1
  39a8c9:	f3 0f 5f c8          	maxss  %xmm0,%xmm1
  39a8cd:	f3 0f 5d d1          	minss  %xmm1,%xmm2
  39a8d1:	f3 0f 2c c2          	cvttss2si %xmm2,%eax
  39a8d5:	40 88 2b             	mov    %bpl,(%rbx)
  39a8d8:	44 88 7b 01          	mov    %r15b,0x1(%rbx)
  39a8dc:	88 43 02             	mov    %al,0x2(%rbx)
  39a8df:	48 83 c4 38          	add    $0x38,%rsp
  39a8e3:	5b                   	pop    %rbx
  39a8e4:	41 5e                	pop    %r14
  39a8e6:	41 5f                	pop    %r15
  39a8e8:	5d                   	pop    %rbp
  39a8e9:	c3                   	ret
  39a8ea:	48 8d 15 0f 60 11 00 	lea    0x11600f(%rip),%rdx        # 4b0900 <anon.0ca6d5d74da8e1f84519bc038cee917a.2.llvm.12506763111498960747>
  39a8f1:	31 ff                	xor    %edi,%edi
  39a8f3:	31 f6                	xor    %esi,%esi
  39a8f5:	ff 15 95 0e 14 00    	call   *0x140e95(%rip)        # 4db790 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
  39a8fb:	48 8d 15 16 60 11 00 	lea    0x116016(%rip),%rdx        # 4b0918 <anon.0ca6d5d74da8e1f84519bc038cee917a.3.llvm.12506763111498960747>
  39a902:	bf 01 00 00 00       	mov    $0x1,%edi
  39a907:	be 01 00 00 00       	mov    $0x1,%esi
  39a90c:	ff 15 7e 0e 14 00    	call   *0x140e7e(%rip)        # 4db790 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
  39a912:	48 8d 15 17 60 11 00 	lea    0x116017(%rip),%rdx        # 4b0930 <anon.0ca6d5d74da8e1f84519bc038cee917a.4.llvm.12506763111498960747>
  39a919:	bf 02 00 00 00       	mov    $0x2,%edi
  39a91e:	be 02 00 00 00       	mov    $0x2,%esi
  39a923:	ff 15 67 0e 14 00    	call   *0x140e67(%rip)        # 4db790 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
  39a929:	cc                   	int3
  39a92a:	cc                   	int3
  39a92b:	cc                   	int3
  39a92c:	cc                   	int3
  39a92d:	cc                   	int3
  39a92e:	cc                   	int3
  39a92f:	cc                   	int3

