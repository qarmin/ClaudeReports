  36687e:	00 
  36687f:	48 8d 74 24 10       	lea    0x10(%rsp),%rsi
  366884:	e8 77 41 00 00       	call   36aa00 <<alloc::vec::Vec<u16> as alloc::vec::spec_from_iter::SpecFromIter<u16, core::iter::adapters::flatten::FlatMap<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<u16>, core::slice::iter::Iter<u16>>, core::slice::iter::Iter<u16>>, [u16; 3], dicom_pixeldata::interleave<u16>::{closure#0}>>>::from_iter>
  366889:	4d 85 f6             	test   %r14,%r14
  36688c:	0f 84 0c 01 00 00    	je     36699e <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x8ae>
  366892:	4d 01 f6             	add    %r14,%r14
  366895:	ba 02 00 00 00       	mov    $0x2,%edx
  36689a:	4c 89 e7             	mov    %r12,%rdi
  36689d:	4c 89 f6             	mov    %r14,%rsi
  3668a0:	ff 15 4a 8d 0b 00    	call   *0xb8d4a(%rip)        # 41f5f0 <_RNvCs6rREvFdRhLb_7___rustc14___rust_dealloc$got>
  3668a6:	e9 f3 00 00 00       	jmp    36699e <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x8ae>
  3668ab:	41 80 bf 9e 00 00 00 	cmpb   $0x0,0x9e(%r15)
  3668b2:	00 
  3668b3:	48 89 84 24 00 01 00 	mov    %rax,0x100(%rsp)
  3668ba:	00 
  3668bb:	48 89 8c 24 10 01 00 	mov    %rcx,0x110(%rsp)
  3668c2:	00 
  3668c3:	48 89 b4 24 20 01 00 	mov    %rsi,0x120(%rsp)
  3668ca:	00 
  3668cb:	0f 84 b0 03 00 00    	je     366c81 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xb91>
  3668d1:	48 89 94 24 c8 00 00 	mov    %rdx,0xc8(%rsp)
  3668d8:	00 
  3668d9:	4d 89 f4             	mov    %r14,%r12
  3668dc:	49 d1 ec             	shr    $1,%r12
  3668df:	48 8d 7c 24 10       	lea    0x10(%rsp),%rdi
  3668e4:	b9 02 00 00 00       	mov    $0x2,%ecx
  3668e9:	41 b8 02 00 00 00    	mov    $0x2,%r8d
  3668ef:	4c 89 e6             	mov    %r12,%rsi
  3668f2:	ba 01 00 00 00       	mov    $0x1,%edx
  3668f7:	e8 34 5f 00 00       	call   36c830 <<alloc::raw_vec::RawVecInner>::try_allocate_in>
  3668fc:	48 8b 44 24 18       	mov    0x18(%rsp),%rax
  366901:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
  366908:	00 
  366909:	83 7c 24 10 01       	cmpl   $0x1,0x10(%rsp)
  36690e:	0f 84 6b 14 00 00    	je     367d7f <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x1c8f>
  366914:	48 8b 44 24 20       	mov    0x20(%rsp),%rax
  366919:	4c 39 ed             	cmp    %r13,%rbp
  36691c:	0f 82 70 14 00 00    	jb     367d92 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x1ca2>
  366922:	4d 03 6f 20          	add    0x20(%r15),%r13
  366926:	4c 89 ef             	mov    %r13,%rdi
  366929:	4c 89 f6             	mov    %r14,%rsi
  36692c:	48 89 c2             	mov    %rax,%rdx
  36692f:	4c 89 e1             	mov    %r12,%rcx
  366932:	49 89 c4             	mov    %rax,%r12
  366935:	e8 e6 22 00 00       	call   368c20 <<byteorder::LittleEndian as byteorder::ByteOrder>::read_u16_into>
  36693a:	4d 01 e6             	add    %r12,%r14
  36693d:	48 8d bc 24 a0 01 00 	lea    0x1a0(%rsp),%rdi
  366944:	00 
  366945:	4c 89 e6             	mov    %r12,%rsi
  366948:	4c 89 f2             	mov    %r14,%rdx
  36694b:	e8 f0 4c 00 00       	call   36b640 <<alloc::vec::Vec<u16> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<u16, core::iter::adapters::map::Map<core::slice::iter::Iter<i16>, dicom_pixeldata::convert_i16_to_u16::{closure#0}>>>::from_iter>
  366950:	48 8b b4 24 d0 00 00 	mov    0xd0(%rsp),%rsi
  366957:	00 
  366958:	48 85 f6             	test   %rsi,%rsi
  36695b:	4c 8b 74 24 08       	mov    0x8(%rsp),%r14
  366960:	0f 84 a1 03 00 00    	je     366d07 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xc17>
  366966:	48 01 f6             	add    %rsi,%rsi
  366969:	ba 02 00 00 00       	mov    $0x2,%edx
  36696e:	4c 89 e7             	mov    %r12,%rdi
  366971:	ff 15 79 8c 0b 00    	call   *0xb8c79(%rip)        # 41f5f0 <_RNvCs6rREvFdRhLb_7___rustc14___rust_dealloc$got>
  366977:	e9 8b 03 00 00       	jmp    366d07 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xc17>
  36697c:	66 0f 6f 84 24 40 01 	movdqa 0x140(%rsp),%xmm0
  366983:	00 00 
  366985:	66 0f 7f 84 24 e0 00 	movdqa %xmm0,0xe0(%rsp)
  36698c:	00 00 
  36698e:	48 8b 84 24 50 01 00 	mov    0x150(%rsp),%rax
  366995:	00 
  366996:	48 89 84 24 f0 00 00 	mov    %rax,0xf0(%rsp)
  36699d:	00 
  36699e:	49 8b 47 78          	mov    0x78(%r15),%rax
  3669a2:	48 b9 00 00 00 00 00 	movabs $0x8000000000000000,%rcx
  3669a9:	00 00 80 
  3669ac:	48 31 c1             	xor    %rax,%rcx
  3669af:	48 85 c0             	test   %rax,%rax
  3669b2:	b8 09 00 00 00       	mov    $0x9,%eax
  3669b7:	48 0f 48 c1          	cmovs  %rcx,%rax
  3669bb:	48 8d 48 fc          	lea    -0x4(%rax),%rcx
  3669bf:	48 83 f9 02          	cmp    $0x2,%rcx
  3669c3:	0f 83 9f 01 00 00    	jae    366b68 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xa78>
  3669c9:	4c 8b a4 24 f0 00 00 	mov    0xf0(%rsp),%r12
  3669d0:	00 
  3669d1:	4d 85 e4             	test   %r12,%r12
  3669d4:	0f 84 98 01 00 00    	je     366b72 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0xa82>
  3669da:	48 8b ac 24 e8 00 00 	mov    0xe8(%rsp),%rbp
  3669e1:	00 
  3669e2:	48 8d 05 67 3e 09 00 	lea    0x93e67(%rip),%rax        # 3fa850 <generic_array::hex::LOWER_CHARS+0x58>
  3669e9:	48 89 84 24 d0 00 00 	mov    %rax,0xd0(%rsp)
  3669f0:	00 
  3669f1:	4c 8b 2d b0 b1 0b 00 	mov    0xbb1b0(%rip),%r13        # 421ba8 <floorf$got>
  3669f8:	0f 1f 84 00 00 00 00 	nopl   0x0(%rax,%rax,1)
  3669ff:	00 
  366a00:	49 83 fc 03          	cmp    $0x3,%r12
  366a04:	b8 03 00 00 00       	mov    $0x3,%eax
  366a09:	49 0f 42 c4          	cmovb  %r12,%rax
  366a0d:	49 83 fc 01          	cmp    $0x1,%r12
  366a11:	0f 84 a8 1a 00 00    	je     3684bf <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x23cf>
  366a17:	49 83 fc 02          	cmp    $0x2,%r12
  366a1b:	0f 86 a5 1a 00 00    	jbe    3684c6 <<dicom_pixeldata::DecodedPixelData>::to_dynamic_image_with_options+0x23d6>
  366a21:	0f b7 4d 00          	movzwl 0x0(%rbp),%ecx
  366a25:	0f 57 c9             	xorps  %xmm1,%xmm1
  366a28:	f3 0f 2a c9          	cvtsi2ss %ecx,%xmm1
  366a2c:	89 c1                	mov    %eax,%ecx
  366a2e:	4c 8d 34 4d 00 00 00 	lea    0x0(,%rcx,2),%r14
  366a35:	00 
  366a36:	49 01 ee             	add    %rbp,%r14
  366a39:	49 29 c4             	sub    %rax,%r12
  366a3c:	66 0f 6e 45 02       	movd   0x2(%rbp),%xmm0
  366a41:	66 0f ef d2          	pxor   %xmm2,%xmm2
  366a45:	66 0f 61 c2          	punpcklwd %xmm2,%xmm0
  366a49:	0f 5b e0             	cvtdq2ps %xmm0,%xmm4
  366a4c:	0f 58 25 cd 12 e1 ff 	addps  -0x1eed33(%rip),%xmm4        # 177d20 <.LCPI17_0>
  366a53:	0f 28 c4             	movaps %xmm4,%xmm0
  366a56:	0f c6 c4 55          	shufps $0x55,%xmm4,%xmm0
  366a5a:	f3 0f 59 05 6e 45 e1 	mulss  -0x1eba92(%rip),%xmm0        # 17afd0 <.LCPI17_1>
  366a61:	ff 
  366a62:	f3 0f 58 c1          	addss  %xmm1,%xmm0
  366a66:	f3 0f 10 1d ae 45 e1 	movss  -0x1eba52(%rip),%xmm3        # 17b01c <.LCPI10_1>
  366a6d:	ff 
  366a6e:	f3 0f 58 c3          	addss  %xmm3,%xmm0
  366a72:	0f 28 d4             	movaps %xmm4,%xmm2
  366a75:	0f 59 15 94 10 e1 ff 	mulps  -0x1eef6c(%rip),%xmm2        # 177b10 <.LCPI17_3>
  366a7c:	0f 28 ea             	movaps %xmm2,%xmm5
  366a7f:	f3 0f 58 e9          	addss  %xmm1,%xmm5
  366a83:	0f c6 d2 55          	shufps $0x55,%xmm2,%xmm2
  366a87:	f3 0f 5c ea          	subss  %xmm2,%xmm5
  366a8b:	f3 0f 58 eb          	addss  %xmm3,%xmm5
  366a8f:	0f 29 ac 24 00 01 00 	movaps %xmm5,0x100(%rsp)
  366a96:	00 
  366a97:	f3 0f 59 25 19 45 e1 	mulss  -0x1ebae7(%rip),%xmm4        # 17afb8 <.LCPI17_4>
  366a9e:	ff 
  366a9f:	f3 0f 58 e1          	addss  %xmm1,%xmm4
