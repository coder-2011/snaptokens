__RINvNtNtCs3fcDmoiW7ZR_10snaptokens6models3bpe17decode_native_vecINtNtNtCsdBPgS5Ywgey_7bincode2de7decoder11DecoderImplNtNtB15_4read11SliceReaderINtNtB17_6config13ConfigurationNtB2k_12LittleEndianNtB2k_6FixintINtB2k_5LimitKj20000000_EEuEmKj4_NvMs6_NtCskumHb0IaX0X_4core3numm13from_le_bytesEB6_:
Lfunc_begin148:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception148
	sub	sp, sp, #128
	.cfi_def_cfa_offset 128
	stp	x28, x27, [sp, #32]
	stp	x26, x25, [sp, #48]
	stp	x24, x23, [sp, #64]
	stp	x22, x21, [sp, #80]
	stp	x20, x19, [sp, #96]
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
	.cfi_remember_state
	mov	x19, x0
	ldr	x9, [x1, #16]
	cmn	x9, #9
	b.hi	LBB258_2
	mov	x20, x1
	add	x8, x9, #8
	str	x8, [x1, #16]
	mov	w10, #536870904
	cmp	x9, x10
	b.ls	LBB258_3
LBB258_2:
	mov	w8, #1
	b	LBB258_5
LBB258_3:
	ldr	x28, [x20, #8]
	cmp	x28, #7
	b.hi	LBB258_7
	mov	w8, #0
	mov	w9, #8
	sub	x9, x9, x28
LBB258_5:
	strb	w8, [x19]
	str	x9, [x19, #8]
LBB258_6:
	.cfi_def_cfa wsp, 128
	ldp	x29, x30, [sp, #112]
	ldp	x20, x19, [sp, #96]
	ldp	x22, x21, [sp, #80]
	ldp	x24, x23, [sp, #64]
	ldp	x26, x25, [sp, #48]
	ldp	x28, x27, [sp, #32]
	add	sp, sp, #128
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
LBB258_7:
	.cfi_restore_state
	ldr	x26, [x20]
	mov	x25, x26
	ldr	x24, [x25], #8
	sub	x23, x28, #8
	stp	x25, x23, [x20]
	lsr	x9, x24, #62
	cbnz	x9, LBB258_10
	lsl	x21, x24, #2
	adds	x27, x8, x21
	b.hs	LBB258_10
	str	x27, [x20, #16]
	mov	w8, #536870912
	cmp	x27, x8
	b.ls	LBB258_11
LBB258_10:
	mov	w8, #1
	strb	w8, [x19]
	b	LBB258_6
LBB258_11:
	lsr	x8, x24, #61
	cmp	x21, x23
	b.ls	LBB258_17
	cbnz	x8, LBB258_18
	cbz	x24, LBB258_26
	bl	__RNvCs9hJ03s5DiqP_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w22, #4
	mov	x0, x21
	mov	w1, #4
	bl	__RNvCs9hJ03s5DiqP_7___rustc12___rust_alloc
	cbz	x0, LBB258_19
	stp	x24, x0, [sp, #8]
	str	xzr, [sp, #24]
	str	x27, [x20, #16]
	cmp	x23, #4
	b.hs	LBB258_31
	b	LBB258_37
LBB258_17:
	cbz	x8, LBB258_20
LBB258_18:
	mov	x22, #0
LBB258_19:
	mov	x0, x22
	mov	x1, x21
	bl	__RNvNtCs6SjEax68zxx_5alloc7raw_vec12handle_error
LBB258_20:
	cbz	x24, LBB258_28
	bl	__RNvCs9hJ03s5DiqP_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w22, #4
	mov	x0, x21
	mov	w1, #4
	bl	__RNvCs9hJ03s5DiqP_7___rustc12___rust_alloc
	cbz	x0, LBB258_19
	sub	x8, x21, #4
	cmp	x8, #60
	b.lo	LBB258_24
	sub	x9, x0, x26
	sub	x9, x9, #8
	cmp	x9, #64
	b.hs	LBB258_39
LBB258_24:
	mov	x8, #0
	mov	x9, x21
	mov	x10, x25
LBB258_25:
	ldr	w11, [x10], #4
	str	w11, [x0, x8, lsl #2]
	add	x8, x8, #1
	subs	x9, x9, #4
	b.ne	LBB258_25
	b	LBB258_29
LBB258_26:
	mov	w8, #4
	stp	xzr, x8, [sp, #8]
	str	xzr, [sp, #24]
LBB258_27:
	ldur	q0, [sp, #8]
	stur	q0, [x19, #8]
	ldr	x8, [sp, #24]
	b	LBB258_30
LBB258_28:
	mov	x8, #0
	mov	w0, #4
LBB258_29:
	add	x9, x25, x21
	subs	x10, x23, x21
	csel	x10, xzr, x10, lo
	stp	x9, x10, [x20]
	stp	x24, x0, [x19, #8]
LBB258_30:
	str	x8, [x19, #24]
	mov	w8, #255
	strb	w8, [x19]
	b	LBB258_6
LBB258_31:
	mov	x25, #0
	sub	x22, x24, #1
	sub	x21, x28, #12
	mov	w23, #12
LBB258_32:
	add	x8, x26, x25, lsl #2
	add	x9, x26, x23
	ldr	w24, [x8, #8]
	stp	x9, x21, [x20]
	ldr	x8, [sp, #8]
	cmp	x25, x8
	b.ne	LBB258_34
Ltmp1504:
	add	x0, sp, #8
	bl	__RNvMs3_NtCs6SjEax68zxx_5alloc7raw_vecINtB5_6RawVecmE8grow_oneCs4P2pF1ObvmT_7bit_vec
Ltmp1505:
LBB258_34:
	ldr	x8, [sp, #16]
	str	w24, [x8, x25, lsl #2]
	add	x8, x25, #1
	str	x8, [sp, #24]
	cmp	x22, x25
	b.eq	LBB258_27
	str	x27, [x20, #16]
	add	x23, x23, #4
	sub	x21, x21, #4
	mov	x25, x8
	cmn	x21, #4
	b.lo	LBB258_32
	add	x23, x21, #4
LBB258_37:
	mov	w8, #4
	sub	x8, x8, x23
	strb	wzr, [x19]
	str	w24, [x19, #4]
	str	x8, [x19, #8]
	ldr	x8, [sp, #8]
	cbz	x8, LBB258_6
	ldr	x0, [sp, #16]
	lsl	x1, x8, #2
	mov	w2, #4
	bl	__RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc
	b	LBB258_6
LBB258_39:
	lsr	x8, x8, #2
	add	x11, x8, #1
	and	x8, x11, #0x7ffffffffffffff0
	sub	x9, x24, x8
	lsl	x9, x9, #2
	add	x10, x25, x8, lsl #2
	add	x12, x0, #32
	add	x13, x26, #40
	and	x14, x11, #0x7ffffffffffffff0
LBB258_40:
	ldp	q0, q1, [x13, #-32]
	ldp	q2, q3, [x13], #64
	stp	q0, q1, [x12, #-32]
	stp	q2, q3, [x12], #64
	subs	x14, x14, #16
	b.ne	LBB258_40
	cmp	x11, x8
	b.ne	LBB258_25
	b	LBB258_29
LBB258_42:
Ltmp1506:
	mov	x19, x0
	ldr	x8, [sp, #8]
	cbz	x8, LBB258_44
	ldr	x0, [sp, #16]
	lsl	x1, x8, #2
	mov	w2, #4
	bl	__RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc
LBB258_44:
	mov	x0, x19
	bl	__Unwind_Resume
Lfunc_end148:
	.cfi_endproc