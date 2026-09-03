000000000039a740 <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut>:
  39a740:	50                   	push   %rax
  39a741:	48 85 d2             	test   %rdx,%rdx
  39a744:	0f 84 a7 00 00 00    	je     39a7f1 <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut+0xb1>
  39a74a:	48 83 fa 01          	cmp    $0x1,%rdx
  39a74e:	0f 84 ae 00 00 00    	je     39a802 <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut+0xc2>
  39a754:	48 83 fa 02          	cmp    $0x2,%rdx
  39a758:	0f 86 bb 00 00 00    	jbe    39a819 <<&dicom_pixeldata::convert_colorspace_u8::{closure#0} as core::ops::function::FnMut<(&mut [u8],)>>::call_mut+0xd9>
  39a75e:	0f b6 06             	movzbl (%rsi),%eax
  39a761:	0f b6 4e 01          	movzbl 0x1(%rsi),%ecx
  39a765:	83 c1 80             	add    $0xffffff80,%ecx
  39a768:	0f b6 56 02          	movzbl 0x2(%rsi),%edx
  39a76c:	83 c2 80             	add    $0xffffff80,%edx
  39a76f:	c1 e0 14             	shl    $0x14,%eax
  39a772:	69 fa 98 6e 16 00    	imul   $0x166e98,%edx,%edi
  39a778:	01 c7                	add    %eax,%edi
  39a77a:	81 c7 00 00 08 00    	add    $0x80000,%edi
  39a780:	c1 ff 14             	sar    $0x14,%edi
  39a783:	44 69 c1 95 81 05 00 	imul   $0x58195,%ecx,%r8d
  39a78a:	69 d2 e6 92 f4 ff    	imul   $0xfff492e6,%edx,%edx
  39a790:	41 01 c0             	add    %eax,%r8d
  39a793:	41 81 c0 00 00 08 00 	add    $0x80000,%r8d
  39a79a:	41 01 d0             	add    %edx,%r8d
  39a79d:	41 c1 f8 14          	sar    $0x14,%r8d
  39a7a1:	69 c9 1d 5a 1c 00    	imul   $0x1c5a1d,%ecx,%ecx
  39a7a7:	01 c8                	add    %ecx,%eax
  39a7a9:	05 00 00 08 00       	add    $0x80000,%eax
  39a7ae:	c1 f8 14             	sar    $0x14,%eax
  39a7b1:	31 c9                	xor    %ecx,%ecx
  39a7b3:	85 ff                	test   %edi,%edi
  39a7b5:	0f 4e f9             	cmovle %ecx,%edi
  39a7b8:	81 ff ff 00 00 00    	cmp    $0xff,%edi
  39a7be:	ba ff 00 00 00       	mov    $0xff,%edx
  39a7c3:	0f 4d fa             	cmovge %edx,%edi
  39a7c6:	45 85 c0             	test   %r8d,%r8d
  39a7c9:	44 0f 4e c1          	cmovle %ecx,%r8d
  39a7cd:	40 88 3e             	mov    %dil,(%rsi)
  39a7d0:	41 81 f8 ff 00 00 00 	cmp    $0xff,%r8d
  39a7d7:	44 0f 4d c2          	cmovge %edx,%r8d
  39a7db:	44 88 46 01          	mov    %r8b,0x1(%rsi)
  39a7df:	85 c0                	test   %eax,%eax
  39a7e1:	0f 4e c1             	cmovle %ecx,%eax
  39a7e4:	3d ff 00 00 00       	cmp    $0xff,%eax
  39a7e9:	0f 4d c2             	cmovge %edx,%eax
  39a7ec:	88 46 02             	mov    %al,0x2(%rsi)
  39a7ef:	58                   	pop    %rax
  39a7f0:	c3                   	ret
  39a7f1:	48 8d 15 38 5f 11 00 	lea    0x115f38(%rip),%rdx        # 4b0730 <anon.0ca6d5d74da8e1f84519bc038cee917a.2.llvm.10002991804575057349>
  39a7f8:	31 ff                	xor    %edi,%edi
  39a7fa:	31 f6                	xor    %esi,%esi
  39a7fc:	ff 15 be 0d 14 00    	call   *0x140dbe(%rip)        # 4db5c0 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
  39a802:	48 8d 15 3f 5f 11 00 	lea    0x115f3f(%rip),%rdx        # 4b0748 <anon.0ca6d5d74da8e1f84519bc038cee917a.3.llvm.10002991804575057349>
  39a809:	bf 01 00 00 00       	mov    $0x1,%edi
  39a80e:	be 01 00 00 00       	mov    $0x1,%esi
  39a813:	ff 15 a7 0d 14 00    	call   *0x140da7(%rip)        # 4db5c0 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>
  39a819:	48 8d 15 40 5f 11 00 	lea    0x115f40(%rip),%rdx        # 4b0760 <anon.0ca6d5d74da8e1f84519bc038cee917a.4.llvm.10002991804575057349>
  39a820:	bf 02 00 00 00       	mov    $0x2,%edi
  39a825:	be 02 00 00 00       	mov    $0x2,%esi
  39a82a:	ff 15 90 0d 14 00    	call   *0x140d90(%rip)        # 4db5c0 <_RNvNtCsc36rpYXAlPq_4core9panicking18panic_bounds_check$got>

