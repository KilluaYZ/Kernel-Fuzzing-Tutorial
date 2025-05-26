# 基础数据结构介绍——XArray

> 参考资料：
>
> https://linuxkernel.org.cn/doc/html/latest/core-api/xarray.html
>
> https://zhuanlan.zhihu.com/p/587184623
>
> https://www.cnblogs.com/Linux-tech/p/12961281.html

## XArray简介

本文将会介绍Linux内核中常用的数据结构之一`XArray`。

`XArray`的设计是为了填补`radix tree`的不足。其一是`radix tree`并不是教科书中经典的树结构，它更像是一个能够自动增长的数组；其二是`raidx tree`需要用户自己加锁，这很容易导致安全漏洞。

`XArray`相比`radix tree`有以下修改：

- XArray默认自己处理了锁，简化了使用。
- 基数树的“预加载”机制允许用户获取锁之前先预先分配内存，这个机制在XArray中被取消了，它太复杂又没有太多实际价值。
- XArray API被分为两部分，普通API和高级API。后者给用户更多可控性，比如用户可以显式管理锁。API可以用于不同的场景，满足不同的需求。比如Page Cache就可以用XArray。普通API完全在高级API的基础上实现，所以普通API也是高级API的使用范例。

## 数据结构

我们以最新的v6.13版本内核为例进行介绍。

xarray这个数据结构主要有两个结构体，xa_node和xarray

### xa_node

```c
/*
 * @count is the count of every non-NULL element in the ->slots array
 * whether that is a value entry, a retry entry, a user pointer,
 * a sibling entry or a pointer to the next level of the tree.
 * @nr_values is the count of every element in ->slots which is
 * either a value entry or a sibling of a value entry.
 */
struct xa_node {
	unsigned char	shift;		/* Bits remaining in each slot */
	unsigned char	offset;		/* Slot offset in parent */
	unsigned char	count;		/* Total entry count */
	unsigned char	nr_values;	/* Value entry count */
	struct xa_node __rcu *parent;	/* NULL at top of tree */
	struct xarray	*array;		/* The array we belong to */
	union {
		struct list_head private_list;	/* For tree user */
		struct rcu_head	rcu_head;	/* Used when freeing node */
	};
	void __rcu	*slots[XA_CHUNK_SIZE];
	union {
		unsigned long	tags[XA_MAX_MARKS][XA_MARK_LONGS];
		unsigned long	marks[XA_MAX_MARKS][XA_MARK_LONGS];
	};
};
```

成员变量：

|成员|含义|
|--|--|
|shift|表示每个槽（slot）中剩余的位数。在XArray中，每个节点可以有多个槽，shift用于确定在这些槽中如何分配位来存储键值。|
|offset|xa_node在父节点的slots数组中的偏移|
|count|xa_node有多少个slots已经被使用|
|nr_values|xa_node有多少个slots存储了Value Entry|
|parent|指向该xa_node的父节点|
|array|array成员指向该xa_node所属的xarray|
|slots|slots是个指针数组，该数组既可以存储下一级的节点, 也可以用于存储即将插入的对象指针|

### xarray

```c
/**
 * struct xarray - The anchor of the XArray.
 * @xa_lock: Lock that protects the contents of the XArray.
 *
 * To use the xarray, define it statically or embed it in your data structure.
 * It is a very small data structure, so it does not usually make sense to
 * allocate it separately and keep a pointer to it in your data structure.
 *
 * You may use the xa_lock to protect your own data structures as well.
 */
/*
 * If all of the entries in the array are NULL, @xa_head is a NULL pointer.
 * If the only non-NULL entry in the array is at index 0, @xa_head is that
 * entry.  If any other entry in the array is non-NULL, @xa_head points
 * to an @xa_node.
 */
struct xarray {
	spinlock_t	xa_lock;
/* private: The rest of the data structure is not to be used directly. */
	gfp_t		xa_flags;
	void __rcu *	xa_head;
};
```

xarray这个数据结构的`xa_head`就指向了第一个`xa_node`。下面我们使用一个具体的例子描述一下这个数据结构。

