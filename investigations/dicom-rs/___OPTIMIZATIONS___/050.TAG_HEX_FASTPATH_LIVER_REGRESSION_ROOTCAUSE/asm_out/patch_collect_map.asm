
/tmp/bench_patched:     file format elf64-x86-64


Disassembly of section .text:

00000000001f5b30 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench>:
  1f5b30:	55                   	push   rbp
  1f5b31:	41 57                	push   r15
  1f5b33:	41 56                	push   r14
  1f5b35:	41 55                	push   r13
  1f5b37:	41 54                	push   r12
  1f5b39:	53                   	push   rbx
  1f5b3a:	48 83 ec 28          	sub    rsp,0x28
  1f5b3e:	4c 8b 3e             	mov    r15,QWORD PTR [rsi]
  1f5b41:	48 8b 6e 08          	mov    rbp,QWORD PTR [rsi+0x8]
  1f5b45:	4c 8b 76 10          	mov    r14,QWORD PTR [rsi+0x10]
  1f5b49:	4c 8b 6e 18          	mov    r13,QWORD PTR [rsi+0x18]
  1f5b4d:	4c 8b 66 40          	mov    r12,QWORD PTR [rsi+0x40]
  1f5b51:	48 8b 1f             	mov    rbx,QWORD PTR [rdi]
  1f5b54:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f5b58:	48 39 33             	cmp    QWORD PTR [rbx],rsi
  1f5b5b:	0f 84 76 02 00 00    	je     1f5dd7 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x2a7>
  1f5b61:	48 8b 43 08          	mov    rax,QWORD PTR [rbx+0x8]
  1f5b65:	c6 04 30 7b          	mov    BYTE PTR [rax+rsi*1],0x7b
  1f5b69:	48 ff c6             	inc    rsi
  1f5b6c:	48 89 73 10          	mov    QWORD PTR [rbx+0x10],rsi
  1f5b70:	b0 01                	mov    al,0x1
  1f5b72:	4d 85 e4             	test   r12,r12
  1f5b75:	75 1a                	jne    1f5b91 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x61>
  1f5b77:	48 39 33             	cmp    QWORD PTR [rbx],rsi
  1f5b7a:	0f 84 8f 02 00 00    	je     1f5e0f <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x2df>
  1f5b80:	48 8b 43 08          	mov    rax,QWORD PTR [rbx+0x8]
  1f5b84:	c6 04 30 7d          	mov    BYTE PTR [rax+rsi*1],0x7d
  1f5b88:	48 ff c6             	inc    rsi
  1f5b8b:	48 89 73 10          	mov    QWORD PTR [rbx+0x10],rsi
  1f5b8f:	31 c0                	xor    eax,eax
  1f5b91:	48 89 7c 24 10       	mov    QWORD PTR [rsp+0x10],rdi
  1f5b96:	88 44 24 18          	mov    BYTE PTR [rsp+0x18],al
  1f5b9a:	eb 49                	jmp    1f5be5 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0xb5>
  1f5b9c:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
  1f5ba0:	49 8d 5d 01          	lea    rbx,[r13+0x1]
  1f5ba4:	48 89 c5             	mov    rbp,rax
  1f5ba7:	49 6b cd 70          	imul   rcx,r13,0x70
  1f5bab:	48 8d 14 08          	lea    rdx,[rax+rcx*1]
  1f5baf:	8b 44 08 60          	mov    eax,DWORD PTR [rax+rcx*1+0x60]
  1f5bb3:	89 44 24 0c          	mov    DWORD PTR [rsp+0xc],eax
  1f5bb7:	48 89 54 24 20       	mov    QWORD PTR [rsp+0x20],rdx
  1f5bbc:	48 8d 7c 24 10       	lea    rdi,[rsp+0x10]
  1f5bc1:	48 8d 74 24 0c       	lea    rsi,[rsp+0xc]
  1f5bc6:	48 8d 54 24 20       	lea    rdx,[rsp+0x20]
  1f5bcb:	e8 d0 f7 ff ff       	call   1f53a0 <_RINvYINtNtCslJ40HWnHMnZ_10serde_json3ser8CompoundQINtNtCscHgRw1M2fX5_5alloc3vec3VechENtB6_16CompactFormatterENtNtCskd70WUOEvQG_10serde_core3ser12SerializeMap15serialize_entryINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2N_RINtB3r_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEECs9xBqbE1F8Gg_5bench>
  1f5bd0:	41 b7 01             	mov    r15b,0x1
  1f5bd3:	41 be 00 00 00 00    	mov    r14d,0x0
  1f5bd9:	49 89 dd             	mov    r13,rbx
  1f5bdc:	48 85 c0             	test   rax,rax
  1f5bdf:	0f 85 e3 01 00 00    	jne    1f5dc8 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x298>
  1f5be5:	49 83 ec 01          	sub    r12,0x1
  1f5be9:	0f 82 ac 01 00 00    	jb     1f5d9b <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x26b>
  1f5bef:	41 f6 c7 01          	test   r15b,0x1
  1f5bf3:	0f 84 60 02 00 00    	je     1f5e59 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x329>
  1f5bf9:	48 85 ed             	test   rbp,rbp
  1f5bfc:	74 22                	je     1f5c20 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0xf0>
  1f5bfe:	0f b7 85 06 05 00 00 	movzx  eax,WORD PTR [rbp+0x506]
  1f5c05:	49 39 c5             	cmp    r13,rax
  1f5c08:	0f 83 c2 00 00 00    	jae    1f5cd0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1a0>
  1f5c0e:	48 89 e8             	mov    rax,rbp
  1f5c11:	e9 e2 00 00 00       	jmp    1f5cf8 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1c8>
  1f5c16:	66 2e 0f 1f 84 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
  1f5c1d:	00 00 00 
  1f5c20:	4d 85 ed             	test   r13,r13
  1f5c23:	74 38                	je     1f5c5d <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x12d>
  1f5c25:	4c 89 e8             	mov    rax,r13
  1f5c28:	4c 89 f5             	mov    rbp,r14
  1f5c2b:	48 83 e0 07          	and    rax,0x7
  1f5c2f:	74 31                	je     1f5c62 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x132>
  1f5c31:	31 c9                	xor    ecx,ecx
  1f5c33:	66 66 66 66 2e 0f 1f 	data16 data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  1f5c3a:	84 00 00 00 00 00 
  1f5c40:	48 8b ad 08 05 00 00 	mov    rbp,QWORD PTR [rbp+0x508]
  1f5c47:	48 ff c1             	inc    rcx
  1f5c4a:	48 39 c8             	cmp    rax,rcx
  1f5c4d:	75 f1                	jne    1f5c40 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x110>
  1f5c4f:	4c 89 e8             	mov    rax,r13
  1f5c52:	48 29 c8             	sub    rax,rcx
  1f5c55:	49 83 fd 08          	cmp    r13,0x8
  1f5c59:	73 15                	jae    1f5c70 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x140>
  1f5c5b:	eb 51                	jmp    1f5cae <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x17e>
  1f5c5d:	4c 89 f5             	mov    rbp,r14
  1f5c60:	eb 4c                	jmp    1f5cae <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x17e>
  1f5c62:	4c 89 e8             	mov    rax,r13
  1f5c65:	49 83 fd 08          	cmp    r13,0x8
  1f5c69:	72 43                	jb     1f5cae <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x17e>
  1f5c6b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  1f5c70:	48 8b 8d 08 05 00 00 	mov    rcx,QWORD PTR [rbp+0x508]
  1f5c77:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5c7e:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5c85:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5c8c:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5c93:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5c9a:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5ca1:	48 8b a9 08 05 00 00 	mov    rbp,QWORD PTR [rcx+0x508]
  1f5ca8:	48 83 c0 f8          	add    rax,0xfffffffffffffff8
  1f5cac:	75 c2                	jne    1f5c70 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x140>
  1f5cae:	45 31 f6             	xor    r14d,r14d
  1f5cb1:	45 31 ed             	xor    r13d,r13d
  1f5cb4:	0f b7 85 06 05 00 00 	movzx  eax,WORD PTR [rbp+0x506]
  1f5cbb:	49 39 c5             	cmp    r13,rax
  1f5cbe:	0f 82 4a ff ff ff    	jb     1f5c0e <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0xde>
  1f5cc4:	66 66 66 2e 0f 1f 84 	data16 data16 cs nop WORD PTR [rax+rax*1+0x0]
  1f5ccb:	00 00 00 00 00 
  1f5cd0:	48 8b 85 d0 04 00 00 	mov    rax,QWORD PTR [rbp+0x4d0]
  1f5cd7:	48 85 c0             	test   rax,rax
  1f5cda:	0f 84 20 01 00 00    	je     1f5e00 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x2d0>
  1f5ce0:	49 ff c6             	inc    r14
  1f5ce3:	44 0f b7 ad 04 05 00 	movzx  r13d,WORD PTR [rbp+0x504]
  1f5cea:	00 
  1f5ceb:	48 89 c5             	mov    rbp,rax
  1f5cee:	66 44 3b a8 06 05 00 	cmp    r13w,WORD PTR [rax+0x506]
  1f5cf5:	00 
  1f5cf6:	73 d8                	jae    1f5cd0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1a0>
  1f5cf8:	4d 85 f6             	test   r14,r14
  1f5cfb:	0f 84 9f fe ff ff    	je     1f5ba0 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x70>
  1f5d01:	4a 8d 0c e8          	lea    rcx,[rax+r13*8]
  1f5d05:	48 81 c1 10 05 00 00 	add    rcx,0x510
  1f5d0c:	4c 89 f2             	mov    rdx,r14
  1f5d0f:	48 83 e2 07          	and    rdx,0x7
  1f5d13:	74 7b                	je     1f5d90 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x260>
  1f5d15:	31 f6                	xor    esi,esi
  1f5d17:	66 0f 1f 84 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
  1f5d1e:	00 00 
  1f5d20:	48 8b 29             	mov    rbp,QWORD PTR [rcx]
  1f5d23:	48 8d 8d 08 05 00 00 	lea    rcx,[rbp+0x508]
  1f5d2a:	48 ff c6             	inc    rsi
  1f5d2d:	48 39 f2             	cmp    rdx,rsi
  1f5d30:	75 ee                	jne    1f5d20 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x1f0>
  1f5d32:	4c 89 f2             	mov    rdx,r14
  1f5d35:	48 29 f2             	sub    rdx,rsi
  1f5d38:	49 83 fe 08          	cmp    r14,0x8
  1f5d3c:	72 43                	jb     1f5d81 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x251>
  1f5d3e:	66 90                	xchg   ax,ax
  1f5d40:	48 8b 09             	mov    rcx,QWORD PTR [rcx]
  1f5d43:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5d4a:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5d51:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5d58:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5d5f:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5d66:	48 8b 89 08 05 00 00 	mov    rcx,QWORD PTR [rcx+0x508]
  1f5d6d:	48 8b a9 08 05 00 00 	mov    rbp,QWORD PTR [rcx+0x508]
  1f5d74:	48 8d 8d 08 05 00 00 	lea    rcx,[rbp+0x508]
  1f5d7b:	48 83 c2 f8          	add    rdx,0xfffffffffffffff8
  1f5d7f:	75 bf                	jne    1f5d40 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x210>
  1f5d81:	31 db                	xor    ebx,ebx
  1f5d83:	e9 1f fe ff ff       	jmp    1f5ba7 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x77>
  1f5d88:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
  1f5d8f:	00 
  1f5d90:	4c 89 f2             	mov    rdx,r14
  1f5d93:	49 83 fe 08          	cmp    r14,0x8
  1f5d97:	73 a7                	jae    1f5d40 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x210>
  1f5d99:	eb e6                	jmp    1f5d81 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x251>
  1f5d9b:	80 7c 24 18 00       	cmp    BYTE PTR [rsp+0x18],0x0
  1f5da0:	74 24                	je     1f5dc6 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x296>
  1f5da2:	48 8b 44 24 10       	mov    rax,QWORD PTR [rsp+0x10]
  1f5da7:	48 8b 18             	mov    rbx,QWORD PTR [rax]
  1f5daa:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f5dae:	48 39 33             	cmp    QWORD PTR [rbx],rsi
  1f5db1:	0f 84 81 00 00 00    	je     1f5e38 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x308>
  1f5db7:	48 8b 43 08          	mov    rax,QWORD PTR [rbx+0x8]
  1f5dbb:	c6 04 30 7d          	mov    BYTE PTR [rax+rsi*1],0x7d
  1f5dbf:	48 ff c6             	inc    rsi
  1f5dc2:	48 89 73 10          	mov    QWORD PTR [rbx+0x10],rsi
  1f5dc6:	31 c0                	xor    eax,eax
  1f5dc8:	48 83 c4 28          	add    rsp,0x28
  1f5dcc:	5b                   	pop    rbx
  1f5dcd:	41 5c                	pop    r12
  1f5dcf:	41 5d                	pop    r13
  1f5dd1:	41 5e                	pop    r14
  1f5dd3:	41 5f                	pop    r15
  1f5dd5:	5d                   	pop    rbp
  1f5dd6:	c3                   	ret
  1f5dd7:	ba 01 00 00 00       	mov    edx,0x1
  1f5ddc:	b9 01 00 00 00       	mov    ecx,0x1
  1f5de1:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f5de7:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
  1f5deb:	48 89 df             	mov    rdi,rbx
  1f5dee:	e8 0d a8 08 00       	call   280600 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f5df3:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
  1f5df7:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f5dfb:	e9 61 fd ff ff       	jmp    1f5b61 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x31>
  1f5e00:	48 8d 3d 99 3e 36 00 	lea    rdi,[rip+0x363e99]        # 559ca0 <anon.b4834912152ca6a00aac418037a14b3b.51.llvm.15879696041251167922>
  1f5e07:	ff 15 23 7f 3a 00    	call   QWORD PTR [rip+0x3a7f23]        # 59dd30 <_RNvNtCsc36rpYXAlPq_4core6option13unwrap_failed$got>
  1f5e0d:	0f 0b                	ud2
  1f5e0f:	ba 01 00 00 00       	mov    edx,0x1
  1f5e14:	b9 01 00 00 00       	mov    ecx,0x1
  1f5e19:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f5e1f:	48 89 3c 24          	mov    QWORD PTR [rsp],rdi
  1f5e23:	48 89 df             	mov    rdi,rbx
  1f5e26:	e8 d5 a7 08 00       	call   280600 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f5e2b:	48 8b 3c 24          	mov    rdi,QWORD PTR [rsp]
  1f5e2f:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f5e33:	e9 48 fd ff ff       	jmp    1f5b80 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x50>
  1f5e38:	ba 01 00 00 00       	mov    edx,0x1
  1f5e3d:	b9 01 00 00 00       	mov    ecx,0x1
  1f5e42:	41 b8 01 00 00 00    	mov    r8d,0x1
  1f5e48:	48 89 df             	mov    rdi,rbx
  1f5e4b:	e8 b0 a7 08 00       	call   280600 <_RINvNvMs2_NtCscHgRw1M2fX5_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9xBqbE1F8Gg_5bench>
  1f5e50:	48 8b 73 10          	mov    rsi,QWORD PTR [rbx+0x10]
  1f5e54:	e9 5e ff ff ff       	jmp    1f5db7 <_RINvYQINtNtCslJ40HWnHMnZ_10serde_json3ser10SerializerQINtNtCscHgRw1M2fX5_5alloc3vec3VechEENtNtCskd70WUOEvQG_10serde_core3ser10Serializer11collect_mapINtCscP45Jb88A1K_10dicom_json9DicomJsonNtNtCsf6WuL4uJU3Z_10dicom_core6header3TagEIB2o_RINtB32_11DataElementNtNtCsl1xZoL9DI0D_12dicom_object3mem16InMemDicomObjectEEINtNtNtNtCsc36rpYXAlPq_4core4iter8adapters3map3MapINtNtNtNtBV_11collections5btree3map6ValuesB30_B3M_ENCINvXs3_NtB2q_3serIB2o_RB46_ENtB1s_9Serialize9serializeB3_E0EECs9xBqbE1F8Gg_5bench+0x287>
  1f5e59:	48 8d 3d 00 40 36 00 	lea    rdi,[rip+0x364000]        # 559e60 <anon.b4834912152ca6a00aac418037a14b3b.82.llvm.15879696041251167922>
  1f5e60:	ff 15 ca 7e 3a 00    	call   QWORD PTR [rip+0x3a7eca]        # 59dd30 <_RNvNtCsc36rpYXAlPq_4core6option13unwrap_failed$got>
  1f5e66:	0f 0b                	ud2
