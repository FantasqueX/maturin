initSidebarItems({"fn":[["check_thread_blocked","检查线程是否在等待某种资源"],["set_waiter_for_thread","设置一个线程等待某个事件 在切换线程进入时会检查是否触发 waiter"],["wake_thread","唤醒某个线程，如 waiter 存在，则返回 true(无论是否之前就被唤醒)。 注意，这不是线程被唤醒的唯一方式。如果在除了 WAITING_BOARD 之外的地方也保存了对应的 Arc 那么 waiter 也可能在其他地方被设置为 woken"]],"static":[["WAITING_BOARD","从 tid 获取信号相关信息"]]});