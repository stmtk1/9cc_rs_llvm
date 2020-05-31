define i32 @main() local_unnamed_addr #0 {
entry:
	%0 = alloca i32, align 4
	store i32 1, i32* %0, align 4
	%1 = load i32, i32* %0, align 4
	%2 = alloca i32, align 4
	store i32 0, i32* %2, align 4
	%3 = load i32, i32* %2, align 4
	%4 = alloca i32, align 4
	store i32 2, i32* %4, align 4
	%5 = load i32, i32* %4, align 4
	%6 = sub i32 %3, %5
	%7 = add i32 %1, %6
	%8 = alloca i32, align 4
	store i32 0, i32* %8, align 4
	%9 = load i32, i32* %8, align 4
	%10 = alloca i32, align 4
	store i32 3, i32* %10, align 4
	%11 = load i32, i32* %10, align 4
	%12 = sub i32 %9, %11
	%13 = mul i32 %7, %12
	%14 = alloca i32, align 4
	store i32 3, i32* %14, align 4
	%15 = load i32, i32* %14, align 4
	%16 = icmp eq i32 %13, %15
	%17 = sext i1 %16 to i32
	ret i32 %17
}
