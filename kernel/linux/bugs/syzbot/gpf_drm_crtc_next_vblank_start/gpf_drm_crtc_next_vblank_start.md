---
title: syzbot上general protection fault in drm_crtc_next_vblank_start
date: 2024-12-27 14:44:00
tags: [linux]
---

地址：https://syzkaller.appspot.com/bug?extid=54280c5aea19802490b5

fix commit:  6f1ccbf07453 drm/vblank: [Fix for drivers that do not drm_vblank_init](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=6f1ccbf07453eb1ee6bb24d6b531b88dd44ad229) 

introduce commit: d39e48ca80c0960b039cb38633957f0040f63e1a [drm/atomic-helper: Set fence deadline for vblank](https://git.kernel.org/pub/scm/linux/kernel/git/next/linux-next.git/commit/?id=d39e48ca80c0960b039cb38633957f0040f63e1a)

影响范围：6.3-rc2 到 6.3-rc4
