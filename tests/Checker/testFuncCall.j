.class public /Users/urthor/projects/VC/src/test/resources/Checker/testFuncCall
.super java/lang/Object
	
	
	; standard class static initializer 
.method static <clinit>()V
	
	
	; set limits used by this method
.limit locals 0
.limit stack 50
	return
.end method
	
	; standard constructor initializer 
.method public <init>()V
.limit stack 1
.limit locals 1
	aload_0
	invokespecial java/lang/Object/<init>()V
	return
.end method
.method add(II)I
L0:
.var 0 is this L/Users/urthor/projects/VC/src/test/resources/Checker/testFuncCall; from L0 to L1
.var 1 is a I from L0 to L1
.var 2 is b I from L0 to L1
.var 3 is z I from L0 to L1
L1:
	nop
	
	; set limits used by this method
.limit locals 4
.limit stack 50
.end method
.method public static main([Ljava/lang/String;)V
L0:
.var 0 is argv [Ljava/lang/String; from L0 to L1
.var 1 is vc$ L/Users/urthor/projects/VC/src/test/resources/Checker/testFuncCall; from L0 to L1
	new /Users/urthor/projects/VC/src/test/resources/Checker/testFuncCall
	dup
	invokenonvirtual /Users/urthor/projects/VC/src/test/resources/Checker/testFuncCall/<init>()V
	astore_1
.var 2 is c I from L0 to L1
	return
L1:
	return
	
	; set limits used by this method
.limit locals 3
.limit stack 50
.end method
