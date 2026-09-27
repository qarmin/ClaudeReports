
baseline_bin:     file format elf64-x86-64


Disassembly of section .text:

0000000000108c80 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>>:
  108c80:	push   rbp
  108c81:	push   r15
  108c83:	push   r14
  108c85:	push   rbx
  108c86:	push   rax
  108c87:	mov    eax,DWORD PTR [rdi+0x18]
  108c8a:	test   rax,rax
  108c8d:	je     108e3d <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1bd>
  108c93:	mov    ecx,DWORD PTR [rdi+0x1c]
  108c96:	test   ecx,ecx
  108c98:	je     108e3d <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1bd>
  108c9e:	movss  xmm1,DWORD PTR [rip+0xfffffffffffa8a3a]        # b16e0 <.LCPI20_0>
  108ca6:	addss  xmm0,xmm1
  108caa:	divss  xmm0,xmm1
  108cae:	mulss  xmm0,xmm0
  108cb2:	mov    rsi,QWORD PTR [rdi+0x8]
  108cb6:	mov    rdx,QWORD PTR [rdi+0x10]
  108cba:	lea    r8,[rax+rax*2]
  108cbe:	xor    r9d,r9d
  108cc1:	movss  xmm1,DWORD PTR [rip+0xfffffffffffa8a5b]        # b1724 <.LCPI10_2>
  108cc9:	movss  xmm2,DWORD PTR [rip+0xfffffffffffa897f]        # b1650 <.LCPI1_4>
  108cd1:	movss  xmm3,DWORD PTR [rip+0xfffffffffffa8a23]        # b16fc <.LCPI0_0>
  108cd9:	movss  xmm4,DWORD PTR [rip+0xfffffffffffa87ff]        # b14e0 <.LCPI0_0>
  108ce1:	movss  xmm5,DWORD PTR [rip+0xfffffffffffa899b]        # b1684 <.LCPI141_2>
  108ce9:	xor    r10d,r10d
  108cec:	nop    DWORD PTR [rax+0x0]
  108cf0:	mov    r11,rax
  108cf3:	mov    rdi,r9
  108cf6:	cs nop WORD PTR [rax+rax*1+0x0]
  108d00:	cmp    rdi,0xfffffffffffffffc
  108d04:	ja     108e55 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1d5>
  108d0a:	lea    rbx,[rdi+0x3]
  108d0e:	cmp    rbx,rdx
  108d11:	ja     108e55 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1d5>
  108d17:	movzx  ebp,BYTE PTR [rsi+rdi*1]
  108d1b:	xorps  xmm7,xmm7
  108d1e:	cvtsi2ss xmm7,ebp
  108d22:	divss  xmm7,xmm1
  108d26:	addss  xmm7,xmm2
  108d2a:	mulss  xmm7,xmm0
  108d2e:	addss  xmm7,xmm3
  108d32:	mulss  xmm7,xmm1
  108d36:	xorps  xmm6,xmm6
  108d39:	ucomiss xmm6,xmm7
  108d3c:	ja     108d49 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0xc9>
  108d3e:	ucomiss xmm7,xmm1
  108d41:	movaps xmm6,xmm7
  108d44:	jbe    108d49 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0xc9>
  108d46:	movaps xmm6,xmm1
  108d49:	ucomiss xmm6,xmm4
  108d4c:	jbe    108e48 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1c8>
  108d52:	ucomiss xmm5,xmm6
  108d55:	jbe    108e48 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1c8>
  108d5b:	movzx  ebp,BYTE PTR [rsi+rdi*1+0x1]
  108d60:	xorps  xmm8,xmm8
  108d64:	cvtsi2ss xmm8,ebp
  108d69:	divss  xmm8,xmm1
  108d6e:	addss  xmm8,xmm2
  108d73:	mulss  xmm8,xmm0
  108d78:	addss  xmm8,xmm3
  108d7d:	mulss  xmm8,xmm1
  108d82:	xorps  xmm7,xmm7
  108d85:	ucomiss xmm7,xmm8
  108d89:	ja     108d98 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x118>
  108d8b:	ucomiss xmm8,xmm1
  108d8f:	movaps xmm7,xmm8
  108d93:	jbe    108d98 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x118>
  108d95:	movaps xmm7,xmm1
  108d98:	ucomiss xmm7,xmm4
  108d9b:	jbe    108e48 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1c8>
  108da1:	ucomiss xmm5,xmm7
  108da4:	jbe    108e48 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1c8>
  108daa:	movzx  ebp,BYTE PTR [rsi+rdi*1+0x2]
  108daf:	xorps  xmm9,xmm9
  108db3:	cvtsi2ss xmm9,ebp
  108db8:	divss  xmm9,xmm1
  108dbd:	addss  xmm9,xmm2
  108dc2:	mulss  xmm9,xmm0
  108dc7:	addss  xmm9,xmm3
  108dcc:	mulss  xmm9,xmm1
  108dd1:	xorps  xmm8,xmm8
  108dd5:	ucomiss xmm8,xmm9
  108dd9:	ja     108de9 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x169>
  108ddb:	ucomiss xmm9,xmm1
  108ddf:	movaps xmm8,xmm9
  108de3:	jbe    108de9 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x169>
  108de5:	movaps xmm8,xmm1
  108de9:	ucomiss xmm8,xmm4
  108ded:	jbe    108e48 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1c8>
  108def:	ucomiss xmm5,xmm8
  108df3:	jbe    108e48 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x1c8>
  108df5:	cvttss2si ebp,xmm7
  108df9:	cvttss2si r14d,xmm6
  108dfe:	cvttss2si r15d,xmm8
  108e03:	mov    BYTE PTR [rsi+rdi*1+0x2],r15b
  108e08:	shl    r15d,0x10
  108e0c:	movzx  ebp,bpl
  108e10:	shl    ebp,0x8
  108e13:	or     ebp,r15d
  108e16:	movzx  r14d,r14b
  108e1a:	or     r14d,ebp
  108e1d:	mov    WORD PTR [rsi+rdi*1],r14w
  108e22:	mov    rdi,rbx
  108e25:	dec    r11
  108e28:	jne    108d00 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x80>
  108e2e:	inc    r10d
  108e31:	add    r9,r8
  108e34:	cmp    r10d,ecx
  108e37:	jne    108cf0 <image::imageops::colorops::contrast_in_place::<image::images::buffer::ImageBuffer<image::color::Rgb<u8>, alloc::vec::Vec<u8>>>+0x70>
  108e3d:	add    rsp,0x8
  108e41:	pop    rbx
  108e42:	pop    r14
  108e44:	pop    r15
  108e46:	pop    rbp
  108e47:	ret
  108e48:	lea    rdi,[rip+0x27a689]        # 3834d8 <anon.630eec85baab103e65858261eaf0e4ee.10.llvm.14494753593894541449>
  108e4f:	call   QWORD PTR [rip+0x29073b]        # 399590 <_RNvNtCsc36rpYXAlPq_4core6option13unwrap_failed$got>
  108e55:	lea    rsi,[rdi+0x3]
  108e59:	lea    rcx,[rip+0x27a4d8]        # 383338 <anon.f87ad866fb15b122646f249161ad5f50.26.llvm.14787641752115823920>
  108e60:	call   QWORD PTR [rip+0x290582]        # 3993e8 <_RNvNtNtCsc36rpYXAlPq_4core5slice5index16slice_index_fail$got>
  108e66:	int3
  108e67:	int3
  108e68:	int3
  108e69:	int3
  108e6a:	int3
  108e6b:	int3
  108e6c:	int3
  108e6d:	int3
  108e6e:	int3
  108e6f:	int3
