---
title: 在编译linux内核v4.x错误undefined reference to `____ilog2_NaN'
date: 2024-12-27 14:44:00
tags: [linux]
---

> 参考：https://nuoye-blog.github.io/2020/08/21/dcb8d699/

# 问题描述

在编译linux内核v4.x的时候出现错误：

```shell
kernel/built-in.o: In function `update_wall_time':
/home/ubn/linux/linux-4.8.13/kernel/time/timekeeping.c:2079: undefined reference to `____ilog2_NaN'
Makefile:951: recipe for target 'vmlinux' failed
make: *** [vmlinux] Error 1
```

# 原因

就是代码写错了，需要打补丁。

# 解决方法

在代码根目录下新建文件patch.diff，内容如下：

```diff
diff --git a/include/linux/log2.h b/include/linux/log2.h
index ef3d4f67118c..c373295f359f 100644
--- a/include/linux/log2.h
+++ b/include/linux/log2.h
@@ -16,12 +16,6 @@
 #include <linux/bitops.h>
 
 /*
- * deal with unrepresentable constant logarithms
- */
-extern __attribute__((const, noreturn))
-int ____ilog2_NaN(void);
-
-/*
  * non-constant log of base 2 calculators
  * - the arch may override these in asm/bitops.h if they can be implemented
  *   more efficiently than using fls() and fls64()
@@ -85,7 +79,7 @@ unsigned long __rounddown_pow_of_two(unsigned long n)
 #define ilog2(n)				\
 (						\
 	__builtin_constant_p(n) ? (		\
-		(n) < 1 ? ____ilog2_NaN() :	\
+		(n) < 2 ? 0 :			\
 		(n) & (1ULL << 63) ? 63 :	\
 		(n) & (1ULL << 62) ? 62 :	\
 		(n) & (1ULL << 61) ? 61 :	\
@@ -148,10 +142,7 @@ unsigned long __rounddown_pow_of_two(unsigned long n)
 		(n) & (1ULL <<  4) ?  4 :	\
 		(n) & (1ULL <<  3) ?  3 :	\
 		(n) & (1ULL <<  2) ?  2 :	\
-		(n) & (1ULL <<  1) ?  1 :	\
-		(n) & (1ULL <<  0) ?  0 :	\
-		____ilog2_NaN()			\
-				   ) :		\
+		1 ) :				\
 	(sizeof(n) <= 4) ?			\
 	__ilog2_u32(n) :			\
 	__ilog2_u64(n)				\
diff --git a/tools/include/linux/log2.h b/tools/include/linux/log2.h
index 41446668ccce..d5677d39c1e4 100644
--- a/tools/include/linux/log2.h
+++ b/tools/include/linux/log2.h
@@ -13,12 +13,6 @@
 #define _TOOLS_LINUX_LOG2_H
 
 /*
- * deal with unrepresentable constant logarithms
- */
-extern __attribute__((const, noreturn))
-int ____ilog2_NaN(void);
-
-/*
  * non-constant log of base 2 calculators
  * - the arch may override these in asm/bitops.h if they can be implemented
  *   more efficiently than using fls() and fls64()
@@ -78,7 +72,7 @@ unsigned long __rounddown_pow_of_two(unsigned long n)
 #define ilog2(n)				\
 (						\
 	__builtin_constant_p(n) ? (		\
-		(n) < 1 ? ____ilog2_NaN() :	\
+		(n) < 2 ? 0 :			\
 		(n) & (1ULL << 63) ? 63 :	\
 		(n) & (1ULL << 62) ? 62 :	\
 		(n) & (1ULL << 61) ? 61 :	\
@@ -141,10 +135,7 @@ unsigned long __rounddown_pow_of_two(unsigned long n)
 		(n) & (1ULL <<  4) ?  4 :	\
 		(n) & (1ULL <<  3) ?  3 :	\
 		(n) & (1ULL <<  2) ?  2 :	\
-		(n) & (1ULL <<  1) ?  1 :	\
-		(n) & (1ULL <<  0) ?  0 :	\
-		____ilog2_NaN()			\
-				   ) :		\
+		1 ) :				\
 	(sizeof(n) <= 4) ?			\
 	__ilog2_u32(n) :			\
 	__ilog2_u64(n)				\

```

输入以下命令应用补丁：

```bash
patch -i patch.diff
```

输入之后，终端会让你输入文件路径，会输入两次，分别输入`include/linux/log2.h`
和`tools/include/linux/log2.h`即可。

打好补丁重新编译，就可以发现问题解决啦~