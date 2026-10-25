extends Node
## Stress the OPCODE_OPERATOR first-run cache race (modules/gdscript/gdscript_vm.cpp:779-810).
## A barrier makes all threads call the SAME never-before-run operator expression
## at (as close as possible to) the same instant, for many distinct expressions in a row.
var barrier_mutex := Mutex.new()
var barrier_count := 0
var barrier_generation := 0
const NUM_THREADS := 32

func op_0(a, b):
	return a + b
func op_1(a, b):
	return a + b
func op_2(a, b):
	return a + b
func op_3(a, b):
	return a + b
func op_4(a, b):
	return a + b
func op_5(a, b):
	return a + b
func op_6(a, b):
	return a + b
func op_7(a, b):
	return a + b
func op_8(a, b):
	return a + b
func op_9(a, b):
	return a + b
func op_10(a, b):
	return a + b
func op_11(a, b):
	return a + b
func op_12(a, b):
	return a + b
func op_13(a, b):
	return a + b
func op_14(a, b):
	return a + b
func op_15(a, b):
	return a + b
func op_16(a, b):
	return a + b
func op_17(a, b):
	return a + b
func op_18(a, b):
	return a + b
func op_19(a, b):
	return a + b
func op_20(a, b):
	return a + b
func op_21(a, b):
	return a + b
func op_22(a, b):
	return a + b
func op_23(a, b):
	return a + b
func op_24(a, b):
	return a + b
func op_25(a, b):
	return a + b
func op_26(a, b):
	return a + b
func op_27(a, b):
	return a + b
func op_28(a, b):
	return a + b
func op_29(a, b):
	return a + b
func op_30(a, b):
	return a + b
func op_31(a, b):
	return a + b
func op_32(a, b):
	return a + b
func op_33(a, b):
	return a + b
func op_34(a, b):
	return a + b
func op_35(a, b):
	return a + b
func op_36(a, b):
	return a + b
func op_37(a, b):
	return a + b
func op_38(a, b):
	return a + b
func op_39(a, b):
	return a + b
func op_40(a, b):
	return a + b
func op_41(a, b):
	return a + b
func op_42(a, b):
	return a + b
func op_43(a, b):
	return a + b
func op_44(a, b):
	return a + b
func op_45(a, b):
	return a + b
func op_46(a, b):
	return a + b
func op_47(a, b):
	return a + b
func op_48(a, b):
	return a + b
func op_49(a, b):
	return a + b
func op_50(a, b):
	return a + b
func op_51(a, b):
	return a + b
func op_52(a, b):
	return a + b
func op_53(a, b):
	return a + b
func op_54(a, b):
	return a + b
func op_55(a, b):
	return a + b
func op_56(a, b):
	return a + b
func op_57(a, b):
	return a + b
func op_58(a, b):
	return a + b
func op_59(a, b):
	return a + b
func op_60(a, b):
	return a + b
func op_61(a, b):
	return a + b
func op_62(a, b):
	return a + b
func op_63(a, b):
	return a + b
func op_64(a, b):
	return a + b
func op_65(a, b):
	return a + b
func op_66(a, b):
	return a + b
func op_67(a, b):
	return a + b
func op_68(a, b):
	return a + b
func op_69(a, b):
	return a + b
func op_70(a, b):
	return a + b
func op_71(a, b):
	return a + b
func op_72(a, b):
	return a + b
func op_73(a, b):
	return a + b
func op_74(a, b):
	return a + b
func op_75(a, b):
	return a + b
func op_76(a, b):
	return a + b
func op_77(a, b):
	return a + b
func op_78(a, b):
	return a + b
func op_79(a, b):
	return a + b
func op_80(a, b):
	return a + b
func op_81(a, b):
	return a + b
func op_82(a, b):
	return a + b
func op_83(a, b):
	return a + b
func op_84(a, b):
	return a + b
func op_85(a, b):
	return a + b
func op_86(a, b):
	return a + b
func op_87(a, b):
	return a + b
func op_88(a, b):
	return a + b
func op_89(a, b):
	return a + b
func op_90(a, b):
	return a + b
func op_91(a, b):
	return a + b
func op_92(a, b):
	return a + b
func op_93(a, b):
	return a + b
func op_94(a, b):
	return a + b
func op_95(a, b):
	return a + b
func op_96(a, b):
	return a + b
func op_97(a, b):
	return a + b
func op_98(a, b):
	return a + b
func op_99(a, b):
	return a + b
func op_100(a, b):
	return a + b
func op_101(a, b):
	return a + b
func op_102(a, b):
	return a + b
func op_103(a, b):
	return a + b
func op_104(a, b):
	return a + b
func op_105(a, b):
	return a + b
func op_106(a, b):
	return a + b
func op_107(a, b):
	return a + b
func op_108(a, b):
	return a + b
func op_109(a, b):
	return a + b
func op_110(a, b):
	return a + b
func op_111(a, b):
	return a + b
func op_112(a, b):
	return a + b
func op_113(a, b):
	return a + b
func op_114(a, b):
	return a + b
func op_115(a, b):
	return a + b
func op_116(a, b):
	return a + b
func op_117(a, b):
	return a + b
func op_118(a, b):
	return a + b
func op_119(a, b):
	return a + b
func op_120(a, b):
	return a + b
func op_121(a, b):
	return a + b
func op_122(a, b):
	return a + b
func op_123(a, b):
	return a + b
func op_124(a, b):
	return a + b
func op_125(a, b):
	return a + b
func op_126(a, b):
	return a + b
func op_127(a, b):
	return a + b
func op_128(a, b):
	return a + b
func op_129(a, b):
	return a + b
func op_130(a, b):
	return a + b
func op_131(a, b):
	return a + b
func op_132(a, b):
	return a + b
func op_133(a, b):
	return a + b
func op_134(a, b):
	return a + b
func op_135(a, b):
	return a + b
func op_136(a, b):
	return a + b
func op_137(a, b):
	return a + b
func op_138(a, b):
	return a + b
func op_139(a, b):
	return a + b
func op_140(a, b):
	return a + b
func op_141(a, b):
	return a + b
func op_142(a, b):
	return a + b
func op_143(a, b):
	return a + b
func op_144(a, b):
	return a + b
func op_145(a, b):
	return a + b
func op_146(a, b):
	return a + b
func op_147(a, b):
	return a + b
func op_148(a, b):
	return a + b
func op_149(a, b):
	return a + b
func op_150(a, b):
	return a + b
func op_151(a, b):
	return a + b
func op_152(a, b):
	return a + b
func op_153(a, b):
	return a + b
func op_154(a, b):
	return a + b
func op_155(a, b):
	return a + b
func op_156(a, b):
	return a + b
func op_157(a, b):
	return a + b
func op_158(a, b):
	return a + b
func op_159(a, b):
	return a + b
func op_160(a, b):
	return a + b
func op_161(a, b):
	return a + b
func op_162(a, b):
	return a + b
func op_163(a, b):
	return a + b
func op_164(a, b):
	return a + b
func op_165(a, b):
	return a + b
func op_166(a, b):
	return a + b
func op_167(a, b):
	return a + b
func op_168(a, b):
	return a + b
func op_169(a, b):
	return a + b
func op_170(a, b):
	return a + b
func op_171(a, b):
	return a + b
func op_172(a, b):
	return a + b
func op_173(a, b):
	return a + b
func op_174(a, b):
	return a + b
func op_175(a, b):
	return a + b
func op_176(a, b):
	return a + b
func op_177(a, b):
	return a + b
func op_178(a, b):
	return a + b
func op_179(a, b):
	return a + b
func op_180(a, b):
	return a + b
func op_181(a, b):
	return a + b
func op_182(a, b):
	return a + b
func op_183(a, b):
	return a + b
func op_184(a, b):
	return a + b
func op_185(a, b):
	return a + b
func op_186(a, b):
	return a + b
func op_187(a, b):
	return a + b
func op_188(a, b):
	return a + b
func op_189(a, b):
	return a + b
func op_190(a, b):
	return a + b
func op_191(a, b):
	return a + b
func op_192(a, b):
	return a + b
func op_193(a, b):
	return a + b
func op_194(a, b):
	return a + b
func op_195(a, b):
	return a + b
func op_196(a, b):
	return a + b
func op_197(a, b):
	return a + b
func op_198(a, b):
	return a + b
func op_199(a, b):
	return a + b
func op_200(a, b):
	return a + b
func op_201(a, b):
	return a + b
func op_202(a, b):
	return a + b
func op_203(a, b):
	return a + b
func op_204(a, b):
	return a + b
func op_205(a, b):
	return a + b
func op_206(a, b):
	return a + b
func op_207(a, b):
	return a + b
func op_208(a, b):
	return a + b
func op_209(a, b):
	return a + b
func op_210(a, b):
	return a + b
func op_211(a, b):
	return a + b
func op_212(a, b):
	return a + b
func op_213(a, b):
	return a + b
func op_214(a, b):
	return a + b
func op_215(a, b):
	return a + b
func op_216(a, b):
	return a + b
func op_217(a, b):
	return a + b
func op_218(a, b):
	return a + b
func op_219(a, b):
	return a + b
func op_220(a, b):
	return a + b
func op_221(a, b):
	return a + b
func op_222(a, b):
	return a + b
func op_223(a, b):
	return a + b
func op_224(a, b):
	return a + b
func op_225(a, b):
	return a + b
func op_226(a, b):
	return a + b
func op_227(a, b):
	return a + b
func op_228(a, b):
	return a + b
func op_229(a, b):
	return a + b
func op_230(a, b):
	return a + b
func op_231(a, b):
	return a + b
func op_232(a, b):
	return a + b
func op_233(a, b):
	return a + b
func op_234(a, b):
	return a + b
func op_235(a, b):
	return a + b
func op_236(a, b):
	return a + b
func op_237(a, b):
	return a + b
func op_238(a, b):
	return a + b
func op_239(a, b):
	return a + b
func op_240(a, b):
	return a + b
func op_241(a, b):
	return a + b
func op_242(a, b):
	return a + b
func op_243(a, b):
	return a + b
func op_244(a, b):
	return a + b
func op_245(a, b):
	return a + b
func op_246(a, b):
	return a + b
func op_247(a, b):
	return a + b
func op_248(a, b):
	return a + b
func op_249(a, b):
	return a + b
func op_250(a, b):
	return a + b
func op_251(a, b):
	return a + b
func op_252(a, b):
	return a + b
func op_253(a, b):
	return a + b
func op_254(a, b):
	return a + b
func op_255(a, b):
	return a + b
func op_256(a, b):
	return a + b
func op_257(a, b):
	return a + b
func op_258(a, b):
	return a + b
func op_259(a, b):
	return a + b
func op_260(a, b):
	return a + b
func op_261(a, b):
	return a + b
func op_262(a, b):
	return a + b
func op_263(a, b):
	return a + b
func op_264(a, b):
	return a + b
func op_265(a, b):
	return a + b
func op_266(a, b):
	return a + b
func op_267(a, b):
	return a + b
func op_268(a, b):
	return a + b
func op_269(a, b):
	return a + b
func op_270(a, b):
	return a + b
func op_271(a, b):
	return a + b
func op_272(a, b):
	return a + b
func op_273(a, b):
	return a + b
func op_274(a, b):
	return a + b
func op_275(a, b):
	return a + b
func op_276(a, b):
	return a + b
func op_277(a, b):
	return a + b
func op_278(a, b):
	return a + b
func op_279(a, b):
	return a + b
func op_280(a, b):
	return a + b
func op_281(a, b):
	return a + b
func op_282(a, b):
	return a + b
func op_283(a, b):
	return a + b
func op_284(a, b):
	return a + b
func op_285(a, b):
	return a + b
func op_286(a, b):
	return a + b
func op_287(a, b):
	return a + b
func op_288(a, b):
	return a + b
func op_289(a, b):
	return a + b
func op_290(a, b):
	return a + b
func op_291(a, b):
	return a + b
func op_292(a, b):
	return a + b
func op_293(a, b):
	return a + b
func op_294(a, b):
	return a + b
func op_295(a, b):
	return a + b
func op_296(a, b):
	return a + b
func op_297(a, b):
	return a + b
func op_298(a, b):
	return a + b
func op_299(a, b):
	return a + b

func _wait_barrier(my_gen: int) -> void:
	barrier_mutex.lock()
	barrier_count += 1
	barrier_mutex.unlock()
	while barrier_generation == my_gen:
		pass

func _runner(_unused) -> void:
	for i in 300:
		var my_gen := barrier_generation
		_wait_barrier(my_gen)
		call("op_%d" % i, 1, 2)

func _ready() -> void:
	print("Stress-testing OPCODE_OPERATOR first-run cache with a thread barrier.")
	var threads: Array[Thread] = []
	for i in NUM_THREADS:
		var t := Thread.new()
		t.start(_runner.bind(i))
		threads.push_back(t)
	for gen in 300:
		# Wait until all threads have joined this generation's barrier, then release them.
		while true:
			barrier_mutex.lock()
			var c := barrier_count
			barrier_mutex.unlock()
			if c >= NUM_THREADS:
				break
			OS.delay_usec(50)
		barrier_mutex.lock()
		barrier_count = 0
		barrier_mutex.unlock()
		barrier_generation += 1
	for t in threads:
		t.wait_to_finish()
	print("No crash within the time limit (did not race this time).")
	get_tree().quit()
