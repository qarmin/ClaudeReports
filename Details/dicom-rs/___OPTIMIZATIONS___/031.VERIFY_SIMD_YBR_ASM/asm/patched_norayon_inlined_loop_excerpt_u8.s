  366df3:	48 c7 84 24 e8 00 00 	movq   $0x1,0xe8(%rsp)
  366dfa:	00 01 00 00 00 
  366dff:	48 c7 84 24 f0 00 00 	movq   $0x0,0xf0(%rsp)
  366e06:	00 00 00 00 00 
  366e0b:	4c 8b 6c 24 18       	mov    0x18(%rsp),%r13
  366e10:	49 8b 47 78          	mov    0x78(%r15),%rax
  366e14:	48 b9 00 00 00 00 00 	movabs $0x8000000000000000,%rcx
  366e1b:	00 00 80 
  366e1e:	48 31 c1             	xor    %rax,%rcx
  366e21:	48 85 c0             	test   %rax,%rax
  366e24:	b8 09 00 00 00       	mov    $0x9,%eax
  366e29:	48 0f 48 c1          	cmovs  %rcx,%rax
  366e2d:	48 8d 48 fc          	lea    -0x4(%rax),%rcx
  366e31:	48 83 f9 02          	cmp    $0x2,%rcx
  366e35:	0f 83 0c 01 00 00    	jae    366f47 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xed7>
  366e3b:	48 8b 84 24 f0 00 00 	mov    0xf0(%rsp),%rax
  366e42:	00 
  366e43:	48 85 c0             	test   %rax,%rax
  366e46:	0f 84 01 01 00 00    	je     366f4d <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xedd>
  366e4c:	48 8b b4 24 e8 00 00 	mov    0xe8(%rsp),%rsi
  366e53:	00 
  366e54:	48 8d 15 e5 36 09 00 	lea    0x936e5(%rip),%rdx        # 3fa540 <generic_array::hex::LOWER_CHARS+0x28>
  366e5b:	48 8d 0d f6 36 09 00 	lea    0x936f6(%rip),%rcx        # 3fa558 <generic_array::hex::LOWER_CHARS+0x40>
  366e62:	31 ff                	xor    %edi,%edi
  366e64:	41 b8 ff 00 00 00    	mov    $0xff,%r8d
  366e6a:	66 0f 1f 44 00 00    	nopw   0x0(%rax,%rax,1)
  366e70:	48 83 f8 03          	cmp    $0x3,%rax
  366e74:	41 b9 03 00 00 00    	mov    $0x3,%r9d
  366e7a:	4c 0f 42 c8          	cmovb  %rax,%r9
  366e7e:	48 83 f8 01          	cmp    $0x1,%rax
  366e82:	0f 84 0f 14 00 00    	je     368297 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x2227>
  366e88:	48 83 f8 02          	cmp    $0x2,%rax
  366e8c:	0f 86 0c 14 00 00    	jbe    36829e <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x222e>
  366e92:	4c 29 c8             	sub    %r9,%rax
  366e95:	44 0f b6 16          	movzbl (%rsi),%r10d
  366e99:	44 0f b6 5e 01       	movzbl 0x1(%rsi),%r11d
  366e9e:	41 83 c3 80          	add    $0xffffff80,%r11d
  366ea2:	44 0f b6 76 02       	movzbl 0x2(%rsi),%r14d
  366ea7:	41 83 c6 80          	add    $0xffffff80,%r14d
  366eab:	41 c1 e2 14          	shl    $0x14,%r10d
  366eaf:	45 69 fe 98 6e 16 00 	imul   $0x166e98,%r14d,%r15d
  366eb6:	45 01 d7             	add    %r10d,%r15d
  366eb9:	41 81 c7 00 00 08 00 	add    $0x80000,%r15d
  366ec0:	41 c1 ff 14          	sar    $0x14,%r15d
  366ec4:	45 69 e3 95 81 05 00 	imul   $0x58195,%r11d,%r12d
  366ecb:	45 69 f6 e6 92 f4 ff 	imul   $0xfff492e6,%r14d,%r14d
  366ed2:	45 01 d4             	add    %r10d,%r12d
  366ed5:	41 81 c4 00 00 08 00 	add    $0x80000,%r12d
  366edc:	45 01 f4             	add    %r14d,%r12d
  366edf:	41 c1 fc 14          	sar    $0x14,%r12d
  366ee3:	45 69 db 1d 5a 1c 00 	imul   $0x1c5a1d,%r11d,%r11d
  366eea:	45 01 da             	add    %r11d,%r10d
  366eed:	41 81 c2 00 00 08 00 	add    $0x80000,%r10d
  366ef4:	41 c1 fa 14          	sar    $0x14,%r10d
  366ef8:	45 85 ff             	test   %r15d,%r15d
  366efb:	44 0f 4e ff          	cmovle %edi,%r15d
  366eff:	41 81 ff ff 00 00 00 	cmp    $0xff,%r15d
  366f06:	45 0f 4d f8          	cmovge %r8d,%r15d
  366f0a:	45 85 e4             	test   %r12d,%r12d
  366f0d:	44 0f 4e e7          	cmovle %edi,%r12d
  366f11:	41 81 fc ff 00 00 00 	cmp    $0xff,%r12d
  366f18:	45 0f 4d e0          	cmovge %r8d,%r12d
  366f1c:	44 88 3e             	mov    %r15b,(%rsi)
  366f1f:	45 85 d2             	test   %r10d,%r10d
  366f22:	44 0f 4e d7          	cmovle %edi,%r10d
  366f26:	44 88 66 01          	mov    %r12b,0x1(%rsi)
  366f2a:	41 81 fa ff 00 00 00 	cmp    $0xff,%r10d
  366f31:	45 0f 4d d0          	cmovge %r8d,%r10d
  366f35:	44 88 56 02          	mov    %r10b,0x2(%rsi)
  366f39:	4c 01 ce             	add    %r9,%rsi
  366f3c:	48 85 c0             	test   %rax,%rax
  366f3f:	0f 85 2b ff ff ff    	jne    366e70 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xe00>
  366f45:	eb 06                	jmp    366f4d <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xedd>
  366f47:	48 83 f8 03          	cmp    $0x3,%rax
  366f4b:	75 24                	jne    366f71 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xf01>
  366f4d:	48 8b 44 24 08       	mov    0x8(%rsp),%rax
  366f52:	44 0f b6 40 31       	movzbl 0x31(%rax),%r8d
  366f57:	48 8d 8c 24 e0 00 00 	lea    0xe0(%rsp),%rcx
  366f5e:	00 
  366f5f:	48 89 df             	mov    %rbx,%rdi
  366f62:	44 89 ee             	mov    %r13d,%esi
  366f65:	89 ea                	mov    %ebp,%edx
  366f67:	e8 a4 ec ff ff       	call   365c10 <<dicom_pixeldata::DecodedPixelData>::rgb_image_with_extend>
  366f6c:	e9 20 f7 ff ff       	jmp    366691 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x621>
  366f71:	49 83 c7 78          	add    $0x78,%r15
  366f75:	48 8d bc 24 10 01 00 	lea    0x110(%rsp),%rdi
  366f7c:	00 
  366f7d:	4c 89 fe             	mov    %r15,%rsi
  366f80:	e8 7b 1a 00 00       	call   368a00 <<dicom_pixeldata::attribute::PhotometricInterpretation as core::clone::Clone>::clone>
  366f85:	48 8b 84 24 20 01 00 	mov    0x120(%rsp),%rax
  366f8c:	00 
