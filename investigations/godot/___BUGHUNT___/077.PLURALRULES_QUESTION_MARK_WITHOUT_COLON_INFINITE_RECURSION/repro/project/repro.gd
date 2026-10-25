extends SceneTree

func _init():
	var po := 'msgid ""\nmsgstr ""\n"Plural-Forms: nplurals=2; plural=n ? 1;\\n"\n\nmsgid "x"\nmsgid_plural "xs"\nmsgstr[0] "a"\nmsgstr[1] "b"\n'
	var f := FileAccess.open("/tmp/bughunt_077.po", FileAccess.WRITE)
	f.store_string(po)
	f.close()
	print("loading .po with plural rule 'n ? 1' (a '?' with no ':')")
	var t = ResourceLoader.load("/tmp/bughunt_077.po")
	print("loaded: ", t)
	quit()
