
/tmp/bench_patched:     file format elf64-x86-64


Disassembly of section .text:

00000000001f53a0 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench>:
  1f53a0:	41 57                	push   r15
  1f53a2:	41 56                	push   r14
  1f53a4:	41 55                	push   r13
  1f53a6:	41 54                	push   r12
  1f53a8:	53                   	push   rbx
  1f53a9:	48 83 ec 10          	sub    rsp,0x10
  1f53ad:	48 89 d3             	mov    rbx,rdx
  1f53b0:	4c 8b 37             	mov    r14,QWORD PTR [rdi]
  1f53b3:	80 7f 08 01          	cmp    BYTE PTR [rdi+0x8],0x1
  1f53b7:	74 1f                	je     1f53d8 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x38>
  1f53b9:	4d 8b 3e             	mov    r15,QWORD PTR [r14]
  1f53bc:	49 8b 47 10          	mov    rax,QWORD PTR [r15+0x10]
  1f53c0:	49 39 07             	cmp    QWORD PTR [r15],rax
  1f53c3:	0f 84 15 01 00 00    	je     1f54de <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x13e>
  1f53c9:	49 8b 4f 08          	mov    rcx,QWORD PTR [r15+0x8]
  1f53cd:	c6 04 01 2c          	mov    BYTE PTR [rcx+rax*1],0x2c
  1f53d1:	48 ff c0             	inc    rax
  1f53d4:	49 89 47 10          	mov    QWORD PTR [r15+0x10],rax
  1f53d8:	c6 47 08 02          	mov    BYTE PTR [rdi+0x8],0x2
  1f53dc:	0f b7 16             	movzx  edx,WORD PTR [rsi]
  1f53df:	44 0f b7 4e 02       	movzx  r9d,WORD PTR [rsi+0x2]
  1f53e4:	89 d0                	mov    eax,edx
  1f53e6:	c1 e8 0c             	shr    eax,0xc
  1f53e9:	48 8d 35 f0 b0 fc ff 	lea    rsi,[rip+0xfffffffffffcb0f0]        # 1c04e0 <anon.c8edfd5acdbb664c31c9bcad33866596.80.llvm.12192744414517703898>
  1f53f0:	0f b6 0c 30          	movzx  ecx,BYTE PTR [rax+rsi*1]
  1f53f4:	89 d0                	mov    eax,edx
  1f53f6:	c1 e8 08             	shr    eax,0x8
  1f53f9:	83 e0 0f             	and    eax,0xf
  1f53fc:	0f b6 04 30          	movzx  eax,BYTE PTR [rax+rsi*1]
  1f5400:	89 d7                	mov    edi,edx
  1f5402:	c1 ef 04             	shr    edi,0x4
  1f5405:	83 e7 0f             	and    edi,0xf
  1f5408:	0f b6 3c 37          	movzx  edi,BYTE PTR [rdi+rsi*1]
  1f540c:	83 e2 0f             	and    edx,0xf
  1f540f:	45 89 c8             	mov    r8d,r9d
  1f5412:	41 c1 e8 0c          	shr    r8d,0xc
  1f5416:	45 0f b6 04 30       	movzx  r8d,BYTE PTR [r8+rsi*1]
  1f541b:	45 89 ca             	mov    r10d,r9d
  1f541e:	41 c1 ea 08          	shr    r10d,0x8
  1f5422:	41 83 e2 0f          	and    r10d,0xf
  1f5426:	45 0f b6 14 32       	movzx  r10d,BYTE PTR [r10+rsi*1]
  1f542b:	45 89 cf             	mov    r15d,r9d
  1f542e:	41 83 e7 0f          	and    r15d,0xf
  1f5432:	41 c1 e9 04          	shr    r9d,0x4
  1f5436:	41 83 e1 0f          	and    r9d,0xf
  1f543a:	45 0f b6 1c 31       	movzx  r11d,BYTE PTR [r9+rsi*1]
  1f543f:	45 0f b6 0c 37       	movzx  r9d,BYTE PTR [r15+rsi*1]
  1f5444:	49 c1 e1 38          	shl    r9,0x38
  1f5448:	49 c1 e3 30          	shl    r11,0x30
  1f544c:	49 c1 e2 28          	shl    r10,0x28
  1f5450:	49 c1 e0 20          	shl    r8,0x20
  1f5454:	0f b6 14 32          	movzx  edx,BYTE PTR [rdx+rsi*1]
  1f5458:	c1 e2 18             	shl    edx,0x18
  1f545b:	c1 e7 10             	shl    edi,0x10
  1f545e:	c1 e0 08             	shl    eax,0x8
  1f5461:	48 09 c8             	or     rax,rcx
  1f5464:	48 09 f8             	or     rax,rdi
  1f5467:	48 09 d0             	or     rax,rdx
  1f546a:	4c 09 c0             	or     rax,r8
  1f546d:	4c 09 d0             	or     rax,r10
  1f5470:	4c 09 d8             	or     rax,r11
  1f5473:	4c 09 c8             	or     rax,r9
  1f5476:	48 89 44 24 08       	mov    QWORD PTR [rsp+0x8],rax
  1f547b:	48 8d 54 24 08       	lea    rdx,[rsp+0x8]
  1f5480:	b9 08 00 00 00       	mov    ecx,0x8
  1f5485:	4c 89 f7             	mov    rdi,r14
  1f5488:	e8 e3 c4 ff ff       	call   1f1970 <_RINvNtCslJ40HWnHMnZ_10serde_json3ser18format_escaped_strQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB2_16CompactFormatterECs9xBqbE1F8Gg_5bench>
  1f548d:	4d 8b 3e             	mov    r15,QWORD PTR [r14]
  1f5490:	49 8b 77 10          	mov    rsi,QWORD PTR [r15+0x10]
  1f5494:	49 39 37             	cmp    QWORD PTR [r15],rsi
  1f5497:	74 27                	je     1f54c0 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x120>
  1f5499:	49 8b 47 08          	mov    rax,QWORD PTR [r15+0x8]
  1f549d:	c6 04 30 3a          	mov    BYTE PTR [rax+rsi*1],0x3a
  1f54a1:	48 ff c6             	inc    rsi
  1f54a4:	49 89 77 10          	mov    QWORD PTR [r15+0x10],rsi
  1f54a8:	48 89 df             	mov    rdi,rbx
  1f54ab:	4c 89 f6             	mov    rsi,r14
  1f54ae:	48 83 c4 10          	add    rsp,0x10
  1f54b2:	5b                   	pop    rbx
  1f54b3:	41 5c                	pop    r12
  1f54b5:	41 5d                	pop    r13
  1f54b7:	41 5e                	pop    r14
  1f54b9:	41 5f                	pop    r15
  1f54bb:	e9 b0 1e 06 00       	jmp    257370 <_RINvXsb_NtCscP45Jb88A1K_10dicom_json3serINtB8_9DicomJsonRINtNtCsf6WuL4uJU3Z_10dicom_core6header11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEENtNtCskd70WUOEvQG_10serde_core3ser9Serialize9serializeQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEEECs9xBqbE1F8Gg_5bench>
  1f54c0:	ba 01 00 00 00       	mov    edx,0x1
  1f54c5:	b9 01 00 00 00       	mov    ecx,0x1
  1f54ca:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f54d0:	4c 89 ff             	mov    rdi,r15
  1f54d3:	e8 28 b1 08 00       	call   280600 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f54d8:	49 8b 77 10          	mov    rsi,QWORD PTR [r15+0x10]
  1f54dc:	eb bb                	jmp    1f5499 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0xf9>
  1f54de:	ba 01 00 00 00       	mov    edx,0x1
  1f54e3:	b9 01 00 00 00       	mov    ecx,0x1
  1f54e8:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f54ee:	49 89 fc             	mov    r12,rdi
  1f54f1:	4c 89 ff             	mov    rdi,r15
  1f54f4:	49 89 f5             	mov    r13,rsi
  1f54f7:	48 89 c6             	mov    rsi,rax
  1f54fa:	e8 01 b1 08 00       	call   280600 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f54ff:	4c 89 e7             	mov    rdi,r12
  1f5502:	4c 89 ee             	mov    rsi,r13
  1f5505:	49 8b 47 10          	mov    rax,QWORD PTR [r15+0x10]
  1f5509:	e9 bb fe ff ff       	jmp    1f53c9 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x29>
