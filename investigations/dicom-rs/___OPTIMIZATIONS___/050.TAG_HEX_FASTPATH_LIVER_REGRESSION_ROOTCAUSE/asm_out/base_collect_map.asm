
/tmp/bench_baseline:     file format elf64-x86-64


Disassembly of section .text:

00000000001f4a90 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench>:
  1f4a90:	55                   	push   rbp
  1f4a91:	41 57                	push   r15
  1f4a93:	41 56                	push   r14
  1f4a95:	41 55                	push   r13
  1f4a97:	41 54                	push   r12
  1f4a99:	53                   	push   rbx
  1f4a9a:	48 83 ec 28          	sub    rsp,0x28
  1f4a9e:	4c 8b 3e             	mov    r15,QWORD PTR [rsi]
  1f4aa1:	48 8b 6e 08          	mov    rbp,QWORD PTR [rsi+0x8]
  1f4aa5:	4c 8b 76 10          	mov    r14,QWORD PTR [rsi+0x10]
  1f4aa9:	4c 8b 6e 18          	mov    r13,QWORD PTR [rsi+0x18]
  1f4aad:	4c 8b 66 40          	mov    r12,QWORD PTR [rsi+0x40]
  1f4ab1:	48 8b 1f             	mov    rbx,QWORD PTR [rdi]
  1f4ab4:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f4ab8:	48 39 33             	cmp    QWORD PTR [rbx],rsi
  1f4abb:	0f 84 76 02 00 00    	je     1f4d37 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x2a7>
  1f4ac1:	48 8b 43 08          	mov    rax,QWORD PTR [rbx+0x8]
  1f4ac5:	c6 04 30 7b          	mov    BYTE PTR [rax+rsi*1],0x7b
  1f4ac9:	48 ff c6             	inc    rsi
  1f4acc:	48 89 73 10          	mov    QWORD PTR [rbx+0x10],rsi
  1f4ad0:	b0 01                	mov    al,0x1
  1f4ad2:	4d 85 e4             	test   r12,r12
  1f4ad5:	75 1a                	jne    1f4af1 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x61>
  1f4ad7:	48 39 33             	cmp    QWORD PTR [rbx],rsi
  1f4ada:	0f 84 8f 02 00 00    	je     1f4d6f <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x2df>
  1f4ae0:	48 8b 43 08          	mov    rax,QWORD PTR [rbx+0x8]
  1f4ae4:	c6 04 30 7d          	mov    BYTE PTR [rax+rsi*1],0x7d
  1f4ae8:	48 ff c6             	inc    rsi
  1f4aeb:	48 89 73 10          	mov    QWORD PTR [rbx+0x10],rsi
  1f4aef:	31 c0                	xor    eax,eax
  1f4af1:	48 89 7c 24 10       	mov    QWORD PTR [rsp+0x10],rdi
  1f4af6:	88 44 24 18          	mov    BYTE PTR [rsp+0x18],al
  1f4afa:	eb 49                	jmp    1f4b45 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0xb5>
  1f4afc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  1f4b00:	49 8d 5d 01          	lea    rbx,[r13+0x1]
  1f4b04:	48 89 c5             	mov    rbp,rax
  1f4b07:	49 6b cd 70          	imul   rcx,r13,0x70
  1f4b0b:	48 8d 14 08          	lea    rdx,[rax+rcx*1]
  1f4b0f:	8b 44 08 60          	mov    eax,DWORD PTR [rax+rcx*1+0x60]
  1f4b13:	89 44 24 0c          	mov    DWORD PTR [rsp+0xc],eax
  1f4b17:	48 89 54 24 20       	mov    QWORD PTR [rsp+0x20],rdx
  1f4b1c:	48 8d 7c 24 10       	lea    rdi,[rsp+0x10]
  1f4b21:	48 8d 74 24 0c       	lea    rsi,[rsp+0xc]
  1f4b26:	48 8d 54 24 20       	lea    rdx,[rsp+0x20]
  1f4b2b:	e8 70 f8 ff ff       	call   1f43a0 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench>
  1f4b30:	41 b7 01             	mov    r15b,0x1
  1f4b33:	41 be 00 00 00 00    	mov    r14d,0x0
  1f4b39:	49 89 dd             	mov    r13,rbx
  1f4b3c:	48 85 c0             	test   rax,rax
  1f4b3f:	0f 85 e3 01 00 00    	jne    1f4d28 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x298>
  1f4b45:	49 83 ec 01          	sub    r12,0x1
  1f4b49:	0f 82 ac 01 00 00    	jb     1f4cfb <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x26b>
  1f4b4f:	41 f6 c7 01          	test   r15b,0x1
  1f4b53:	0f 84 60 02 00 00    	je     1f4db9 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x329>
  1f4b59:	48 85 ed             	test   rbp,rbp
  1f4b5c:	74 22                	je     1f4b80 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0xf0>
  1f4b5e:	0f b7 85 06 05 00 00 	movzx  eax,WORD PTR [rbp+0x506]
  1f4b65:	49 39 c5             	cmp    r13,rax
  1f4b68:	0f 83 c2 00 00 00    	jae    1f4c30 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1a0>
  1f4b6e:	48 89 e8             	mov    rax,rbp
  1f4b71:	e9 e2 00 00 00       	jmp    1f4c58 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1c8>
  1f4b76:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
  1f4b7d:	00 00 00 
  1f4b80:	4d 85 ed             	test   r13,r13
  1f4b83:	74 38                	je     1f4bbd <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x12d>
  1f4b85:	4c 89 e8             	mov    rax,r13
  1f4b88:	4c 89 f5             	mov    rbp,r14
  1f4b8b:	48 83 e0 07          	and    rax,0x7
  1f4b8f:	74 31                	je     1f4bc2 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x132>
  1f4b91:	31 c9                	xor    ecx,ecx
  1f4b93:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  1f4b9a:	84 00 00 00 00 00 
  1f4ba0:	48 8b ad 08 05 00 00 	mov    rbp,QWORD PTR [rbp+0x508]
  1f4ba7:	48 ff c1             	inc    rcx
  1f4baa:	48 39 c8             	cmp    rax,rcx
  1f4bad:	75 f1                	jne    1f4ba0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x110>
  1f4baf:	4c 89 e8             	mov    rax,r13
  1f4bb2:	48 29 c8             	sub    rax,rcx
  1f4bb5:	49 83 fd 08          	cmp    r13,0x8
  1f4bb9:	73 15                	jae    1f4bd0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x140>
  1f4bbb:	eb 51                	jmp    1f4c0e <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x17e>
  1f4bbd:	4c 89 f5             	mov    rbp,r14
  1f4bc0:	eb 4c                	jmp    1f4c0e <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x17e>
  1f4bc2:	4c 89 e8             	mov    rax,r13
  1f4bc5:	49 83 fd 08          	cmp    r13,0x8
  1f4bc9:	72 43                	jb     1f4c0e <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x17e>
  1f4bcb:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  1f4bd0:	48 8b 8d 08 05 00 00 	mov    rcx,QWORD PTR [rbp+0x508]
  1f4bd7:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4bde:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4be5:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4bec:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4bf3:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4bfa:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4c01:	48 8b a9 08 05 00 00 	mov    rbp,QWORD PTR [rcx+0x508]
  1f4c08:	48 83 c0 f8          	add    rax,0xfffffffffffffff8
  1f4c0c:	75 c2                	jne    1f4bd0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x140>
  1f4c0e:	45 31 f6             	xor    r14d,r14d
  1f4c11:	45 31 ed             	xor    r13d,r13d
  1f4c14:	0f b7 85 06 05 00 00 	movzx  eax,WORD PTR [rbp+0x506]
  1f4c1b:	49 39 c5             	cmp    r13,rax
  1f4c1e:	0f 82 4a ff ff ff    	jb     1f4b6e <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0xde>
  1f4c24:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  1f4c2b:	00 00 00 00 00 
  1f4c30:	48 8b 85 d0 04 00 00 	mov    rax,QWORD PTR [rbp+0x4d0]
  1f4c37:	48 85 c0             	test   rax,rax
  1f4c3a:	0f 84 20 01 00 00    	je     1f4d60 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x2d0>
  1f4c40:	49 ff c6             	inc    r14
  1f4c43:	44 0f b7 ad 04 05 00 	movzx  r13d,WORD PTR [rbp+0x504]
  1f4c4a:	00 
  1f4c4b:	48 89 c5             	mov    rbp,rax
  1f4c4e:	66 44 3b a8 06 05 00 	cmp    r13w,WORD PTR [rax+0x506]
  1f4c55:	00 
  1f4c56:	73 d8                	jae    1f4c30 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1a0>
  1f4c58:	4d 85 f6             	test   r14,r14
  1f4c5b:	0f 84 9f fe ff ff    	je     1f4b00 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x70>
  1f4c61:	4a 8d 0c e8          	lea    rcx,[rax+r13*8]
  1f4c65:	48 81 c1 10 05 00 00 	add    rcx,0x510
  1f4c6c:	4c 89 f2             	mov    rdx,r14
  1f4c6f:	48 83 e2 07          	and    rdx,0x7
  1f4c73:	74 7b                	je     1f4cf0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x260>
  1f4c75:	31 f6                	xor    esi,esi
  1f4c77:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
  1f4c7e:	00 00 
  1f4c80:	48 8b 29             	mov    rbp,QWORD PTR [rcx]
  1f4c83:	48 8d 8d 08 05 00 00 	lea    rcx,[rbp+0x508]
  1f4c8a:	48 ff c6             	inc    rsi
  1f4c8d:	48 39 f2             	cmp    rdx,rsi
  1f4c90:	75 ee                	jne    1f4c80 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1f0>
  1f4c92:	4c 89 f2             	mov    rdx,r14
  1f4c95:	48 29 f2             	sub    rdx,rsi
  1f4c98:	49 83 fe 08          	cmp    r14,0x8
  1f4c9c:	72 43                	jb     1f4ce1 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x251>
  1f4c9e:	66 90                	xchg   ax,ax
  1f4ca0:	48 8b 09             	mov    rcx,QWORD PTR [rcx]
  1f4ca3:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4caa:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4cb1:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4cb8:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4cbf:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4cc6:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f4ccd:	48 8b a9 08 05 00 00 	mov    rbp,QWORD PTR [rcx+0x508]
  1f4cd4:	48 8d 8d 08 05 00 00 	lea    rcx,[rbp+0x508]
  1f4cdb:	48 83 c2 f8          	add    rdx,0xfffffffffffffff8
  1f4cdf:	75 bf                	jne    1f4ca0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x210>
  1f4ce1:	31 db                	xor    ebx,ebx
  1f4ce3:	e9 1f fe ff ff       	jmp    1f4b07 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x77>
  1f4ce8:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
  1f4cef:	00 
  1f4cf0:	4c 89 f2             	mov    rdx,r14
  1f4cf3:	49 83 fe 08          	cmp    r14,0x8
  1f4cf7:	73 a7                	jae    1f4ca0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x210>
  1f4cf9:	eb e6                	jmp    1f4ce1 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x251>
  1f4cfb:	80 7c 24 18 00       	cmp    BYTE PTR [rsp+0x18],0x0
  1f4d00:	74 24                	je     1f4d26 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x296>
  1f4d02:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  1f4d07:	48 8b 18             	mov    rbx,QWORD PTR [rax]
  1f4d0a:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f4d0e:	48 39 33             	cmp    QWORD PTR [rbx],rsi
  1f4d11:	0f 84 81 00 00 00    	je     1f4d98 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x308>
  1f4d17:	48 8b 43 08          	mov    rax,QWORD PTR [rbx+0x8]
  1f4d1b:	c6 04 30 7d          	mov    BYTE PTR [rax+rsi*1],0x7d
  1f4d1f:	48 ff c6             	inc    rsi
  1f4d22:	48 89 73 10          	mov    QWORD PTR [rbx+0x10],rsi
  1f4d26:	31 c0                	xor    eax,eax
  1f4d28:	48 83 c4 28          	add    rsp,0x28
  1f4d2c:	5b                   	pop    rbx
  1f4d2d:	41 5c                	pop    r12
  1f4d2f:	41 5d                	pop    r13
  1f4d31:	41 5e                	pop    r14
  1f4d33:	41 5f                	pop    r15
  1f4d35:	5d                   	pop    rbp
  1f4d36:	c3                   	ret
  1f4d37:	ba 01 00 00 00       	mov    edx,0x1
  1f4d3c:	b9 01 00 00 00       	mov    ecx,0x1
  1f4d41:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f4d47:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
  1f4d4b:	48 89 df             	mov    rdi,rbx
  1f4d4e:	e8 8d e3 0b 00       	call   2b30e0 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f4d53:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
  1f4d57:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f4d5b:	e9 61 fd ff ff       	jmp    1f4ac1 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x31>
  1f4d60:	48 8d 3d 39 3e 36 00 	lea    rdi,[rip+0x363e39]        # 558ba0 <anon.b4834912152ca6a00aac418037a14b3b.51.llvm.15879696041251167922>
  1f4d67:	ff 15 93 7d 3a 00    	call   QWORD PTR [rip+0x3a7d93]        # 59cb00 <_RNvNtCsc36rpYXAlPq_4core6option13unwrap_failed$got>
  1f4d6d:	0f 0b                	ud2
  1f4d6f:	ba 01 00 00 00       	mov    edx,0x1
  1f4d74:	b9 01 00 00 00       	mov    ecx,0x1
  1f4d79:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f4d7f:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
  1f4d83:	48 89 df             	mov    rdi,rbx
  1f4d86:	e8 55 e3 0b 00       	call   2b30e0 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f4d8b:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
  1f4d8f:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f4d93:	e9 48 fd ff ff       	jmp    1f4ae0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x50>
  1f4d98:	ba 01 00 00 00       	mov    edx,0x1
  1f4d9d:	b9 01 00 00 00       	mov    ecx,0x1
  1f4da2:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f4da8:	48 89 df             	mov    rdi,rbx
  1f4dab:	e8 30 e3 0b 00       	call   2b30e0 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f4db0:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f4db4:	e9 5e ff ff ff       	jmp    1f4d17 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x287>
  1f4db9:	48 8d 3d a0 3f 36 00 	lea    rdi,[rip+0x363fa0]        # 558d60 <anon.b4834912152ca6a00aac418037a14b3b.82.llvm.15879696041251167922>
  1f4dc0:	ff 15 3a 7d 3a 00    	call   QWORD PTR [rip+0x3a7d3a]        # 59cb00 <_RNvNtCsc36rpYXAlPq_4core6option13unwrap_failed$got>
  1f4dc6:	0f 0b                	ud2
