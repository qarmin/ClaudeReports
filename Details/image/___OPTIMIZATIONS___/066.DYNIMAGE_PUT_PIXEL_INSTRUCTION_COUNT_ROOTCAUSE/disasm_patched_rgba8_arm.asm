   15113:	mov    eax,DWORD PTR [rdi+0x20]
   15116:	mov    r10d,DWORD PTR [rdi+0x24]
   1511a:	cmp    edx,r10d
   1511d:	setae  r11b
   15121:	cmp    esi,eax
   15123:	jae    155ff <<image::images::dynimage::DynamicImage as image::images::generic_image::GenericImage>::put_pixel+0x78f>
   15129:	test   r11b,r11b
   1512c:	jne    1560b <<image::images::dynimage::DynamicImage as image::images::generic_image::GenericImage>::put_pixel+0x79b>
   15132:	mov    edx,edx
   15134:	imul   rdx,rax
   15138:	mov    eax,esi
   1513a:	add    rax,rdx
   1513d:	mov    rdx,QWORD PTR [rdi+0x18]
   15141:	lea    rsi,[rax*4+0x3]
   15149:	shl    rax,0x2
   1514d:	cmp    rsi,rdx
   15150:	jae    158e1 <<image::images::dynimage::DynamicImage as image::images::generic_image::GenericImage>::put_pixel+0xa71>
   15156:	mov    rdx,QWORD PTR [rdi+0x10]
   1515a:	shl    r8d,0x8
   1515e:	movzx  esi,r9b
   15162:	movzx  edi,r8w
   15166:	or     edi,esi
   15168:	shl    edi,0x8
   1516b:	and    ecx,0xff0000ff
   15171:	or     ecx,edi
   15173:	mov    DWORD PTR [rdx+rax*1],ecx
   15176:	add    rsp,0x30
   1517a:	pop    rbx
   1517b:	ret
