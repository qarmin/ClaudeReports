
/tmp/bench_baseline:     file format elf64-x86-64


Disassembly of section .text:

00000000001f43a0 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench>:
  1f43a0:	41 57                	push   r15
  1f43a2:	41 56                	push   r14
  1f43a4:	41 55                	push   r13
  1f43a6:	41 54                	push   r12
  1f43a8:	53                   	push   rbx
  1f43a9:	48 89 d3             	mov    rbx,rdx
  1f43ac:	49 89 f7             	mov    r15,rsi
  1f43af:	4c 8b 37             	mov    r14,QWORD PTR [rdi]
  1f43b2:	80 7f 08 01          	cmp    BYTE PTR [rdi+0x8],0x1
  1f43b6:	74 1f                	je     1f43d7 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x37>
  1f43b8:	4d 8b 26             	mov    r12,QWORD PTR [r14]
  1f43bb:	49 8b 74 24 10       	mov    rsi,QWORD PTR [r12+0x10]
  1f43c0:	49 39 34 24          	cmp    QWORD PTR [r12],rsi
  1f43c4:	74 5e                	je     1f4424 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x84>
  1f43c6:	49 8b 44 24 08       	mov    rax,QWORD PTR [r12+0x8]
  1f43cb:	c6 04 30 2c          	mov    BYTE PTR [rax+rsi*1],0x2c
  1f43cf:	48 ff c6             	inc    rsi
  1f43d2:	49 89 74 24 10       	mov    QWORD PTR [r12+0x10],rsi
  1f43d7:	c6 47 08 02          	mov    BYTE PTR [rdi+0x8],0x2
  1f43db:	4c 89 ff             	mov    rdi,r15
  1f43de:	4c 89 f6             	mov    rsi,r14
  1f43e1:	e8 3a bc 08 00       	call   280020 <_RINvXsg_NtCscP45Jb88A1K_10dicom_json3serINtB8_9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagENtNtCskd70WUOEvQG_10serde_core3ser9Serialize9serializeINtNtCslJ40HWnHMnZ_10serde_json3ser16MapKeySerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB2t_16CompactFormatterEECs9xBqbE1F8Gg_5bench>
  1f43e6:	48 85 c0             	test   rax,rax
  1f43e9:	74 0a                	je     1f43f5 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x55>
  1f43eb:	5b                   	pop    rbx
  1f43ec:	41 5c                	pop    r12
  1f43ee:	41 5d                	pop    r13
  1f43f0:	41 5e                	pop    r14
  1f43f2:	41 5f                	pop    r15
  1f43f4:	c3                   	ret
  1f43f5:	4d 8b 3e             	mov    r15,QWORD PTR [r14]
  1f43f8:	49 8b 77 10          	mov    rsi,QWORD PTR [r15+0x10]
  1f43fc:	49 39 37             	cmp    QWORD PTR [r15],rsi
  1f43ff:	74 4b                	je     1f444c <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0xac>
  1f4401:	49 8b 47 08          	mov    rax,QWORD PTR [r15+0x8]
  1f4405:	c6 04 30 3a          	mov    BYTE PTR [rax+rsi*1],0x3a
  1f4409:	48 ff c6             	inc    rsi
  1f440c:	49 89 77 10          	mov    QWORD PTR [r15+0x10],rsi
  1f4410:	48 89 df             	mov    rdi,rbx
  1f4413:	4c 89 f6             	mov    rsi,r14
  1f4416:	5b                   	pop    rbx
  1f4417:	41 5c                	pop    r12
  1f4419:	41 5d                	pop    r13
  1f441b:	41 5e                	pop    r14
  1f441d:	41 5f                	pop    r15
  1f441f:	e9 2c b8 08 00       	jmp    27fc50 <_RINvXsb_NtCscP45Jb88A1K_10dicom_json3serINtB8_9DicomJsonRINtNtCsf6WuL4uJU3Z_10dicom_core6header11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEENtNtCskd70WUOEvQG_10serde_core3ser9Serialize9serializeQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEEECs9xBqbE1F8Gg_5bench>
  1f4424:	ba 01 00 00 00       	mov    edx,0x1
  1f4429:	b9 01 00 00 00       	mov    ecx,0x1
  1f442e:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f4434:	49 89 fd             	mov    r13,rdi
  1f4437:	4c 89 e7             	mov    rdi,r12
  1f443a:	e8 a1 ec 0b 00       	call   2b30e0 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f443f:	4c 89 ef             	mov    rdi,r13
  1f4442:	49 8b 74 24 10       	mov    rsi,QWORD PTR [r12+0x10]
  1f4447:	e9 7a ff ff ff       	jmp    1f43c6 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x26>
  1f444c:	ba 01 00 00 00       	mov    edx,0x1
  1f4451:	b9 01 00 00 00       	mov    ecx,0x1
  1f4456:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f445c:	4c 89 ff             	mov    rdi,r15
  1f445f:	e8 7c ec 0b 00       	call   2b30e0 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f4464:	49 8b 77 10          	mov    rsi,QWORD PTR [r15+0x10]
  1f4468:	eb 97                	jmp    1f4401 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench+0x61>
