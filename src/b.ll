define i32 @main() local_unnamed_addr #0 {
entry:
	%0 = alloca i32, align 4
	store i32 9, i32* %0, align 4
	%1 = load i32, i32* %0, align 4
	%2 = alloca i32, align 4
	store i32 10, i32* %2, align 4
	%3 = load i32, i32* %2, align 4
	%4 = mul i32 %1, %3
	%5 = alloca i32, align 4
	store i32 100, i32* %5, align 4
	%6 = load i32, i32* %5, align 4
	%7 = add i32 %4, %6
	ret i32 %7
}
