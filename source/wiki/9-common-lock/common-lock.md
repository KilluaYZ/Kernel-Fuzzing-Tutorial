# 线程同步的几种方式

> 该教程是<https://www.cnblogs.com/sherlock-lin/p/14538083.html这个文章的整理和补充（不是搬运，侵权必删）>

在多线程编程过程中，有一个绕不过的问题就是如何合理地进行线程间同步，因为多个线程读写同一个资源的时候，会导致资源的冲突。锁的不合理使用，会导致资源利用率低、死锁、不可重复读等问题。最近在阅读 qemu 和 linux 内核源码的时候，各种锁也是满天飞，所以在此整理一下常用的线程同步机制。这里我们使用 pthread 库作为示例，其他库类似，概念是一样的。

## 互斥锁（Mutex）

互斥锁是最常见的线程同步方式，它是一种特殊的变量，有 lock 和 unlock 两种状态。一旦获取就会上锁，且只能由该线程解锁，期间其他线程无法获取。

示例代码：

```c
pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;

void *thread_func(void *arg) {
    pthread_mutex_lock(&mutex);
    // 临界区代码，只有一个线程能执行
    pthread_mutex_unlock(&mutex);
    return NULL;
}
```

- 优点
  - 简单易用，适用于简单的同步场景。
- 缺点
  - 重复锁定和解锁，每次都会检查共享数据结构，浪费时间和资源
  - 繁忙查询的效率非常低

## 条件变量（Condition）

互斥锁简单，但是存在一个问题，就是线程在等待条件变量的时候，会一直占用一个互斥锁，这是非常浪费的。条件变量可以让线程在等待条件变量的时候，释放掉锁，让其他线程可以继续执行。

具体来说，当线程在等待某些满足条件时线程会释放占有的互斥锁，进入睡眠状态，一旦条件满足，线程会被唤醒，然后重新占有互斥锁，继续执行。

```c++
#include <iostream>
#include <unistd.h>

using namespace std;

pthread_cond_t qready = PTHREAD_COND_INITIALIZER;//初始构造条件变量
pthread_mutex_t qlock = PTHREAD_MUTEX_INITIALIZER;//初始构造锁

int x = 10, y = 20;

void *func1(void *arg) {
  cout << "func1" << endl;
  pthread_mutex_lock(&qlock);
  while (x < y) {
    // 如果x < y，那么等待qready条件变量
    // 释放锁，进入睡眠状态，等待条件变量被唤醒
    // 一旦条件变量被唤醒，重新获取锁，继续执行
    pthread_cond_wait(&qready, &qlock);
  }
  pthread_mutex_unlock(&qlock);
  cout << "func1 end" << endl;
}

void *func2(void *arg) {
  cout << "func2" << endl;
  // 如果没有条件变量，那么func2在这里不会获取到锁
  // 因为func1已经获取到了锁
  pthread_mutex_lock(&qlock);
  // func2修改了x和y的值
  x = 20, y = 10;
  cout << "x & y changed" << endl;
  pthread_mutex_unlock(&qlock);
  if (x > y) {
    // 如果x > y，那么唤醒qready条件变量
    // 唤醒条件变量会唤醒等待的线程
    // 被唤醒的线程会重新获取锁，继续执行
    pthread_cond_signal(&qready);
  }
  cout << "func2 end" << endl;
}

int main() {
  pthread_t tid1, tid2;
  int res = pthread_create(&tid1, nullptr, func1, nullptr);
  if (res) {
    cout << "pthread 1 create error" << endl;
    return res;
  }
  sleep(2);
  res = pthread_create(&tid2, nullptr, func2, nullptr);
  if (res) {
    cout << "pthread 2 create error" << endl;
    return res;
  }
  sleep(10);
  return 0;
}

```

条件变量需要和互斥锁一起使用，因为条件变量是在互斥锁的基础上实现的。如果没有条件变量，那么 func2 要等待 func1 释放锁之后，才能继续执行，而使用了条件变量之后，func1 发现条件不满足，会先将锁释放，让 func2 执行，然后 func2 修改了 x 和 y 的值，然后唤醒条件变量，func1 被唤醒，重新获取锁，继续执行。

pthread_cond_signal 函数只会唤醒一个等待的线程，如果有多个线程阻塞时，会根据优先级高低和入队顺序决定消耗哪一个线程。而 pthread_cond_broadcast 函数会唤醒所有等待的线程。

- 优点
  - 提高了互斥锁的使用效率

## 读写锁（Reader-Writer Lock）

读写锁也称为共享-独占锁，它允许多个线程同时读取共享数据，但只允许一个线程写入共享数据。
读写锁有以下几个状态：

- 读锁定状态（Read Lock）：多个线程可以同时持有读锁，但是不能持有写锁，如果有线程想要请求写锁，则会被阻塞。
- 写锁定状态（Write Lock）：只有一个线程可以持有写锁，其他线程不能持有读锁或写锁。
- 如果处于读锁定状态，且有一个的线程请求写锁，那么另外的线程想要请求读锁会被阻塞，避免了读模式下长期占用，导致写锁长期被阻塞。

示例代码：

```c++
#include <stdio.h>
#include <pthread.h>

// 共享资源
int shared_data = 0;
pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER;

// 读线程函数
void *reader(void *arg) {
    int id = *(int *)arg;
    pthread_rwlock_rdlock(&rwlock);
    printf("Reader %d: read shared_data = %d\n", id, shared_data);
    pthread_rwlock_unlock(&rwlock);
    return NULL;
}

// 写线程函数
void *writer(void *arg) {
    int id = *(int *)arg;
    pthread_rwlock_wrlock(&rwlock);
    shared_data++;
    printf("Writer %d: updated shared_data to %d\n", id, shared_data);
    pthread_rwlock_unlock(&rwlock);
    return NULL;
}

int main() {
    pthread_t threads[5];
    int ids[5] = {1, 2, 3, 4, 5};

    // 创建3个读线程和2个写线程
    pthread_create(&threads[0], NULL, reader, &ids[0]);
    pthread_create(&threads[1], NULL, reader, &ids[1]);
    pthread_create(&threads[2], NULL, writer, &ids[2]);
    pthread_create(&threads[3], NULL, reader, &ids[3]);
    pthread_create(&threads[4], NULL, writer, &ids[4]);

    // 等待所有线程完成
    for (int i = 0; i < 5; i++) {
        pthread_join(threads[i], NULL);
    }

    pthread_rwlock_destroy(&rwlock);
    return 0;
}
```

运行结果如下：

```
Reader 1: read shared_data = 0
Reader 2: read shared_data = 0
Writer 3: updated shared_data to 1
Reader 4: read shared_data = 1
Writer 5: updated shared_data to 2
```

## 信号量（Semphore）

互斥锁只允许一个线程进入临界区，但是有时候，我们的业务场景允许多个线程进入临界区，这时候就需要使用信号量。信号量是一个计数器，它可以用来控制多个线程对共享资源的访问。以下的示例代码模拟了营业厅两个窗口处理业务的场景，有 10 个客户进入营业厅，当发现窗口满则等待，当有可用窗口时则接受服务：

```c++
#include <stdio.h>
#include <pthread.h>
#include <unistd.h>
#include <semaphore.h>

#define CUSTOMER_NUM 10

// 定义一个信号量
sem_t sem;

void *get_server(void *thread_id) {
    int cusotmer_id = *((int *) thread_id);
    // 如果信号量的值为0，则会阻塞当前线程，直到信号量的值大于0
    if (sem_wait(&sem) == 0) {
        printf("customer %d receive server ...\n", cusotmer_id);
        // 当有一个线程执行完之后，信号量的值会加1
        // 这样其他线程就可以继续执行了
        sem_post(&sem);
    }
}

int main() {
    // 初始化信号量，第二个参数是0，表示初始值为2，即最多有2个线程可以同时访问共享资源
    sem_init(&sem, 0, 2);
    pthread_t customers[CUSTOMER_NUM];
    int customer_id[CUSTOMER_NUM];//用户id

    for (int index = 0; index < CUSTOMER_NUM; index++) {
        customer_id[index] = index;
        printf("customer %d arrived\n", customer_id[index]);
        // 创建CUSTOMER_NUM个线程，模拟多个客户进入营业厅
        int res = pthread_create(&customers[index], nullptr, get_server, &customer_id[index]);
        if (res) {
            printf("create thread error\n");
            return res;
        }
    }
    // 等待所有线程执行完毕
    for (int index = 0; index < CUSTOMER_NUM; index++) {
        pthread_join(customers[index], nullptr);
    }
    sem_destroy(&sem);

    return 0;
}

```

执行结果如下：

```
customer 0 arrived
customer 1 arrived
customer 2 arrived
customer 1 receive server ...
customer 2 receive server ...
customer 3 arrived
customer 4 arrived
customer 0 receive server ...
customer 3 receive server ...
customer 4 receive server ...
customer 5 arrived
customer 6 arrived
customer 5 receive server ...
customer 6 receive server ...
customer 7 arrived
customer 8 arrived
customer 9 arrived
customer 8 receive server ...
customer 7 receive server ...
customer 9 receive server ...
```
