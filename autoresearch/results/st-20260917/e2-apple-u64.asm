__RINvNtNtCs3fcDmoiW7ZR_10snaptokens6models3bpe17decode_native_vecINtNtNtCsdBPgS5Ywgey_7bincode2de7decoder11DecoderImplNtNtB15_4read11SliceReaderINtNtB17_6config13ConfigurationNtB2k_12LittleEndianNtB2k_6FixintINtB2k_5LimitKj20000000_EEuEyKj8_NvMs7_NtCskumHb0IaX0X_4core3numy13from_le_bytesEB6_:
Lfunc_begin149:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception149
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
	b.hi	LBB259_2
	mov	x20, x1
	add	x8, x9, #8
	str	x8, [x1, #16]
	mov	w10, #536870904
	cmp	x9, x10
	b.ls	LBB259_3
LBB259_2:
	mov	w8, #1
	b	LBB259_5
LBB259_3:
	ldr	x28, [x20, #8]
	cmp	x28, #7
	b.hi	LBB259_7
	mov	w8, #0
	mov	w9, #8
	sub	x9, x9, x28
LBB259_5:
	strb	w8, [x19]
	str	x9, [x19, #8]
LBB259_6:
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
LBB259_7:
	.cfi_restore_state
	ldr	x26, [x20]
	mov	x24, x26
	ldr	x23, [x24], #8
	sub	x25, x28, #8
	stp	x24, x25, [x20]
	lsr	x9, x23, #61
	cbnz	x9, LBB259_10
	lsl	x21, x23, #3
	adds	x27, x8, x21
	b.hs	LBB259_10
	str	x27, [x20, #16]
	mov	w8, #536870912
	cmp	x27, x8
	b.ls	LBB259_11
LBB259_10:
	mov	w8, #1
	strb	w8, [x19]
	b	LBB259_6
LBB259_11:
	lsr	x8, x23, #60
	cmp	x21, x25
	b.ls	LBB259_21
	cbnz	x8, LBB259_22
	cbz	x23, LBB259_30
	bl	__RNvCs9hJ03s5DiqP_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w22, #8
	mov	x0, x21
	mov	w1, #8
	bl	__RNvCs9hJ03s5DiqP_7___rustc12___rust_alloc
	cbz	x0, LBB259_23
	mov	x21, #0
	stp	x23, x0, [sp, #8]
	sub	x22, x28, #16
	mov	w24, #16
	str	xzr, [sp, #24]
	b	LBB259_17
LBB259_16:
	str	x25, [x0, x21, lsl #3]
	add	x21, x21, #1
	str	x21, [sp, #24]
	sub	x22, x22, #8
	add	x24, x24, #8
	cmp	x23, x21
	b.eq	LBB259_31
LBB259_17:
	str	x27, [x20, #16]
	cmn	x22, #8
	b.hs	LBB259_35
	add	x8, x26, x21, lsl #3
	add	x9, x26, x24
	ldr	x25, [x8, #8]
	stp	x9, x22, [x20]
	ldr	x8, [sp, #8]
	cmp	x21, x8
	b.ne	LBB259_16
Ltmp1507:
	add	x0, sp, #8
	bl	__RNvMs3_NtCs6SjEax68zxx_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs3fcDmoiW7ZR_10snaptokens
Ltmp1508:
	ldr	x0, [sp, #16]
	b	LBB259_16
LBB259_21:
	cbz	x8, LBB259_24
LBB259_22:
	mov	x22, #0
LBB259_23:
	mov	x0, x22
	mov	x1, x21
	bl	__RNvNtCs6SjEax68zxx_5alloc7raw_vec12handle_error
LBB259_24:
	cbz	x23, LBB259_32
	bl	__RNvCs9hJ03s5DiqP_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w22, #8
	mov	x0, x21
	mov	w1, #8
	bl	__RNvCs9hJ03s5DiqP_7___rustc12___rust_alloc
	cbz	x0, LBB259_23
	sub	x8, x21, #8
	cmp	x8, #56
	b.lo	LBB259_28
	sub	x9, x0, x26
	sub	x9, x9, #8
	cmp	x9, #64
	b.hs	LBB259_37
LBB259_28:
	mov	x8, #0
	mov	x9, x21
	mov	x10, x24
LBB259_29:
	ldr	x11, [x10], #8
	str	x11, [x0, x8, lsl #3]
	add	x8, x8, #1
	subs	x9, x9, #8
	b.ne	LBB259_29
	b	LBB259_33
LBB259_30:
	mov	w8, #8
	stp	xzr, x8, [sp, #8]
	str	xzr, [sp, #24]
LBB259_31:
	ldur	q0, [sp, #8]
	stur	q0, [x19, #8]
	ldr	x8, [sp, #24]
	b	LBB259_34
LBB259_32:
	mov	x8, #0
	mov	w0, #8
LBB259_33:
	add	x9, x24, x21
	subs	x10, x25, x21
	csel	x10, xzr, x10, lo
	stp	x9, x10, [x20]
	stp	x23, x0, [x19, #8]
LBB259_34:
	str	x8, [x19, #24]
	mov	w8, #255
	strb	w8, [x19]
	b	LBB259_6
LBB259_35:
	neg	x8, x22
	strb	wzr, [x19]
	str	x8, [x19, #8]
	ldr	x8, [sp, #8]
	cbz	x8, LBB259_6
	ldr	x0, [sp, #16]
	lsl	x1, x8, #3
	mov	w2, #8
	bl	__RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc
	b	LBB259_6
LBB259_37:
	lsr	x8, x8, #3
	add	x11, x8, #1
	and	x8, x11, #0x3ffffffffffffff8
	sub	x9, x23, x8
	lsl	x9, x9, #3
	add	x10, x24, x8, lsl #3
	add	x12, x0, #32
	add	x13, x26, #40
	and	x14, x11, #0x3ffffffffffffff8
LBB259_38:
	ldp	q0, q1, [x13, #-32]
	ldp	q2, q3, [x13], #64
	stp	q0, q1, [x12, #-32]
	stp	q2, q3, [x12], #64
	subs	x14, x14, #8
	b.ne	LBB259_38
	cmp	x11, x8
	b.ne	LBB259_29
	b	LBB259_33
LBB259_40:
Ltmp1509:
	mov	x19, x0
	ldr	x8, [sp, #8]
	cbz	x8, LBB259_42
	ldr	x0, [sp, #16]
	lsl	x1, x8, #3
	mov	w2, #8
	bl	__RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc
LBB259_42:
	mov	x0, x19
	bl	__Unwind_Resume
Lfunc_end149:
	.cfi_endproc