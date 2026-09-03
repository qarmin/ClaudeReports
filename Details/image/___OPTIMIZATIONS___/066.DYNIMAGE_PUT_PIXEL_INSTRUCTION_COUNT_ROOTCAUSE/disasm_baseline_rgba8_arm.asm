   15283:	mov    eax,DWORD PTR [rdi+0x20]
   15286:	mov    r8d,DWORD PTR [rdi+0x24]
   1528a:	cmp    edx,r8d
   1528d:	setae  r9b
   15291:	cmp    esi,eax
   15293:	jae    15756 <<image::images::dynimage::DynamicImage as image::images::generic_image::GenericImage>::put_pixel+0x776>
   15299:	test   r9b,r9b
   1529c:	jne    15762 <<image::images::dynimage::DynamicImage as image::images::generic_image::GenericImage>::put_pixel+0x782>
   152a2:	mov    edx,edx
   152a4:	imul   rdx,rax
   152a8:	mov    eax,esi
   152aa:	add    rax,rdx
   152ad:	mov    rdx,QWORD PTR [rdi+0x18]
   152b1:	lea    rsi,[rax*4+0x3]
   152b9:	shl    rax,0x2
   152bd:	cmp    rsi,rdx
   152c0:	jae    15a38 <<image::images::dynimage::DynamicImage as image::images::generic_image::GenericImage>::put_pixel+0xa58>
   152c6:	mov    rdx,QWORD PTR [rdi+0x10]
   152ca:	mov    DWORD PTR [rdx+rax*1],ecx
   152cd:	add    rsp,0x30
   152d1:	pop    rbx
   152d2:	ret
